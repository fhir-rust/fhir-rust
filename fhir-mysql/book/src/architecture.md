# Architecture

Three crates. There is no server crate and no CLI binary in this workspace
(`C0.17`, `C0.18`) — the two this page used to list beside them
(`fhir-mysql-server`, a `fhir-mysql` binary) do not exist here; that was
`fhir-postgresql`'s book text, uncorrected (audit **F-56**).

- **`fhir-mysql-map`** — the relational map model (a compact, serialized
  description of every table, column, and element), the generic engine —
  `shred` (JSON → rows) and `reconstruct` (rows → JSON) — canonical JSON
  (`canon.rs`, M14.20), the accent/case fold (`fold.rs`), and this engine's
  `ddl.rs`. Reconstruction audits row consumption: every stored row must be
  used exactly once, so schema drift or corruption surfaces as an integrity
  error instead of silent data loss. `shred.rs`, `reconstruct.rs`, `value.rs`,
  `fold.rs`, `canon.rs`, and `model.rs` are identical (modulo whitespace)
  across all six ports (`X15.1`, `M14.2`); only `ddl.rs` is MySQL-specific.
- **`fhir-mysql-gen`** — reads a FHIR specification package
  (StructureDefinitions + SearchParameters) and builds the map: identifier
  fitting under a 63-byte budget (`G2.4`, tighter than MySQL's own 64-character
  limit, so one generated map serves all six engines — `M14.1`), width-based
  force-splitting at 150 columns (`SPLIT_WIDTH`), cycle detection (type cycles
  spill into a `_deep` table; `contentReference` recursion shares one table via
  signed ordinal lanes), and the search compiler (`search.rs`) that resolves
  SearchParameter FHIRPath expressions by walking the map tree. Identical
  across all six ports; this crate carries no MySQL-specific code (`M14.1`).
- **`fhir-mysql-store`** — the one MySQL-specific runtime crate:
  `mysql_async` with a connection pool (`M14.27`), the search-SQL builder
  (`mysql_search.rs` — forked from the shared design rather than
  parameterized, since the ports are independent by construction, `M14.0a`),
  TLS configuration (`ssl.rs`, `O10.7`), and `mysql.rs`, which holds every
  operation this port exposes: `init`, `upgrade`, `backfill_norm`, `put`,
  `get`, `history`, `vread`, `delete`, `purge`, `search` /
  `search_full` / `search_page`, `log_access`, and `verify_audit`. It depends
  on [`fhir-store`](../../../fhir-store/) for the engine-agnostic half —
  `Audit`, `AccessRecord`, the hash-chain primitives in `chain.rs`, and the
  result types — and re-exports it, so `fhir_mysql_store::Audit` resolves
  without an extra dependency line.

No optimistic concurrency (no `put_audited`, no `expected_version`), no
`transact_audited`, and no conditional create or delete exist anywhere in this
crate — see the [conformance matrix](../../../spec/databases/conformance-matrix.md)
before assuming otherwise. `upgrade` and `backfill_norm` do exist, closing this
port's share of **F-15**; see [Operations](operations.md).

The decisive design choice, shared by every port, is **metadata over
codegen**: rather than generating Rust for 3 versions × ~150 resource types,
the generator emits data (the map) and one engine interprets it. The engine is
a few thousand lines, tested once, correct for every resource type — and the
map doubles as documentation, carrying the FHIR path of every column.

Design decisions and their reasons live in `plan.md`; the work breakdown is
`tasks.md` — read it against
[`spec/audit.md`](../../../spec/databases/audit.md) before trusting a `[x]`,
per finding **F-27**. Normative behaviour is
[`spec/index.md`](../../spec/index.md) plus the
[dialect annex](../../spec/14-mysql-dialect.md), which is itself still marked
*proposed* (`X15.9`).
