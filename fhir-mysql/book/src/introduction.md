# Introduction

> ## ⚠ Read this first
>
> This book was written for `fhir-postgresql` and copied to every port. The
> engine-specific text is now corrected throughout.
>
> What several chapters still call `fhir-mysql serve` is really
> **`fhir-loco`** — a separate crate (Loco.rs, Axum, Tokio, Hyper) that mounts
> a FHIR® REST API over a store. `fhir-mysql` itself is a **library**: no binary,
> no `serve`, no HTTP surface
> ([`C0.17`](../../../spec/databases/00-conformance.md),
> [`C0.18`](../../../spec/databases/00-conformance.md); audit **F-56**).
>
> Read any endpoint, status code or `serve` command below as `fhir-loco`'s
> behaviour, not this crate's. The
> [conformance matrix](../../../spec/databases/conformance-matrix.md) is the
> status document to trust.

fhir-mysql stores [FHIR](https://hl7.org/fhir/) resources in MySQL 8.4 as
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

The trade fhir-mysql makes is generation over convention: the schema (7,355
tables for R5) is generated from the FHIR specification itself, and a
single generic engine shreds and reconstructs every resource type by
walking the generated map. Nothing about a specific resource is
hand-written.

## Why not JSON storage?

MySQL has a native `JSON` type, and it is deliberately not used for live
data (it is used nowhere in the schema except as an *unused* option — see
[storage model](storage-model.md)). JSON-column storage makes writing FHIR
trivial and everything downstream harder: queries become path-spelunking,
the planner sees no per-column statistics, value typing is enforced nowhere,
and analytical SQL reads like an apology. For a clinical system the important
operations are reads, searches, joins, and audits — exactly what normalized
storage is good at. `JSON` is also specifically unsafe for the one place this
project used to consider it: it re-normalizes on write, so the bytes read
back would not be the bytes a hash-chain signed (`M14.0g`, `M14.19`).

## Status

**Store** level (`C0.8`): the generator, the shred/reconstruct engine, the
MySQL 8.4 store, search, history, and the audit chain all work, and a live
MySQL 8.4 gate exercises 102 tests in CI, including a TLS suite — see the
[conformance matrix](../../../spec/databases/conformance-matrix.md) for what
that claim does and does not cover, and `doc/benchmarks.md` for what has
actually been measured on this engine versus what that page still owes
(**F-64**): install throughput and bulk-load numbers have not been measured
here — the previous version of this page carried `fhir-postgresql`'s own
figures with the crate name substituted, including a `bench.rs` harness this
port does not have. What *is* measured: schema scale (7,355 tables for R5),
search-parameter compilation (94.8% of R5's SearchParameters, corrected to
92.4% pending a regenerated asset — see [Search](search.md)), and full-corpus
round-trip (7,399/7,399 official examples, lossless, at the map layer).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
