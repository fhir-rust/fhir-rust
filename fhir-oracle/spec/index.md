# fhir-oracle specification

**Engine:** Oracle Database · **Conformance level:** Store (`C0.8`)

Normative behaviour for this port is the monorepo core, plus this port's
departures — most of which are now live-verified, with transport security
and install atomicity still open.

- **The core** — [`../../spec/index.md`](../../spec/databases/index.md). Sections 0–16,
  engine-neutral, shared by all six ports. Requirement ids (`M3.16b`, `PR12.6`,
  `T11.12`) mean the same thing here as anywhere.
- **This port's departures** — [`14-oracle-dialect.md`](14-oracle-dialect.md),
  numbered `M14.x`. **Status: proposed** (`X15.9`); the type mapping,
  namespace, and store decisions are live-verified, transport security
  (`M14.22`) and install atomicity (`M14.18`) are not.

Contributor guidance: [`../../AGENTS.md`](../../AGENTS.md).

> **The annex was rewritten** (**F-16**, fixed). It previously contained the
> `fhir-mysql` annex with three lines changed — titled "14. MySQL dialect",
> targeting "MySQL 8.0 or later, InnoDB, `utf8mb4`", with the word "Oracle"
> appearing only in the three substituted crate names.
>
> It is now a **decision list**: what must be settled, and why the obvious
> answer is wrong. That is what `X15.6` asks for when nothing has been decided,
> and it is considerably more useful than another engine's confident answers.
>
> **Every `M14.x` id from the old file is withdrawn, not reused** (`C0.5`).

> ## The store now runs live, with a confirmed gap
>
> **The DDL emitter was MySQL, is now Oracle, and has been executed** (**F-08**,
> closed 2026-08-03). The full R5 schema — 9,636 statements — installs on 26ai
> with 0 invalid objects. Its eleven MySQL-asserting tests were `#[ignore]`d and
> have since been replaced with Oracle-asserting ones.
>
> **The store connects, and its full surface is live-tested** (**F-68**,
> superseding **F-66**'s "compiles but never connected"). Oracle Instant
> Client for macOS arm64 is a direct, no-login download; installed, `crates/
> fhir-oracle-store` connected to a live `gvenzl/oracle-free:23-slim-faststart`
> and `tests/oracle_store.rs` runs `init`/`put`/`get`/`delete`/`history`/
> `vread`/`verify_audit`/`purge`/`log_access`/`search` against it — **7 of 7
> tests pass, 0 ignored.** Doing so found and fixed five real defects (see
> `14-oracle-dialect.md` `M14.5`, `M14.19`, `M14.34`, and `audit.md` **F-68**
> for the full account): Oracle's username case-folding requiring an uppercase
> schema, `R4.5`'s presumed mechanism failing outright, a double
> schema-qualification bug, a timestamp-binding bug, and a boolean bound as
> text in token search.
>
> **What is still open:** `R4.5` has no working mechanism on this port — the
> only candidate named in the annex was tried live and removed after it broke
> every read (`M14.19`). There is no `concurrency.rs` verifying `H5.4` under
> contention, no `redaction.rs`, no `upgrade`/`backfill_norm`, and no map test
> directory. Conformance level is **Store**, not Reference — see the
> [conformance matrix](../../spec/databases/conformance-matrix.md).

## What is decided, and what is not

The annex answers one item and opens the rest deliberately.

| `X15.6` item | State |
| --- | --- |
| **Engine floor (`S1.4`)** | **Decided: 12.2** (`M14.2`). Identifiers were 30 bytes before 12.2 and 128 after, so the shared 63-byte budget is legal on 12.2+ and *silently truncating* below it — the exact collision `G2.4` exists to prevent. 23ai was considered for its native `BOOLEAN` and rejected (`M14.4`). |
| Namespace (`S1.2`) | **Decided and live-verified: three uppercase users, one per version** — Oracle folds an unquoted username to uppercase for session identity, so the schema bound in `RelMap` must be uppercase too, or every statement fails `ORA-01031` (`M14.5`) |
| `Bool` | Decided: `NUMBER(1)` + `CHECK`, following from `M14.4` (`M14.8`); binds as `i64` 0/1, and as of `M14.34` a search predicate against it MUST also bind `i64`, not the string `"true"`/`"false"` |
| `Numeric` (`M3.6a`) | Decided: a character type. `NUMBER` normalizes `1.50` to `1.5` (`M14.7`) |
| **`Text` / `CLOB`** | **Decided** (`M14.9`) — see below |
| `TextC` (`M3.6b`) | Open — collation and `NLS_SORT`; note `CHAR` is blank-padded (`M14.10`) |
| `Timestamptz` / `Date` | `TIMESTAMP(6)`/`DATE` (`M14.11`); binding MUST use a typed `chrono` value, not a plain string left to implicit conversion (`M14.34`) |
| `Jsonb` (`M3.6c`) | Decided: `CLOB`, never the `JSON` type (`M14.12`) |
| `ords` (`M3.4a`) | Open between `VARCHAR2` and `RAW`; watch Oracle's `''`-is-`NULL` rule (`M14.13`–`M14.14`) |
| Idempotence (`G2.5`) | Decided in shape: a PL/SQL block swallowing ORA-00955, which makes every statement a block (`M14.15`) |
| Append-only (`M3.17`) | `RAISE_APPLICATION_ERROR(-20001, …)` (`M14.16`) |
| Erasure (`M3.18`) | `SYS_CONTEXT` + an application context — a heavier dependency than any other port's (`M14.17`) |
| Snapshot reads (`R4.5`) | **Open, and worse than undecided: the one named candidate fails live.** `SET TRANSACTION READ ONLY` breaks with `ORA-01466` on any session that has run DDL (`M14.19`) |
| Write serialization (`H5.4`) | Decided and implemented: `SELECT … FOR UPDATE` (`M14.20`); not yet verified under concurrent writers |
| Driver | Decided and live-verified: the `oracle` crate (ODPI-C), synchronous, wrapped in `spawn_blocking` (`M14.23`) |
| Transport security (`O10.7`) | Open (`M14.22`) |

The `VARCHAR2`/`CLOB` boundary is worth reading in full (`M14.9`). It is the SQL
Server port's `NVARCHAR(MAX)` problem, sharper in the direction that matters:
`NVARCHAR(MAX)` still compares with `=`, so those searches are correct and merely
scan, whereas an Oracle `CLOB` does not — so the same design would make some
searches *not work at all*.

## Open findings against this port

- **F-08 (High)** — ~~`ddl.rs` is verbatim MySQL and cannot produce an Oracle
  schema.~~ **Closed 2026-08-03.** The emitter is Oracle and the full R5 schema
  (9,636 statements) installed on 26ai with 0 invalid objects. See `M14.28`–
  `M14.32` for the departures it required, and `M14.29a` for the guard that
  failed open.
- **F-06 (High)** — CI and `scripts/db.sh` provisioned **MySQL 8.4** and invoked
  `--test mysql_ddl`, a target that does not exist here. **Fixed:** the
  live-database gate has been **removed** rather than repointed — there is
  nothing to point it at, and a removed gate is at least honestly absent
  (`M14.24`).
- **F-09 (Medium)** — no declared engine floor under a 63-byte identifier
  budget. **Fixed:** `M14.2` declares 12.2 and `M14.3` requires `init` to verify
  it.
- **F-16 (High)** — the annex described MySQL. **Fixed**, as above.
- **F-01 (High)** — the README claimed corpus results and a `serve` command in a
  port with no store. **Fixed.**
- **F-02**, **F-03** — crate description and stale driver comment. **Fixed.**
- **F-11** — shared; see the [register](../../spec/databases/audit.md).
- **F-66** — a full store existed in source, written with no Instant Client;
  it compiled and built but had never connected to a database. **Superseded
  by F-68**, below — the premise no longer holds, but the finding is kept for
  the record of what was and was not known at the time.
- **F-68** — Instant Client installed, the store connected to a live Oracle
  for the first time, and its full surface passed 7 of 7 tests in
  `tests/oracle_store.rs`. Found and fixed five real defects along the way.
  Conformance level moved from Scaffold to **Store**. See the
  [register](../../spec/databases/audit.md) **F-68**.

## Contents of the core

| | Section | Prefix |
| --- | --- | --- |
| 0 | [Conformance](../../spec/databases/00-conformance.md) | `C0.x` |
| 1 | [Scope](../../spec/databases/01-scope.md) | `S1.x` |
| 2 | [Schema generation](../../spec/databases/02-schema-generation.md) | `G2.x` |
| 3 | [Storage model](../../spec/databases/03-storage-model.md) | `M3.x` |
| 4 | [Shredding and reconstruction](../../spec/databases/04-shredding-and-reconstruction.md) | `R4.x` |
| 5 | [Versioning and history](../../spec/databases/05-versioning-and-history.md) | `H5.x` |
| 6 | [Search](../../spec/databases/06-search.md) | `P6.x` |
| 9 | [Validation](../../spec/databases/09-validation.md) | `V9.x` |
| 10 | [Operations](../../spec/databases/10-operations.md) | `O10.x` |
| 11 | [Conformance testing](../../spec/databases/11-conformance-testing.md) | `T11.x` |
| 12 | [Trust, principal, and audit](../../spec/databases/12-trust-principal-and-audit.md) | `PR12.x` |
| 13 | [Compliance mapping](../../spec/databases/13-compliance-mapping.md) | — |
| **14** | [**Oracle dialect**](14-oracle-dialect.md) | `M14.x` |
| 15 | [Portability and dialects](../../spec/databases/15-portability-and-dialects.md) | `X15.x` |
| 16 | [Repository and release](../../spec/databases/16-repository-and-release.md) | `W16.x` |
| — | [Locale and accent folding](../../spec/databases/locale-accent-folding.md) | `Lx` |

Sections 7 and 8 are retired (`C0.15`).
