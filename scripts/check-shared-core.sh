#!/usr/bin/env bash
# Does the shared Rust core still agree across all six ports?
#
# Spec X15.1 requires that the pure-Rust core — the modules that operate on Rust
# types and never emit SQL — be identical in every port, after normalizing the
# crate-name substitution and dialect-annex cross-references. Spec W16.6
# requires CI to verify it.
#
# Why this exists (audit F-10). At the time it was written the core *was*
# identical everywhere, and it was identical because the ports had been forked
# recently, not because anything checked. Sections 1-13 of the specification
# were identical for the same reason and had already begun to drift; the fix
# there was to keep one copy. That fix is not available for the code, because
# each port must compile on its own, so the next best thing is a gate that
# fails the moment a copy moves.
#
#   scripts/check-shared-core.sh            check, print a summary
#   scripts/check-shared-core.sh --diff     also print the offending diffs
#
# Exits non-zero on any divergence.
set -euo pipefail

cd "$(dirname "$0")/.."

SHOW_DIFF=0
[ "${1:-}" = "--diff" ] && SHOW_DIFF=1

BASELINE="fhir-sqlite"
PORTS=(fhir-postgresql fhir-mysql fhir-mariadb fhir-mssql fhir-oracle)

# The modules X15.1 lists. `ddl.rs` is deliberately absent: it is one of exactly
# two places a dialect may differ, the other being the store crate.
MAP_MODULES=(model shred reconstruct value fold canon error lib)
GEN_MODULES=(assets build lib names search spec)

# Normalize away what is *allowed* to differ:
#
#   - the crate name, in every spelling the tree uses
#     (fhir_sqlite_map / fhir-sqlite-map / FHIR_SQLITE_*)
#   - dialect-annex cross-references (M14.7 here is M14.11 there), since each
#     port numbers its own annex independently (C0.7)
#
# Anything else is a divergence.
normalize() {
  sed -E \
    -e 's/fhir[-_](postgresql|sqlite|mysql|mariadb|mssql|oracle)/fhir_PORT/g' \
    -e 's/FHIR_(POSTGRESQL|SQLITE|MYSQL|MARIADB|MSSQL|ORACLE)/FHIR_PORT/g' \
    -e 's/M14\.[0-9]+[a-z]?/M14.N/g' \
    "$1"
}

# The same content as a token stream: one token per line, whitespace discarded.
#
# Why the pass/fail decision is made on this and not on `normalize` (audit F-48).
# `normalize` rewrites the crate name to a constant, but it cannot undo what the
# *length* of that name already did to the layout. rustfmt wraps at a column, so
# a longer crate name pushes a line over and splits it, and the two files then
# differ by whitespace alone:
#
#   fhir-mysql  (69 cols)  std::env::var("FHIR_MYSQL_SPEC_DIR").ok().map(..),
#   fhir-sqlite (70 cols)  std::env::var("FHIR_SQLITE_SPEC_DIR")
#                              .ok()
#                              .map(..),
#
# Measured across all six ports the split boundary is exactly 69/70 columns, so
# this is layout caused by the name, not divergent code — and no amount of
# name-substitution can see that. A line-based gate reports it as a divergence
# nobody can fix, which is how gates end up permanently red and then ignored.
#
# A token is a run of `[A-Za-z0-9_]` or a single other non-space character, so
# punctuation survives: `a + b` and `a - b` are still different, and only
# whitespace is discarded.
#
# What this deliberately stops seeing: whitespace *inside* a string literal, and
# the rewrapping of a doc comment. The first is acceptable here because these
# modules are the pure-Rust core and never emit SQL — that is X15.1's own
# criterion for what belongs in this set. The second is the same rustfmt-by-name
# -length effect one level up, so ignoring it is the point rather than a
# concession.
tokenize() {
  normalize "$1" | python3 -c '
import re, sys
for tok in re.findall(r"[A-Za-z0-9_]+|[^\s]", sys.stdin.read()):
    print(tok)
'
}

# Known divergences, tracked in spec/audit.md, exempted so that this gate can
# protect against *new* drift instead of sitting red and being ignored.
#
# Every entry names the finding that tracks it. Each one is PRINTED on every
# run, never silently skipped: a gate that quietly tolerates exceptions is the
# same failure as a test that quietly skips (T11.12). Removing an entry is part
# of closing its finding.
#
# Format: "<port>:<label>  # <finding> — <why>"
# Empty, and it should stay that way. The two entries that used to live here were
# F-07's — `fhir-postgresql` was the only map crate without `canon.rs`, because
# it derived its chain pre-image from `jsonb` in SQL. Closing F-07 ported
# `canon.rs` in and removed both, so the shared core is now uniform across all
# six ports with nothing excused.
#
# Adding an entry here is a last resort, not a way to make this script quiet: an
# exemption is a divergence that survives review, so it MUST cite the audit
# finding or the numbered M14.x departure that authorizes it (C0.12).
EXEMPT=()
EXEMPT_REASON=""

is_exempt() {
  local key="$1"
  # `${EXEMPT[@]+...}` guards the empty case: under `set -u`, bash 3.2 (which
  # macOS still ships) treats an empty array expansion as an unbound variable
  # and aborts. That made this gate *crash* instead of reporting, and only ever
  # on the path that matters — `is_exempt` is reached only when a file has
  # already diverged, so the bug hid exactly the output it was called to print.
  for e in ${EXEMPT[@]+"${EXEMPT[@]}"}; do
    [ "$e" = "$key" ] && return 0
  done
  return 1
}

fail=0
checked=0
missing=0
exempted=0

check_one() {
  local rel_baseline="$1" port="$2" rel_port="$3" label="$4"
  local key="$port:$label"

  if [ ! -f "$rel_port" ]; then
    if is_exempt "$key"; then
      printf '  %-16s %-28s MISSING   (exempt)\n' "$port" "$label"
      exempted=$((exempted + 1))
      return
    fi
    printf '  %-16s %-28s MISSING\n' "$port" "$label"
    missing=$((missing + 1))
    fail=1
    return
  fi

  checked=$((checked + 1))
  # The verdict is on tokens (F-48); the diff shown to a human is on lines,
  # because a token-per-line diff is unreadable.
  local n
  n=$(diff <(tokenize "$rel_baseline") <(tokenize "$rel_port") | grep -c '^[<>]' || true)
  if [ "$n" -eq 0 ]; then
    return
  fi

  if is_exempt "$key"; then
    printf '  %-16s %-28s %s differing token(s) (exempt)\n' "$port" "$label" "$n"
    exempted=$((exempted + 1))
    return
  fi

  printf '  %-16s %-28s %s differing token(s)\n' "$port" "$label" "$n"
  fail=1
  if [ "$SHOW_DIFF" -eq 1 ]; then
    diff -u <(normalize "$rel_baseline") <(normalize "$rel_port") | sed 's/^/      /' || true
  fi
}

echo "Shared-core check (X15.1), baseline: $BASELINE"
echo

for m in "${MAP_MODULES[@]}"; do
  base="$BASELINE/crates/$BASELINE-map/src/$m.rs"
  [ -f "$base" ] || { echo "  baseline missing $base" >&2; exit 2; }
  for p in "${PORTS[@]}"; do
    check_one "$base" "$p" "$p/crates/$p-map/src/$m.rs" "map/src/$m.rs"
  done
done

for m in "${GEN_MODULES[@]}"; do
  base="$BASELINE/crates/$BASELINE-gen/src/$m.rs"
  [ -f "$base" ] || { echo "  baseline missing $base" >&2; exit 2; }
  for p in "${PORTS[@]}"; do
    check_one "$base" "$p" "$p/crates/$p-gen/src/$m.rs" "gen/src/$m.rs"
  done
done

# The gen crate's tests. `gen/` is dialect-independent in full — X15.1 does not
# carve out its tests — and until F-48 these were unwatched, so the three that
# are identical were identical by convention rather than by gate. Adding them
# only became possible once the verdict moved to tokens: two of the five differ
# by rustfmt wrapping alone.
#
# `<engine>_ddl.rs` and the store suites are NOT here. They live in the map and
# store crates, which are the two places a dialect is allowed to differ.
GEN_TESTS=(assets_current corpus proptest_roundtrip roundtrip adjuncts_in_ddl path_bound)
for m in "${GEN_TESTS[@]}"; do
  base="$BASELINE/crates/$BASELINE-gen/tests/$m.rs"
  [ -f "$base" ] || { echo "  baseline missing $base" >&2; exit 2; }
  for p in "${PORTS[@]}"; do
    check_one "$base" "$p" "$p/crates/$p-gen/tests/$m.rs" "gen/tests/$m.rs"
  done
done

# Developer tools that live inside the gen crate. They are shared core for the
# same reason the modules are: six copies that drift are six behaviours, and
# `regen-assets` decides what ships in assets/ (G2.1, G2.2).
GEN_BINS=(regen-assets)
for b in "${GEN_BINS[@]}"; do
  base="$BASELINE/crates/$BASELINE-gen/src/bin/$b.rs"
  [ -f "$base" ] || { echo "  baseline missing $base" >&2; exit 2; }
  for p in "${PORTS[@]}"; do
    check_one "$base" "$p" "$p/crates/$p-gen/src/bin/$b.rs" "gen/src/bin/$b.rs"
  done
done

echo
if [ "$exempted" -gt 0 ]; then
  echo "  ---"
  echo "  $exempted known divergence(s) exempted:"
  echo "    $EXEMPT_REASON"
  echo
fi

if [ "$fail" -eq 0 ]; then
  echo "OK: $checked files identical across all six ports (plus $exempted exempt)."
  exit 0
fi

cat <<'MSG'

FAILED: the shared core has diverged.

X15.1 requires these modules be identical in every port; W16.7 requires a change
to shared code be applied to every port in the same commit. A port left behind
is not "pending", it is a divergence.

If the difference is deliberate, it is in the wrong file: a dialect difference
belongs in map/src/ddl.rs or the store crate, and — where it changes what the
core requires — in a numbered M14.x departure in that port's annex (C0.12).

Run with --diff to see what moved.
MSG
[ "$missing" -gt 0 ] && echo "($missing file(s) absent entirely.)"
exit 1
