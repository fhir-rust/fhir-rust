# Getting started

You need SQLite 3 and Rust. Nothing else: the SQLite engine is compiled in via
`rusqlite`'s bundled feature, so there is no server to install, start, or
point this at (`M14.23`).

**There is no `fhir-sqlite` binary.** No `init`, `load`, or `serve` command
exists in this workspace, and none ever will unless a CLI crate is added —
none is planned (`C0.17`, `C0.18`). This is a library you call from your own
`async fn main`.

## Add the dependencies

```toml
[dependencies]
fhir-sqlite-map   = { path = "crates/fhir-sqlite-map", features = ["r5"] }
fhir-sqlite-store = { path = "crates/fhir-sqlite-store" }
tokio      = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

The `r5`/`r4`/`r3` features on `fhir-sqlite-map` gate **compilation** of the
bundled relational map for that FHIR version, not which one you use at
runtime — `r5` is on by default. All three maps travel inside the published
crate regardless of which features are enabled (~2.5 MB total); the features
only decide which `RelMap::bundled(...)` calls succeed in your binary.

## Open a store and write a resource

This is real, compiling code — every call below matches a signature in
`fhir-sqlite-store/src/sqlite.rs`, exercised by
`fhir-sqlite-store/tests/sqlite_store.rs`.

```rust
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::sqlite::SqliteStore;
use fhir_sqlite_store::Audit;

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
// The relational map ships as a committed, generated asset — no FHIR
// specification packages needed at runtime.
let map = Arc::new(RelMap::bundled("r5")?);

// One file per FHIR version. `open` creates the file (and its parent
// directories) if it does not exist, and sets four pragmas that are load-
// bearing rather than tuning: foreign_keys=ON (so a rewrite's DELETE cascades
// into child tables instead of orphaning rows), journal_mode=WAL (so readers
// never block the writer), busy_timeout=30000 (so a second writer waits
// instead of failing immediately), and synchronous=FULL (this stores health
// records under a hash chain; a torn write is not an acceptable trade).
let store = SqliteStore::open("clinic.sqlite", map).await?;

// Installs in one transaction and records the map's own checksum, so a later
// run can tell whether the schema still matches this map. Re-running with the
// same checksum after a fresh install fails — this is `init`, not `upgrade`.
store.init("clinic-2026-08").await?;

let patient = serde_json::json!({
    "resourceType": "Patient",
    "id": "example",
    "name": [{ "family": "Ærø", "given": ["Anna"] }],
    "birthDate": "1974-12"   // a partial date, preserved verbatim
});

let audit = Audit::cli(); // records `cli:$USER` as the actor
let put = store.put(&patient, &audit).await?;
assert_eq!(put.version_id, 1);

let back = store.get("Patient", "example").await?.unwrap();
assert_eq!(back, patient);   // losslessly, including "1974-12"

// Accent- and case-insensitive by construction: "aero" finds "Ærø" (search.md).
let hits = store.search("Patient", &[("name".into(), "aero".into())], 50, 0).await?;
assert_eq!(hits, vec!["example".to_string()]);
# Ok(())
# }
```

`SqliteStore::open` does not attach a version's database until the first call
that needs it (`init`, `put`, `get`, `search`, …). When it does, it derives
the attached file's name from the path you gave `open`: `clinic.sqlite` plus
schema `r5` becomes a second file, `clinic-r5.sqlite`, attached as `ATTACH
DATABASE 'clinic-r5.sqlite' AS "r5"`. That is this port's replacement for
PostgreSQL's `CREATE SCHEMA` — a schema is a file, not a namespace inside a
server (`M14.16`).

## Writing tests against a real file, not `:memory:`

Every test in this crate uses a real scratch file under `target/`, not
SQLite's `:memory:` pseudo-path. That is not a stylistic choice: because each
FHIR version attaches its **own** database file derived from the main path's
file stem, `SqliteStore::open(":memory:", map)` would try to attach a file
literally named `:memory:-r5.sqlite` on disk — not an in-memory database. If
you want a throwaway store, use a temporary directory:

```rust,ignore
let dir = tempfile::tempdir()?;
let store = SqliteStore::open(dir.path().join("test.sqlite"), map).await?;
```

(`tempfile` is not a dependency of this crate; add it to your own if you want
this pattern. The test suite instead uses a hand-rolled scratch helper under
`target/test-scratch/`, to keep dev-dependencies minimal — see
`crates/fhir-sqlite-store/tests/sqlite_store.rs`.)

## What is actually here

Three crates, no binary, no server:

| Crate | What it does |
| --- | --- |
| `fhir-sqlite-gen` | compiles FHIR specification packages into a relational map and search-parameter compilation, at asset-build time — identical across all six ports (`X15.1`) |
| `fhir-sqlite-map` | the map types, `shred`, `reconstruct`, `fold`, `canon`, and this engine's `ddl.rs` — everything but `ddl.rs` is also shared across all six ports |
| `fhir-sqlite-store` | `SqliteStore`: `rusqlite`-backed driver, transactions, the search SQL builder, install/upgrade — this crate is where SQLite-specific code actually lives |

`fhir-sqlite-store` depends on and re-exports [`fhir-store`](../../../fhir-store/)
for the engine-agnostic pieces — `Audit`, `AccessRecord`, the hash-chain
primitives — so `fhir_sqlite_store::Audit` and `fhir_sqlite_store::chain::KeyRing`
both resolve without a separate dependency.

The chapters that follow cover the storage model, querying with SQL, search,
FHIR-version handling, operations, the trust boundary, and the crate
architecture — read in that order if you are new to the port.
