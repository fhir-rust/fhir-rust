# Install

These are **libraries**. There is no CLI and no server crate in any database
port (`C0.17`, `C0.18`) — you add a crate to a Rust project and call it. The one
exception is [`fhir-loco`](fhir-loco/), a separate crate that mounts a FHIR® REST
API over a store; it is a binary you run.

All 34 crates are published on crates.io. Nothing needs to be built from source,
and no FHIR specification package needs downloading: the relational maps are
compiled into the map crates.

## Requirements

| | |
| --- | --- |
| Rust | **1.96**, every crate in the repository alike. The policy is current-minus-two (`RV1.1`) |
| Edition | 2024 |
| A database | only for the ports, and only the one you pick — see the table below |

`fhir-sqlite` needs no server at all, so it is the shortest path to a working
store.

## Pick a database

| Port | Engine | Declared floor | Conformance level |
| --- | --- | --- | --- |
| `fhir-postgresql` | PostgreSQL | 18 | **Reference** — full store, full test suite |
| `fhir-sqlite` | SQLite | 3.35+ | Store — embeddable, no server |
| `fhir-mysql` | MySQL | 8.4 | Store |
| `fhir-mariadb` | MariaDB | 11.4 | Store |
| `fhir-mssql` | SQL Server | 2019+ | Store — see the advisory note below |
| `fhir-oracle` | Oracle Database | 12.2+ | Store — `R4.5` snapshot reads are an open gap |

Read [`doc/choosing-an-engine.md`](doc/choosing-an-engine.md) before deciding,
and the [conformance matrix](spec/databases/conformance-matrix.md) before
relying on any of it. The levels are what has been *verified for that port*, not
what its code contains.

> **`fhir-mssql` carries a known advisory risk.** Four TLS advisories reach the
> shipping `fhir-mssql-store` through its driver stack, recorded as **F-67**.
> The project accepted this risk formally on 2026-08-28 rather than chase a
> replacement — investigated and priced a driver replacement first, found none
> viable (see `M14.34` in `fhir-mssql/spec/14-mssql-dialect.md`) — but that is
> the project's risk tolerance, not necessarily yours. Run `cargo audit` and
> make your own decision before depending on that port.

## Install a store

Each port ships three crates. You normally need two:

```toml
[dependencies]
fhir-sqlite-map   = "0.5"   # the relational map, compiled in
fhir-sqlite-store = "0.5"   # the store: put/get/delete/history/search/audit
tokio      = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

Substitute the engine name for any other port — `fhir-postgresql-map` and
`fhir-postgresql-store`, `fhir-oracle-map` and `fhir-oracle-store`, and so on.
The third crate, `fhir-<engine>-gen`, is the schema generator; you need it only
if you are regenerating assets from FHIR specification packages.

### FHIR versions

R5 is on by default. Add the others as features, on **both** crates:

```toml
fhir-sqlite-map   = { version = "0.5", features = ["r3", "r4"] }
fhir-sqlite-store = { version = "0.5", features = ["r3", "r4"] }
```

## Use it

```rust
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::{Audit, sqlite::SqliteStore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = Arc::new(RelMap::bundled("r5")?);

    let store = SqliteStore::open("clinic.sqlite", map).await?;
    store.init("r5-baseline").await?;        // ~7,400 tables, one transaction

    let patient = serde_json::json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{ "family": "Ærø", "given": ["Anna"] }],
        "birthDate": "1974-12"               // a partial date, preserved verbatim
    });

    store.put(&patient, &Audit::cli()).await?;

    let back = store.get("Patient", "example").await?.unwrap();
    assert_eq!(back, patient);               // losslessly, including "1974-12"

    let hits = store.search("Patient", &[("name".into(), "aero".into())], 50, 0).await?;
    println!("{hits:?}");
    Ok(())
}
```

`store.init` creates the schema — 7,355 tables for R5 — in one transaction. It
takes seconds, not minutes, and you do it once per database.

Then query it as the relational schema it is:

```sql
SELECT n.family, count(o.id) AS observations
  FROM patient p
  JOIN patient_name n ON n.rid = p.id AND n.ords = '{1}'
  LEFT JOIN observation o
    ON o.subject_ref_type = 'Patient' AND o.subject_ref_id = p.id
 GROUP BY n.family
 ORDER BY observations DESC;
```

## Connecting to a server engine

The five server ports take a DSN rather than a file path. Each port's
`README.md` documents its connection type and options; the shapes differ because
the drivers do.

**Transport is encrypted by default and this is deliberate** — the connection
carries PHI (`O10.7`). On PostgreSQL, `PGSSLMODE` defaults to a verifying
`Require`; set `PGSSLMODE=prefer` to relax it and `PGSSLROOTCERT` for a private
CA. Other ports have equivalent knobs in their annexes.

## Install the model crate on its own

[`fhir/`](fhir/) is independent of all of the above: FHIR as Rust types, no
database, no I/O.

```toml
[dependencies]
fhir = { version = "4.1", features = ["r5"] }   # or r2, r3, r4, r4b, r6
```

The database ports do **not** depend on it: they shred `serde_json::Value`
against a generated relational map rather than against Rust types.

## Install the HTTP surface

[`fhir-loco`](fhir-loco/) is a FHIR RESTful API server on Loco, Axum, Tokio and
Hyper, over `fhir-sqlite` or `fhir-postgresql`:

```sh
cargo install fhir-loco
```

Its endpoints, configuration and status-code behaviour are specified in
[`fhir-loco/spec/`](fhir-loco/spec/index.md) (`SV1.x`–`SV4.x`). Note `SV3.11`:
a non-loopback plaintext bind refuses to boot without an explicit
acknowledgement from the deployment.

## Build from source

```sh
git clone https://github.com/fhir-rust/fhir-rust.git
cd fhir-rust
cargo build --all-targets      # inside any one family's directory
cargo test                     # unit and integration tests, no database needed
```

Each workspace is independent; there is no root workspace spanning all four
families.

To run the tests that need a real engine, each port ships a container script
that pins the same version CI uses, so a green local run and a green CI run mean
the same thing:

```sh
cd fhir-postgresql
scripts/db.sh up        # start the container and wait until it answers
scripts/db.sh corpus    # lay out the FHIR example corpus the tests need
scripts/db.sh test      # run the live suite against it
scripts/db.sh down      # stop and remove it
```

Podman by default, Docker if that is what is installed. `fhir-sqlite` has one
too, though it needs no server.

Before touching any file that is shared across all six ports — `shred.rs`,
`reconstruct.rs`, `fold.rs`, `canon.rs`, and everything under `gen/` — run the
gate that keeps them identical:

```sh
./scripts/check-shared-core.sh          # --diff to see what moved
```

[`AGENTS.md`](AGENTS.md) explains why that matters and what else does.

## Verify what you installed

```sh
cargo audit                              # known advisories; see F-67 above
./scripts/check-published-match.sh       # every crate's source version vs crates.io
```

Documentation for every published crate is on
[docs.rs](https://docs.rs/fhir-sqlite-store).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
