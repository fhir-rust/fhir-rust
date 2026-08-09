# 14. SQL Server dialect

**Status: proposed.** A draft for review, not yet ratified (`X15.9`), so it MUST
NOT be cited as evidence for a conformance level until it is.

This annex records where the SQL Server port departs from the
[monorepo core](../../spec/databases/index.md), and — as importantly — where it does not.
Requirements are numbered `M14.x` and use RFC 2119 keywords.

Target: **Microsoft SQL Server 2016 or later** (see `M14.3`), one database, one
schema per FHIR version.

> **This file was rewritten.** It previously contained the `fhir-mysql` annex
> with three lines changed — titled "14. MySQL dialect", declaring the target as
> "MySQL 8.0 or later, InnoDB, `utf8mb4`", carrying a section headed
> "Relationship to fhir-mariadb", and mentioning SQL Server nowhere
> ([`audit.md`](../../spec/databases/audit.md) **F-16**).
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

  **Settled.** [Unbounded string search](../../spec/databases/unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) (`U1`–`U10`, `P6.9`) is now
  normative and supersedes the persisted-computed-column sketch this
  requirement used to carry. A text column this engine cannot index gets a
  **bounded adjunct** (`<col>_idx`, `NVARCHAR(450) COLLATE
  Latin1_General_100_BIN2`) *and* a **checksum adjunct** (`<col>_h`, `CHAR(64)
  COLLATE Latin1_General_100_BIN2`) in the generated map.

  Why both, when `NVARCHAR(MAX)` already answers `=` here and only the *index*
  is missing: a bounded prefix cannot answer equality, so an index on it must be
  confirmed against the source column anyway (`U7`), and the checksum turns that
  confirmation from a scan into a seek. The sibling Oracle port needs the
  checksum for a stronger reason — a `CLOB` answers no comparison at all — and
  `U2` requires both everywhere rather than letting the two ports diverge on
  which half they implement.

  It belongs in the **generated map**, not in `ddl.rs` (`X15.1`: shared across
  all six ports). Until it lands, this port MUST NOT claim `P6.4a`.

- **M14.32** **`U10` record: which columns get adjuncts, and what the bound
  is.** Every column a `string` search parameter targets, and only those — the
  generator adds the pair in `add_adjunct_columns`, gated on
  `ddl::TEXT_ADJUNCTS`, which is `true` here. The bound *n* is **450
  characters**, matching what `col_sql` declares for `ColTy::TextIdx` and what
  `shred.rs` truncates to in `ADJUNCT_BOUND`. 450 is SQL Server's limit for a
  non-clustered index key at 900 bytes over a 2-byte character type.

- **M14.33** **The checksum column is `BINARY(32)`** — SHA-256's raw bytes, as
  `U4a` now requires normatively.

  This requirement previously recorded the opposite: `CHAR(64)` holding
  lowercase hex, chosen to avoid adding a `SqlVal::Bytes` variant that every
  store crate would have to bind, since per-port binding of a new value type is
  where **F-20** was found. The owner decided for binary, `U4a` was written to
  say so, and the variant was added.

  The `F-20` risk is therefore live rather than avoided, and `U4a` converts it
  into an obligation: this port MUST have a test that round-trips a digest
  through its driver and fails if the binding is wrong. It has no store yet, so
  that test arrives with one — and until it does, the binding here is unproven,
  which is a reason this port stays at Scaffold and not a detail.

  `col_sql` emits `BINARY(32)`; `bytea` on PostgreSQL, `BLOB` on SQLite,
  `BINARY(32)` on MySQL and MariaDB, `RAW(32)` on Oracle. All five are the same
  32 bytes.

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
  (**F-25**). No live run could have caught it *at the time*: the DDL test
  installs a fresh schema, where the audit envelope arrives through `CREATE
  TABLE`, so the upgrade path went unexercised against a server until
  `MsSqlStore::upgrade` was written and run live (**F-15**, below) — which is
  what finally executed this statement shape for the first time.

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

- **M14.35** **The store's `upgrade` MUST run its whole additive-plus-destructive
  apply inside one `BEGIN TRANSACTION` / `COMMIT TRANSACTION`, `ROLLBACK
  TRANSACTION` on the first failure.** T-SQL DDL participates in a transaction
  like any other statement — unlike MySQL and MariaDB, whose DDL commits
  implicitly, so `fhir-mysql-store`'s own doc comment records a failed upgrade
  as unpreventable and merely reports how far it got. That regression does not
  apply here: a failed `MsSqlStore::upgrade` leaves the schema exactly as it
  was, never half-upgraded, and this MUST NOT be weakened into a
  report-and-continue story the way MySQL's had to be.

  `backfill_norm` runs **outside** this transaction, after it commits, in its
  own bounded batches — it is a bulk data write over existing rows, not schema
  DDL, and there is no reason to hold schema locks for however long a large
  backfill takes.

- **M14.36** **Table drops in the destructive diff MUST be ordered children
  before their base table.** Every non-`Base` table carries `FOREIGN KEY (rid)
  REFERENCES base(id)` (`create_table`, `M14.12`-adjacent), and SQL Server
  refuses `DROP TABLE` on a table something else still references — error 3726
  — regardless of `ON DELETE CASCADE`, which governs `DELETE`, not `DROP
  TABLE`. A destructive diff that drops both a resource's base table and its
  child tables in map-iteration order therefore fails unpredictably depending
  on `HashMap` order.

  Found live, not by reading the manual: the first version of
  `MsSqlStore::diff_maps` dropped tables in whatever order a `HashMap` yielded
  them, and `destructive_changes_succeed_with_the_flag` failed on its first run
  against `azure-sql-edge` with `Could not drop object 'basic' because it is
  referenced by a FOREIGN KEY constraint`. Fixed by partitioning the destructive
  table-drop statements into non-`Base` and `Base` buckets and emitting the
  former first.

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

- **M14.34** **That choice carries four unfixed advisories, and — unlike when
  this requirement was first written — they now reach a shipping crate.**
  `tiberius 0.12.3` — confirmed still the newest release, 2026-08-04 — pins
  `rustls 0.21`, which pins `rustls-webpki ^0.101`. That chain has three
  vulnerabilities (RUSTSEC-2026-0098, -0099, -0104: name-constraint handling
  for URI and wildcard names, and a reachable panic parsing certificate
  revocation lists) plus one unmaintained crate (RUSTSEC-2025-0134,
  `rustls-pemfile`). The fixes live in `rustls-webpki >= 0.103.12`, which
  requires `rustls 0.23`, which no tiberius release supports.

  **What changed.** This requirement originally recorded these as reached
  only through `tiberius` as a *dev*-dependency of the DDL test — narrow, and
  correctly so at the time. `fhir-mssql-store` (`crates/fhir-mssql-store`,
  a real, published crate) now depends on `tiberius` directly, confirmed by
  running `cargo tree -p fhir-mssql-store -e normal`, and the four advisories
  are in that tree. The `deny.toml` comments said "verified absent from
  `cargo tree -e normal`" for two more weeks than that was true; nobody
  re-ran the check when the store landed. Corrected 2026-08-04, and cited
  here so the same drift does not repeat: **this port stores patient data
  through code with unpatched TLS certificate-validation defects.**

  **What was tried, 2026-08-04, and did not work.** `native-tls` — this
  requirement's own suggested escape from `rustls` — was substituted for
  `rustls` in both the workspace `Cargo.toml` and a standalone probe, and
  fails the TLS handshake outright against this port's own test server, with
  or without certificate trust: `Error forming TLS connection: connection
  closed via error`, even under `TrustConfig::TrustAll`. `native-tls` on this
  host resolves to Apple's Secure Transport (`security-framework`), which is
  itself deprecated by Apple and evidently will not complete a handshake
  against whatever `azure-sql-edge` presents (see `M14.24`'s sibling finding
  on the certificate's own malformed X.509 version). This was reverted; the
  driver is `rustls` again. `cargo tree` and `cargo deny check advisories`
  confirm the four advisories genuinely disappear under `native-tls` — the
  chain is gone, not merely hidden — so it remains the fix *if* the handshake
  problem is solved separately (a different host, a different TLS backend
  configuration, or Microsoft fixing the certificate `azure-sql-edge`
  generates), just not one this pass could ship.

  They stay ignored in `deny.toml`, with the corrected reasoning above rather
  than the stale one, because there is no fix available today: no newer
  tiberius, no working alternative TLS backend on this host, and the only
  other pure-Rust TDS driver does not exist. This is now a standing residual
  risk requiring an explicit owner decision — accept it formally, pursue a
  different driver, or drop TLS as this port's transport story — not a
  research question with an obvious next experiment.

  **What this does *not* undermine: `tests/ssl_live.rs` genuinely confirms
  the verification *mechanism* works** — `TrustServerCertificate=false`
  measurably rejects `azure-sql-edge`'s self-signed certificate, reproducibly,
  which `TrustServerCertificate=true` accepts. The two findings are
  independent and both true: the code path that decides trust/no-trust is
  exercised and behaves correctly, and the certificate-parsing code inside
  that same dependency chain has three unpatched CVEs. `O10.7` requires both
  a working mechanism *and* a trustworthy implementation of it; this port has
  the first and not the second, so it MUST NOT claim `O10.7` satisfied,
  though it is closer to satisfying it — and more precisely diagnosed — than
  before this pass.

## Snapshot isolation and write serialization — decided and verified

- **M14.25** **Snapshot isolation (`R4.5`) is decided: `SNAPSHOT`, not
  `READ_COMMITTED_SNAPSHOT`, and both the decision and the fix were reached by
  running things, not by reading the manual.**

  `fhir-mssql-store`'s `get` wraps its base-plus-child-table read in `BEGIN
  TRANSACTION … ROLLBACK TRANSACTION`, which is not by itself the same claim
  as snapshot isolation — under this engine's default `READ COMMITTED`, each
  statement inside that transaction sees the latest committed data as of when
  *it* runs, not one consistent instant for the whole transaction, unlike
  PostgreSQL's or MySQL's default. `tests/concurrency.rs`'s
  `reads_never_tear_under_concurrent_writes` reproduced exactly this live: a
  reader observed `active` from one write interleaved with `name`/`telecom`
  from another.

  The first fix attempted was the simpler-sounding of the two candidates —
  `READ_COMMITTED_SNAPSHOT` — enabled at the database level. Run live, it did
  **not** stop the torn read: RCSI gives each *statement* its own snapshot,
  not the *transaction* one shared snapshot across every statement in it,
  which is the actual requirement. The fix that measurably works is `get`
  issuing `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` immediately before
  `BEGIN TRANSACTION`, which requires `ALLOW_SNAPSHOT_ISOLATION` enabled at
  the database level — confirmed by the same test passing five consecutive
  runs after the change and none before it.

  Getting either isolation option set at all first required a database this
  port could run `ALTER DATABASE` against: every DSN used by this port used to
  omit `database=` and land in `master`, where SQL Server refuses the option
  outright (`Option 'READ_COMMITTED_SNAPSHOT' cannot be set in database
  'master'.`). `scripts/db.sh`'s `post_ready` now creates a dedicated
  `fhir_mssql` database and enables both `READ_COMMITTED_SNAPSHOT` (harmless,
  and still worth having for ordinary reads elsewhere) and
  `ALLOW_SNAPSHOT_ISOLATION` on it, once, before any pooled connection exists
  — `ALTER DATABASE` for either option would otherwise have to wait out every
  active transaction.

  `SET TRANSACTION ISOLATION LEVEL` is session-scoped, not
  transaction-scoped, so on this *pooled* connection `get` resets it back to
  `READ COMMITTED` before the connection returns to the pool — the same leak
  discipline `purge`'s `SESSION_CONTEXT` erasure flag already needed, for the
  same reason: a later, unrelated caller must not silently inherit it.

- **M14.26** **Write serialization (`H5.4`) is decided and live-verified:**
  `SELECT … FROM base WITH (UPDLOCK, ROWLOCK) WHERE [id] = @P1` before reading
  the chain tip in both `put` and `delete`, holding the lock until
  commit/rollback so a second writer for the same id blocks rather than races
  the tip read. `tests/concurrency.rs`'s
  `racing_writers_get_distinct_versions_and_a_verifiable_chain` confirms 8 of
  8 racing writers get distinct, consecutive versions and a chain that still
  verifies afterward.

## What this port has not decided

Stated explicitly, because `X15.6` treats silence as a defect rather than as
"nothing to say".

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

- **M14.37** **`path` binds to `NVARCHAR(path_bound)`; the conversion is one
  transactional upgrade.** Decided 2026-08-09 (`U12a`, **F-47** step 2);
  the code lands with F-47 steps 3–4, and until it does the current
  `NVARCHAR(MAX)` stands and this port does not claim `U12` for `path`.

  Today `create_table` hardcodes `path` as `NVARCHAR(MAX)`, which cannot
  be part of an index key (`M14.15`). (Corrected 2026-08-10: an earlier
  revision said the `MAX` binding "drags adjunct columns" — it does not.
  `path` has never had adjuncts on any port; the ext/deep adjunct set is
  `url`, `v_text`, `leaf`, and the U-file itself says "path is
  deliberately not in this set". The cost of `MAX` is unindexability and
  the unsatisfied `U12`, nothing more.) The decided binding reads the
  map's recorded `path_bound` (`U12a`) instead of a constant, so the DDL
  follows the asset (`G2.2`).

  The conversion, in order, inside the transactional upgrade (`M14.35`):
  pre-check `MAX(LEN("path"))` against the bound and refuse with the count
  and maximum if any row exceeds it; then `ALTER COLUMN`. The index the
  narrowing makes possible MAY follow when a search first filters on
  `path` — none does today, so the narrowing buys `U12` and an honest
  schema now, indexability later. A failed step rolls the whole upgrade
  back — this engine's advantage over the sibling Oracle port, whose
  conversion cannot be atomic (`M14.38` there).

  `v_kind` is already `CHAR(1)` on this port; nothing changes for it.

---

Part of the [fhir-mssql specification](index.md), which is part of the
[fhir-databases specification](../../spec/databases/index.md).
