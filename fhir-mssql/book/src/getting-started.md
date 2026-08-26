# Getting started

You need SQL Server 2016 or later (`M14.3`) and Rust.

**There is no `fhir-mssql` binary.** The commands this chapter used to show —
`fhir-mssql init`, `load`, `serve` — do not exist in any port (`C0.17`,
`C0.18`); they were `fhir-postgresql`'s book text. This is a library you
call, and the signatures below are checked against
`crates/fhir-mssql-store/src/mssql.rs` and `tests/mssql_store.rs`, not
invented for this page:

```rust,ignore
use std::sync::Arc;
use fhir_mssql_map::model::RelMap;
use fhir_mssql_store::mssql::MsSqlStore;

// The generated relational map ships compiled into the crate, one per FHIR
// version (feature-gated: `r5`, `r4`, `r3`).
let map: Arc<RelMap> = Arc::new(RelMap::bundled("r5")?);

// The DSN is a tiberius ADO connection string. It MUST name a database
// (`database=fhir_mssql`), not the default `master` — snapshot isolation
// (M14.25, needed for `get` under concurrent writers) requires
// ALLOW_SNAPSHOT_ISOLATION at the database level, and SQL Server refuses
// that option on `master`. `scripts/db.sh up` provisions and prints a DSN
// with the right database already named.
let dsn = "server=tcp:127.0.0.1,1433;user=sa;password=…;\
           TrustServerCertificate=true;database=fhir_mssql";
let store = MsSqlStore::connect(dsn, map).await?;

// Applies the generated DDL idempotently and records the map checksum.
store.init("my-checksum").await?;

// crate::Audit — actor, actor_source, client, request_id, reason. This
// port has no principal source of its own; a caller (e.g. fhir-loco) fills
// it in from whatever authenticated it the request.
let audit = fhir_mssql_store::Audit::default();

let put = store.put(&patient_json, &audit).await?;      // PutOutcome { id, version_id, created }
let back = store.get("Patient", &put.id).await?;         // Option<serde_json::Value>
let hits = store.search(
    "Patient",
    &[("family".into(), "Aero".into())],
    10,
    0,
).await?;                                                 // Vec<String> of ids
```

`connect` does not select the map's schema as the connection's default
database context: every statement this store issues is schema-qualified, so
a typo in the map's `schema` field fails loudly instead of silently landing
on whatever schema happened to be current.

## What is actually here

Three crates, no binary:

| Crate | What it does |
| --- | --- |
| `fhir-mssql-gen` | compiles the FHIR® specification packages into a relational map and the DDL |
| `fhir-mssql-map` | the map types, shred, reconstruct, fold, canon, and this engine's `ddl.rs` |
| `fhir-mssql-store` | `mssql.rs` (the driver and operations), `mssql_search.rs` (the search-SQL builder), `pool.rs` (a `bb8` pool over `tiberius`, which has no built-in one) |

`fhir-mssql-store` also depends on the shared [`fhir-store`](../../../fhir-store)
crate for the engine-agnostic half — `Audit`, `AccessRecord`, `PutOutcome`,
`SearchOutcome`, `ChainBreak`, and the hash-chain functions — and re-exports
it, so `fhir_mssql_store::Audit` resolves without an extra dependency.

Each FHIR version installs into its own SQL Server schema (`r5`, `r4`, `r3`)
inside whatever database the DSN names.

## Running against a real server

```sh
scripts/db.sh up      # SQL Server 2022 in a container (arm64: set
                       # FHIR_MSSQL_IMAGE=mcr.microsoft.com/azure-sql-edge)
scripts/db.sh test    # up, then the live suite
export FHIR_MSSQL_TEST_DSN='...'   # scripts/db.sh up prints this
cargo test -p fhir-mssql-store -- --test-threads=1
scripts/db.sh down
```

`--test-threads=1` is not optional for the store suite: each live test
installs and drops its own schema against `azure-sql-edge`, and running them
concurrently produced a live deadlock (SQL Server error 1205) during schema
teardown — a container-load artifact under heavy concurrent DDL, not a bug
in what any one test exercises. `cargo test` with no DSN set passes anyway:
the corpus- and database-driven tests self-skip, printing why on stderr.

The chapters that follow describe the storage model, the search compiler,
and the trust boundary in terms of this store's actual `mssql.rs` and
`mssql_search.rs` behaviour, live-verified against `azure-sql-edge`
(**F-65**) rather than carried over from `fhir-postgresql`.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
