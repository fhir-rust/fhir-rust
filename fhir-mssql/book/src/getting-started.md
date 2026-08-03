# Getting started

You need SQL Server 2016 or later and Rust.

**There is no `fhir-mssql` binary.** The commands this chapter used to show —
`fhir-mssql init`, `load`, `serve` — do not exist in any port (`C0.17`,
`C0.18`); they were `fhir-postgresql`'s book text, and it does not have them
either. This is a library you call:

```rust,ignore
use std::sync::Arc;
use fhir_mssql_map::model::RelMap;

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
| `fhir-mssql-gen` | compiles the FHIR specification packages into a relational map and the DDL |
| `fhir-mssql-map` | the map types, shred, reconstruct, fold, and this engine's `ddl.rs` |
| `fhir-mssql-store` | the driver and the operations — see the port README for its level |

Each FHIR version installs into its own SQL Server schema (`r5`, `r4`, `r3`)
inside whatever database you point at.

The chapters that follow describe the storage model, the search compiler and
the trust boundary. Those are accurate in substance; where they name
PostgreSQL, or a `serve` command, or an HTTP status code, read the banner in
the [Introduction](introduction.md) — that text has not been corrected yet
(audit **F-56**).
