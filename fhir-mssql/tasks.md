# fhir-mssql — work breakdown

**Conformance level: Scaffold (`C0.8`).** This file lists what has been done in
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

## What does not exist

Not "planned and unstarted" — **absent**. Each of these is why the port is
Scaffold rather than Store.

- [ ] **A store.** `crates/fhir-mssql-store/src/lib.rs` is 48 lines: it
  re-exports the shared audit chain and defines an error type. There is no
  connection, no transaction, no operation.
- [ ] **A database driver.** `tiberius` is a dependency of the *map* crate, for the DDL test only. The store crate depends on no driver.
- [ ] **Any write through the schema by this port.** Every behaviour verified
  so far was verified by hand or by DDL install alone.
- [ ] **Map tests beyond DDL**, a live round-trip, history, search execution,
  concurrency, redaction, audit, upgrade, or benchmarks.

## Not decided, not merely undone

The REST server and CLI milestones in the old file were `[x]`. They are removed
rather than unticked, because unticking would assert they are *planned for this
port*, and that has never been decided — `C0.17`/`C0.18` say no port has such a
crate, while §10 and §12 still specify a service (**F-05**). Whether these
libraries grow a server is the owner's call and is tracked in **F-27**, not
here.

## Next, in order

1. Decide the driver (see above); it gates everything below.
2. A live test that installs the generated schema and **fails** rather than
   skips when the DSN is set (`T11.12`, `T11.13`).
3. `init` / `put` / `get` against a real server, then the corpus round-trip that
   `C0.8` requires for **Store**.

The [conformance matrix](../spec/databases/conformance-matrix.md) is the status
document to trust. This file is a plan; it is not evidence (`C0.9`).
