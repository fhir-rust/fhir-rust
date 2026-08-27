#!/usr/bin/env bash
# Does every crate root declare #![forbid(unsafe_code)]?
#
# The model family has required this since 2026-08-06 (spec R13.14, task T39);
# the database ports, fhir-store, fhir-loco and the fuzz crates adopted it on
# 2026-08-26. This script is what stops the next new crate from quietly not
# having it, which is how the model family's facade ended up as the only crate
# that did.
#
# `forbid` rather than `deny` is the point: `deny` can be lifted by an `#[allow]`
# anywhere inside the crate, so it records an intention. `forbid` cannot be
# lifted at all, so it records a guarantee — which is the thing a reader
# evaluating clinical software actually wants to know.
#
# Scope: every crate root in the repository. For a normal crate that is
# src/lib.rs or src/main.rs; for a cargo-fuzz crate it is each file under
# fuzz_targets/, because each is its own [[bin]] and therefore its own crate
# root. The fuzz crates are nightly-only tooling that is never published, and
# they were expected to be an exemption — the libfuzzer-sys `fuzz_target!` macro
# expands to an `extern "C"` entry point, so the assumption was that a target
# could not compile under `forbid`. It was checked rather than assumed, and it
# was wrong: all seven fuzz workspaces build clean with the attribute
# (`cargo +nightly check --all-targets`, 2026-08-26). There is no exemption.
#
# Integration tests under tests/ are separate crates and are NOT covered by a
# crate-root attribute. That is deliberate and load-bearing: mutating the
# environment is `unsafe` in edition 2024, and three ports' SSL-default tests
# need to do exactly that. They live in tests/ so the library keeps the stronger
# rule rather than the rule being weakened to accommodate a test.
#
#   scripts/check-forbid-unsafe.sh          check
#   scripts/check-forbid-unsafe.sh --list   also list compliant crate roots
#
# Exits non-zero if any crate root is missing the attribute.
set -euo pipefail

cd "$(dirname "$0")/.."
LIST="${1:-}"

fail=0
found=0

check_root() {
  local label="$1" root="$2"
  found=$((found + 1))
  if grep -q '^#!\[forbid(unsafe_code)\]' "$root"; then
    [ "$LIST" = "--list" ] && echo "ok   $label ($root)"
    return 0
  fi
  echo "FAIL $label — $root does not declare #![forbid(unsafe_code)]"
  fail=$((fail + 1))
}

while IFS= read -r manifest; do
  grep -q '^\[package\]' "$manifest" || continue

  name=$(sed -n 's/^name = "\(.*\)"/\1/p' "$manifest" | head -1)
  dir=$(dirname "$manifest")

  # A cargo-fuzz crate has one crate root per target, not a single lib.rs.
  if [ -d "$dir/fuzz_targets" ]; then
    while IFS= read -r target; do
      check_root "$name:$(basename "$target" .rs)" "$target"
    done < <(find "$dir/fuzz_targets" -name '*.rs' | sort)
    continue
  fi

  root=""
  for candidate in "$dir/src/lib.rs" "$dir/src/main.rs"; do
    [ -f "$candidate" ] && root="$candidate" && break
  done

  if [ -z "$root" ]; then
    echo "FAIL $name — no crate root found in $dir"
    fail=$((fail + 1))
    continue
  fi

  check_root "$name" "$root"
# -prune, not -not -path: the latter still descends into target/ before
  # filtering, which on a repo with nine cargo workspaces and their build
  # artifacts on disk makes this walk minutes instead of seconds. -prune
  # skips the subtree entirely.
  done < <(find . -name target -prune -o -name Cargo.toml -print | sort)

echo
if [ "$fail" -gt 0 ]; then
  echo "$fail of $found crate root(s) do not forbid unsafe code."
  echo "Fix: add this to the crate root, above the first item:"
  echo
  echo "    #![forbid(unsafe_code)]"
  exit 1
fi

echo "All $found crate roots declare #![forbid(unsafe_code)]."
