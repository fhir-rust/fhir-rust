# fhir-oracle — work breakdown

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

- [x] **Workspace.** Three crates — `fhir-oracle-map`, `-gen`, `-store`. No CLI
  crate, no server crate; none of the six ports has either (`C0.17`, `C0.18`).
- [x] **Spec ingestion and relational map.** Shared `gen/`: R3, R4 and R5
  definitions compile to a `RelMap`.
- [x] **Shred and reconstruct.** Shared. The full example corpus round-trips
  **in memory** — 1,664 R3, 2,911 R4, 2,824 R5, 0 failures.
- [x] **Committed map assets.** `regen-assets` plus the `G2.2` drift gate.
- [x] **Search-parameter compilation.** Shared, including the `U1`–`U13`
  adjunct channel this port needs because it cannot index its unbounded text
  type (`TEXT_ADJUNCTS = true`).
- [x] **DDL emitter.** Oracle, and **executed**: the full R5 schema — 158 resources, 9,636 statements — installed on Oracle 26ai with 0 invalid objects and 0 unindexable search targets (**F-08**). Verified by hand at first (**F-51**), and since **F-68** it is also what every `tests/oracle_store.rs::init_installs_tables_and_triggers` run installs live.
- [x] **A store.** `crates/fhir-oracle-store/src/` implements `connect`,
  `init`, `put`, `get`, `delete`, `history`, `vread`, `verify_audit`, `purge`,
  `log_access`, `search`/`search_full`/`search_page` — connected to a live
  `gvenzl/oracle-free:23-slim-faststart` for the first time 2026-08-04
  (**F-68**). `tests/oracle_store.rs`: 7 of 7 tests pass, 0 ignored.
- [x] **A database driver.** The `oracle` crate (ODPI-C/OCI), synchronous,
  wrapped in `spawn_blocking`. Needs Oracle Instant Client on the host at
  connection time (not build time) — a direct, no-login download for macOS
  arm64; see `scripts/db.sh`'s header comment.
- [x] **Write through the schema by this port**, live: `put`/`delete`
  exercised in every `tests/oracle_store.rs` run against a real database, not
  by hand.

## What does not exist

Not "planned and unstarted" — **absent**.

- [ ] **A working `R4.5` mechanism.** The candidate this port's annex named,
  `SET TRANSACTION READ ONLY`, was tried live and fails outright with
  `ORA-01466` on any session that has run DDL. `get` currently has no
  snapshot-isolation protection at all. This is an open, confirmed gap, not
  merely an unverified requirement — see `M14.19` and **F-68**.
- [ ] **A concurrency test.** `H5.4` (serialized `version_id`) is implemented
  via `SELECT … FOR UPDATE`, but no test races concurrent writers against it
  the way `fhir-mssql`'s and `fhir-mysql`'s `concurrency.rs` do.
- [ ] **A redaction test**, or benchmarks.
- [x] **`upgrade` / `backfill_norm`** — *done 2026-08-09* (**F-15**'s last
  port, **F-47** step 1). Diffs the stored map asset, applies resumable DDL
  (`M14.35`), chunks the meta asset past `ORA-01461` (`M14.36`), backfills
  by ROWID keyset because a `CLOB` source cannot be value-compared
  (`M14.37`). Live-verified: `tests/upgrade.rs`, 9 tests, mutation-checked
  (skipping the backfill fails the seeded-patient search).
- [ ] **A transport-security decision** (`O10.7`, `M14.22`) — the live tests
  connect over a plain local port with no encryption configured either way.
- [ ] **A live CI gate.** The Oracle job was removed rather than faked
  (**F-06**); nothing in CI runs this port's tests. `scripts/db.sh`
  (`gvenzl/oracle-free:23-slim-faststart`) is the local gate.

## Not decided, not merely undone

The REST server and CLI milestones in the old file were `[x]`. They are removed
rather than unticked, because unticking would assert they are *planned for this
port*, and that has never been decided — `C0.17`/`C0.18` say no port has such a
crate, while §10 and §12 still specify a service (**F-05**). Whether these
libraries grow a server is the owner's call and is tracked in **F-27**, not
here.

## Next, in order

1. A working `R4.5` mechanism — the annex's one named candidate is now known
   not to work (`M14.19`); a replacement is untried.
2. `concurrency.rs`, racing writers against `put`/`delete` to verify `H5.4`
   under contention, not just in sequence.
3. `redaction.rs`.
4. The transport-security decision (`O10.7`, `M14.22`).
   (`upgrade`/`backfill_norm` was this list's item 4 until 2026-08-09 —
   done, see above.)

The [conformance matrix](../spec/databases/conformance-matrix.md) is the status
document to trust. This file is a plan; it is not evidence (`C0.9`).
