# fhir-sqlite — FHIR in SQLite, relationally

Store [FHIR](https://hl7.org/fhir/) resources in SQLite as **real relational
tables** — typed columns, child tables, foreign keys, and check constraints —
not JSON blobs. Get them back losslessly.

No server, no daemon, one file per FHIR version. The SQLite engine is
**bundled** rather than linked from the host, so the schema runs against a
pinned version rather than whatever the machine happens to ship.

- **Given a FHIR resource**, `fhir-sqlite` shreds it into normalized tables
  generated from the FHIR specification itself.
- **Given a type and id**, it reconstructs the identical resource from those
  tables — decimal precision, partial dates, and array order included.

Supported FHIR versions: **R5 (5.0.0, default), R4 (4.0.1), R3 (3.0.2)**, each
with its own generated schema in its own database file.

> **Status: Store level, pre-release.** The generator, the shred/reconstruct
> engine, the SQLite store, search, history, and the audit chain work.
>
> The suite needs no server and no environment variables, so it always runs:
> 43 tests covering round-trip by column type, concurrency, redaction, search,
> history, and the audit chain. Writing the concurrency and redaction suites
> found four real defects, three of them High — see **F-20** to **F-23** in the
> [audit register](../spec/audit.md). Notably, `Patient.active` did not survive
> a round trip, and nothing in 27 passing tests noticed, because every fixture
> was built from strings.
>
> **Not yet:** `transact_audited` returns `Unsupported` (deliberately — see
> below), and `resign_history`, `chain_witness`, and `export` are unimplemented;
> each fails saying so rather than pretending. Two `T11.8`
> gaps remain: per-algorithm independent tamper detection, and the
> truncated-chain-versus-checkpoint case. Read the
> [conformance matrix](../spec/conformance-matrix.md) before deploying.
>
> Normative behaviour: the shared [core spec](../spec/index.md) plus this port's
> [dialect annex](spec/14-sqlite-dialect.md).

## Why relational

JSON storage makes writing FHIR easy and querying it painful. Normalized
storage inverts that trade, and for a production clinical system the trade is
right:

- **Integrity the database enforces** — `CHECK` constraints on choice elements,
  enum columns from FHIR value sets, reference columns that can be joined.
- **SQL that reads like the domain** — `SELECT family FROM patient_name`, no
  `->>'…'` path spelunking.
- **Search that is just SQL** — FHIR search parameters compile to indexed
  predicates on ordinary columns.

## Quick start

This is a **library**, not a CLI. There is no `fhir-sqlite` binary and no
server crate in this workspace.

```toml
[dependencies]
fhir-sqlite-map   = { path = "crates/fhir-sqlite-map" }
fhir-sqlite-store = { path = "crates/fhir-sqlite-store" }
tokio      = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

```rust
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::{Audit, sqlite::SqliteStore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The relational map ships as a committed asset — no FHIR packages needed.
    let bytes = std::fs::read("assets/fhir-sqlite-relmap-r5.json.gz")?;
    let map = Arc::new(RelMap::from_gz_bytes(&bytes)?);

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

## The API

| Operation | Notes |
| --- | --- |
| `SqliteStore::open(path, map)` | one file per FHIR version |
| `init(checksum)` | idempotent; refuses a different artifact |
| `put`, `put_audited` | one transaction per resource |
| `get`, `vread`, `history` | |
| `delete`, `delete_audited` | soft: history survives |
| `search`, `search_full`, `search_page` | cursor paging |
| `conditional_create_audited`, `conditional_delete_audited` | |
| `verify_audit`, `log_access`, `emit_checkpoint` | |
| `purge` | GDPR Art. 17 erasure, leaves a tombstone |
| `transact_audited` | returns `Unsupported` — see below |
| `upgrade`, `backfill_norm` | additive diff applies; dropping needs `--allow-destructive` |
| `resign_history`, `chain_witness`, `export` | unimplemented |

## What is specific to SQLite

Full detail in the [dialect annex](spec/14-sqlite-dialect.md); the highlights:

- **`Numeric`, `Date`, `Timestamptz`, and `Jsonb` are all `TEXT`.** A decimal
  cannot be `REAL`, because binary floating point cannot hold `1.50` distinctly
  from `1.5`, and round-trip fidelity is the invariant the project exists to
  keep. Dates are fixed-width ISO-8601, normalized in Rust so lexicographic
  order equals chronological order.
- **`ords` is `TEXT`**, holding the same array literal every other port stores.
  The database never orders, compares, or subscripts it, so a text image
  suffices — but PostgreSQL's `ords[1] = 1` subscript idiom does **not** work
  here. Use `ords LIKE '{1,%'`.
- **The engine is bundled and pinned**, because the generated DDL depends on
  version-specific behaviour and "works on mine" is not a trade this project
  can afford.
- **No TLS, because there is no connection.** The obligation is at rest: file
  permissions and storage encryption are the deployment's to set.
- **An upgrade needs a database `init` recorded the map asset into.** That began
  with this revision; a database installed earlier has nothing to diff against,
  so `upgrade` refuses it by name and the migration is still a reload. There is
  no way around it — the old map was never written down, and inferring it from
  the installed schema would be guessing where guessing wrong corrupts data.

### `transact_audited` refuses, on purpose

A FHIR transaction Bundle is atomic by definition, and a compensating unwind is
not: readers between operations observe a half-applied bundle, and a process
dying mid-unwind leaves partial state permanently. Returning `Unsupported` is
the correct answer until `put` and `delete` are split so their bodies can run
inside a caller-supplied `BEGIN IMMEDIATE`.

## Production posture

Transactional writes, version history and an audit envelope on every resource,
a tamper-evident hash chain under SHA-256 **and** SHA3-256, disclosure logging,
append-only history enforced by a trigger, and erasure that leaves a verifiable
tombstone.

`fhir-sqlite` handles PHI. It does **not** authenticate or authorize — that is
the deployment's perimeter — but it does record who acted, on every write and
every read. See the [trust boundary](../doc/trust-boundary.md) for the full
table of what is guaranteed here and what you must provide.

## Documentation

- **[The book](book/src/SUMMARY.md)** — getting started, storage model,
  querying, search, operations, architecture.
- [`spec/index.md`](spec/index.md) — this port's spec index and departures; the
  normative core is shared at [`../spec/`](../spec/index.md).
- [`../doc/`](../doc/index.md) — monorepo tutorials and examples.
- [`../spec/conformance-matrix.md`](../spec/conformance-matrix.md) — what this
  port actually satisfies.
- [`plan.md`](plan.md) · [`tasks.md`](tasks.md) · [`CHANGELOG.md`](CHANGELOG.md)
- [`doc/benchmarks.md`](doc/benchmarks.md) · [`doc/ci.md`](doc/ci.md)

## License

MIT OR Apache-2.0.
