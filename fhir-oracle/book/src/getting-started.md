# Getting started

You need three things, none of them a `fhir-oracle` binary — there is no CLI
in any of these six ports (`C0.17`, `C0.18`):

1. **An Oracle server, 12.2 or later.** For local development,
   [`gvenzl/oracle-free:23-slim-faststart`](https://hub.docker.com/r/gvenzl/oracle-free)
   runs on arm64 and boots to ready in about 13 seconds. `scripts/db.sh up`
   starts it, provisions three uppercase users (`R3`, `R4`, `R5` — see below),
   and prints the environment variables the live test suite reads.
2. **Oracle Instant Client on the host**, separately from the server. The
   `oracle` crate `dlopen`s `libclntsh` at *connection* time, not build time
   — `cargo check`/`cargo build` work without it, but every live call fails
   `DPI-1047: Cannot locate a 64-bit Oracle Client library` without it. macOS
   arm64 has a direct, no-login download; see `scripts/db.sh`'s header
   comment for the current URL.
3. **Rust.**

## The three-part credential

Unlike the other five ports, this crate takes a username, password, and
connect string as three separate arguments — there is no single DSN string
to parse, because the `oracle` crate doesn't have one:

```rust,ignore
use std::sync::Arc;
use fhir_oracle_map::model::RelMap;
use fhir_oracle_store::oracle::OracleStore;

let mut map = RelMap::bundled("r5")?;   // the generated relational map, compiled in
map.schema = "R5".to_string();          // MUST be uppercase — see below

let store = OracleStore::connect(
    "r5",                       // Oracle username (case-insensitive at login)
    "your-password",
    "localhost:1521/FHIR",      // Easy Connect string, or a TNS alias
    Arc::new(map),
)
.await?;

store.init("a-checksum-string").await?;   // installs the schema; idempotent

let put = store.put(&patient_json, &fhir_oracle_store::Audit::cli()).await?;
let back = store.get("Patient", &put.id).await?;
```

This is a real, compiling shape — see `tests/oracle_store.rs` for the exact,
currently-passing version, and run it yourself:

```sh
DYLD_LIBRARY_PATH=~/lib scripts/db.sh test -p fhir-oracle-store --test oracle_store
```

## Why the schema must be uppercase

This is the sharpest way this port differs from its siblings, and it was
found live, not by reading a manual: Oracle folds an **unquoted** username to
uppercase for session identity (`SELECT USER FROM DUAL`) regardless of how it
was spelled at creation. A user created as `CREATE USER r5 IDENTIFIED BY
...` is really `R5`. If `RelMap.schema` is left lowercase (`"r5"`, matching
every other port's convention), every generated statement addresses
`"r5".*` while the session identity is `"R5"` — and every one of them fails
`ORA-01031: insufficient privileges`.

The fix, and the one this port's `scripts/db.sh` and test suite both use: create
the three version users **unquoted** (naturally uppercase — `CREATE USER R5
IDENTIFIED BY ...`) and set `RelMap.schema` to match, uppercase. See `M14.5`
in the [dialect annex](../../spec/14-oracle-dialect.md) for the full account.

## What is actually here

Three crates, no binary:

| Crate | What it does |
| --- | --- |
| `fhir-oracle-gen` | compiles the FHIR specification packages into a relational map and the DDL |
| `fhir-oracle-map` | the map types, shred, reconstruct, fold, and this engine's `ddl.rs` |
| `fhir-oracle-store` | the driver and the operations — `connect`, `init`, `put`, `get`, `delete`, `history`, `vread`, `search`, `verify_audit`, `purge`, `log_access` |

Each FHIR version is its own Oracle **user** (`R3`, `R4`, `R5`), because
Oracle conflates user and schema (`M14.5`). This port does **not** create
them — `scripts/db.sh`'s `post_ready` step does, for local development, but a
real deployment must provision them itself (`M14.28`).

The chapters that follow describe the storage model, the search compiler and
the trust boundary, rewritten to describe this engine specifically rather
than a copied template — see [`audit.md`](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/audit.md)
**F-56** and **F-68** for that history.
