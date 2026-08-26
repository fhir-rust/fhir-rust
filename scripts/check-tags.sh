#!/usr/bin/env bash
# Do the git tags name published versions the way TG1 requires?
#
# spec/git-tags-name-published-versions/ defines a tag as naming an
# independently-versioned unit: a workspace whose members inherit
# `version.workspace = true`, or a single crate. The name is `<unit>-v<version>`.
#
# What this checks (TG1.11), all offline:
#   - every tag is well-formed and names a unit that exists
#   - no two tags name the same unit and version
#   - which units have a current source version with no tag  [reported only]
#
# The last one reports rather than fails: a version bump legitimately precedes
# its publication, so failing there would block ordinary work.
#
#   scripts/check-tags.sh
#
# Exits non-zero only on a malformed or unknown tag.
set -euo pipefail

cd "$(dirname "$0")/.."

# Units: a port workspace (members inherit version), else a single crate.
declare -a UNIT_NAME UNIT_VERSION
add_unit() { UNIT_NAME+=("$1"); UNIT_VERSION+=("$2"); }

version_of() { sed -n 's/^version = "\(.*\)"/\1/p' "$1" | head -1; }

for port in fhir-postgresql fhir-sqlite fhir-mysql fhir-mariadb fhir-mssql fhir-oracle; do
  add_unit "$port" "$(version_of "$port/Cargo.toml")"
done
add_unit "fhir-store" "$(version_of fhir-store/Cargo.toml)"
add_unit "fhir-loco"  "$(version_of fhir-loco/Cargo.toml)"
add_unit "fhir"       "$(version_of fhir/Cargo.toml)"
for d in fhir/fhir-*/; do
  [ -f "$d/Cargo.toml" ] || continue
  add_unit "$(basename "$d")" "$(version_of "$d/Cargo.toml")"
done

known=" ${UNIT_NAME[*]} "
fail=0

# 1. Every tag is well-formed and names a known unit.
while IFS= read -r tag; do
  [ -z "$tag" ] && continue
  if [[ ! "$tag" =~ ^(.+)-v([0-9]+\.[0-9]+\.[0-9]+.*)$ ]]; then
    echo "FAIL $tag — not <unit>-v<version> (TG1.3)"; fail=$((fail + 1)); continue
  fi
  unit="${BASH_REMATCH[1]}"
  case "$known" in
    *" $unit "*) ;;
    *) echo "FAIL $tag — names no unit in this repository (TG1.2)"; fail=$((fail + 1)) ;;
  esac
done < <(git tag)

# 2. No duplicates. git tag names are unique, so this can only fire on a
#    case-insensitive collision, which some filesystems create.
dupes=$(git tag | tr 'A-Z' 'a-z' | sort | uniq -d || true)
if [ -n "$dupes" ]; then
  echo "FAIL tags collide when case is ignored:"; echo "$dupes" | sed 's/^/       /'
  fail=$((fail + 1))
fi

# 3. Report units whose current version is untagged.
untagged=0
for i in "${!UNIT_NAME[@]}"; do
  want="${UNIT_NAME[$i]}-v${UNIT_VERSION[$i]}"
  if ! git rev-parse -q --verify "refs/tags/$want" >/dev/null; then
    [ "$untagged" -eq 0 ] && echo "Units whose current source version has no tag (TG1.11, reported):"
    echo "       $want"
    untagged=$((untagged + 1))
  fi
done

echo
if [ "$fail" -gt 0 ]; then
  echo "$fail tag problem(s). See spec/git-tags-name-published-versions/."
  exit 1
fi
echo "$(git tag | wc -l | tr -d ' ') tag(s) well-formed; $untagged unit(s) untagged at their current version."
