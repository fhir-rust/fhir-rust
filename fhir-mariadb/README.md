# fhir-mariadb — FHIR in MariaDB 11.4, relationally

Store [FHIR](https://hl7.org/fhir/) resources in MariaDB 11.4 as **real relational
tables** — typed columns, child tables, foreign keys, and check constraints —
not JSON blobs. Get them back losslessly.

- **Given a FHIR resource**, `fhir-mariadb` shreds it into normalized tables
  generated from the FHIR specification itself.
- **Given a type and id**, it reconstructs the identical resource from those
  tables — decimal precision, partial dates, and array order included.

Supported FHIR versions: **R5 (5.0.0, default), R4 (4.0.1), R3 (3.0.2)**, each
with its own generated schema in its own database (`r5`, `r4`, `r3`).

> **Status: Store level, pre-release.** The generator, the shred/reconstruct
> engine, the MariaDB 11.4 store, search, history, and the audit chain work, and
> a live MariaDB 11.4 gate runs them in CI.
>
> **Recently fixed, and worth knowing about:** reading any resource carrying a
> boolean, an integer, or a date **panicked** inside the store — the read path
> asked the driver for a `String` and MariaDB returns `Value::Int`/`Value::Date`
> over the binary protocol. Since almost every real `Patient` has `active` or
> `birthDate`, this port could not read real FHIR data. Fixed and verified
> against a live server; see **F-20**. It went unnoticed because no test fixture
> contained a boolean, an integer, or a date.
>
> Two more were found the same way, by porting the concurrency suite: reads
> **tore** under concurrent writes (no read transaction, **F-21**), and
> `version_id` was assigned with **no row lock**, so only 1 of 8 concurrent
> writes to a resource succeeded — the rest failed on a raw duplicate-key error
> (**F-24**). Both fixed; 8 of 8 now succeed with a verifying chain.
>
> **Not yet:** there is no optimistic concurrency at all — no `put_audited` and
> no `expected_version` anywhere in the crate — and no `transact_audited`,
> conditional create/delete, `init --upgrade`, `emit_checkpoint`,
> `chain_witness`, or `resign_history`. Read the
> [conformance matrix](../spec/databases/conformance-matrix.md) before deploying.
>
> Normative behaviour: the shared [core spec](../spec/databases/index.md) plus this port's
> [dialect annex](spec/14-mariadb-dialect.md).

## Why relational

JSON storage makes writing FHIR easy and querying it painful. Normalized
storage inverts that trade, and for a production clinical system the trade is
right:

- **Integrity the database enforces** — `CHECK` constraints on choice elements,
  enum columns from FHIR value sets, reference columns that can be joined.
- **SQL that reads like the domain** — `SELECT family FROM patient_name`, no
  JSON path spelunking, and the planner sees real column statistics.
- **Search that is just SQL** — FHIR search parameters compile to indexed
  predicates on ordinary columns.

## Quick start

This is a **library**, not a CLI. There is no `fhir-mariadb` binary and no server
crate in this workspace.

```toml
[dependencies]
fhir-mariadb-map   = { path = "crates/fhir-mariadb-map" }
fhir-mariadb-store = { path = "crates/fhir-mariadb-store" }
tokio      = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

```rust
use std::sync::Arc;
use fhir_mariadb_map::model::RelMap;
use fhir_mariadb_store::{Audit, mariadb::MariaDbStore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The relational map ships as a committed asset — no FHIR packages needed.
        let map = Arc::new(RelMap::bundled("r5")?);   // compiled in (feature `r5`)

    let store = MariaDbStore::connect("mysql://root@127.0.0.1:3306", map).await?;
    store.init("r5-baseline").await?;

    let patient = serde_json::json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{ "family": "Ærø", "given": ["Anna"] }],
        "birthDate": "1974-12"               // a partial date, preserved verbatim
    });

    store.put(&patient, &Audit::cli()).await?;

    let back = store.get("Patient", "example").await?.unwrap();
    assert_eq!(back, patient);               // losslessly, including "1974-12"

    // Accent- and case-insensitive by construction: "aero" finds "Ærø".
    let hits = store.search("Patient", &[("name".into(), "aero".into())], 50, 0).await?;
    println!("{hits:?}");
    Ok(())
}
```

Then query it as what it is:

```sql
SELECT n.family, count(o.id) AS observations
  FROM patient p
  JOIN patient_name n ON n.rid = p.id AND n.ords = '{1}'
  LEFT JOIN observation o
    ON o.subject_ref_type = 'Patient' AND o.subject_ref_id = p.id
 GROUP BY n.family
 ORDER BY observations DESC;
```

Every child table addresses its rows with `rid` (the resource id) and `ords`
(the 1-based index path through repeating elements), so arbitrarily nested —
even recursive — structure stays joinable.

## What is specific to MariaDB 11.4

Full detail in the [dialect annex](spec/14-mariadb-dialect.md); the highlights:

- **`Numeric` is `TEXT`, not `DECIMAL`.** `DECIMAL(65,30)` returns `1.50` as
  `1.500000000000000000000000000000` — a fixed declared scale cannot preserve a
  per-value lexical form, and round-trip fidelity is the invariant this project
  exists to keep. Range search is served by a derived sort column.
- **`TextC` is `TEXT COLLATE utf8mb4_nopad_bin`.** Binary, and **NO PAD**: under a
  PAD SPACE collation `'Smith' = 'Smith '` is true, which would silently widen
  `:exact` matching and weaken primary keys. That is MariaDB's spelling of the property; MySQL spells it `utf8mb4_0900_bin`.
- **`Timestamptz` is `DATETIME(6)`, not `TIMESTAMP`.** `TIMESTAMP` converts on
  the session time zone and its range ends in 2038.
- **`Jsonb` is `LONGTEXT`, not `JSON`.** The hash chain commits to bytes
  canonicalized in Rust; a `JSON` column re-normalizes what it is given, so the
  bytes read back would not be the bytes signed and every chain would fail
  verification.
- **`ords` is `TEXT`**, holding the same array literal every other port stores.
  PostgreSQL's `ords[1] = 1` subscript idiom does not work here; use
  `ords LIKE '{1,%'`.

### Independent from fhir-mysql

This project began as a fork of the sibling `fhir-mysql` port, and the two are
**independent from here on** (`M14.0a`–`M14.0c`). This port should use whatever
MariaDB does best and must not restrict itself to syntax MySQL also accepts. A
schema installed here is not required to be readable by `fhir-mysql`, and
nothing should be held back to keep it so.

What the two do continue to share is *behaviour*, not SQL: the round-trip
engine, the fold, the canonical JSON, and the stored `ords` image are identical
by construction.

## Running the tests

Most of what this library guarantees is a database guarantee, so the live suite
is the gate that means something:

```sh
scripts/db.sh up      # start pinned MariaDB 11.4 in a container
scripts/db.sh test    # up, then the live suite
scripts/db.sh down
```

`cargo test` alone passes with no database, because the corpus- and
database-driven tests self-skip.

## Production posture

Transactional writes, version history and an audit envelope on every resource,
a tamper-evident hash chain under SHA-256 **and** SHA3-256, disclosure logging,
append-only history enforced by triggers, and erasure that leaves a verifiable
tombstone.

`fhir-mariadb` handles PHI. It does **not** authenticate or authorize — that is the
deployment's perimeter — but it does record who acted, on every write and every
read. See the [trust boundary](../doc/trust-boundary.md) for the full table of
what is guaranteed here and what you must provide.

## Documentation

- **[The book](book/src/SUMMARY.md)** — getting started, storage model,
  querying, search, operations, architecture.
- [`spec/index.md`](spec/index.md) — this port's spec index and departures; the
  normative core is shared at [`../spec/`](../spec/databases/index.md).
- [`../doc/`](../doc/index.md) — monorepo tutorials and examples.
- [`../spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) — what this
  port actually satisfies.
- [`plan.md`](plan.md) · [`tasks.md`](tasks.md) · [`CHANGELOG.md`](CHANGELOG.md)
- [`doc/benchmarks.md`](doc/benchmarks.md) · [`doc/ci.md`](doc/ci.md)

## License

MIT OR Apache-2.0.
