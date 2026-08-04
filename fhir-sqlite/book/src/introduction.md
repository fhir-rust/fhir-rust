# Introduction

> ## About this book
>
> This book was originally `fhir-postgresql`'s, copied to every port with the
> engine name substituted (audit **F-56**). The name substitution was fixed
> first; this revision (2026-08-04) rewrote the chapter *content* against
> `fhir-sqlite`'s actual source, tests, and the
> [conformance matrix](../../../spec/databases/conformance-matrix.md) —
> SQLite is embedded and file-based, which the earlier text did not reflect
> anywhere it mattered (server flags, TLS, a CLI that does not exist).
>
> `fhir-sqlite` is a **library**: no binary, no `serve`, no HTTP surface
> ([`C0.17`](../../../spec/databases/00-conformance.md),
> [`C0.18`](../../../spec/databases/00-conformance.md)). The REST server that
> can be put in front of it is [`fhir-loco`](../../../fhir-loco/) — a separate
> crate (Loco.rs, Axum, Tokio, Hyper). Nothing in this book describes
> `fhir-loco`'s behaviour; if you are looking for endpoints, status codes, or
> a `serve` command, they are not here.

fhir-sqlite stores [FHIR](https://hl7.org/fhir/) resources in SQLite 3 as
**real relational tables** — typed columns, child tables, primary and foreign
keys — not JSON blobs.

Two claims define the project, and both are enforced by tests:

1. **Losslessness.** Any valid FHIR resource that goes in comes back
   semantically identical — array order, decimal precision, partial dates,
   extensions, and all. The entire official example corpus for R3, R4, and R5
   (7,399 resources: 1,664 + 2,911 + 2,824) round-trips through the
   shred/reconstruct engine (`R4.2`, audit **F-42**), in memory, with no store
   involved.
2. **Relational honesty.** Data lives in typed columns you can query, join,
   index, and constrain with ordinary SQL. The only text-as-document storage
   in the system is write-once history snapshots and anonymous contained
   resources — never data the schema claims to model as columns.

The trade fhir-sqlite makes is generation over convention: the schema (7,355
tables for R5, 5,672 for R4, 3,827 for R3 — the same figures for every port,
since the generator is byte-identical across all six, `X15.1`) is generated
from the FHIR specification itself, and one generic engine shreds and
reconstructs every resource type by walking the generated map. Nothing about
a specific resource type is hand-written.

## Why not a JSON column?

Storing each resource as one JSON document makes writing trivial and
everything downstream harder: queries become path-spelunking, the query
planner sees no per-column statistics, value typing is enforced nowhere, and
ordinary SQL against clinical data reads like an apology. For a system whose
job is search, joins, and audits — not documents — normalized storage is the
right trade, and it is the reason `fhir-sqlite` exists rather than a thin
wrapper storing `json()`.

## What is actually different about this port

SQLite is not a smaller PostgreSQL. It is a different kind of engine, and
that shows up in ways this book states rather than glosses over:

- **No server, no connection, no TLS.** A store is a file path. `O10.7`
  (encrypted transport) does not apply; the obligation that replaces it is at
  rest — file permissions and, if you need it, disk or SQLCipher encryption —
  and that is the deployment's job, not this library's.
- **One writer at a time, structurally.** SQLite admits a single writer.
  `BEGIN IMMEDIATE` is what PostgreSQL's advisory locks and `SELECT … FOR
  UPDATE` bought in that store; here the engine enforces it.
- **Dynamic typing.** Every column declared here is an affinity, not a
  constraint (`M14.10`). The store, not the database, is what rejects a
  malformed value.
- **A decimal is `TEXT`, never `REAL`.** Binary floating point cannot hold
  `1.50` distinct from `1.5`, and `M3.6` requires the original textual
  precision to survive. See [The storage model](storage-model.md).

Full detail is the dialect annex,
[`spec/14-sqlite-dialect.md`](../../spec/14-sqlite-dialect.md) — read it
before changing anything that touches SQL.

## Status

**Store level, nearing Reference** (per the
[conformance matrix](../../../spec/databases/conformance-matrix.md),
measured 2026-08-03). 105 tests, including concurrency, redaction,
round-trip-by-column-type, and upgrade+backfill — none of them need a server,
because there is no server to need. `transact_audited` returns `Unsupported`
by design; see [Operations](operations.md). Two `T11.8` gaps remain
(per-algorithm independent tamper detection under a shared test file, and the
truncated-chain-versus-checkpoint case), and `chain_witness` /
`resign_history` do not exist yet.

Schema-scale and search-compilation figures come from the generator and are
identical across all six ports (`X15.1`); throughput numbers specific to this
port's own engine have not been measured — a `bench.rs` harness does not
exist here yet (audit **F-64**). Do not trust a number in this book that is
not attributed to a source you can check.
