# fhir-mssql — work breakdown

**Conformance level: Store (`C0.8`).** This file lists what has been done in
*this port* and what has not. Nothing here is inherited.

## Why this file was rewritten

Until 2026-08-03 this was `fhir-mysql/tasks.md` with the crate name
substituted. It ticked off a store, a CLI, a REST server, and 40-odd hardening
milestones, citing acceptance runs against **MySQL 8.4** — an engine this port
has never provisioned. None of that work exists here, and a `[x]` asserts it is
finished, which is a stronger claim than the READMEs made before **F-01** fixed
them. Recorded as [`audit.md`](../spec/databases/audit.md) **F-27**, class 2.

The old file is not preserved. It described another port's history; there is
nothing in it to recover.

## What exists

The generator and the in-memory engine are **shared** across all six ports and
are as correct here as anywhere (`X15.1`). They are listed because they work,
not because this port did them.

- [x] **Workspace.** Three crates — `fhir-mssql-map`, `-gen`, `-store`. No CLI
  crate, no server crate; none of the six ports has either (`C0.17`, `C0.18`).
- [x] **Spec ingestion and relational map.** Shared `gen/`: R3, R4 and R5
  definitions compile to a `RelMap`.
- [x] **Shred and reconstruct.** Shared. The full example corpus round-trips
  **in memory** — 1,664 R3, 2,911 R4, 2,824 R5, 0 failures.
- [x] **Committed map assets.** `regen-assets` plus the `G2.2` drift gate.
- [x] **Search-parameter compilation.** Shared, including the `U1`–`U13`
  adjunct channel this port needs because it cannot index its unbounded text
  type (`TEXT_ADJUNCTS = true`).
- [x] **DDL emitter.** T-SQL, and **executed**: the generated schema installs on SQL Server 2022 via `tests/mssql_ddl.rs` — 131 statements, 102 tables, 4 triggers. That test was flaky until **F-52**; it now passes 8 consecutive runs.
- [x] **A store.** `crates/fhir-mssql-store/src/mssql.rs`: `connect`, `init`,
  `put`, `get`, `delete`, `history`, `vread`, `verify_audit`, `purge`,
  `log_access`. `mssql` (a fork of `tiberius`, since 2026-08-29 — **F-67**),
  pooled via a hand-written `bb8::ManageConnection` (`pool.rs`, no mature
  driver-specific pool exists). Live-verified — this is not a claim from
  reading the code (**F-65**).
- [x] **A search builder.** `mssql_search.rs`: `@Pn` placeholders, bracketed
  identifiers, `OFFSET … FETCH`, `FLOAT` not `DECIMAL` casts (`M14.8`), no
  `NULLS LAST`. Wired to `search`/`search_full`/`search_page`.
- [x] **Live test suite.** `tests/mssql_store.rs` (13), `concurrency.rs` (2),
  `redaction.rs` (2), `roundtrip_types.rs` (6), `ssl_live.rs` (1),
  `upgrade.rs` (9) — **33 of 33 green** against `azure-sql-edge`, 0
  `#[ignore]`d. Run with `--test-threads=1` (concurrent DDL deadlocks the
  container). These 33 run **locally** via `scripts/db.sh`; the workflow
  provisions SQL Server 2022 but invokes only the map crate's `mssql_ddl`
  test. (The workflow lives at the repository root as
  `fhir-mssql-ci.yml` since the F-49 consolidation, 2026-08-06.)
- [x] **`R4.5` (snapshot reads).** `get` issues `SET TRANSACTION ISOLATION
  LEVEL SNAPSHOT` before `BEGIN TRANSACTION`, backed by
  `ALLOW_SNAPSHOT_ISOLATION` on a dedicated `fhir_mssql` database
  (`scripts/db.sh`'s `post_ready`, since `master` refuses the option). Two
  tries: `READ_COMMITTED_SNAPSHOT` alone was tried live first and did not
  stop the torn read `tests/concurrency.rs` reproduces — it gives each
  statement its own snapshot, not the whole transaction one. See `M14.25`
  and **F-65**.
- [x] **`init --upgrade` and `backfill_norm`.** Closes this port's share of
  audit **F-15**. `MsSqlStore::upgrade`/`backfill_norm` diff the installed map
  asset against the current one, apply the additive diff, reconcile the
  schema-wide objects and audit-envelope columns the per-resource diff cannot
  see, apply the destructive diff (refused without `allow_destructive`), and
  backfill folded search columns in bounded resumable batches. Unlike
  `fhir-mysql`/`fhir-mariadb`, the DDL apply is one transaction — T-SQL DDL is
  transactional, so a failure rolls back rather than leaving a half-upgraded
  schema (`M14.35`). Live-verified: `tests/upgrade.rs`, 9 tests. Found live:
  destructive table drops must be ordered children-before-base or SQL Server
  refuses `DROP TABLE` with error 3726 (`M14.36`).

## What does not exist

Not "planned and unstarted" — **absent**.

- [ ] **`conditional_create_audited`, `put_audited`, `transact_audited`.** No
  optimistic concurrency, no conditional operations, no atomic Bundles.
- [x] **`path` to `NVARCHAR(path_bound)`** (`M14.37`, `U12a`) — *done
  2026-08-10* (**F-47** steps 3–4). Fresh installs bounded by
  `create_table`; existing installs converted by `upgrade`
  (`convert_path_columns`: catalog-driven, data pre-check, `ALTER COLUMN`
  inside the transaction; widening additive, narrowing refused).
  Live-verified: `tests/upgrade.rs` 12 tests incl. the three conversion
  tests, mutation-checked (disabling the conversion fails the pre-U12a
  test). `U12` holds for `path` here; matrix flipped, **F-47 closed**.
- [ ] **Verification against full SQL Server.** Only `azure-sql-edge`
  (`M14.31`) — an arm64 subset of the product.
- [x] **`O10.7`.** The mechanism is confirmed live (`tests/ssl_live.rs`):
  `TrustServerCertificate=false` reproducibly rejects `azure-sql-edge`'s
  self-signed certificate. Was diagnosed but not claimed from 2026-08-04:
  the driver's TLS dependency chain carried four unpatched advisories
  reaching the shipping store crate (**F-67**), and `native-tls` failed the
  handshake on this host, so there was no available fix at the time — a
  risk accepted formally 2026-08-28 after pricing a driver replacement
  (`M14.34`) and finding none viable. **Resolved 2026-08-29**: switched from
  `tiberius` (0.12.3, its last release) to `mssql`, a fork the owner
  published to carry the fixes forward. `O10.7` is now claimed as well as
  diagnosed. **F-67 closed.**

## Not decided, not merely undone

The REST server and CLI milestones in the old file were `[x]`. They are removed
rather than unticked, because unticking would assert they are *planned for this
port*, and that has never been decided — `C0.17`/`C0.18` say no port has such a
crate, while §10 and §12 still specify a service (**F-05**). Whether these
libraries grow a server is the owner's call and is tracked in **F-27**, not
here.

## Next, in order

1. Settle `O10.7`'s verification half.
2. Verification against full SQL Server, not only `azure-sql-edge`.

The [conformance matrix](../spec/databases/conformance-matrix.md) is the status
document to trust. This file is a plan; it is not evidence (`C0.9`).
