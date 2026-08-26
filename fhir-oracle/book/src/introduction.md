# Introduction

> ## ⚠ Read this first
>
> This book was written for `fhir-postgresql` and copied to every port. The
> engine-specific text is now corrected throughout.
>
> What several chapters still call `fhir-oracle serve` is really
> **`fhir-loco`** — a separate crate (Loco.rs, Axum, Tokio, Hyper) that mounts
> a FHIR® REST API over a store. `fhir-oracle` itself is a **library**: no
> binary, no `serve`, no HTTP surface
> ([`C0.17`](../../../spec/databases/00-conformance.md),
> [`C0.18`](../../../spec/databases/00-conformance.md); audit **F-56**, banner
> added under **F-83**).
>
> Read any endpoint, status code or `serve` command below as `fhir-loco`'s
> behaviour, not this crate's. The
> [conformance matrix](../../../spec/databases/conformance-matrix.md) is the
> status document to trust.

fhir-oracle stores [FHIR](https://hl7.org/fhir/) resources in Oracle
Database 12.2 or later as **real relational tables** — typed columns, child
tables, primary and foreign keys — not JSON blobs.

It is a **library**, not a server. There is no `fhir-oracle` binary, no
`serve` command, and no REST API in this workspace (`C0.17`, `C0.18`). If you
want HTTP, mount [`fhir-loco`](https://github.com/fhir-rust/fhir-rust/tree/main/fhir-loco)
— a separate crate — over a store; it currently mounts `fhir-sqlite`, not
this one.

Two claims define the project, and both are enforced by tests:

1. **Losslessness.** Any valid FHIR resource that goes in comes back
   semantically identical — array order, decimal precision, partial dates,
   extensions, and all. The entire official example corpus for R3, R4, and
   R5 (7,399 resources) round-trips through the shred/reconstruct engine —
   shared, engine-independent Rust, correct here as in every other port —
   with zero failures.
2. **Relational honesty.** Live data lives in typed columns you can query,
   join, index, and constrain with ordinary SQL. The only `CLOB` in the
   system holds write-once history snapshots, unbounded text search columns
   (backed by a bounded/digest adjunct pair — see [The storage
   model](storage-model.md)), and anonymous contained resources — never data
   the schema claims to model directly.

The trade fhir-oracle makes is generation over convention: the schema (7,358
tables for R5) is generated from the FHIR specification itself, and a single
generic engine shreds and reconstructs every resource type by walking the
generated map. Nothing about a specific resource is hand-written.

## Why not a JSON column?

A JSON column makes writing FHIR trivial and everything downstream harder:
queries become path-spelunking, the optimizer sees no per-column statistics,
value typing is enforced nowhere, and analytical SQL reads like an apology.
For a clinical system the important operations are reads, searches, joins,
and audits — exactly what normalized storage is good at. This is sharper on
Oracle than on most engines: a `CLOB` (the type any sufficiently long text
column must use) cannot even be compared with `=` or indexed at all — see
[The storage model](storage-model.md) for how search still works over one.

## Status

**Store level** (`C0.8`), reached 2026-08-04. The DDL emitter installs the
full R5 schema — 9,636 statements — on Oracle 26ai with 0 invalid objects.
The store connects and its full surface — `put`, `get`, `delete`, `history`,
`vread`, `search`, `verify_audit`, `purge`, `log_access` — is live-tested
against a real Oracle in `tests/oracle_store.rs`: 7 of 7 tests pass, 0
ignored.

Not **Reference**: `R4.5` (snapshot reads under concurrent writers) has no
working mechanism on this engine — the candidate this port's annex named was
tried live and found to break every read (see [Operations](operations.md)).
There is no concurrency test, no redaction test, and no `upgrade`/
`backfill_norm`. The [conformance
matrix](https://github.com/fhir-rust/fhir-rust/blob/main/spec/databases/conformance-matrix.md)
is the document to trust for the exact, current state — this book is a tour,
not the source of truth.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
