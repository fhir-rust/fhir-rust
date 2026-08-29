# fhir-mssql specification

**Engine:** Microsoft SQL Server · **Conformance level:** Store (`C0.8`)

Normative behaviour for this port is the monorepo core, plus this port's
departures.

- **The core** — [`../../spec/index.md`](../../spec/databases/index.md). Sections 0–16,
  engine-neutral, shared by all six ports. Requirement ids (`M3.16b`, `PR12.6`,
  `T11.12`) mean the same thing here as anywhere.
- **This port's departures** — [`14-mssql-dialect.md`](14-mssql-dialect.md),
  numbered `M14.x`. **Status: proposed** (`X15.9`), so it MUST NOT be cited as
  evidence for a conformance level until ratified.

Contributor guidance: [`../../AGENTS.md`](../../AGENTS.md).

> **The annex was rewritten** (**F-16**, fixed). It previously contained the
> `fhir-mysql` annex with three lines changed — titled "14. MySQL dialect",
> targeting "MySQL 8.0 or later, InnoDB, `utf8mb4`", mentioning SQL Server
> nowhere — while this port's `ddl.rs` is genuine, deliberate T-SQL. The
> specification contradicted its own working implementation.
>
> **Every `M14.x` id from the old file is withdrawn, not reused** (`C0.5`). A
> citation of an `M14.x` in `fhir-mssql` predating the rewrite is void, and
> should be traced to the MySQL annex it actually came from.

## What this port departs on

Summarized from the annex; the annex governs.

| `X15.6` item | SQL Server |
| --- | --- |
| Engine floor (`S1.4`) | **2016** — set by `SESSION_CONTEXT()` and `CREATE OR ALTER` (`M14.3`) |
| Namespace (`S1.2`) | SQL Server schema inside one database (`M14.4`) |
| Quoting | brackets, never double quotes — those depend on session state (`M14.5`) |
| `ColTy` binding (`M3.6`) | `BIT`, `INT`, `BIGINT`, `NVARCHAR(MAX)`, `NVARCHAR(450) COLLATE Latin1_General_100_BIN2`, `DATE`, `DATETIME2(6)` (`M14.6`) |
| Character type | `NVARCHAR` never `VARCHAR` — `VARCHAR` is a code page unless collated UTF-8 (`M14.7`) |
| `ords` binding (`M3.4a`) | `VARBINARY(255)` holding the shared text image — the only port that uses bytes (`M14.13`) |
| Resource ids | `NVARCHAR(64)`, an exact bound from the FHIR® `id` production (`M14.12`) |
| Index limits (`P6.4a`) | **departure** — 900-byte key cap; `NVARCHAR(MAX)` columns dropped from indexes, so token searches scan (`M14.16`) |
| Idempotence (`G2.5`) | `IF NOT EXISTS (SELECT … FROM sys.objects)` guards; no `IF NOT EXISTS` clause exists (`M14.17`) |
| Append-only (`M3.17`) | `CREATE OR ALTER TRIGGER … INSTEAD OF`, `THROW 50000`; no `DROP` window (`M14.19`) |
| Erasure (`M3.18`) | `SESSION_CONTEXT` — T-SQL's nearest equivalent to `SET LOCAL` (`M14.21`) |
| Paging / placeholders | `OFFSET … FETCH`, `@P1`, no `NULLS LAST` (`M14.22`) |
| Transport (`O10.7`) | `mssql` (a `tiberius` fork since 2026-08-29) + rustls (`M14.24`); verification mechanism confirmed live (`tests/ssl_live.rs`). The driver's TLS dependency chain carried 4 unpatched CVEs reaching the shipping store crate — `native-tls` tried as an escape, fails the handshake on this host — resolved by the driver switch, not by an escape (`M14.34`, **F-67 closed**). Claimed |
| Snapshot isolation (`M14.25`) | **decided and verified**: `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` in `get`, backed by `ALLOW_SNAPSHOT_ISOLATION` on a dedicated `fhir_mssql` database — `READ_COMMITTED_SNAPSHOT` alone was tried first and live-confirmed *not* to fix it |
| Write serialization (`M14.26`) | **decided and verified**: `WITH (UPDLOCK, ROWLOCK)`, 8 of 8 racing writers got distinct consecutive versions live |
| Undecided | install atomicity at scale (`M14.27`) |

## Open findings against this port

- **F-01 (High)** — the README claimed 7,399 corpus resources round-trip and a
  `serve` command works. There is **no store implementation**:
  `crates/fhir-mssql-store/src/` holds `lib.rs` and `chain.rs`, and there are no
  store tests. **Fixed:** the README now describes the port as it is.
- **F-06 (High)** — CI and `scripts/db.sh` provisioned **MySQL 8.4** and
  invoked `--test mysql_ddl`, a target that does not exist here, so the job
  failed on `error: no test target named mysql_ddl` and the T-SQL DDL had never
  executed a single assertion. **Fixed:** CI now provisions SQL Server 2022,
  runs `--test mssql_ddl`, and sets `FHIR_MSSQL_REQUIRE_DB=1` so an absent
  database fails rather than skips.
- **F-16 (High)** — the annex described MySQL. **Fixed**, as above.
- **F-02**, **F-03** — crate description and stale driver comment. **Fixed.**
- **F-11** — shared; see the [register](../../spec/databases/audit.md).
- **F-65** — this port gained a real store; five defects found and fixed live
  (a cross-column collation conflict, an unchecked keyed audit tag, `connect`
  reporting success against an unreachable server, a doubled erasure count,
  and the `R4.5` torn read — see `M14.25` above, **fixed** in a follow-up pass
  after the first attempt, `READ_COMMITTED_SNAPSHOT` alone, was tried live and
  found insufficient). See the [register](../../spec/databases/audit.md)
  **F-65**.
- **F-67 (High), closed 2026-08-29** — `deny.toml` ignored 4 TLS advisories on
  the reasoning that `tiberius` was only a dev-dependency; that stopped being
  true the moment **F-65** gave this port a store, unnoticed for two days.
  Corrected first (2026-08-04), then formally risk-accepted by the owner
  (2026-08-28) after `native-tls` and two other alternatives were
  investigated and found nonviable (`M14.34`). Actually resolved the next
  day: the owner published `mssql`, a `tiberius` fork maintained to carry
  security fixes tiberius's last release (0.12.3) never did, and this port's
  driver now depends on it — none of the four advisory packages remain
  anywhere in the dependency tree, verified with `cargo tree`. The same
  investigation confirmed `O10.7`'s trust/no-trust mechanism genuinely works
  (`tests/ssl_live.rs`); it is now claimed as well as verified. See `M14.34`
  and the [register](../../spec/databases/audit.md) **F-67**.

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
| **14** | [**SQL Server dialect**](14-mssql-dialect.md) | `M14.x` |
| 15 | [Portability and dialects](../../spec/databases/15-portability-and-dialects.md) | `X15.x` |
| 16 | [Repository and release](../../spec/databases/16-repository-and-release.md) | `W16.x` |
| — | [Locale and accent folding](../../spec/databases/locale-accent-folding.md) | `Lx` |

Sections 7 and 8 are retired (`C0.15`).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
