# Comparisons

Where this project sits relative to the FHIR® implementations you have probably
already looked at, and to the other FHIR crates in Rust.

**Assessed 2026-08-26 against each project's public documentation.** Every
statement about another project is a statement about someone else's software,
made by someone who does not maintain it — so it is written at a level of
generality it can carry, and you should verify anything decision-critical
against that project's own documentation rather than against this page. If
something here is wrong or has gone stale, that is a defect worth reporting.

## The one-sentence difference

Nearly every FHIR server stores the resource as a **document** — JSON or JSONB
in a column, or a document database — and builds **separate index tables** so
that FHIR search can find it. That makes writes simple and makes analytics
painful, because the clinical content is inside an opaque value that SQL can
only pick at with path expressions.

This project inverts the trade. A resource is **shredded into typed relational
tables** — one per element group, with real columns, foreign keys, check
constraints and enum-backed value sets — and reconstructed byte-identically on
read. The cost is thousands of generated tables (7,355 for R5) and a schema no
human wrote. The benefit is that `SELECT family FROM r5.patient_name` is just
SQL, and the query planner sees real column statistics.

**Neither approach is correct in general.** If your workload is
write-heavy FHIR REST traffic with occasional search, the document approach is
simpler and better proven. If your workload is analytics, quality measures,
research extracts, or anything where a data engineer writes SQL against clinical
content, this one removes an entire layer of pain.

## The FHIR platform ecosystem

These are mature, deployed, and in most respects far ahead of this project.

| Project | Language | Storage approach | What it has that this project does not |
| --- | --- | --- | --- |
| [HAPI FHIR](https://hapifhir.io/) | Java | resource as a serialized document plus generated search-index tables | the reference open-source implementation; profile and IG validation, terminology services, subscriptions, years of production deployment, a large community, commercial support |
| [Firely](https://fire.ly/) (SDK, Vonk server) | .NET | document store with search indexes | the reference .NET SDK, a validator many projects depend on, IG tooling, commercial support and certification work |
| [Medplum](https://www.medplum.com/) | TypeScript | PostgreSQL, resource as JSONB plus lookup tables | a full application platform — auth, SMART on FHIR, bots, a UI component library, a hosted offering |
| [Aidbox](https://www.health-samurai.io/aidbox) (Health Samurai) | Clojure | PostgreSQL, JSONB-centred | a commercial FHIR platform with SQL-on-FHIR tooling, terminology, and production support |
| [LinuxForHealth FHIR](https://linuxforhealth.github.io/FHIR/) | Java | resource payload plus search-parameter tables | IBM-originated server with bulk data, extensive IG support and a long conformance record |
| [Microsoft FHIR Server](https://github.com/microsoft/fhir-server) | .NET | SQL Server or Cosmos DB, resource plus search index tables | a managed cloud path, SMART, `$export`, real-world scale |
| [Google Open Health Stack](https://developers.google.com/open-health-stack) / Cloud Healthcare API | mixed | managed, proprietary | Android/offline FHIR tooling and a managed API |

**What all of them have and this project does not**, stated once rather than
repeated per row: production deployments, profile and implementation-guide
validation, terminology services, SMART on FHIR, subscriptions, a community, a
support contract you can buy, and a track record. This project has **no known
deployment**, is pre-release, and its
[conformance matrix](spec/databases/conformance-matrix.md) is deliberately
narrower than any of theirs.

**What this project has that they do not:** the resource itself as queryable
relational structure across six SQL engines from one specification, with lossless
round-trip as a tested invariant — decimal precision and partial dates included —
and a tamper-evident audit chain in the storage layer rather than above it.

### These are not mutually exclusive

The common and sensible arrangement is a mature FHIR server at the edge and a
relational store behind it for analytics. Nothing here competes with HAPI for
serving a FHIR API; [`fhir-loco`](fhir-loco/) exists to translate HTTP to store
calls and get status codes right, not to be a platform.

### The adjacent idea: SQL on FHIR

[SQL on FHIR](https://sql-on-fhir.org/) (ViewDefinition) addresses the same pain
from the other direction: define flattened views over document-stored FHIR. It
is a standard, it is implemented in several of the platforms above, and if your
constraint is "I must keep my existing FHIR server", it is the better answer.
The difference is where the structure lives — a view computed over JSON, versus
a schema the database enforces with types and constraints. That difference shows
up as integrity guarantees and planner statistics, not as syntax.

## The Rust FHIR crates

Rust has a small, active FHIR ecosystem, and — checked at the date above —
**none of these does relational shredding.** They are complements, not
competitors, and this section is a map rather than a table of winners.

| Crate | What it does | Relationship to this project |
| --- | --- | --- |
| [octofhir](https://octofhir.tech/) — `octofhir-fhirpath`, `octofhir-fhirschema`, `octofhir-fhir-model` | FHIRPath engine, FHIRSchema conversion and validation, shared model traits | the closest thing to a natural partner: a FHIRPath engine over our stored resources, or FHIRSchema validation in front of `put`, would serve both projects |
| [fhirbolt](https://lib.rs/crates/fhirbolt) | JSON and XML serialization for R4/R4B/R5 | overlaps `fhir/`'s serialization role; different design trade-offs |
| [helios-fhir](https://crates.io/crates/helios-fhir) | strongly typed model for R4–R6 with FHIRPath integration | overlaps `fhir/` directly; worth comparing if the model crate is all you need |
| [fhir-sdk](https://crates.io/crates/fhir-sdk) | FHIR client with generated models | a client, where we are storage — no overlap |
| [fhir-rs](https://crates.io/crates/fhir-rs) | FHIR implementation, self-described as draft | earlier-stage; same space as `fhir/` |

If you only need FHIR types in Rust, evaluate `fhir/` against `fhirbolt` and
`helios-fhir` on their merits — ours is generated from the official
specification packages, models R2 through R6 in code — six releases, one
cargo feature each — and
carries `arbitrary_precision` plus `float_roundtrip` because decimal fidelity is
a guarantee here rather than an option. That is a real difference, and it is not
the only axis that matters.

## Choosing

| If you need | Choose |
| --- | --- |
| A FHIR API server in production, today | HAPI, Firely, Medplum, Microsoft, or a managed API — not this |
| Profile/IG validation or terminology | any of the above; this project's validation is structural only (`V9.x`), terminology is a declared gap |
| SQL over clinical content, with the database enforcing integrity | this project |
| Flattened views without leaving your existing server | SQL on FHIR / ViewDefinition |
| FHIR types in Rust, no database | `fhir/`, or `fhirbolt`, or `helios-fhir` |
| FHIRPath in Rust | `octofhir-fhirpath` |
| A relational FHIR store on SQL Server or Oracle | this project |

## What would change this page

An honest comparison should say what evidence would move it. Three things
would: a published benchmark of a normalized query against the same data in
JSONB ([`BENCHMARKS.md`](BENCHMARKS.md) records that this does not exist), an
Inferno run against `fhir-loco`, and a first production deployment. Until those
exist, treat the right-hand column of the first table as the decisive one.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
