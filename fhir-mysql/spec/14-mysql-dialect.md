# 14. MySQL dialect

**Status: proposed.** This section is a draft for review, not yet ratified.
Sections 1–13 were inherited verbatim from the PostgreSQL original and still
describe PostgreSQL; where they conflict with this section, this section is the
intended target and the older text is what needs amending. Requirements are
numbered `M14.x` and use RFC 2119 keywords.

This annex records where the MySQL port departs from the storage model in
section 3, and — as importantly — where it does not. Every departure MUST
preserve the invariants that are not about SQL at all: round-trip fidelity
(section 4), search semantics (section 6), and the history hash chain
(section 12).

Target: MySQL 8.0 or later, InnoDB, `utf8mb4`. Window functions, `LATERAL`,
`JSON_TABLE`, and multi-valued `SIGNAL` are all assumed and all require 8.0.

## Relationship to fhir-mariadb

The sibling `fhir-mariadb` project began as a fork of this one. The two are
**independent from here on**.

- **M14.0a** This port SHOULD use whatever MySQL does best, and MUST NOT
  restrict itself to syntax MariaDB also accepts. Divergence is expected, not a
  defect to be reconciled.
- **M14.0b** A schema installed by this port is **not** required to be readable
  by `fhir-mariadb`, and vice versa. The emitted SQL will differ. No requirement
  in this specification depends on the two being interchangeable, and nothing
  should be held back to keep them so.
- **M14.0c** What the two ports MUST continue to share is *behaviour*, not SQL:
  round-trip fidelity (section 4), search semantics (section 6), and the
  canonical form the hash chain commits to (M14.19/M14.20). Two servers must
  answer a FHIR® request identically; how they spell the DDL is their own
  business. This is what makes divergence safe — the conformance suite, not the
  schema text, is the shared contract.

### Divergences

- **M14.0d** `utf8mb4_0900_bin` is the exact-comparison collation, MySQL 8's NO
  PAD binary collation. This is a correctness requirement, not a preference: the
  obvious `utf8mb4_bin` is **PAD SPACE**, under which `'Smith' = 'Smith '`
  evaluates true — which would silently widen `:exact` matching and weaken
  primary-key identity. PostgreSQL's `COLLATE "C"` does not pad. MariaDB spells
  the same property `utf8mb4_nopad_bin`.
- **M14.0e** Triggers are emitted as `DROP … IF EXISTS` followed by
  `CREATE TRIGGER`, because MySQL has neither `CREATE OR REPLACE TRIGGER` nor
  `CREATE TRIGGER IF NOT EXISTS`. MariaDB has the former and its port uses it,
  emitting half as many statements.
- **M14.0f** The history audit envelope's `ALTER TABLE … ADD COLUMN` statements
  are **not** idempotent, because MySQL has no `ADD COLUMN IF NOT EXISTS`. The
  caller MUST diff against `information_schema.columns` first. MariaDB has it and
  its port keeps the PostgreSQL original's blind-apply contract.
- **M14.0g** MySQL's native `JSON` type and multi-valued indexes are available
  and MUST NOT be used for the signed `resource` column, for the reason in
  M14.14 — a `JSON` column re-normalizes on write, so the bytes read back would
  not be the bytes the chain signed. Availability is not a reason to adopt.

## What does not change

- **M14.1** `fhir-mysql-gen` MUST NOT change. Its two dialect-looking
  constants are already correct for MySQL: `PG_MAX_IDENT = 63` is *tighter*
  than MySQL's 64-character identifier limit, and `SPLIT_WIDTH = 150` is well
  below MySQL's 1017-column ceiling. Identifier fitting and table splitting
  carry over unmodified.
- **M14.2** `shred.rs`, `reconstruct.rs`, `value.rs`, `fold.rs`, and `model.rs`
  MUST NOT change. They operate on Rust types and never emit SQL.
- **M14.3** `chain.rs` MUST NOT change except as required by M14.16.
- **M14.4** The transport surface MUST stay encrypted and verifying by
  default, in this port's own mechanism (amended 2026-08-06; **F-54**).
  Unlike the SQLite port, MySQL is a network server, so there is a link to
  protect — but not with the PostgreSQL original's machinery: the store reads
  `FHIR_MYSQL_SSL_MODE` in MySQL's own `--ssl-mode` vocabulary (not libpq's
  `sslmode`/`PGSSLMODE`) via `ssl::SslMode`, defaulting to `VERIFY_IDENTITY`,
  live-verified by `tests/ssl_live.rs`. `SslPolicy` and the plaintext-refusal
  bind guard do not exist here — a library binds no socket — and the TLS-only
  CI job was removed (T72).

## Accent and case folding

- **M14.5** The port MUST NOT emit a `_norm` SQL function. In the PostgreSQL
  original the function `fhir_postgresql_norm` is emitted into the schema but
  never called: folding is performed in Rust by `fold::fold` on the write path
  (the shredder fills the materialized `_norm` column), on the read path (the
  search term is folded before binding), and in the upgrade backfill. Every
  index is on a plain materialized column, not a function expression.
- **M14.6** The port MUST NOT substitute a MySQL accent-insensitive collation
  (`utf8mb4_0900_ai_ci`) for the folded column. It is tempting — MySQL can do
  accent- and case-insensitive comparison natively and indexably — but it would
  put the fold rules in the server, where they can no longer be guaranteed to
  agree with `fold::fold`. Two folding implementations that disagree produce a
  patient who cannot be found, which is precisely the failure `fold.rs` exists
  to prevent. Folding stays in Rust.
- **M14.7** Conversely, `ColTy::TextC` MUST map to an explicitly binary
  collation. MySQL's *default* collation is accent- and case-insensitive, so a
  column declared plain `TEXT` would silently acquire fuzzy equality — the
  opposite of what `text COLLATE "C"` means, and enough to break `:exact`
  matching, primary-key identity, and the prefix range scans in section 6.

## The `ords` column

- **M14.8** `ords` MUST be a `VARBINARY(255)` column, on the `Elem`, `Ext`, and
  `Deep` table kinds. It MUST NOT be modelled as a set of fixed ordinal
  columns, nor as a normalized side table. (`Contained` carries a scalar
  `ord smallint`, not `ords`; that becomes `SMALLINT` and is unaffected.)
- **M14.9** The stored image MUST remain the array literal produced by
  `fmt_ords` — `{1,2}`, `{}`, `{-1,3}` — so that `fmt_ords` and `parse_ords`
  are unchanged and a database can be compared value-for-value against a
  PostgreSQL instance. `VARBINARY` rather than `VARCHAR` because the image is
  pure ASCII, making it one byte per character against the index key budget of
  M14.12, and because binary comparison is exact by construction.

  *Rationale.* `ords` looks like the hardest thing to port and is not. The
  database never orders, compares, subscripts, or unnests it: there is no
  `ORDER BY ords` anywhere in the tree, no `@>`, `&&`, `array_length`, or
  `unnest` applied to it, `search.rs` does not mention `ords` at all (child
  tables correlate on `rid` alone), and reconstruction is order-insensitive by
  construction — rows land in hash maps keyed on `ords` before any resource is
  rebuilt, and array order comes from the numeric value of the last ordinal
  (`reconstruct.rs:38-90`, `:276`, `:414`). Reads are already issued unordered
  and pipelined concurrently, so row arrival order is arbitrary by design. The
  only thing the database does with `ords` is enforce uniqueness as part of a
  primary key and return it intact. It already crosses the wire as text in
  both directions — `($n::text)::smallint[]` on insert, `SELECT "ords"::text`
  on read — so the change is: alter the type in three `CREATE TABLE` branches
  in `ddl.rs`, and drop the three casts and three projections in
  `store/src/lib.rs`.

- **M14.10** Three properties of the value domain MUST survive:
  - **Negative ordinals are meaningful.** When two cyclic `contentReference`
    referrers share one table, the second pushes negated ordinals so paths stay
    unambiguous (`model.rs:211`; 3 such elements in R5, 2 in R4, 1 in R3). The
    domain is `-32767..=-1 ∪ 1..=32767`; `0` never occurs. Any unsigned or
    magnitude-only encoding is wrong.
  - **The empty array is valid and frequent.** Resource-level extensions and
    element ids shred with `ords = {}` into a `NOT NULL` primary-key column.
    Empty MUST remain storable and distinguishable from `NULL` and from every
    other value; note that MySQL permits the empty string in a `NOT NULL`
    column, so `{}` stores as the two-byte image and no special case is needed.
  - **Depth is unbounded for recursive types.** Non-recursive tables reach
    depth 6 at most in R5, but 23 R5 resource types (18 R4, 13 R3) own
    recursive tables — `Questionnaire.item.item`, `QuestionnaireResponse`,
    `ImplementationGuide`, `StructureMap`, and others — whose depth is
    data-dependent and uncapped.
- **M14.11** Because a primary-key column MUST be bounded in MySQL, M14.10's
  unbounded depth becomes a real limit rather than a theoretical one.
  `VARBINARY(255)` admits roughly 36 levels of nesting at the worst-case seven
  bytes per level (`-32767,`), and far more in practice. Exceeding it MUST fail
  loudly at shred time with a dedicated error naming the resource and path —
  it MUST NOT be silently truncated, which would collide two distinct rows in
  the primary key and corrupt reconstruction. `push_ord` already has the shape
  for this (it rejects over-long arrays with `"array too long"`).

- **M14.12** **The hardest problem in this port, and it is not `ords`.**
  InnoDB limits an index key to 3072 bytes, and MySQL cannot index an
  unbounded `TEXT` column at all without a prefix length. The `Ext` and `Deep`
  primary keys both include `path` and `leaf`, which are unbounded `text` in
  the PostgreSQL original:

  - `Ext`: `PRIMARY KEY (rid, path, ords, modifier, ext_ord, leaf)`
  - `Deep`: `PRIMARY KEY (rid, path, ords, leaf)`

  A prefix index MUST NOT be used to satisfy these. A prefix index does not
  enforce uniqueness over the full value, so two rows differing only beyond the
  prefix would be rejected as duplicates — silently losing data that
  reconstruction would then report as an ordinal gap.

  The port MUST therefore replace the natural primary key on `Ext` and `Deep`
  with a surrogate: a `BINARY(32)` column holding a SHA-256 over the
  canonically-joined natural key components, declared `PRIMARY KEY`, with the
  natural columns retained as ordinary columns and a non-unique index on
  `rid` to serve the read path (which only ever filters `WHERE rid = ?`).

  **Open decisions.** (a) Whether `Elem` also needs this — its key is only
  `(rid, ords)`, which at `VARCHAR(64)` + `VARBINARY(255)` fits comfortably, so
  probably not, and keeping the natural key where possible is preferable. (b)
  Whether `rid` should be `VARCHAR(64)` with the `ascii` charset: the FHIR `id`
  type is constrained to `[A-Za-z0-9\-\.]{1,64}`, so 64 ASCII bytes is exact
  and spec-faithful rather than an arbitrary cap. This SHOULD be adopted.
  (c) Whether the surrogate hash should be a MySQL `GENERATED` column
  (self-maintaining, but puts the canonical join rule in the server) or
  computed in Rust (consistent with M14.6's reasoning). Rust is recommended,
  for the same reason.

- **M14.13** One user-facing behaviour regresses and MUST be documented rather
  than quietly dropped. `book/src/querying.md:29-30` teaches PostgreSQL array
  idioms to end users: `ords = '{1}'` addresses the first instance of a
  repeating element, and `ords[1] = 1` matches any descendant of the first
  instance. The equality form survives verbatim; the subscript form does not.
  This is the only array subscripting in the repo, and the book MUST be amended
  for this port — with `ords LIKE '{1,%'` offered as the nearest equivalent,
  noting that it is a prefix match on the stored image rather than a typed
  subscript.

## Type mapping

- **M14.14** `col_sql` MUST map `ColTy` as follows.

  | `ColTy` | PostgreSQL | MySQL |
  |---|---|---|
  | `Bool` | `boolean` | `TINYINT(1)` |
  | `Int` | `integer` | `INT` |
  | `BigInt` | `bigint` | `BIGINT` |
  | `Numeric` | `numeric` | `TEXT` + `DOUBLE` sort column — see M14.15 |
  | `Text` | `text` | `TEXT` (`utf8mb4`) |
  | `TextC` | `text COLLATE "C"` | `TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_bin` — see M14.7 |
  | `Date` | `date` | `DATE` |
  | `Timestamptz` | `timestamptz` | `DATETIME(6)`, UTC — see M14.15 |
  | `Jsonb` | `jsonb` | `JSON` — see M14.17 |

- **M14.15** FHIR `decimal` cannot be stored as `DECIMAL`: M3.6 requires that
  a decimal's "original textual precision MUST survive round-trip", and
  `DECIMAL(65,30)` returns `1.50` as `1.500000000000000000000000000000` — a
  fixed declared scale cannot preserve a per-value lexical form. `DOUBLE` is
  worse. PostgreSQL satisfies M3.6 with `numeric` alone because `numeric`
  preserves the scale as written.

  The port MUST therefore follow the pattern M3.6 already establishes for
  dates: store the verbatim lexical form in a `TEXT` column and emit a
  derived, indexed `<name>_sort` column of `DOUBLE` for ordering and range
  search. Retrieval reads the verbatim column; numeric and quantity range
  predicates read the sort column.

  *Note.* This is not a new mechanism — `date` already maps to `text` + a
  `date` sort column, and `dateTime`/`instant` to `text` + a `timestamptz`
  sort column. Extending it to `decimal` keeps one convention. It does mean
  `ColTy::Numeric` acquires a companion column in the generated map, which is
  the only place in this annex that touches the column model rather than a type
  name; `add_norm_columns` is the precedent to follow. **Open decision:** a
  `DECIMAL(65,30)` sort column would give exact comparison within its range
  instead of `DOUBLE`'s binary rounding, at the cost of storage and of a hard
  range limit. `DECIMAL` is probably the better choice here precisely because
  MySQL has it and SQLite does not — the two ports need not agree.

- **M14.16** `Timestamptz` MUST become `DATETIME(6)` with values normalized to
  UTC in Rust before binding. `TIMESTAMP` MUST NOT be used: it silently
  converts on the session time zone and its range ends in 2038. Fidelity is
  unaffected — under M3.6 the verbatim value already lives in a separate `text`
  column, so this is an ordering key only. The sentinel
  `'infinity'::timestamptz` MUST be replaced by `'9999-12-31 23:59:59.999999'`;
  `+ interval '1 second'` becomes `+ INTERVAL 1 SECOND`, which MySQL supports
  natively.
- **M14.17** `bytea` MUST become `VARBINARY(n)` where a bound is known and
  `BLOB` otherwise; `bigserial` MUST become `BIGINT AUTO_INCREMENT` (MySQL
  permits one per table and requires it be indexed); `char(1)` MUST become
  `CHAR(1)`.
- **M14.18** Every generated table MUST stay within MySQL's 65,535-byte
  row-size limit. `TEXT` and `BLOB` columns cost only their in-row pointer, so
  a 150-column table of `TEXT` is safe; the requirement exists because
  substituting `VARCHAR(n)` for `TEXT` as a local optimisation would silently
  blow the limit on wide tables. `VARCHAR` MUST NOT be introduced for data
  columns without re-checking the row budget.

- **M14.18a** *Amends `M14.18`.* The 65,535-byte limit is not the one that
  binds. InnoDB runs a second, stricter check at CREATE time against the
  row-format page limit (~8,126 bytes with the default `DYNAMIC` format),
  and under it a `TEXT` column is *not* just a pointer: it charges ~41
  bytes (measured by bisection on a live server: 195 `TEXT` columns
  install, 196 fail with `ERROR 1118` — **F-90**). "A 150-column table of
  `TEXT` is safe" therefore held only per expansion, not per table: sibling
  expansions summed past the check. The bound is now enforced where the
  table shapes are decided — the shared generator's `G2.6a` charged-byte
  budget — not in this dialect; this annex's obligation is unchanged from
  `M14.18`'s second half: substituting `VARCHAR(n)` for `TEXT` as a local
  optimisation still requires re-checking the row budget, which `G2.6a`'s
  charge model does not cover.

## Canonical JSON and the hash chain

- **M14.19** This is the port's most consequential departure after M14.12, and
  MUST be settled before any store code is written.

  The history hash chain does not commit to the submitted resource text. It
  commits to *PostgreSQL's* `jsonb` canonical rendering: `store/src/lib.rs`
  computes `(($1::text)::jsonb)::text AS canon`, and `chain.rs:218` states the
  requirement — "`resource` MUST be the **stored** normalized form
  (`jsonb::text`), not the submitted text: `jsonb` reorders keys and rewrites
  number spellings". MySQL's `JSON` type also normalizes and sorts keys, but
  not by PostgreSQL's rule and not with PostgreSQL's serialization, so the
  bytes differ and every chain would fail verification against a PostgreSQL
  original.

- **M14.20** Canonicalization MUST therefore move out of SQL and into Rust,
  with a single defined serializer used by both the writer and the verifier.

  **RFC 8785 (JSON Canonicalization Scheme) MUST NOT be used.** It is the
  obvious standard to reach for and it is wrong here: JCS §3.2.2.3 serializes
  numbers via ECMAScript `Number::toString`, i.e. as IEEE-754 doubles. That
  renders `1.50` as `1.5` and `123456789012345678901234567890.123` as
  `1.2345678901234568e29` — destroying exactly the decimal precision M3.6
  requires, and making two resources that differ in a clinically meaningful
  decimal hash identically.

  The canonical form MUST instead be:
  - **Object keys** sorted by UTF-8 byte order. (A fresh rule is chosen rather
    than reproducing PostgreSQL's `jsonb` ordering — which sorts by key length
    first, then bytewise — because compatibility with existing chains is
    already being broken and a simple rule is easier to reimplement.)
  - **Numbers** emitted as their parsed lexeme, verbatim. `serde_json` with
    the `arbitrary_precision` feature — already enabled workspace-wide — does
    this: `1.50`, `0.1`, and 30-significant-digit decimals all survive
    unchanged. Note it does normalize an exponent's sign (`1e2` → `1e+2`),
    which is deterministic and therefore acceptable.
  - **Strings** escaped minimally and deterministically: only the escapes JSON
    requires, `\uXXXX` lowercase-hex for control characters, no gratuitous
    escaping of non-ASCII.
  - **No insignificant whitespace**, and no space after `:` or `,`.
  - **Duplicate object keys** are out of scope for the canonicalizer: by the
    time a resource is a parsed value they cannot exist, because the JSON
    parser has already collapsed them (last one wins). Any policy about
    rejecting them MUST be enforced on ingest instead. The canonicalizer is
    consequently **infallible** — every parsed value has exactly one canonical
    form — and MUST NOT carry an error path no caller can reach.

  Implemented as `canon::canonicalize` in the map crate, with the properties
  above pinned by unit tests (`crates/fhir-mysql-map/src/canon.rs`). It is
  deliberately in the map crate rather than the store crate: it is
  dialect-neutral and depends on nothing but `serde_json`.

  *Consequence, and an argument for doing this upstream.* Once the canonical
  form is Rust-side, a history chain becomes portable: the same resource yields
  the same preimage in PostgreSQL, SQLite, and MySQL, so a database can be
  migrated between dialects and re-verified. Under the status quo it cannot be.
  This is a breaking change to existing PostgreSQL chains, but the machinery to
  absorb it already exists — `ALGORITHMS` is a versioned array, and
  `resign_history` / `chain-resign` exist to re-sign.

## Namespaces, atomicity, and concurrency

- **M14.21** A PostgreSQL schema maps to a MySQL *database*. The
  `"{schema}"."{table}"` qualified-name shape therefore survives, with
  backtick quoting substituted for double quotes throughout.
- **M14.22** **The atomic-install guarantee is lost and the spec MUST say so.**
  MySQL DDL implicitly commits, so the staged install the PostgreSQL original
  performs — build under a temporary schema, then `DROP SCHEMA … CASCADE` and
  `ALTER SCHEMA … RENAME TO` in one transaction — has no MySQL equivalent.
  `RENAME TABLE` is atomic per statement but cannot practically carry 7,355
  tables.

  The port MUST install into a staging database and then publish it by flipping
  an explicit readiness marker that every reader checks, so a half-installed
  schema is never served. Installation MUST be idempotent and resumable.
  Section 10 MUST be amended: an interrupted install leaves a staging database
  behind that an operator has to drop, which is a real operational regression
  from PostgreSQL and MUST be documented rather than discovered.
- **M14.23** `pg_tables` and `information_schema.schemata` introspection MUST
  be replaced by `information_schema.tables` and
  `information_schema.schemata` (which MySQL also provides).
- **M14.24** The isolation and locking constructs port cleanly, and MySQL is
  the better fit of the two targets here:
  - `REPEATABLE READ READ ONLY` for `snapshot()` → `START TRANSACTION READ
    ONLY`; InnoDB's default isolation is already `REPEATABLE READ`.
  - `SELECT … FOR UPDATE` → supported natively, unchanged.
  - `SELECT pg_advisory_xact_lock($1)` → `SELECT GET_LOCK(?, ?)`. Note the
    semantic difference that MUST be handled: MySQL's named locks are
    *session*-scoped, not transaction-scoped, so they MUST be released
    explicitly (`RELEASE_LOCK`) on every path including error paths, rather
    than relying on transaction end.
- **M14.25** `SET LOCAL fhir_mysql.erasure = 'on'` MUST become a MySQL user
  session variable (`SET @erasure = 'on'`), which is a close analogue. `SET
  LOCAL search_path` MUST be dropped in favour of qualified names, and
  `plan_cache_mode = force_generic_plan` has no equivalent — the EXPLAIN audit
  MUST be rewritten against MySQL's `EXPLAIN FORMAT=JSON` and its guarantees
  restated, since the forced-generic-plan premise does not hold.
- **M14.26** The append-only history guard MUST be reimplemented as MySQL
  `BEFORE UPDATE` / `BEFORE DELETE` triggers raising
  `SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = …`, with the erasure escape
  conditioned on M14.25's session variable. plpgsql, `TG_OP`, and
  `current_setting` have no equivalents; note that MySQL needs one trigger per
  event per table, so the emitted trigger count doubles.

## Driver and query construction

- **M14.27** The port SHOULD use `mysql_async`. Decision D5's reasoning for
  rejecting `sqlx` — that the SQL is generated and dynamic, so compile-time
  checked queries buy nothing — applies unchanged.
- **M14.28** Placeholders MUST become `?`, and the "everything crosses the wire
  as text with explicit casts" convention MUST be replaced by native typed
  binding. The 65,535-parameter chunking in `insert_shredded` remains correct
  (MySQL shares that limit) but MUST additionally respect
  `max_allowed_packet`, which bounds total statement size and has no
  PostgreSQL counterpart.
- **M14.29** Query-construction changes, all mechanical:
  - `ON CONFLICT DO NOTHING` → `INSERT … ON DUPLICATE KEY UPDATE` on a
    no-op assignment (preferred over `INSERT IGNORE`, which also swallows
    unrelated errors).
  - `SELECT DISTINCT ON (…)` → `ROW_NUMBER()` in a subquery.
  - `unnest($1::text[])` → `JSON_TABLE`, or a `VALUES` list.
  - `LEFT JOIN LATERAL` → supported natively in 8.0.14+, unchanged.
  - `= ANY($1)` → `IN (…)` with generated placeholders.
  - `ILIKE` → `LIKE` against the folded column from M14.5, *not* against a
    `_ci` collation.
  - `LIKE … ESCAPE '\'` → unchanged, but note MySQL also treats `\` as a
    string escape, so the pattern needs double-escaping.
  - `ORDER BY … NULLS LAST` → MySQL has no `NULLS LAST`; it MUST be emulated
    with a leading `ISNULL(col)` sort term.

## Testing

- **M14.30** The test strategy carries over largely unchanged, since MySQL is
  a server: `*_TEST_DB` still gates the live suite, and CI still supplies the
  database as a service container.
- **M14.31** The `--test-threads=1` constraint MUST be re-evaluated rather than
  copied. It was adopted because concurrent full-schema installs exhaust
  *PostgreSQL's* `max_locks_per_transaction` budget; MySQL's DDL does not take
  transaction-scoped locks the same way, so the constraint may be unnecessary —
  but MySQL has its own table-count and metadata-lock pressures, so this MUST
  be measured, not assumed.
- **M14.32** `concurrency.rs` SHOULD port with only the locking-primitive
  substitutions of M14.24, since MySQL is genuinely multi-writer. The
  session-scoped `GET_LOCK` semantics of M14.24 MUST have their own test.

## Upgrade

*Added when `upgrade` landed here, closing this port's share of **F-15**.*

- **M14.33** `init` MUST record the **map asset itself** in `fhir_mysql_meta`,
  gzipped and hex-coded under `map_asset`, alongside `map_checksum` and
  `fhir_version`.

  This is what makes an upgrade possible at all: an upgrade diffs the installed
  map against the current one, and a checksum says only *that* something
  changed, never *what*. Hex rather than base64 to match every other port byte
  for byte.

  An install predating this MUST be refused with a message distinguishing it
  from "not installed" — the remedies differ, `init` versus a reload.

- **M14.34** `fhir_mysql_meta`.`value` MUST be `LONGTEXT`, not `TEXT`.

  MySQL's `TEXT` caps at 65,535 bytes. The hex-coded R5 map asset is **about
  2.4 MB** — 37x over — so `M14.33` is impossible in a `TEXT` column: the insert
  fails outright in strict mode, and in a non-strict configuration it
  **truncates**. A truncated map asset still decodes far enough to look like a
  map and would produce a *wrong diff*, which is the worst failure an upgrade
  can have. `TEXT` was what the column had before this revision, so no installed
  database can hold an asset; those must be reinstalled.

- **M14.35** An upgrade MUST report how many statements it had already applied
  when one fails.

  MySQL commits DDL implicitly (`M14.22`), so an upgrade that fails partway
  leaves a schema that is neither the old one nor the new one. Unlike
  `fhir-sqlite`, where the whole upgrade is one transaction, that cannot be
  prevented here — so it MUST be *reported*, naming the count, the fact that
  those statements remain, and the failing statement.

- **M14.36** The reconcile step MUST filter the access-log index statements
  against `information_schema.statistics`, and the history audit envelope
  against `information_schema.columns`.

  `schema_wide_objects` is only partly idempotent: its two tables carry
  `IF NOT EXISTS`, its three indexes do not, because the emitter writes a bare
  `CREATE INDEX`. Re-applying the list wholesale therefore fails with
  `Duplicate key name` on the second run (**F-28**). The doc comment on that
  function claimed unqualified idempotence until F-28; nothing had noticed
  because nothing re-applied the list.

  Both filters MUST be computed **after** the additive statements have been
  applied, not before. A history table created moments earlier by `create_table`
  already carries the audit envelope, so a filter built beforehand emits an
  `ADD COLUMN` for every column that table is about to gain and fails with
  `Duplicate column name 'actor'`. This was written the wrong way round first
  and a live server is what caught it.

- **M14.37** The `_norm` backfill (`L13`, `L14`) MUST run as **part of** the
  upgrade, not as a step afterwards. An upgrade that adds a folded column
  without populating it leaves every existing row NULL, and since every
  non-`:exact` string search compares that column, those resources silently stop
  matching their own values.

  It MUST fold distinct *values* rather than rows, in bounded batches, selecting
  only rows still NULL — which is what makes it resumable.


- **M14.38** The `O10.4c` re-shred MUST run as **one transaction per
  resource**, and the upgrade as a whole is **reported-partial, not atomic**.

  This is `M14.22` reaching migration: MySQL commits DDL implicitly, so an
  upgrade cannot be a single transaction and a failure part-way already leaves
  a schema that is neither the old one nor the new one — which `M14.35`
  requires be *reported*, with how many statements had been applied. The
  re-shred inherits exactly that and adds one guarantee of its own, which the
  storage engine can give even though the DDL cannot: each resource moves
  inside an InnoDB transaction, so **no resource is ever half-carried**.

  What a deployment MUST be told, in the documentation as well as here:

  - **Nothing is dropped until every moved source is verified empty**, in the
    same call, so a failure leaves the data in place and the old columns
    readable.
  - **Rerunning resumes.** A resource already carried re-shreds to itself and
    costs a read.
  - **Reads of un-carried resources under-return the moved element while it
    runs.** Once the additive DDL has landed, a resource not yet carried
    reconstructs under the new map, which no longer reads the old column. The
    window closes when the last resource is carried. SQLite has no such window
    because its whole upgrade is one transaction (`fhir-sqlite M14.31`);
    PostgreSQL has the same window for the same reason as here
    (`fhir-postgresql M14.29`). `M14.x` is per-port under `C0.7`, so those
    citations are qualified by port and a bare number in this annex means this
    annex.

  The third point is the one that a dialect story is tempted to omit. `O10.4`
  requires each dialect to state its failure story; stating only the two
  reassuring halves would satisfy that on paper and not in fact.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
