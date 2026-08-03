# fhir-oracle-map

The relational map, and the engine that shreds FHIR resources into rows and reconstructs them back.

Part of **`fhir-oracle`**, which stores FHIR R3, R4 and R5 resources in Oracle Database as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

> **This port has no store. Its DDL is Oracle and has been executed** — the full R5 schema, 9,636 statements, installs on Oracle 26ai with 0 invalid objects (**F-08** closed, 2026-08-03). What is missing is the runtime: no driver, no store, so nothing has been written through the schema.

## Install

```toml
[dependencies]
fhir-oracle-map = "0.4.0"
serde_json = "1"
```

## What it does

A FHIR resource goes in as `serde_json::Value` and comes out as rows; the rows
go back in and the identical resource comes out.

```rust
use std::sync::Arc;
use fhir_oracle_map::{RelMap, shred, reconstruct};

// The map is compiled into this crate (feature `r5`, on by default).
let map = Arc::new(RelMap::bundled("r5")?);   // compiled in, no file needed

let patient = serde_json::json!({
    "resourceType": "Patient",
    "id": "example",
    "name": [{ "family": "\u{c6}r\u{f8}", "given": ["Anna"] }],
    "birthDate": "1974-12"          // a partial date, preserved verbatim
});

let rm = map.resources.get("Patient").unwrap();
let out = shred(rm, &patient)?;     // -> rows, ready to INSERT
```

There is **no per-resource generated Rust**. One generic engine walks any
resource against the map. Three FHIR versions × ~150 resource types × deep
nesting would explode compile times for no runtime benefit.

## Modules

| Module | Role |
| --- | --- |
| `model` | the relational map: every table, column and type the generator produced |
| `shred` | JSON → rows |
| `reconstruct` | rows → the identical JSON |
| `fold` | the case- and accent-insensitive fold behind search (`P6.6`) |
| `canon` | canonical JSON, for the tamper-evident hash chain (`M3.16`) |
| `value` | leaf-value parsing and formatting |
| `ddl` | **the only dialect-specific file** — emits MySQL DDL — **not** Oracle's (**F-08**) |

Everything except `ddl.rs` is byte-identical across all six ports and must stay
that way (`X15.1`); `scripts/check-shared-core.sh` gates it in CI.

## What is guaranteed

- **Lossless round-trip** (`R4.2`) — decimal precision and partial dates
  included. `9.60` does not come back as `9.6`, and `1974-12` does not become
  `1974-12-01`.
- **Deterministic column naming** — a name collision cannot silently rename a
  data column, because every derived column is claimed from the same registry.
- **One definition of string identity** — the fold is pure Rust, so the write
  path and the query path cannot disagree (`L1`).

## Derived columns

`ords` addresses rows inside repeating elements, so arbitrarily nested — even
recursive — structure stays joinable. Beyond that the shredder writes columns
the resource does not contain:

| Column | Purpose |
| --- | --- |
| `<col>_norm` | the folded form, so prefix search is case- and accent-insensitive (`VARCHAR2(450 CHAR)` for the adjunct only) |
| `<col>_idx` | bounded adjunct — `VARCHAR2(450 CHAR)` for the adjunct only, 450 characters (`U1`, `U5`, `U10`) |
| `<col>_h` | SHA-256 of the whole value, 32 raw bytes (`U1`, `U4a`) |

The last two exist because this engine cannot index or compare the source text
type as bound. They are an **access path, never an answer**: `U6` and `U7`
require a query to confirm against the source column, because a digest match
alone is one collision away from returning another patient's record.
All derived columns are written by the shredder and never read by the
reconstructor (`U3`), so none of them can affect round-trip fidelity or enter
the hash-chain pre-image.

## Further reading

| | |
| --- | --- |
| [Port README](../../README.md) | this engine's overview, quick start, and trust boundary |
| [Dialect annex](../../spec/14-oracle-dialect.md) | every declared departure from the shared core, by number |
| [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/index.md) | the normative core, shared by all six ports |
| [Conformance matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md) | what each port actually satisfies today |
| [Audit register](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md) | every known divergence, with evidence |

## Status, honestly

A conformance level is a claim about what has been **verified for this port**,
not about what its code contains. This crate is part of a port at
**Scaffold** level (`C0.8`): the DDL emitter is real Oracle and has been executed by hand against 26ai, but no test in this port runs it, and `C0.9` requires the level be justified by tests that run. **Schema** needs a live test — which needs an Oracle driver decision. No store.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.
