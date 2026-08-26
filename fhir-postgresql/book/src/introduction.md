# Introduction

> ## ⚠ Read this first
>
> Where this book describes a `fhir-postgresql serve` command, endpoints or HTTP
> status codes, it means **`fhir-loco`** — a separate crate (Loco.rs, Axum,
> Tokio, Hyper) that mounts a FHIR® REST API over a store.
>
> `fhir-postgresql` itself is a **library**: no binary, no `serve`, no HTTP
> surface ([`C0.17`](../../../spec/databases/00-conformance.md),
> [`C0.18`](../../../spec/databases/00-conformance.md); audit **F-56**). The
> [conformance matrix](../../../spec/databases/conformance-matrix.md) is the
> status document to trust.

fhir-postgresql is a **Rust library** that stores [FHIR](https://hl7.org/fhir/)
resources in PostgreSQL 18 as **real relational tables** — typed columns,
child tables, primary and foreign keys — not JSON or JSONB blobs. It gives
them back losslessly through an `async` API (`Store::put`, `::get`,
`::search`, …); it does not speak HTTP. A FHIR RESTful API over this crate is
a separate concern — see the banner above.

Two claims define the project:

1. **Losslessness**, enforced by tests. Any valid FHIR resource that goes in
   comes back semantically identical — array order, decimal precision,
   partial dates, extensions, and all. The entire official example corpus for
   R3, R4, and R5 (7,399 resources) round-trips in memory, and 7,396 of them
   (three lack ids and are skipped) round-trip through live PostgreSQL 18 —
   see [`doc/benchmarks.md`](../../doc/benchmarks.md) for how that was
   measured.
2. **Relational honesty.** Live data lives in typed columns you can query,
   join, index, and constrain with ordinary SQL. The only JSONB in the
   system holds write-once history snapshots and anonymous contained
   resources — never data the schema claims to model (`M14.13` records the
   one place this crate still binds `jsonb` against the core spec's
   preference for `text`).

The trade fhir-postgresql makes is generation over convention: the schema (7,355
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

Functional end to end and pre-release. See `tasks.md` in the repository
for the milestone ledger and `doc/benchmarks.md` for measured numbers
(6,146 resources/s bulk load; 1.18 ms average reconstruction reads;
index-verified searches at 100k resources).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
