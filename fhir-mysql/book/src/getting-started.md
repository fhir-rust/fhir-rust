# Getting started

You need MySQL 8.4 and Rust.

**There is no `fhir-mysql` binary.** The commands this chapter used to show —
`fhir-mysql init`, `load`, `serve` — do not exist in any port (`C0.17`,
`C0.18`); they were `fhir-postgresql`'s book text, and it does not have them
either. This is a library you call:

```rust,ignore
use std::sync::Arc;
use fhir_mysql_map::model::RelMap;

// The generated relational map ships with the crate.
let map: Arc<RelMap> = Arc::new(RelMap::bundled("r5")?);

// Install the schema, then write and read resources through the store API.
// See the crate documentation for the exact signatures — they differ by
// engine, which is the one place these ports deliberately diverge.
```

The [port README](https://github.com/fhir-rust/fhir-rust) carries a compiled,
current example; this chapter does not, because an example that is not compiled
is how the previous version of this page came to describe a binary that was
never built (audit **F-56**).

## What is actually here

Three crates, no binary:

| Crate | What it does |
| --- | --- |
| `fhir-mysql-gen` | compiles the FHIR® specification packages into a relational map and the DDL |
| `fhir-mysql-map` | the map types, shred, reconstruct, fold, and this engine's `ddl.rs` |
| `fhir-mysql-store` | the driver and the operations — see the port README for its level |

Each FHIR version installs into its own MySQL database (`r5`, `r4`, `r3`);
MySQL has no separate schema concept, so a database *is* the namespace.

The chapters that follow describe the storage model, the search compiler and
the trust boundary. Those are accurate in substance; where they name
PostgreSQL, or a `serve` command, or an HTTP status code, read the banner in
the [Introduction](introduction.md) — that text has not been corrected yet
(audit **F-56**).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
