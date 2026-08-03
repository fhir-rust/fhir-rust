#!/usr/bin/env bash
# Do the Rust examples in doc/ and README.md compile?
#
# Nothing built them until 2026-08-03 (audit F-60). They are marked ```rust,
# which tells a reader they are code rather than pseudocode, and that claim was
# never checked — in a repository whose recurring defect is documentation
# describing software that does not exist.
#
# How it works. Each ```rust block is wrapped in a preamble that supplies the
# four names the prose has already established by the time a reader reaches it
# — `map`, `store`, `patient`, `audit` — and compiled with `cargo check`. A
# block that needs something else, or that shows a fragment rather than a
# statement, must be marked ```rust,ignore and say why in the prose.
#
# `ignore` is not a way to silence this. It is a claim that the block is
# illustrative, and it should be rare: F-60 found exactly two, both for an API
# that exists on one port only.
#
#   scripts/check-doc-examples.sh          check
#   scripts/check-doc-examples.sh --keep   leave the scratch crate for inspection
#
# Exits non-zero if any block fails to compile.
set -euo pipefail

cd "$(dirname "$0")/.."
REPO="$PWD"
KEEP=0
[ "${1:-}" = "--keep" ] && KEEP=1

WORK="$(mktemp -d)"
trap '[ "$KEEP" = 1 ] || rm -rf "$WORK"' EXIT

mkdir -p "$WORK/src"
cat > "$WORK/Cargo.toml" <<EOF
[package]
name = "doc-examples"
version = "0.0.0"
edition = "2024"

[dependencies]
fhir-sqlite-map = { path = "$REPO/fhir-sqlite/crates/fhir-sqlite-map", features = ["r5"] }
fhir-sqlite-store = { path = "$REPO/fhir-sqlite/crates/fhir-sqlite-store" }
fhir-store = { path = "$REPO/fhir-store" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
anyhow = "1"

[workspace]
EOF

python3 - "$REPO" "$WORK" <<'PYEOF'
import re, sys, pathlib, json
repo, work = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
srcs = sorted(repo.glob("doc/*.md")) + [repo / "README.md"]
blocks = []
for p in srcs:
    for m in re.finditer(r"```rust\n(.*?)```", p.read_text(), re.S):
        line = p.read_text()[: m.start()].count("\n") + 1
        blocks.append({"file": str(p.relative_to(repo)), "line": line, "code": m.group(1)})
(work / "blocks.json").write_text(json.dumps(blocks))
print(f"  {len(blocks)} block(s) from {len(srcs)} file(s)")
PYEOF

PREAMBLE='#![allow(unused_imports, unused_variables, unused_mut, dead_code)]
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::sqlite::SqliteStore;
use fhir_sqlite_store::{Audit, StoreError};
use fhir_sqlite_map::{shred, reconstruct};
use fhir_store::{AccessRecord, CondCreate, HistEntry, PutOutcome};

#[allow(unused)]
async fn example() -> anyhow::Result<()> {
    let map = Arc::new(RelMap::bundled("r5")?);
    let store = SqliteStore::open("doc.sqlite", map.clone()).await?;
    let patient = serde_json::json!({"resourceType": "Patient", "id": "example"});
    let updated = patient.clone();
    let audit = Audit::cli();
    let want = "r5-baseline";
    let (actor, source) = ("dr.jones@clinic.example", "header:X-Fhir-Principal");
'

fail=0
total=0
python3 - "$WORK" <<'PYEOF' > "$WORK/list.txt"
import json, pathlib, sys
work = pathlib.Path(sys.argv[1])
for i, b in enumerate(json.loads((work / "blocks.json").read_text())):
    print(f"{i}\t{b['file']}\t{b['line']}")
PYEOF

while IFS=$'\t' read -r idx file line; do
  total=$((total + 1))
  python3 - "$WORK" "$idx" "$PREAMBLE" <<'PYEOF'
import json, pathlib, sys
work, idx, pre = pathlib.Path(sys.argv[1]), int(sys.argv[2]), sys.argv[3]
b = json.loads((work / "blocks.json").read_text())[idx]
code = b["code"].rstrip()
# A block may end in a trailing expression (`store.search(...).await?`) rather
# than a statement. Appending `Ok(())` after one is a syntax error created by
# this harness, not by the documentation, so terminate it first.
if code and not code.rstrip().endswith((";", "}", "{")):
    code += ";"
(work / "src" / "main.rs").write_text(pre + code + "\n    Ok(())\n}\n\nfn main() {}\n")
PYEOF
  if ! out=$(cd "$WORK" && cargo check -q 2>&1); then
    fail=$((fail + 1))
    printf '  FAIL %s:%s\n' "$file" "$line"
    printf '%s\n' "$out" | grep -E '^error' | head -3 | sed 's/^/        /'
  fi
done < "$WORK/list.txt"

echo
if [ "$fail" -eq 0 ]; then
  echo "OK: all $total documentation example(s) compile."
  exit 0
fi
cat >&2 <<MSG
$fail of $total example(s) do not compile.

Either fix the example, or mark it \`\`\`rust,ignore and say in the prose why it
cannot be compiled — that a reader must supply something, or that the API is
available on one port only. Do not mark it ignore to make this quiet: an
unmarked block is a promise that the code works.
MSG
exit 1
