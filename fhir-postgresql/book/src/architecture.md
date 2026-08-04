# Architecture

**Three crates**, all in this workspace (`Cargo.toml`'s `members`). There is
no server crate and no CLI binary — an earlier version of this chapter listed
`fhir-postgresql-server` (axum) and a `fhir-postgresql` CLI as though they
existed here; they do not (`C0.17`, `C0.18`, audit **F-56**). A FHIR RESTful
API over this store is [`fhir-loco`](../../../fhir-loco/), a separate crate
outside this workspace.

- **`fhir-postgresql-map`** — the relational map model (a compact,
  serialized description of every table, column, and element) and the
  generic engine: `shred` (JSON → rows), `reconstruct` (rows → JSON), `fold`
  (the case/accent folding behind search), `canon` (canonical JSON for the
  hash chain pre-image, `M14.12`), and `ddl.rs` — the **only** dialect-specific
  file in this crate, and the only one this port's departures are allowed to
  touch (`X15.1`). Reconstruction audits row consumption: every stored row
  must be used exactly once, or it returns `ShredError::Integrity` naming how
  many rows of how many were left over — schema drift or corruption surfaces
  as an error instead of silently dropped data.
- **`fhir-postgresql-gen`** — a **build-time** tool, not a runtime
  dependency. Reads a FHIR specification package (StructureDefinitions +
  SearchParameters) and builds the map: identifier fitting under this
  port's 63-byte budget (`PG_MAX_IDENT`, `M14.2`), width-based force-splitting
  once a table would exceed `SPLIT_WIDTH` (150 columns), cycle detection
  (type cycles spill into a `*_deep` table; `contentReference` recursion
  shares one table via ordinal sign lanes, `model.rs`), and the search
  compiler (`gen/src/search.rs`) that resolves each SearchParameter's
  FHIRPath expression by walking the map tree — 92.4% of R5's 1,972
  parameters resolve to a concrete target after **F-38**; the rest are
  recorded unsupported with a reason rather than guessed at. The output
  ships as a committed, compiled-in asset
  (`crates/fhir-postgresql-map/assets/*.json.gz`), so a consumer of `-map`
  never needs the FHIR specification packages themselves.
- **`fhir-postgresql-store`** — `tokio-postgres` + `deadpool` (plan decision
  D5: the SQL here is generated and dynamic, so `sqlx`'s compile-time query
  checking buys nothing). Transactional writes with history append,
  optimistic concurrency (`expected_version`, `StoreError::Conflict`),
  multi-op transaction Bundles (`transact`/`transact_audited`), snapshotted
  multi-table reads (`REPEATABLE READ READ ONLY`, `M14.15`), search
  execution (`search.rs`, compiling `&[(String, String)]` params to SQL
  against the map), install/upgrade, the tamper-evident chain
  (`verify_audit`, `chain_witness`, `resign_history`, re-exported from the
  shared, engine-agnostic [`fhir-store`](../../../fhir-store/) crate along
  with `Audit` and `AccessRecord`), and erasure (`purge`). Every value
  crosses the wire as text with explicit casts
  (`($n::text)::numeric`, `($n::text)::smallint[]`), which is what keeps
  decimal scale and partial-date lexical fidelity intact in both directions
  (`M14.25`).

## Why a fourth crate is shared, not vendored

`fhir-postgresql-store` depends on [`fhir-store`](../../../fhir-store/) for
everything about persistence that is **not** SQL-dialect-specific: the
`Audit`/`AccessRecord`/`PutOutcome`/`ChainBreak` types, and the hash-chain
math in `chain.rs` (618 lines, byte-identical across all six ports before it
was factored out — audit **F-45**). That crate opens no socket and speaks no
HTTP; the boundary it draws is "authentication is elsewhere, but the record
of who did what is not nowhere" (`fhir-store`'s own module doc, `PR12.1`–`4`).
One consequence worth knowing before you go looking for an engine-specific
name: `fhir-store`'s chain-key environment variables are literally spelled
`FHIR_SQLITE_CHAIN_KEY*`, unconditionally, regardless of which port loads
them — see [Operations](operations.md#keying-the-chain).

## The decisive design choice

**Metadata over codegen.** Rather than generating Rust for 3 FHIR versions ×
~150 resource types × deep nesting, the generator emits *data* (the
relational map, `RelMap`) and one generic engine interprets it at both
shred and reconstruct time. The engine is a few thousand lines, tested once,
and correct for every resource type by construction rather than by
per-type review — and the map doubles as documentation, since every table
and column carries the FHIRPath that produced it.

The cost of generating from the specification rather than hand-writing is
scale: **7,355 tables for R5** alone (5,672 for R4, 3,827 for R3). That is
routine for a database and impossible to hand-maintain, which is the actual
argument for generation — not compile time, but that nobody could keep
~150 resource types' worth of hand-written shredding logic correct across
three FHIR versions as the specification moves.

## Where to read next

- [The storage model](storage-model.md) — what the map and DDL emitter
  actually produce, with real generated SQL.
- [Operations](operations.md) — the `Store` methods this crate exposes for
  install, upgrade, and the audit chain.
- `plan.md` — design decisions D1–D20, risks, and milestones (note: `plan.md`
  predates the `fhir-loco` split and still describes an in-workspace server
  in places; the [conformance matrix](../../../spec/databases/conformance-matrix.md)
  is the current status to trust, not `plan.md`).
- [`spec/index.md`](../../spec/index.md) — this port's normative index; the
  shared core is [`spec/databases/index.md`](../../../spec/databases/index.md).
