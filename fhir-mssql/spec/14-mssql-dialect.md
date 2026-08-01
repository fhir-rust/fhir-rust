# 14. SQL Server dialect

**Status: proposed.** A draft for review, not yet ratified (`X15.9`), so it MUST
NOT be cited as evidence for a conformance level until it is.

This annex records where the SQL Server port departs from the
[monorepo core](../../spec/index.md), and — as importantly — where it does not.
Requirements are numbered `M14.x` and use RFC 2119 keywords.

Target: **Microsoft SQL Server 2016 or later** (see `M14.3`), one database, one
schema per FHIR version.

> **This file was rewritten.** It previously contained the `fhir-mysql` annex
> with three lines changed — titled "14. MySQL dialect", declaring the target as
> "MySQL 8.0 or later, InnoDB, `utf8mb4`", carrying a section headed
> "Relationship to fhir-mariadb", and mentioning SQL Server nowhere
> ([`audit.md`](../../spec/audit.md) **F-16**).
>
> That mattered more than the analogous problem in code, because this port's
> `ddl.rs` is genuine, deliberate T-SQL: the specification contradicted its own
> working implementation, and a reader following it would have rejected correct
> code.
>
> **Requirement numbering restarts here.** The `M14.x` ids in the previous file
> were MySQL's requirements wearing this port's name. `C0.5` makes ids
> permanent, so those numbers are **withdrawn, not reused**: no `M14.x` below
> means what the same number meant in the copied file. Any citation of an
> `M14.x` in `fhir-mssql` predating this rewrite is void, and should be traced
> to the MySQL annex it actually came from.

## What does not change

- **M14.1** The pure-Rust core — `model.rs`, `shred.rs`, `reconstruct.rs`,
  `value.rs`, `fold.rs`, `canon.rs`, `error.rs`, and all of `gen/` — MUST NOT
  differ from the other ports (`X15.1`). It operates on Rust types and never
  emits SQL.
- **M14.2** The identifier budget of 63 and the split width of 150 (`G2.4`,
  `G2.6`) carry over unmodified. Both are *tighter* than SQL Server's limits —
  128-character identifiers, 1024 columns — and `X15.3` requires keeping the
  monorepo's shared budget so a generated name is legal on every engine.

## Engine floor and namespaces

- **M14.3** The engine floor is **SQL Server 2016** (`S1.4`), set by the
  features this port's DDL actually uses:

  | Feature | Introduced |
  |---|---|
  | `DATETIME2` | 2008 |
  | `OFFSET … FETCH` | 2012 |
  | `THROW` | 2012 |
  | `SESSION_CONTEXT()` | **2016** |
  | `CREATE OR ALTER` | **2016 SP1** |

  The last two set the floor, and both are load-bearing (`M14.19`, `M14.21`).

- **M14.4** Each FHIR version installs into its own **SQL Server schema** inside
  one database (`S1.2`), created with
  `IF SCHEMA_ID('r5') IS NULL EXEC('CREATE SCHEMA [r5]')`. This is closer to the
  PostgreSQL original than to MySQL, where a version would be a whole database.

## Identifiers and quoting

- **M14.5** Generated identifiers MUST be **bracketed** (`[name]`), never
  double-quoted. Double quotes work only under `QUOTED_IDENTIFIER ON` — the
  default, but *session state*, and a schema must not depend on session state.

## Type mapping

- **M14.6** `col_sql` binds `ColTy` (`M3.6`) as follows.

  | `ColTy` | SQL Server |
  |---|---|
  | `Bool` | `BIT` |
  | `Int` | `INT` |
  | `BigInt` | `BIGINT` |
  | `Numeric` | `NVARCHAR(MAX)` |
  | `Text` | `NVARCHAR(MAX)` |
  | `TextC` | `NVARCHAR(450) COLLATE Latin1_General_100_BIN2` |
  | `Date` | `DATE` |
  | `Timestamptz` | `DATETIME2(6)` |
  | `Jsonb` | `NVARCHAR(MAX)` |

- **M14.7** Character columns MUST be `NVARCHAR`, never `VARCHAR`. SQL Server's
  `VARCHAR` is a single-byte code page unless the column carries a UTF-8
  collation; FHIR text is Unicode, and losing a patient name to a code page is
  not a trade worth making for storage.

- **M14.8** `Numeric` MUST NOT be `DECIMAL` (`M3.6a`). `DECIMAL(38,10)` returns
  `1.50` as `1.5000000000` — a fixed declared scale cannot preserve a per-value
  lexical form, and `M3.6` requires the original textual precision survive
  round-trip. Range search is served by a derived sort column, not by this one.

- **M14.9** `TextC` MUST be a **binary, code-point** collation, and `BIN2`
  rather than the deprecated `BIN` (`M3.6b`).

  SQL Server's default collation is case- **and** accent-insensitive, so a
  column left at the default would silently acquire fuzzy equality — the
  opposite of what this column exists for, and enough to break `:exact` matching
  and key identity. `BIN2` compares by code point rather than by the old
  byte-wise rule, which is what the folded column's Rust-side ordering assumes.

  The width, 450, is not arbitrary: 450 × 2 bytes = 900, exactly the index key
  limit (`M14.15`).

- **M14.10** `Timestamptz` MUST be `DATETIME2(6)`, not `DATETIME`, which rounds
  to 1/300th of a second and would silently alter a timestamp the hash chain
  commits to. `DATETIMEOFFSET` MUST NOT be used: every value is normalized to
  UTC in Rust before binding, so an offset column would store a zero offset and
  invite the belief that local times are preserved.

- **M14.11** `Jsonb` MUST be `NVARCHAR(MAX)`, not the JSON type (`M3.6c`). The
  hash chain commits to bytes canonicalized in Rust (`X15.2`, implemented here
  in `canon.rs`); a column that re-normalizes what it is given would make the
  bytes read back differ from the bytes signed, and every chain would fail
  verification.

- **M14.12** Resource id columns (`id`, `rid`) MUST be
  `NVARCHAR(64) COLLATE Latin1_General_100_BIN2`. 64 is an **exact bound**, not
  a guess: the FHIR `id` production is `[A-Za-z0-9\-\.]{1,64}` (`R4.6`).
  Bounding it is what lets `id` and `rid` be keyed and foreign-keyed without a
  prefix.

## The `ords` column

- **M14.13** `ords` MUST be `VARBINARY(255)`, holding **the same text image**
  every other port stores (`M3.4b`, `X15.5`) — `{1,2}`, `{}`, `{-1,3}`.
  `VARBINARY` keeps it one byte per character against the index key budget and
  compares exactly.

  This is the port's one genuinely unusual binding: PostgreSQL has `smallint[]`,
  the other ports use a character type, this one uses bytes. All three store the
  identical image, so `fmt_ords` and `parse_ords` are shared unmodified and a
  database compares value-for-value against any other port's.

- **M14.14** `M3.4a`'s three value-domain properties survive: negative ordinals
  appear verbatim in the image, the empty path is the two-byte string `{}`, and
  depth is bounded only by 255 bytes of image rather than by a fixed width.

## Index key limits

- **M14.15** SQL Server caps an index key at **900 bytes** (1700 for a
  nonclustered index), which is *tighter* than MySQL's 3072. `Ext` and `Deep`
  tables MUST therefore keep the hash surrogate key the MySQL port introduced,
  and it matters more here: their natural key includes two unbounded text
  columns and cannot be a primary key at all.

- **M14.16** **Departure from `P6.4a`: decided, not yet implemented.** A column
  bound to `NVARCHAR(MAX)` cannot be part of an index key, so `Text`, `Numeric`,
  and `Jsonb` columns are **skipped** when emitting indexes.

  The common case is fine: `TextC` is `NVARCHAR(450)`, and the folded companion
  column that every non-`:exact` string search actually compares carries that
  type, so it indexes. What does not index is a token's `system` and `code`,
  which are `Text`. Those searches remain **correct, and scan**.

  `P6.4a` requires that a port either narrow the bound type or add an indexable
  derived column rather than silently drop the index. This port currently does
  neither, which is why this is a departure and not a note.

  **Settled.** [Unbounded string search](../../spec/unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) (`U1`–`U10`, `P6.9`) is now
  normative and supersedes the persisted-computed-column sketch this
  requirement used to carry. A text column this engine cannot index gets a
  **bounded adjunct** (`<col>_idx`, `NVARCHAR(450)`) *and* a **checksum
  adjunct** (`<col>_h`, `BINARY(32)`) in the generated map.

  Why both, when `NVARCHAR(MAX)` already answers `=` here and only the *index*
  is missing: a bounded prefix cannot answer equality, so an index on it must be
  confirmed against the source column anyway (`U7`), and the checksum turns that
  confirmation from a scan into a seek. The sibling Oracle port needs the
  checksum for a stronger reason — a `CLOB` answers no comparison at all — and
  `U2` requires both everywhere rather than letting the two ports diverge on
  which half they implement.

  It belongs in the **generated map**, not in `ddl.rs` (`X15.1`: shared across
  all six ports). Until it lands, this port MUST NOT claim `P6.4a`.

  SQL Server has no prefix-length index syntax at all, so the prefix arithmetic
  the MySQL port needed does not apply here; the question is only which columns
  are indexable in the first place.

## Idempotence

- **M14.17** There is no `IF NOT EXISTS` on `CREATE TABLE`. Idempotence MUST be
  spelled with an `IF NOT EXISTS (SELECT … FROM sys.objects)` guard around the
  statement, which is why the system-table DDL reads wordier than elsewhere.
- **M14.18** SQL Server likewise has no `ADD … IF NOT EXISTS`, so upgrade DDL is
  **not** idempotent on its own and MUST carry the same guard, diffing against
  `sys.columns` and applying only what is missing.

## Upgrade DDL

- **M14.32** Column additions MUST be spelled `ALTER TABLE t ADD col type`.
  T-SQL has **no `COLUMN` keyword** here; `ADD COLUMN` is MySQL and PostgreSQL
  syntax and SQL Server's parser rejects it outright.

  The emitter carried `ADD COLUMN` from the MySQL original until this revision
  (**F-25**). No live run could have caught it: the DDL test installs a fresh
  schema, where the audit envelope arrives through `CREATE TABLE`, so the
  upgrade path has never been executed against a server.

- **M14.33** Every column the upgrade path adds MUST be nullable **or** carry a
  `DEFAULT`. SQL Server refuses to add a `NOT NULL` column with no default to a
  table that has rows, and every history table an upgrade touches has rows.

  Concretely, `actor` MUST be `NVARCHAR(MAX) NOT NULL DEFAULT
  'unauthenticated'`, as in the PostgreSQL original and `fhir-sqlite`. The MySQL
  and MariaDB ports omit the default because *their* engines forbid one on
  `TEXT`; this port has no such limit, and copying the omission was a defect
  rather than a departure (**F-26**). Rows predating the envelope then read as
  `unauthenticated` — the honest answer for a change recorded before there was
  anywhere to record a principal — instead of the upgrade failing.

## Append-only history

- **M14.19** Append-only enforcement MUST use `CREATE OR ALTER TRIGGER` with
  `INSTEAD OF UPDATE` and `INSTEAD OF DELETE`, raising `THROW 50000` naming
  `M3.17`.

  `CREATE OR ALTER` is T-SQL's own, so each guard is **one idempotent
  statement** — closer to the PostgreSQL original than MySQL manages. No
  `DROP TRIGGER` is emitted, and a `DROP`-then-`CREATE` pair MUST NOT be used:
  it leaves a window in which history is unguarded.

- **M14.20** `UPDATE` MUST never be permitted. There is no legitimate reason to
  rewrite a history row in place.

- **M14.21** `DELETE` MUST be permitted only when the session context value
  `fhir_mssql_erasure` is `'on'`, which is how erasure (`M3.18`) is performed.

  `SESSION_CONTEXT()` is T-SQL's nearest equivalent to PostgreSQL's `SET LOCAL`:
  per-session, and it survives inside a transaction. `sp_set_session_context`
  sets it.

  The guard is **not** a defence against the application itself, which can set
  the value; it is a defence against the far likelier accident — ordinary code,
  a migration, or a stray `DELETE` touching history at all.

## Paging and placeholders

- **M14.22** Paging MUST use `OFFSET n ROWS FETCH NEXT m ROWS ONLY`, not
  `LIMIT`. Parameter placeholders are `@P1`, `@P2`, ….
- **M14.23** `NULLS LAST` does not exist in T-SQL and MUST NOT be emitted;
  ordering that needs it MUST express it another way.

## Driver and transport

- **M14.24** The driver is `tiberius` — pure-Rust TDS, so no ODBC or FreeTDS on
  the host — with `rustls`, because SQL Server negotiates TLS during login even
  for an otherwise plaintext connection (`O10.7`).

## What this port has not decided

Stated explicitly, because `X15.6` treats silence as a defect rather than as
"nothing to say".

- **M14.25** **Snapshot isolation (`R4.5`) is undecided.** SQL Server's
  `SNAPSHOT` level provides the stable snapshot the requirement needs, but it
  must be enabled at the database level
  (`ALTER DATABASE … SET ALLOW_SNAPSHOT_ISOLATION ON`) — a deployment action
  this port has not specified. `READ COMMITTED SNAPSHOT` is the alternative and
  has different semantics for writers. Choosing between them, and stating what
  `init` must verify, is required before this port has a store.
- **M14.26** **Write serialization (`H5.4`) is undecided** — the T-SQL
  equivalent of `SELECT … FOR UPDATE` for ordering `version_id` assignment and
  the chain append.
- **M14.27** **Install atomicity at scale (`G2.5`) is untested.** SQL Server has
  transactional DDL; whether ~7,400 tables in one transaction is workable, and
  what the staged-schema equivalent would be, is unknown.
- **M14.28** **There is no store crate implementation.**
  `crates/fhir-mssql-store/src/` contains `lib.rs` and `chain.rs` and nothing
  else, and there are no store tests. This port is **Scaffold** level (`C0.8`).

## Testing

- **M14.29** The DDL execution test (`crates/fhir-mssql-map/tests/mssql_ddl.rs`)
  is this port's only live evidence. It MUST run against SQL Server, and CI MUST
  provision SQL Server rather than a substitute (`O10.12`).

  Until this revision it did neither. CI provisioned `mysql:8.4`, set
  `FHIR_MSSQL_TEST_DSN` to a MySQL DSN, and invoked `--test mysql_ddl` — a
  target that does not exist in this package. The job failed on
  `error: no test target named mysql_ddl` and had never executed a single
  assertion (**F-06**).

- **M14.30** The test MUST **fail rather than skip** when a database is
  required. It self-skips without `FHIR_MSSQL_TEST_DSN`, which is right locally
  and wrong in CI, where a skip is indistinguishable from a pass (`T11.12`,
  `T11.13`). `FHIR_MSSQL_REQUIRE_DB=1` makes an absent or unreachable database
  an error, and CI sets it.

- **M14.31** Prior verification was against `azure-sql-edge`, the arm64 build,
  which is a **subset** of the product. A pass there is good evidence and not a
  conformance claim. Verification against full SQL Server is required before
  this port claims Schema level.

---

Part of the [fhir-mssql specification](index.md), which is part of the
[fhir-databases specification](../../spec/index.md).
