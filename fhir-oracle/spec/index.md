# fhir-oracle specification

**Engine:** Oracle Database · **Conformance level:** Scaffold (`C0.8`)

Normative behaviour for this port is the monorepo core, plus this port's
departures — of which most are, honestly, still open questions.

- **The core** — [`../../spec/index.md`](../../spec/databases/index.md). Sections 0–16,
  engine-neutral, shared by all six ports. Requirement ids (`M3.16b`, `PR12.6`,
  `T11.12`) mean the same thing here as anywhere.
- **This port's departures** — [`14-oracle-dialect.md`](14-oracle-dialect.md),
  numbered `M14.x`. **Status: proposed, and mostly undecided** (`X15.9`).

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

> ## ⚠ Nothing in this port is Oracle yet
>
> **The DDL emitter is MySQL.** `crates/fhir-oracle-map/src/ddl.rs::col_sql`
> emits `TEXT`, `TINYINT(1)`, `DATETIME(6)`, `LONGTEXT`, and
> `COLLATE utf8mb4_0900_bin` — none of which exist in Oracle — and its comments
> still discuss MySQL's 2038 `TIMESTAMP` range. Its eleven MySQL-asserting tests
> are `#[ignore]`d so a green run cannot be mistaken for Oracle conformance,
> which is the right call. Tracked as [`audit.md`](../../spec/databases/audit.md)
> **F-08**.
>
> **There is no store.** `crates/fhir-oracle-store/src/` holds `lib.rs` and
> `chain.rs`; there is no driver in the workspace. There is no map test
> directory.
>
> The code has been honest about this throughout — `tasks.md` says "Scaffold
> only… Nothing here is an Oracle schema", and the workspace `Cargo.toml` now
> explains why it carries no driver. The README was not, and has been rewritten.

## What is decided, and what is not

The annex answers one item and opens the rest deliberately.

| `X15.6` item | State |
| --- | --- |
| **Engine floor (`S1.4`)** | **Decided: 12.2** (`M14.2`). Identifiers were 30 bytes before 12.2 and 128 after, so the shared 63-byte budget is legal on 12.2+ and *silently truncating* below it — the exact collision `G2.4` exists to prevent. 23ai was considered for its native `BOOLEAN` and rejected (`M14.4`). |
| Namespace (`S1.2`) | Open — Oracle conflates user and schema (`M14.5`) |
| `Bool` | Decided: `NUMBER(1)` + `CHECK`, following from `M14.4` (`M14.8`) |
| `Numeric` (`M3.6a`) | Decided: a character type. `NUMBER` normalizes `1.50` to `1.5` (`M14.7`) |
| **`Text` / `CLOB`** | **Open, and the hardest problem** (`M14.9`) — see below |
| `TextC` (`M3.6b`) | Open — collation and `NLS_SORT`; note `CHAR` is blank-padded (`M14.10`) |
| `Timestamptz` | Open — `TIMESTAMP(6)`, `WITH TIME ZONE` or not (`M14.11`) |
| `Jsonb` (`M3.6c`) | Decided: `CLOB`, never the `JSON` type (`M14.12`) |
| `ords` (`M3.4a`) | Open between `VARCHAR2` and `RAW`; watch Oracle's `''`-is-`NULL` rule (`M14.13`–`M14.14`) |
| Idempotence (`G2.5`) | Decided in shape: a PL/SQL block swallowing ORA-00955, which makes every statement a block (`M14.15`) |
| Append-only (`M3.17`) | `RAISE_APPLICATION_ERROR(-20001, …)` (`M14.16`) |
| Erasure (`M3.18`) | `SYS_CONTEXT` + an application context — a heavier dependency than any other port's (`M14.17`) |
| Snapshot / serialization / driver | Open (`M14.19`–`M14.23`) |

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
