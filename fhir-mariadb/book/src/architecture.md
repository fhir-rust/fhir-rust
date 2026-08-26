# Architecture

Three crates, and no more — there is no server crate and no CLI crate in this
workspace (`C0.17`, `C0.18`; the earlier version of this page listed a fourth
and fifth that were never built here, audit **F-56**). The REST surface, if a
deployment wants one, is a separate crate, [`fhir-loco`](../../../fhir-loco/),
mounted over a store.

- **fhir-mariadb-map** — the relational map model (a compact, serialized
  description of every table, column, and element) and the generic engine:
  `shred` (JSON → rows), `reconstruct` (rows → JSON), `fold` (accent/case
  normalization), `canon` (the Rust-side canonical JSON the hash chain
  commits to — `M14.19`/`M14.20`), and this engine's DDL emitter (`ddl.rs`,
  the only dialect-specific file in this crate). Reconstruction audits row
  consumption — every stored row must be used exactly once, so schema drift
  or corruption surfaces as an integrity error instead of silent data loss.
  `model.rs`, `shred.rs`, `reconstruct.rs`, `value.rs`, `fold.rs`, and
  `canon.rs` are byte-identical (modulo whitespace) across all six ports
  (`X15.1`); only `ddl.rs` is this engine's own.
- **fhir-mariadb-gen** — reads a FHIR® specification package
  (StructureDefinitions + SearchParameters) and builds the map: identifier
  fitting under a 63-byte budget (`PG_MAX_IDENT`, tighter than MariaDB's own
  64-character limit, so one map serves all six), width-based force-splitting
  at 150 columns (`SPLIT_WIDTH`, below MariaDB's 1017-column ceiling), cycle
  detection (type cycles spill; contentReference recursion shares tables via
  ordinal sign lanes), and the search compiler that resolves FHIRPath
  expressions by walking the map tree — **92.4%** of R5's SearchParameters
  resolve to a concrete `(table, column)` target (`P6.1`, corrected under
  audit **F-38** from an earlier 94.8% that silently dropped a `where()`
  value restriction); the rest are recorded unsupported with a reason rather
  than guessed at. This crate is also
  byte-identical across all six ports (`M14.1`): its two dialect-looking
  constants are already correct for MariaDB.
- **fhir-mariadb-store** — `mysql_async` over a connection pool. This is
  where MariaDB actually differs from the other five ports: transactional
  writes (`put`, `delete`) that take `SELECT … FOR UPDATE` on the base row
  before reading the history chain tip, so concurrent writers to one
  resource serialize instead of racing (`H5.4`, closing audit **F-24**);
  reads (`get`) run inside their own `REPEATABLE READ` transaction so a
  multi-table reconstruction sees one snapshot (`R4.5`, closing **F-21**);
  `Ext`/`Deep` tables get a Rust-computed `BINARY(32)` surrogate primary key
  (`M14.12`); `init`/`upgrade`/`backfill_norm` for schema management; and
  `search`/`search_full`/`search_page` for query execution. There is no
  optimistic concurrency, no `transact_audited`, and no conditional
  create/delete anywhere in this crate — only `put`, `get`, `delete`,
  `history`, `vread`, `purge`, and `verify_audit`. It depends on
  [`fhir-store`](../../../fhir-store/) for the engine-agnostic half: `Audit`,
  `AccessRecord`, and the hash-chain machinery in `chain.rs`, shared rather
  than duplicated across all six ports (closing **F-45**).

The decisive design choice is **metadata over codegen**: rather than
generating Rust for 3 versions × ~150 resource types, the generator emits
data (the map) and one engine interprets it. The engine is a few thousand
lines, tested once, correct for every resource type — and the map doubles as
documentation, carrying the FHIR path of every column.

## Independent from fhir-mysql

This port began as a fork of the sibling `fhir-mysql` port, and the two are
now independent (`M14.0a`–`M14.0c`): this port uses whatever MariaDB does
best — `CREATE OR REPLACE TRIGGER`, `ALTER TABLE … ADD COLUMN IF NOT EXISTS`,
`utf8mb4_nopad_bin` — rather than restricting itself to syntax MySQL also
accepts, and a schema installed here is not required to be readable by
`fhir-mysql`. What the two continue to share is *behaviour* — round-trip
fidelity, search semantics, the canonical JSON form the chain commits to —
not SQL text.

Design decisions D1–D14, risks, and milestones live in `plan.md`; the
normative behaviour is `spec/index.md` plus this port's annex,
`spec/14-mariadb-dialect.md`.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
