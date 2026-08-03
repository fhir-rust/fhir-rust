# fhir-postgresql — FHIR in PostgreSQL, relationally

Store [FHIR](https://hl7.org/fhir/) resources in PostgreSQL 18 as **real
relational tables** — typed columns, child tables, foreign keys, and check
constraints — not JSON or JSONB blobs. Get them back losslessly.

- **Given a FHIR resource**, `fhir-postgresql` shreds it into normalized tables
  generated from the FHIR specification itself.
- **Given a type and id**, it reconstructs the identical resource from those
  tables — decimal precision, partial dates, and array order included.

Supported FHIR versions: **R5 (5.0.0, default), R4 (4.0.1), R3 (3.0.2)** —
each with its own generated schema, installed side by side in PostgreSQL
schemas `r5`, `r4`, `r3`.

> **Status: Reference level, pre-release.** This is the reference port: the
> only one whose test suite substantiates its claims, and the source of the
> numbers below.
>
> The generator, the shred/reconstruct engine, the PostgreSQL store, and search
> all work. All **7,399 official FHIR example resources** (R3 + R4 + R5)
> round-trip **losslessly** through the fully normalized schema — in memory,
> through live PostgreSQL 18, and 10,000 generated property-test cases besides.
> 94.8% of R5 search parameters compile to indexed SQL. Bulk load runs at 6,146
> resources/sec with 1.18 ms reads. Concurrency, audit, redaction, and upgrade
> suites run against a live server in CI.
>
> **This is a library.** There is no CLI crate and no server crate in this
> workspace — `crates/` contains `-map`, `-gen`, and `-store`. Until 2026-07-31
> this file documented `cargo install --path crates/fhir-postgresql` and a
> `fhir-postgresql serve` REST API; neither exists here, and spec sections 7
> (REST API) and 8 (CLI) are retired for that reason
> ([`C0.15`](../spec/databases/00-conformance.md), audit **F-01**).
>
> **TLS verifies by default.** `PGSSLMODE` defaults to `require`, which
> validates the server certificate *and* hostname — stricter than libpq, whose
> `require` validates nothing. Set `PGSSLMODE=prefer` or `disable` only if you
> mean it (**F-17**, `M14.27`).
>
> The hash-chain pre-image is now computed in Rust from the shared `canon.rs`
> (**F-07** fixed, [`M14.12`](spec/14-postgresql-dialect.md)), so a chain
> written here can be verified by any port holding that file. That was a
> **format change**: a database written before this release must be reloaded,
> and its existing rows will report as chain breaks until it is.
>
> Normative behaviour: the shared [core spec](../spec/databases/index.md) plus this
> port's [dialect annex](spec/14-postgresql-dialect.md); measurements:
> [`doc/benchmarks.md`](doc/benchmarks.md).

## Why relational

JSONB storage makes writing FHIR easy and querying it painful. Normalized
storage inverts that trade, and for a production clinical system the trade is
right:

- **Integrity the database enforces** — enum columns backed by FHIR value
  sets, `CHECK` constraints on choice elements, typed dates and decimals,
  reference columns that can be joined and (optionally) constrained.
- **SQL that reads like the domain** — `SELECT family FROM r5.patient_name`,
  no `->>'…'` path spelunking, and the query planner sees real column
  statistics.
- **Search that is just SQL** — FHIR search parameters compile to indexed
  predicates on ordinary columns.

## Quick start

```toml
[dependencies]
fhir-postgresql-map   = { path = "crates/fhir-postgresql-map" }
fhir-postgresql-store = { path = "crates/fhir-postgresql-store" }
tokio      = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

```rust
use std::sync::Arc;
use fhir_postgresql_map::model::RelMap;
use fhir_postgresql_store::{Store, pg_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The relational map ships as a committed asset — no FHIR packages needed.
        let map = Arc::new(RelMap::bundled("r5")?);   // compiled in (feature `r5`)

    // TLS verifies by default (PGSSLMODE=require). Set PGSSLROOTCERT if
    // your server uses a private CA.
    let cfg = pg_config(Some("host=localhost user=you dbname=clinic"))?;
    let store = Store::connect(cfg, map).await?;
    store.init("r5-baseline").await?;   // 7,355 tables, staged and renamed in

    let patient = serde_json::json!({
        "resourceType": "Patient",
        "id": "example",
        "name": [{ "family": "Ærø", "given": ["Anna"] }],
        "birthDate": "1974-12"          // a partial date, preserved verbatim
    });

    store.put(&patient).await?;

    let got = store.get("Patient", "example").await?.unwrap();
    assert_eq!(got.resource, patient);  // losslessly, including "1974-12"

    // Accent- and case-insensitive by construction: "aero" finds "Ærø".
    let hits = store.search("Patient", &[("name".into(), "aero".into())], 50, 0).await?;
    println!("{hits:?}");
    Ok(())
}
```

Then query relationally:

```sql
SELECT n.family, count(o.id) AS observations
  FROM r5.patient p
  JOIN r5.patient_name n ON n.rid = p.id AND n.ords = '{1}'
  LEFT JOIN r5.observation o
    ON o.subject_ref_type = 'Patient' AND o.subject_ref_id = p.id
 GROUP BY n.family
 ORDER BY observations DESC;
```

Every child table addresses its rows with `rid` (the resource id) and
`ords smallint[]` (the 1-based index path through repeating elements), so
arbitrarily nested — even recursive — structure stays joinable. PostgreSQL is
the **only** port with a native array type here, so `ords[1] = 1` works only on
this one; `ords LIKE '{1,%'` is the portable form.

## The API

| Operation | Notes |
| --- | --- |
| `Store::connect(cfg, map)` | SSL policy from `PGSSLMODE`; see `M14.27` |
| `init`, `init --upgrade`, `drop_schema` | staged install; the only port with `upgrade` |
| `put`, `put_audited`, `put_if` | one transaction per resource |
| `get`, `vread`, `history`, `status` | reads run in one `REPEATABLE READ READ ONLY` snapshot |
| `delete`, `delete_audited` | soft: history survives |
| `search`, `search_full`, `search_page` | cursor paging |
| `transact_audited` | genuinely atomic transaction Bundles |
| `conditional_create`, `conditional_delete` | with `_audited` variants |
| `verify_audit`, `chain_witness`, `resign_history` | tamper evidence |
| `log_access`, `emit_checkpoint` | disclosure logging |
| `purge` | GDPR Art. 17 erasure, leaves a tombstone |

## Architecture in one paragraph

A build-time generator (`fhir-postgresql-gen`) reads each FHIR version's
StructureDefinitions and SearchParameters and emits two artifacts per
version: the **DDL** (every resource's base table plus child tables for
repeating and nested elements) and a compact **relational map**. At runtime a
single generic engine walks any resource against the map to shred it into
rows, and walks the map in reverse to reconstruct the identical resource —
round-trip fidelity is a tested invariant, including decimal precision.
Search parameters compile against the same map into SQL. Storage access is
tokio-postgres with a deadpool pool, every value crossing the wire as text with
explicit casts so lexical fidelity survives in both directions; every write is
one transaction, with optimistic concurrency available through `put_if`. The
[`fhir`](https://crates.io/crates/fhir) crate supplies the typed R3/R4/R5
model for optional strict validation.

## Production posture

`fhir-postgresql` targets mission-critical clinical deployment: transactional
writes, version history and an audit envelope on every resource, optimistic
locking, a tamper-evident hash chain under SHA-256 **and** SHA3-256, disclosure
logging, append-only history enforced by a trigger, erasure that leaves a
verifiable tombstone, structured logging with `tracing`, connection pooling,
and a versioned `init --upgrade` migration path — the only port that has one.

Metrics, health endpoints, and HTTP-level concerns belong to a service layer
that does not exist in this monorepo; the requirements for it are retained and
marked `[service]` in [`../spec/10-operations.md`](../spec/databases/10-operations.md).

`fhir-postgresql` handles PHI. It does **not** authenticate or authorize — that
is the deployment's perimeter — but it does record who acted, on every write and
every read. See the [trust boundary](../doc/trust-boundary.md) for the full
table of what is guaranteed here and what you must provide. Two operational
notes that are easy to miss: TLS now verifies by default (`M14.27`), so the
thing to check is that `PGSSLROOTCERT` points at your CA if the server uses a
private one — and supply the chain key from a **file** rather than an
environment variable (`M3.16b`).

## Documentation

- **[The book](book/src/SUMMARY.md)** — getting started, the storage
  model, querying, search, operations, architecture. Its REST chapters describe
  a service layer that does not exist here (`C0.17`).
- [`spec/index.md`](spec/index.md) — this port's spec index; the normative
  core is shared at [`../spec/`](../spec/databases/index.md), and this port's departures
  are in [`spec/14-postgresql-dialect.md`](spec/14-postgresql-dialect.md).
- [`../spec/conformance-matrix.md`](../spec/databases/conformance-matrix.md) — what this
  port actually satisfies, requirement by requirement.
- [`../doc/`](../doc/index.md) — monorepo tutorials and examples.
- [`plan.md`](plan.md) — design decisions, risks, milestones.
- [`tasks.md`](tasks.md) — the implementation work breakdown.
- [`doc/benchmarks.md`](doc/benchmarks.md) — measured performance.
- [`doc/ci.md`](doc/ci.md) — the gates on GitHub and Codeberg, and how
  releases are cut.
- [`CHANGELOG.md`](CHANGELOG.md).

## License

MIT OR Apache-2.0.
