# Introduction

> ## ⚠ Read this first
>
> This book was written for `fhir-postgresql` and copied to every port. The
> engine-specific text is now corrected throughout (audit **F-56**).
>
> Anything below that names `fhir-mssql serve`, an HTTP endpoint, or a status
> code is really **`fhir-loco`** — a separate crate (Loco.rs, Axum, Tokio,
> Hyper) that mounts a FHIR REST API over a store. `fhir-mssql` itself is a
> **library**: no binary, no `serve`, no HTTP surface, no CLI
> ([`C0.17`](../../../spec/databases/00-conformance.md),
> [`C0.18`](../../../spec/databases/00-conformance.md)).
>
> The [conformance matrix](../../../spec/databases/conformance-matrix.md) is
> the status document to trust over this book.

fhir-mssql stores [FHIR](https://hl7.org/fhir/) resources in Microsoft SQL
Server 2016 or later as **real relational tables** — typed columns, child
tables, primary and foreign keys — not JSON blobs.

It is a **library**. It does not serve HTTP: there is no `serve` command and
no REST API in this workspace (`C0.17`). A caller links `fhir-mssql-store`,
holds an `MsSqlStore`, and calls `put`/`get`/`search`/… directly — see
[Getting started](getting-started.md).

Two claims define the project, and both are enforced by tests:

1. **Losslessness.** Any valid FHIR resource that goes in comes back
   semantically identical — array order, decimal precision, partial dates,
   extensions, and all. The entire official example corpus for R3, R4, and
   R5 — **7,399 resources** (R3 1,664 / R4 2,911 / R5 2,824) — round-trips
   through the shred/reconstruct engine with 0 failures (`R4.2`, **F-42**).
   That result is in-memory, map-layer, and needs no database. This port hit
   the same defect class as the other non-reference ports (`cell_text`,
   `hist_entry`, `[ords]`) and fixed it before any store code shipped;
   `roundtrip_types.rs` now also guards it live against `azure-sql-edge`, 6
   of 6 passing.
2. **Relational honesty.** Live data lives in typed columns you can query,
   join, index, and constrain with ordinary T-SQL. The only column holding
   JSON text is write-once history snapshots and anonymous contained
   resources — never data the schema claims to model.

The trade fhir-mssql makes is generation over convention: the schema (7,355
tables for R5) is generated from the FHIR specification itself, and a
single generic engine shreds and reconstructs every resource type by
walking the generated map. Nothing about a specific resource is
hand-written.

## Why not a JSON column?

A single `NVARCHAR(MAX)` holding the whole resource (or SQL Server's native
`JSON` functions over one) makes writing FHIR trivial and everything
downstream harder: queries become path-spelunking, the query optimizer sees
no per-column statistics, value typing is enforced nowhere, and analytical
T-SQL reads like an apology. For a clinical system the important operations
are reads, searches, joins, and audits — exactly what normalized storage is
good at. `M14.11` is the specific reason this port never binds SQL Server's
`JSON` type for anything the hash chain commits to: it re-normalizes on the
way in, and a re-normalized value is not the byte sequence the chain signed.

## Status

**Store** (`C0.8`): a real `tiberius` store — `connect`, `init`, `put`,
`get`, `delete`, `history`, `vread`, `search`/`search_full`/`search_page`,
`verify_audit`, `purge`, `log_access` — live-verified against
`azure-sql-edge` by 24 tests, 0 `#[ignore]`d (**F-65**). What does not exist:
`conditional_create_audited`, `put_audited`, `upgrade`, `backfill_norm`,
`transact_audited`. `O10.7` (encrypted transport) is diagnosed, not
satisfied — see [The trust boundary](trust-boundary.md).

No benchmark has been run against a live store. `doc/benchmarks.md` in this
port's repository root records schema-scale and in-memory round-trip figures
only; the numbers this page used to carry (6,146 resources/s bulk load,
1.18 ms average reconstruction reads, index-verified searches at 100k
resources) were `fhir-postgresql`'s own, copied in by the same defect
**F-64** fixed in that file. There is no `bench.rs` in this workspace.

See the [conformance matrix](../../../spec/databases/conformance-matrix.md)
for the requirement-by-requirement detail, and this port's
[`README.md`](../../README.md) for what compiles today.
