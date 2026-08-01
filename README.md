# fhir-databases

**Store [FHIR](https://hl7.org/fhir/) resources in a SQL database as real
relational tables** — typed columns, child tables, foreign keys, check
constraints — not JSON blobs. Get them back losslessly.

Six ports, one specification, one engine.

| Port | Database | Status |
| --- | --- | --- |
| [`fhir-postgresql`](fhir-postgresql/) | PostgreSQL 18 | **Reference** — full store, full test suite |
| [`fhir-sqlite`](fhir-sqlite/) | SQLite 3 | **Store** — native, embeddable, no server |
| [`fhir-mysql`](fhir-mysql/) | MySQL 8.4 | **Store** |
| [`fhir-mariadb`](fhir-mariadb/) | MariaDB 11.4 | **Store** |
| [`fhir-mssql`](fhir-mssql/) | SQL Server | **Scaffold** — DDL only, no store |
| [`fhir-oracle`](fhir-oracle/) | Oracle Database | **Scaffold** — DDL is still MySQL's |

Those are conformance levels, defined in
[`spec/00-conformance.md`](spec/00-conformance.md). They are the honest version:
what has been *verified for that port*, not what its code contains. The
[conformance matrix](spec/conformance-matrix.md) breaks them down requirement by
requirement, and it is the document to read before choosing one.

> **Status: pre-release.** Each port's README now describes that port at its own
> conformance level; until 2026-07-31 all six carried the PostgreSQL reference's
> measured results with the engine name substituted, including in two ports with
> no store at all ([`spec/audit.md`](spec/audit.md) **F-01**, fixed).
>
> The per-port `book/` directories have **not** been rewritten and still contain
> PostgreSQL text. The [conformance matrix](spec/conformance-matrix.md) is the
> status document to trust, and [`spec/audit.md`](spec/audit.md) lists the eight
> findings still open.

## Also here: openEHR

The repository additionally contains an **openEHR** family of seven crates,
which shares this repository's engineering discipline and none of its code. They
are listed here because a reader who finds them by `ls` deserves to know what
they are before reading them.

| Crate | Purpose | Level |
| --- | --- | --- |
| [`openehr`](openehr/) | The Reference Model: data types, structures, EHR, change control, AQL parsing, redaction, a tamper-evident hash chain | 237 of 291 requirements verified |
| [`openehr-store`](openehr-store/) | The shared schema, dialect trait, commit rules, and conformance suite the five engine crates depend on | — |
| [`openehr-sqlite`](openehr-sqlite/) | SQLite 3 | **Store** — full suite against a real database |
| [`openehr-postgresql`](openehr-postgresql/) | PostgreSQL 18 | **Schema** — the server executes the DDL |
| [`openehr-mysql`](openehr-mysql/) | MySQL 8.4 | **Schema** — the server executes the DDL |
| [`openehr-mssql`](openehr-mssql/) | SQL Server 2022 | **Dialect** — DDL emitted, never executed |
| [`openehr-oracle`](openehr-oracle/) | Oracle Database | **Dialect** — DDL emitted, never executed |

Levels are defined in
[`openehr-store/spec/conformance.md`](openehr-store/spec/conformance.md). They
are deliberately stricter than a reader might expect: *Schema* means a real
server parsed the DDL, ran it twice, and was observed refusing to mutate an
append-only row — and reaching it found three defects in crates whose golden
DDL tests were all green ([`openehr/spec/audit.md`](openehr/spec/audit.md)
**A-13**, **A-14**, **A-15**).

Unlike the six FHIR ports, these five engine crates do **not** each own a copy
of the core. There is one `openehr-store`, and a dialect owns only type
spellings, quoting, placeholders, and append-only enforcement — which is why
**F-08** cannot recur there.

Two things openEHR-side are deliberately out of scope: archetypes and ADL, and
executing AQL (it is parsed and statically checked, never run).

## Why relational

JSON storage makes writing FHIR easy and querying it painful. Normalized storage
inverts that trade, and for a production clinical system the trade is right:

- **Integrity the database enforces** — enum columns backed by FHIR value sets,
  `CHECK` constraints on choice elements, typed dates and decimals, reference
  columns that can be joined.
- **SQL that reads like the domain** — `SELECT family FROM r5.patient_name`, no
  `->>'…'` path spelunking, and the query planner sees real column statistics.
- **Search that is just SQL** — FHIR search parameters compile to indexed
  predicates on ordinary columns.

The cost is thousands of generated tables per FHIR version — 7,355 for R5. That
is fine for a database and impossible for a human, so everything is generated
and every name is recorded in the relational map.

## How it works

```
FHIR spec packages          ──gen──▶  relational map + DDL  ──▶  assets/*.json.gz
(StructureDefinitions,                                            (committed)
 SearchParameters)
                                          │
  FHIR resource  ──shred──▶  rows  ───────┼───────  database
  FHIR resource  ◀─reconstruct──  rows  ──┘
```

A build-time generator reads each FHIR version's StructureDefinitions and
SearchParameters and emits two artifacts per version: the **DDL** and a compact
**relational map**. At runtime one generic engine walks any resource against the
map to shred it into rows, and walks the map in reverse to reconstruct the
identical resource. There is no per-resource handwritten code and no generated
Rust — three versions × ~150 resource types × deep nesting would explode compile
times for zero runtime benefit.

Round-trip fidelity is a tested invariant, decimal precision and partial dates
included. Search parameters compile against the same map into SQL.

**What differs between ports is two files.** `map/src/ddl.rs` decides which SQL
the generator emits; the `store` crate decides driver, transactions, and
placeholder syntax. Everything else — shred, reconstruct, fold, canonical JSON,
the whole generator — is byte-identical across all six.

## Quick start

The ports are **libraries**. There is no CLI and no server crate in this
monorepo, in any port ([`C0.18`](spec/00-conformance.md)).

SQLite needs no server, so it is the shortest path to a working store:

```toml
[dependencies]
fhir-sqlite-map   = { path = "fhir-sqlite/crates/fhir-sqlite-map" }
fhir-sqlite-store = { path = "fhir-sqlite/crates/fhir-sqlite-store" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

```rust
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::{Audit, sqlite::SqliteStore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The relational map ships as a committed asset — no FHIR packages needed.
    let bytes = std::fs::read("fhir-sqlite/assets/fhir-sqlite-relmap-r5.json.gz")?;
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

Then query it as what it is — a relational schema:

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

Full walkthroughs: [`doc/`](doc/index.md).

## Which port

| If you want | Use |
| --- | --- |
| Production today, with the full test suite behind it | `fhir-postgresql` |
| Embedded, no server, a single file | `fhir-sqlite` |
| An existing MySQL or MariaDB estate | `fhir-mysql`, `fhir-mariadb` |
| SQL Server or Oracle | not yet — see the [matrix](spec/conformance-matrix.md) |

[`doc/choosing-an-engine.md`](doc/choosing-an-engine.md) has the long version,
including what each engine costs you in its type bindings.

## What these libraries guarantee, and what they do not

Because the data is protected health information, the boundary is stated rather
than implied ([§12](spec/12-trust-principal-and-audit.md)):

| Guaranteed here | Your deployment must provide |
| --- | --- |
| Attribution on every write | Authentication |
| A disclosure record on every read | Authorization, scopes, compartments, consent |
| Tamper-evident history (SHA-256 + SHA3-256, optional HMAC) | TLS termination |
| Append-only history enforced in the database | Rate limiting per identity |
| Erasure that leaves a verifiable tombstone | Terminology validation |
| No PHI in logs at default level | Everything else a certified system needs |

These are components, not certified systems. They cannot make a deployment
compliant; they are built so as not to be the reason one cannot be.
[§13](spec/13-compliance-mapping.md) maps each obligation to a numbered
requirement and to the test that evidences it.

## Documentation

| | |
| --- | --- |
| [`doc/index.md`](doc/index.md) | **documentation hub** — tutorials, examples, comparisons |
| [`spec/index.md`](spec/index.md) | the normative specification, shared by all six ports |
| [`spec/conformance-matrix.md`](spec/conformance-matrix.md) | what each port actually satisfies |
| [`spec/audit.md`](spec/audit.md) | every known divergence, with evidence |
| [`AGENTS.md`](AGENTS.md) | contributing — human or agent |
| `fhir-<engine>/book/` | that port's user guide |
| `fhir-<engine>/plan.md` | that port's design decisions |

## Specification-driven

Behaviour is decided in [`spec/`](spec/index.md) before it is written in Rust,
and every requirement keeps a permanent number so a test, a commit, and an
auditor's workpaper can all point at the same thing years later.

The specification is **one copy, at the root**. Until this revision it was six
copies, one per port, identical apart from the product name — and they had begun
to drift exactly as you would expect. Each port now states only where it
*departs*, in a numbered dialect annex, so a departure has to be written down to
exist.

## License

MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only.
