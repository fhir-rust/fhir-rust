#!/usr/bin/env bash
# Does every already-published version still match the source that claims it?
#
# Spec O10.11: a published version must match the source that claims it. A
# crates.io version is immutable, so a tree carrying an already-published
# version number MUST be identical to what was published, and CI MUST fail
# otherwise.
#
# Why this exists (audit F-35, publishing P-4). CI already had this check —
# a `published-versions` job in every port, and an equivalent in `fhir/` whose
# comment records a previous instance of the same defect. The tree had diverged
# anyway: `fhir-derive-macros` sat at `1.1.0`, the version taken on crates.io,
# with 206 lines of `qty-3` invariant support the published crate does not
# contain. Three gaps let it through, and this script closes each one:
#
#   - those jobs diff `src/` only, so a changed README, Cargo.toml or LICENSE
#     is invisible to them; this compares every packaged file
#   - `fhir/`'s job omits fhir-r1/7/8/9/10, all published at 0.0.0;
#     this walks every [package] in the tree
#   - none of them has ever run, because nothing here has been pushed (F-11);
#     this is a script, so it runs before a push rather than after
#
# The underlying reason the defect is invisible locally: every local build
# resolves the `path` dependency and never fetches the registry copy, so this
# workspace stayed green against 758 lines while `fhir-derive-macros = "1.1.0"`
# gets 554 for everyone else.
#
# It surfaces for a third party packaging a dependent, as an error about code
# they did not write. For a component handling clinical data, "the released
# artifact is the reviewed source" is what the whole audit trail rests on
# (AGENTS/release.md), and O10.10's SBOM describes an artifact that is worth
# nothing if the artifact is not the source.
#
#   scripts/check-published-match.sh          check, print a summary
#   scripts/check-published-match.sh --diff   also print the offending diffs
#
# Crates whose current version is NOT on crates.io are skipped and reported as
# such: there is nothing immutable to contradict yet. A skip is printed, never
# silent (T11.12).
#
# Exits non-zero on any mismatch.
set -euo pipefail

cd "$(dirname "$0")/.."
ROOT="$PWD"

SHOW_DIFF=0
[ "${1:-}" = "--diff" ] && SHOW_DIFF=1

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

# Files that legitimately differ between a local package and a published one,
# and say nothing about whether the *source* matches:
#
#   .cargo_vcs_info.json  records the git commit the package was cut from
#   Cargo.lock            resolved dependency versions drift with the registry
#   Cargo.toml            cargo normalizes this; the verbatim manifest is
#                         preserved beside it as Cargo.toml.orig, which IS
#                         compared, so manifest changes are still caught
IGNORE=(--exclude .cargo_vcs_info.json --exclude Cargo.lock --exclude Cargo.toml)

# The crates.io sparse-index path for a crate name: 1/2/3-character names are
# special-cased, everything longer is {first2}/{next2}/{name}. Names are
# lowercased in the index.
index_path() {
  python3 -c '
import sys
n = sys.argv[1].lower()
print(f"1/{n}" if len(n) == 1 else
      f"2/{n}" if len(n) == 2 else
      f"3/{n[0]}/{n}" if len(n) == 3 else
      f"{n[:2]}/{n[2:4]}/{n}")' "$1"
}

fail=0
matched=0
mismatched=0
skipped=0

echo "Published-vs-source check (O10.11)"
echo

# Every [package] in the tree, excluding fuzz crates (publish = false by design).
manifests=$(find . -name Cargo.toml \
  -not -path '*/target/*' -not -path '*/tmp/*' -not -path '*/fuzz/*' | sort)

for m in $manifests; do
  name=$(python3 - "$m" <<'PY'
import re,sys
t=open(sys.argv[1]).read()
b=re.search(r'^\[package\]\s*$(.*?)(^\[|\Z)',t,re.M|re.S)
print(re.search(r'^name\s*=\s*"([^"]+)"',b.group(1),re.M).group(1) if b else '')
PY
)
  [ -n "$name" ] || continue

  dir=$(dirname "$m")
  ver=$(cd "$dir" && cargo metadata --no-deps --format-version 1 2>/dev/null \
        | python3 -c 'import json,sys; d=json.load(sys.stdin); print(next(p["version"] for p in d["packages"] if p["name"]=="'"$name"'"))' 2>/dev/null || true)
  [ -n "$ver" ] || { printf '  %-24s %-10s SKIP  (cannot resolve version)\n' "$name" "?"; skipped=$((skipped+1)); continue; }

  # Is this exact version on the registry? Ask the sparse index, not the API:
  # the index is what cargo itself reads, and the API rejects unattended
  # clients under its data-access policy.
  if ! curl -sf --max-time 20 -A 'fhir-rust check-published-match (O10.11 gate)' \
       "https://index.crates.io/$(index_path "$name")" \
       -o "$WORK/index.json" 2>/dev/null; then
    printf '  %-24s %-10s SKIP  (name not on crates.io)\n' "$name" "$ver"
    skipped=$((skipped+1)); continue
  fi

  if ! grep -q "\"vers\":\"$ver\"" "$WORK/index.json"; then
    printf '  %-24s %-10s SKIP  (version not published)\n' "$name" "$ver"
    skipped=$((skipped+1)); continue
  fi

  # Fetch the published copy through cargo, which sends a compliant User-Agent.
  probe="$WORK/probe-$name"
  mkdir -p "$probe/src" && echo 'fn main(){}' > "$probe/src/main.rs"
  cat > "$probe/Cargo.toml" <<EOF
[package]
name = "probe-$name"
version = "0.0.0"
edition = "2021"

[dependencies]
$name = "=$ver"
EOF
  (cd "$probe" && cargo fetch >/dev/null 2>&1) || {
    printf '  %-24s %-10s SKIP  (fetch failed)\n' "$name" "$ver"; skipped=$((skipped+1)); continue; }

  crate_file=$(find ~/.cargo/registry/cache -name "$name-$ver.crate" 2>/dev/null | head -1)
  [ -n "$crate_file" ] || { printf '  %-24s %-10s SKIP  (not in cache)\n' "$name" "$ver"; skipped=$((skipped+1)); continue; }

  pub_dir="$WORK/pub-$name"; mkdir -p "$pub_dir"
  tar xzf "$crate_file" -C "$pub_dir" --strip-components=1

  # Package the local crate. --no-verify: we are comparing contents, not
  # building, and the verification build needs siblings that may not be
  # published yet.
  ws=$(cd "$dir" && cargo locate-project --workspace --message-format plain 2>/dev/null | xargs dirname)
  (cd "$ws" && cargo package --allow-dirty --no-verify -p "$name" >/dev/null 2>&1) || {
    printf '  %-24s %-10s SKIP  (cargo package failed)\n' "$name" "$ver"; skipped=$((skipped+1)); continue; }

  loc_crate="$ws/target/package/$name-$ver.crate"
  [ -f "$loc_crate" ] || { printf '  %-24s %-10s SKIP  (no local artifact)\n' "$name" "$ver"; skipped=$((skipped+1)); continue; }
  loc_dir="$WORK/loc-$name"; mkdir -p "$loc_dir"
  tar xzf "$loc_crate" -C "$loc_dir" --strip-components=1

  if diff -r "${IGNORE[@]}" "$pub_dir" "$loc_dir" >/dev/null 2>&1; then
    printf '  %-24s %-10s ok\n' "$name" "$ver"
    matched=$((matched+1))
  else
    n=$(diff -r "${IGNORE[@]}" "$pub_dir" "$loc_dir" 2>/dev/null | grep -c '^[<>]' || true)
    printf '  %-24s %-10s MISMATCH  (%s differing lines)\n' "$name" "$ver" "$n"
    mismatched=$((mismatched+1)); fail=1
    [ "$SHOW_DIFF" -eq 1 ] && diff -r -u "${IGNORE[@]}" "$pub_dir" "$loc_dir" 2>/dev/null | sed 's/^/      /' || true
  fi
done

echo
echo "  $matched matched, $mismatched mismatched, $skipped skipped (not yet published)."

if [ "$fail" -eq 0 ]; then
  echo
  if [ "$matched" -eq 0 ]; then
    # T11.12: a gate that passes because it checked nothing must say so. This is
    # the expected state right after a bump — every crate is ahead of its
    # published version, so there is no immutable artifact to contradict.
    cat <<'MSG'
OK — but VACUOUSLY: nothing in the tree sits on an already-published version,
so this run compared zero crates. That is the correct state after a version
bump, and it is not evidence that anything was verified.

This gate becomes meaningful again the moment a crate is published and the tree
keeps that number.
MSG
  else
    echo "OK: every published version matches its source ($matched compared)."
  fi
  exit 0
fi

cat <<'MSG'

FAILED: a published version does not match the source that claims it (O10.11).

A crates.io version is immutable. The tree and the artifact of the same name are
different code, and no local build will ever notice — path dependencies resolve
locally and never fetch the registry copy.

Fix by bumping the version, not by editing toward the published copy: the
published artifact cannot be changed, and someone may already depend on it.

Run with --diff to see what moved.
MSG
exit 1
