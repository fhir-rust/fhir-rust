# fhir-postgresql-map

The relational map, and the engine that shreds FHIR resources into rows and reconstructs them back.

Part of **`fhir-postgresql`**, which stores FHIR R3, R4 and R5 resources in PostgreSQL 18 as real relational tables — typed columns, child tables, foreign keys, check constraints — and gives them back losslessly.

## Install

```toml
[dependencies]
fhir-postgresql-map = "0.4.0"
serde_json = "1"
```

## What it does

A FHIR resource goes in as `serde_json::Value` and comes out as rows; the rows
go back in and the identical resource comes out.

```rust
use std::sync::Arc;
use fhir_postgresql_map::{RelMap, shred, reconstruct};

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
| `ddl` | **the only dialect-specific file** — emits PostgreSQL DDL |

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
| `<col>_norm` | the folded form, so prefix search is case- and accent-insensitive (`text COLLATE "C"`) |

This engine indexes and compares its bound text type directly, so `U9`
**forbids** the unbounded-string adjuncts here — `ddl::TEXT_ADJUNCTS` is
`false` and a map generated for this port carries no `_idx` or `_h` column.

All derived columns are written by the shredder and never read by the
reconstructor (`U3`), so none of them can affect round-trip fidelity or enter
the hash-chain pre-image.

## Further reading

| | |
| --- | --- |
| [Port README](../../README.md) | this engine's overview, quick start, and trust boundary |
| [Dialect annex](../../spec/14-postgresql-dialect.md) | every declared departure from the shared core, by number |
| [Specification](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/index.md) | the normative core, shared by all six ports |
| [Conformance matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md) | what each port actually satisfies today |
| [Audit register](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md) | every known divergence, with evidence |

## Status, honestly

A conformance level is a claim about what has been **verified for this port**,
not about what its code contains. This crate is part of a port at
**Reference** level: full store; 8 test files including concurrency, audit, redaction, upgrade and a benchmark; live PostgreSQL 18 gate in CI.

The conformance matrix is the document to trust — a README, a book chapter, and
a `tasks.md` checkbox have all been wrong in this repository before.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — you choose.
