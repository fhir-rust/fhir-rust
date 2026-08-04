# Introduction

> ## ⚠ Read this first
>
> This book was written for `fhir-postgresql` and copied to every port. The
> engine-specific text is now corrected throughout.
>
> What several chapters still call `fhir-mariadb serve` is really
> **`fhir-loco`** — a separate crate (Loco.rs, Axum, Tokio, Hyper) that mounts
> a FHIR REST API over a store. `fhir-mariadb` itself is a **library**: no binary,
> no `serve`, no HTTP surface
> ([`C0.17`](../../../spec/databases/00-conformance.md),
> [`C0.18`](../../../spec/databases/00-conformance.md); audit **F-56**).
>
> Read any endpoint, status code or `serve` command below as `fhir-loco`'s
> behaviour, not this crate's. The
> [conformance matrix](../../../spec/databases/conformance-matrix.md) is the
> status document to trust.

fhir-mariadb stores [FHIR](https://hl7.org/fhir/) resources in MariaDB 11.4 as
**real relational tables** — typed columns, child tables, primary and
foreign keys — not JSON blobs.

It is a **library**. It does not serve HTTP: there is no `serve` command and no
REST API in this workspace (`C0.17`).

Two claims define the project, and both are enforced by tests:

1. **Losslessness.** Any valid FHIR resource that goes in comes back
   semantically identical — array order, decimal precision, partial dates,
   extensions, and all. The entire official example corpus for R3, R4, and
   R5 (7,399 resources) round-trips through the shred/reconstruct engine,
   and ten thousand generated property-test cases besides.
2. **Relational honesty.** Live data lives in typed columns you can query,
   join, index, and constrain with ordinary SQL. The only JSON in the
   system holds write-once history snapshots and anonymous contained
   resources — never data the schema claims to model.

The trade fhir-mariadb makes is generation over convention: the schema (7,355
tables for R5) is generated from the FHIR specification itself, and a
single generic engine shreds and reconstructs every resource type by
walking the generated map. Nothing about a specific resource is
hand-written.

## Why not JSONB?

JSONB storage makes writing FHIR trivial and everything downstream harder:
queries become path-spelunking, the planner sees no per-column statistics,
value typing is enforced nowhere, and analytical SQL reads like an
apology. For a clinical system the important operations are reads,
searches, joins, and audits — exactly what normalized storage is good at.

## Status

**Store** level (`C0.8`): the generator, shred/reconstruct, the MariaDB 11.4
store, search, history, and the audit chain all work, and a live MariaDB 11.4
gate runs them in CI — 102 tests, green (measured 2026-08-03; see the
[conformance matrix](../../../spec/databases/conformance-matrix.md)).

Two things this port does **not** have: optimistic concurrency (no
`put_audited`, no `expected_version` anywhere in the crate) and
`transact_audited` or conditional create/delete. `upgrade` and `backfill_norm`
do exist, closing this port's share of **F-15**.

`doc/benchmarks.md` states plainly what is and is not measured for this
engine specifically: schema scale (7,355 tables for R5) and search-parameter
compilation (**92.4%** of R5 resolve, `P6.1`, corrected under audit **F-38**
from an earlier 94.8%) come from the generator, which is
byte-identical across all six ports, so those figures apply here too. Install
timing, bulk-load throughput, and per-read latency numbers do **not** exist
for `fhir-mariadb` — this book used to state PostgreSQL's own measurements as
this port's (audit **F-64**), and that page now says so explicitly rather than
repeating the substitution here.
