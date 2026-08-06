# fhir-rust

A Rust monorepo for [FHIR](https://hl7.org/fhir/): the data model, six
relational database ports, and an HTTP surface over them.

| Family | What it is | Where |
| --- | --- | --- |
| **Model** | FHIR R3/R4/R5 (and R2/R6 in spec) as serde-serializable Rust types, generated from the official specification packages | [`fhir/`](fhir/) |
| **Persistence core** | the engine-agnostic half of storage: audit chain, attribution, result types | [`fhir-store/`](fhir-store/) |
| **Databases** | those resources stored as **real relational tables**, six SQL engines | [`fhir-postgresql/`](fhir-postgresql/) + 5 siblings |
| **HTTP surface** | a FHIR RESTful API server over one of those stores | [`fhir-loco/`](fhir-loco/) |

They stack in one direction: the model knows nothing about databases, the
database ports carry no HTTP or CLI, and the server adds only status codes.
[`spec/index.md`](spec/index.md) is the root of every specification and says
which one governs what.

## The database ports

**Store FHIR resources in a SQL database as real relational tables** — typed
columns, child tables, foreign keys, check constraints — not JSON blobs. Get
them back losslessly.

Six ports, one specification, one engine.

| Port | Database | Status |
| --- | --- | --- |
| [`fhir-postgresql`](fhir-postgresql/) | PostgreSQL 18 | **Reference** — full store, full test suite |
| [`fhir-sqlite`](fhir-sqlite/) | SQLite 3 | **Store** — native, embeddable, no server |
| [`fhir-mysql`](fhir-mysql/) | MySQL 8.4 | **Store** |
| [`fhir-mariadb`](fhir-mariadb/) | MariaDB 11.4 | **Store** |
| [`fhir-mssql`](fhir-mssql/) | SQL Server | **Store** — live-verified against `azure-sql-edge`, incl. `upgrade` (**F-65**) |
| [`fhir-oracle`](fhir-oracle/) | Oracle Database | **Store** — live-verified against `gvenzl/oracle-free` (**F-68**); no `upgrade`, `R4.5` open |

Those are conformance levels, defined in
[`spec/00-conformance.md`](spec/databases/00-conformance.md). They are the honest version:
what has been *verified for that port*, not what its code contains. The
[conformance matrix](spec/databases/conformance-matrix.md) breaks them down requirement by
requirement, and it is the document to read before choosing one.

> **Status: pre-release.** Each port's README now describes that port at its own
> conformance level; until 2026-07-31 all six carried the PostgreSQL reference's
> measured results with the engine name substituted, including in two ports with
> no store at all ([`spec/audit.md`](spec/databases/audit.md) **F-01**, fixed).
>
> The per-port `book/` directories were rewritten on 2026-08-03: each names its
> own engine and attributes every `serve`/endpoint to `fhir-loco`
> (**F-56**). The [conformance matrix](spec/databases/conformance-matrix.md) is
> the status document to trust, and [`spec/audit.md`](spec/databases/audit.md)
> lists the five findings still open.

## Also here: the model crate

[`fhir/`](fhir/) is the FHIR data model itself — every resource, complex
datatype, primitive newtype, and code system as idiomatic Rust, produced by a
generator that reads the official specification JSON. It has no database
dependency and no I/O, and the database ports do not depend on it: they shred
`serde_json::Value` against a generated relational map rather than against Rust
types. The two families share a domain and a discipline, not code.

| | |
| --- | --- |
| Releases modelled in code | R5 (5.0.0), R4 (4.0.1), R3 (3.0.2) |
| Releases modelled in its spec | additionally R2 (1.0.2) and R6 (ballot, `publish = false`) |
| Shape | one crate per release behind a cargo feature, ~135k generated lines each |
| Specification | [`fhir/spec/index.md`](fhir/spec/index.md) — 14 sections, ids `R1.x`–`R14.x` |

Its requirement ids share the `R4` prefix with this specification's §4 and mean
something entirely different; [`spec/index.md`](spec/index.md) records the clash
and how to cite around it.

## Also here: the persistence core and the HTTP surface

[`fhir-store/`](fhir-store/) is the **engine-agnostic half of persistence** —
the tamper-evident audit chain, the attribution and disclosure records, and the
value types every port's operations return. It links no driver and opens no
socket. It exists because `chain.rs` alone was 618 lines byte-identical in all
six ports, and unwatched by the shared-core gate.

[`fhir-loco/`](fhir-loco/) is a FHIR RESTful API server — Loco, Axum, Tokio and
Hyper — over `fhir-sqlite`. The ports deliberately ship no
server so that a program wanting FHIR storage does not also acquire a web
framework; this is where that surface lives, and its job is narrow: translate
HTTP to store calls and get the status codes right, so that a resource which was
deleted answers `410 Gone` and one that never existed answers `404 Not Found`.

It has **no specification yet**, which means none of those externally visible
promises can be cited by number or shown to have regressed. That gap is recorded
in [`spec/index.md`](spec/index.md#gaps) and, since it is now known to be *the*
service §10 and §12 describe, in [`spec/audit.md`](spec/databases/audit.md)
**F-58**.

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

The ports are **libraries**: no CLI, and no server crate *in any port*
([`C0.17`](spec/databases/00-conformance.md),
[`C0.18`](spec/databases/00-conformance.md)). The REST server is a separate
crate — [`fhir-loco`](fhir-loco/), on Loco.rs, Axum, Tokio and Hyper — which
mounts a FHIR API over a store.

SQLite needs no server, so it is the shortest path to a working store:

```toml
[dependencies]
fhir-sqlite-map   = "0.4"
fhir-sqlite-store = "0.4"   # add `features = ["r3", "r4"]` for other versions
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde_json = "1"
```

```rust
use std::sync::Arc;
use fhir_sqlite_map::model::RelMap;
use fhir_sqlite_store::{Audit, sqlite::SqliteStore};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The map is compiled in — feature `r5`, on by default. No file to read,
    // and no FHIR specification packages needed.
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
| SQL Server or Oracle | not yet — see the [matrix](spec/databases/conformance-matrix.md) |

[`doc/choosing-an-engine.md`](doc/choosing-an-engine.md) has the long version,
including what each engine costs you in its type bindings.

## What these libraries guarantee, and what they do not

Because the data is protected health information, the boundary is stated rather
than implied ([§12](spec/databases/12-trust-principal-and-audit.md)):

| Guaranteed here | Your deployment must provide |
| --- | --- |
| Attribution on every write | Authentication — unless you run [`fhir-loco`](fhir-loco/), which verifies a PASETO v4.public token on every request and has no unauthenticated mode |
| A disclosure record on every read | Authorization, scopes, compartments, consent |
| Tamper-evident history (SHA-256 + SHA3-256, optional HMAC) | TLS termination |
| Append-only history enforced in the database | Rate limiting per identity |
| Erasure that leaves a verifiable tombstone | Terminology validation |
| No PHI in logs at default level | Everything else a certified system needs |

These are components, not certified systems. They cannot make a deployment
compliant; they are built so as not to be the reason one cannot be.
[§13](spec/databases/13-compliance-mapping.md) maps each obligation to a numbered
requirement and to the test that evidences it.

## Documentation

| | |
| --- | --- |
| [`index.md`](index.md) | **every entry point in the repository**, in one page |
| [`doc/index.md`](doc/index.md) | documentation hub — tutorials, examples, comparisons |
| [`spec/index.md`](spec/index.md) | **the root of every specification** — which one governs what |
| [`spec/databases/index.md`](spec/databases/index.md) | the normative core, shared by all six ports |
| [`spec/databases/conformance-matrix.md`](spec/databases/conformance-matrix.md) | what each port actually satisfies |
| [`spec/databases/audit.md`](spec/databases/audit.md) | every known divergence, with evidence |
| [`spec/publishing.md`](spec/publishing.md) | what blocks these crates from crates.io |
| [`fhir/spec/index.md`](fhir/spec/index.md) | the model crate's specification |
| [`AGENTS.md`](AGENTS.md) | contributing — human or agent |
| `fhir-<engine>/book/` | that port's user guide |
| `fhir-<engine>/plan.md` | that port's design decisions |

## Specification-driven

Behaviour is decided in [`spec/`](spec/index.md) before it is written in Rust,
and every requirement keeps a permanent number so a test, a commit, and an
auditor's workpaper can all point at the same thing years later.

The database specification is **one copy, at the root**, in
[`spec/databases/`](spec/databases/index.md). Until this revision it was six
copies, one per port, identical apart from the product name — and they had begun
to drift exactly as you would expect. Each port now states only where it
*departs*, in a numbered dialect annex, so a departure has to be written down to
exist.

The model crate keeps its own specification in [`fhir/spec/`](fhir/spec/index.md),
because it is a different artifact with a different release cadence and a
different conformance question. Neither family's requirements bind the other;
[`spec/index.md`](spec/index.md) states that precedence rule and the one
prefix collision between them.

## License

`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only` — declared
identically by all 33 crates in the repository, across all four families. `OR`
means you choose; no obligation from one option carries into another.

Full statement: [`LICENSE.md`](LICENSE.md). Versions already on crates.io keep
the terms they were published under, since a published version is immutable.
