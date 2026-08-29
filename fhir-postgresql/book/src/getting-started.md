# Getting started

`fhir-postgresql` is a library, not a binary. There is no `cargo install`, no
`fhir-postgresql init` shell command, and no `serve` — see the banner on the
[introduction](introduction.md). Everything below is Rust, called from your
own `main.rs` or service.

You need PostgreSQL 18 and Rust 1.96+.

## Add the dependencies

```toml
[dependencies]
fhir-postgresql-map   = { path = "crates/fhir-postgresql-map" }
fhir-postgresql-store = { path = "crates/fhir-postgresql-store" }
tokio      = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

(Outside this monorepo these would be version dependencies —
`fhir-postgresql-map = "0.4"` — once published; see the port
[`README.md`](../../README.md) for the current publishing status.)

## Connect, install, write, read

This mirrors the program the port `README.md` verifies; it is not
illustrative pseudocode — every call is a real, checked method on `Store`.

```rust
use std::sync::Arc;
use fhir_postgresql_map::model::RelMap;
use fhir_postgresql_store::{Store, pg_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The relational map ships as a committed, compiled-in asset — no FHIR
    // specification packages needed at runtime (feature `r5`, on by default).
    let map = Arc::new(RelMap::bundled("r5")?);

    // Connection settings come from the standard PG* environment variables
    // (PGHOST, PGUSER, PGDATABASE, PGPASSWORD, PGPORT, PGSSLMODE, ...), or
    // pass an explicit DSN to `pg_config`. TLS verifies the server
    // certificate and hostname by default (`PGSSLMODE=require`, `M14.27`);
    // see the [dialect annex](../../spec/14-postgresql-dialect.md) before
    // relaxing it.
    let cfg = pg_config(Some("host=localhost user=you dbname=clinic"))?;
    let store = Store::connect(cfg, map).await?;

    // Applies the generated DDL — 7,355 tables for R5 — staged under a
    // temporary schema and renamed into place in one statement (`M14.14`).
    // The checksum names *this* install; re-running it with the same
    // checksum is a no-op, and installing over a schema built from a
    // different map is refused.
    store.init("r5-baseline").await?;

    let patient = serde_json::json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{ "family": "Ærø", "given": ["Anna"] }],
        "birthDate": "1974-12"          // a partial date, preserved verbatim
    });

    let outcome = store.put(&patient).await?;
    println!("wrote version {}", outcome.version_id);

    let got = store.get("Patient", "example").await?.unwrap();
    assert_eq!(got.resource, patient);  // losslessly, including "1974-12"

    // Accent- and case-insensitive by construction: "aero" finds "Ærø".
    let hits = store.search("Patient", &[("name".into(), "aero".into())], 50, 0).await?;
    println!("{hits:?}");
    Ok(())
}
```

See [`crates/fhir-postgresql-store/src/lib.rs`](../../crates/fhir-postgresql-store/src/lib.rs)
for the exact signatures of `connect`, `init`, `put`, `get`, and `search`.

## Recording who did it

`put` and `get` above carry no attribution: `put` internally calls the
audited form with `Audit::unattributed()`, which records the actor as
`"unauthenticated"` rather than leaving the column blank (`M3.15`). A caller
that knows who is acting should say so:

```rust
use fhir_postgresql_store::Audit;

let audit = Audit::principal("practitioner-42", "header:X-Fhir-Principal")
    .with_reason(Some("treatment".to_string()));

store.put_audited(&patient, None, &audit).await?;
```

`None` here means "no optimistic-concurrency check"; pass
`Some(expected_version)` (or `Some(0)` for "must not already exist yet") to
get a `StoreError::Conflict { expected, found }` instead of a silent
overwrite when another writer got there first.

## Reads are a database guarantee, not just a library one

`store.get(...)` runs every multi-table read inside one
`REPEATABLE READ READ ONLY` transaction (`R4.5`, `M14.15`), so a concurrent
write between statements cannot produce a resource that never existed —
base columns from one version, child rows from the next. That guarantee is
only as real as the database enforcing it; `cargo test` with no server
configured proves nothing about it — see [Operations](operations.md) and the
port's `AGENTS.md` on why the live suite is the gate that matters.

## Next

- [The storage model](storage-model.md) — what `init` actually creates.
- [Querying with SQL](querying.md) — the tables are yours; query them
  directly alongside the `Store` API.
- [Search](search.md) — what `store.search(...)` supports today.
- [Operations](operations.md) — install, upgrade, backup, and the audit
  chain, called from Rust.
