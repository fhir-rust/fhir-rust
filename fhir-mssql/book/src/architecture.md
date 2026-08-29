# Architecture

Three crates — this workspace has never had five, and as of this pass one
of the three is a real store rather than a stub:

- **fhir-mssql-map** — the relational map model (a compact, serialized
  description of every table, column, and element) and the generic engine:
  `shred` (JSON → rows), `reconstruct` (rows → JSON), `fold` (search-value
  normalization), `canon` (the hash chain's Rust-side canonical form), and
  `ddl.rs`, this engine's T-SQL emitter. Reconstruction audits row
  consumption — every stored row must be used exactly once, so schema drift
  or corruption surfaces as an integrity error instead of silent data loss
  (`R4.7`). Everything here except `ddl.rs` is byte-identical (modulo
  whitespace) to the other five ports' `map` crates (`X15.1`).
- **fhir-mssql-gen** — reads a FHIR® specification package
  (StructureDefinitions + SearchParameters) and builds the map: identifier
  fitting under a 63-byte budget (below every supported engine's own limit,
  so one map serves all six), width-based force-splitting at 150 columns
  (`SPLIT_WIDTH`), cycle detection (type cycles spill; contentReference
  recursion shares tables via ordinal sign lanes), and the search compiler
  that resolves FHIRPath expressions by walking the map tree. Shared code,
  identical across all six ports including its own tests.
- **fhir-mssql-store** — `mssql.rs` (the `mssql` driver, a fork of
  `tiberius` since 2026-08-29: `connect`, `init`, `put`, `get`, `delete`,
  `history`, `vread`, `verify_audit`, `purge`, `log_access`),
  `mssql_search.rs` (the search-SQL builder), and `pool.rs` (a from-scratch
  `bb8::ManageConnection`, since `mssql` ships no pool of its own).
  Live-verified against `azure-sql-edge` by 40 tests, 0
  `#[ignore]`d (**F-65**, **F-15**). It also depends on the shared
  [`fhir-store`](../../../fhir-store) crate for the engine-agnostic half —
  `Audit`, `AccessRecord`, `PutOutcome`, `SearchOutcome`, `ChainBreak`, and
  the hash-chain functions — rather than duplicating them, and re-exports
  that crate so `fhir_mssql_store::Audit` resolves without an extra
  dependency. **Not implemented:** `conditional_create_audited`,
  `put_audited` (optimistic concurrency), `transact_audited`, `upgrade`,
  `backfill_norm`.

There is no `fhir-mssql-server` and no `fhir-mssql` CLI binary in this
workspace, and there never has been one described accurately — the earlier
version of this page invented an axum server crate and a CLI that this
repository does not contain (`C0.17`, `C0.18`). The REST server for this
family is [`fhir-loco`](../../../fhir-loco), a separate crate in the
monorepo that mounts a FHIR API over a store; it is not part of this port
and not covered by this book.

The decisive design choice is **metadata over codegen**: rather than
generating Rust for 3 versions × ~150 resource types, the generator emits
data (the map) and one engine interprets it. The engine is a few thousand
lines, tested once, correct for every resource type — and the map doubles
as documentation, carrying the FHIR path of every column.

Where this port genuinely diverges from the other five, in one place:
`ddl.rs` and everything under `fhir-mssql-store` — bracketed identifiers,
`NVARCHAR` throughout, `VARBINARY` `ords`, `SET TRANSACTION ISOLATION LEVEL
SNAPSHOT` for `get`, `WITH (UPDLOCK, ROWLOCK)` for write serialization, and
the TLS advisories described in
[The trust boundary](trust-boundary.md#o107-diagnosed-not-satisfied) — each
numbered in [`spec/14-mssql-dialect.md`](../../spec/14-mssql-dialect.md)
(`M14.x`). Design decisions D1–D14, risks, and milestones live in
`plan.md`; the normative behaviour is [`spec/index.md`](../../spec/index.md).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
