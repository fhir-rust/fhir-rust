# 14. SQLite dialect

**Status: proposed.** This section is a draft for review, not yet ratified.
Sections 1–13 were inherited verbatim from the PostgreSQL original and still
describe PostgreSQL; where they conflict with this section, this section is the
intended target and the older text is what needs amending. Requirements are
numbered `M14.x` and use RFC 2119 keywords.

This annex records where the SQLite port departs from the storage model in
section 3, and — as importantly — where it does not. Every departure MUST
preserve the invariants that are not about SQL at all: round-trip fidelity
(section 4), search semantics (section 6), and the history hash chain
(section 12).

## What does not change

- **M14.1** `fhir-sqlite-gen` MUST NOT change. Its two dialect-looking
  constants are already correct for SQLite: `PG_MAX_IDENT = 63` is *tighter*
  than SQLite's identifier limit (effectively unbounded), and
  `SPLIT_WIDTH = 150` is far below `SQLITE_MAX_COLUMN` (2000 by default).
  Identifier fitting and table splitting therefore carry over unmodified.
- **M14.2** `shred.rs`, `reconstruct.rs`, `value.rs`, `fold.rs`, and `model.rs`
  MUST NOT change. They operate on Rust types and never emit SQL.
- **M14.3** `chain.rs` MUST NOT change except as required by M14.15.

## Accent and case folding

- **M14.4** The port MUST NOT emit a `_norm` SQL function. In the PostgreSQL
  original the function `fhir_postgresql_norm` is emitted into the schema but
  never called: folding is performed in Rust by `fold::fold` on the write path
  (the shredder fills the materialized `_norm` column), on the read path (the
  search term is folded before binding), and in the upgrade backfill. Every
  index is on a plain materialized column, not a function expression.
- **M14.5** Consequently the `unaccent` extension dependency — risk R7 in
  `plan.md`, and the `translate()` fallback it motivated — does not apply to
  this port at all. Folding is pure Rust and therefore identical across
  dialects by construction.

## The `ords` column

- **M14.6** `ords` MUST be a `TEXT` column, on the `Elem`, `Ext`, and `Deep`
  table kinds. It MUST NOT be modelled as a set of fixed ordinal columns, nor
  as a normalized side table. (`Contained` carries a scalar `ord smallint`,
  not `ords`; that becomes `INTEGER` and is unaffected by this section.)
- **M14.7** The stored image MUST remain the array literal produced by
  `fmt_ords` — `{1,2}`, `{}`, `{-1,3}` — so that `fmt_ords` and `parse_ords`
  are unchanged and a database can be compared value-for-value against a
  PostgreSQL instance.

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
  on read — so the whole change is: drop the type from three `CREATE TABLE`
  branches in `ddl.rs`, and drop the three casts and three projections in
  `store/src/lib.rs`.

- **M14.8** Three properties of the value domain MUST survive, and together
  they rule out the obvious alternatives:
  - **Negative ordinals are meaningful.** When two cyclic `contentReference`
    referrers share one table, the second pushes negated ordinals so paths stay
    unambiguous (`model.rs:211`; 3 such elements in R5, 2 in R4, 1 in R3). The
    domain is `-32767..=-1 ∪ 1..=32767`; `0` never occurs. Any unsigned or
    magnitude-only encoding is wrong.
  - **The empty array is valid and frequent.** Resource-level extensions and
    element ids shred with `ords = {}` into a `NOT NULL` primary-key column.
    Empty MUST remain storable and distinguishable from `NULL` and from every
    other value. `reconstruct.rs:106` uses the empty key as the base-row
    sentinel.
  - **Depth is unbounded for recursive types.** Non-recursive tables reach
    depth 6 at most in R5, but 23 R5 resource types (18 R4, 13 R3) own
    recursive tables — `Questionnaire.item.item`, `QuestionnaireResponse`,
    `ImplementationGuide`, `StructureMap`, and others — whose depth is
    data-dependent and uncapped. A fixed-width encoding covers ~99.9% of
    tables and still fails.

  A variable-length text image satisfies all three without special handling.
  Lossy encodings are at least loud rather than silent: reconstruction audits
  total row consumption and reports `"{} of {} stored rows unconsumed"`
  (`reconstruct.rs:118-124`).

- **M14.9** One user-facing behaviour regresses and MUST be documented rather
  than quietly dropped. `book/src/querying.md:29-30` teaches PostgreSQL array
  idioms to end users: `ords = '{1}'` addresses the first instance of a
  repeating element, and `ords[1] = 1` matches any descendant of the first
  instance. The equality form survives a `TEXT` column verbatim; the
  subscript form does not. This is the only array subscripting in the repo,
  and the book MUST be amended for this port — with `ords LIKE '{1,%'` offered
  as the nearest equivalent, noting that it is a prefix match on the text
  image rather than a typed subscript.

## Type mapping

- **M14.10** `col_sql` MUST map `ColTy` as follows. SQLite's dynamic typing
  means these are affinities, not constraints; the port MUST NOT rely on the
  database to reject a wrong type.

  | `ColTy` | PostgreSQL | SQLite |
  |---|---|---|
  | `Bool` | `boolean` | `INTEGER` (0/1) |
  | `Int` | `integer` | `INTEGER` |
  | `BigInt` | `bigint` | `INTEGER` |
  | `Numeric` | `numeric` | `TEXT` + `REAL` sort column — see M14.11 |
  | `Text` | `text` | `TEXT` |
  | `TextC` | `text COLLATE "C"` | `TEXT COLLATE BINARY` |
  | `Date` | `date` | `TEXT` — fixed-width ISO-8601, see M14.12 |
  | `Timestamptz` | `timestamptz` | `TEXT` — fixed-width ISO-8601 UTC, see M14.12 |
  | `Jsonb` | `jsonb` | `TEXT` — see M14.17 |

- **M14.11** FHIR `decimal` cannot be stored as `REAL`: M3.6 requires that a
  decimal's "original textual precision MUST survive round-trip", and binary
  floating point represents neither `1.50` distinctly from `1.5` nor `0.1` at
  all. PostgreSQL satisfies M3.6 with `numeric` alone because `numeric`
  preserves the scale as written; SQLite has no such type.

  The port MUST therefore follow the pattern M3.6 already establishes for
  dates: store the verbatim lexical form in a `TEXT` column and emit a
  derived, indexed `<name>_sort` column of `REAL` for ordering and range
  search. Retrieval reads the verbatim column; numeric and quantity range
  predicates (`gt`, `lt`, `ge`, `le`, and the implicit-precision `eq` range)
  read the sort column.

  *Note.* This is not a new mechanism — `date` already maps to
  `text` + a `date` sort column, and `dateTime`/`instant` to `text` + a
  `timestamptz` sort column. Extending it to `decimal` keeps one convention
  rather than introducing a second. It does mean `ColTy::Numeric` acquires a
  companion column in the generated map, which is the only place in this annex
  that touches the column model rather than a type name; the derived-column
  machinery in `add_norm_columns` is the obvious precedent to follow.
  **Open decision:** the `REAL` sort column makes range search subject to
  binary rounding at the 17th significant digit. For clinical quantities this
  is immaterial and FHIR search is precision-tolerant by design, but it is a
  documented inexactness the PostgreSQL original does not have.
- **M14.12** `Date` and `Timestamptz` sort columns MUST be normalized in Rust
  before binding and stored as fixed-width ISO-8601 (`YYYY-MM-DD` and
  `YYYY-MM-DDTHH:MM:SS.ffffffZ`, the latter in UTC), so that lexicographic
  comparison equals chronological comparison. Fidelity is unaffected: under
  M3.6 the verbatim value already lives in a separate `text` column, so these
  are ordering keys only. The PostgreSQL sentinel `'infinity'::timestamptz`
  MUST be replaced by `'9999-12-31T23:59:59.999999Z'`, and
  `+ interval '1 second'` by `datetime(<col>, '+1 second')` or Rust-side
  arithmetic.
- **M14.13** `bytea` MUST become `BLOB`; `bigserial` MUST become
  `INTEGER PRIMARY KEY AUTOINCREMENT`; `char(1)` MUST become `TEXT`.

## Canonical JSON and the hash chain

- **M14.14** This is the port's most consequential departure and MUST be
  settled before any store code is written.

  The history hash chain does not commit to the submitted resource text. It
  commits to *PostgreSQL's* `jsonb` canonical rendering:
  `store/src/lib.rs` computes `(($1::text)::jsonb)::text AS canon`, and
  `chain.rs:218` states the requirement — "`resource` MUST be the **stored**
  normalized form (`jsonb::text`), not the submitted text: `jsonb` reorders
  keys and rewrites number spellings". SQLite's `json()` does not reproduce
  that form: it preserves insertion order rather than sorting keys, and
  minifies differently.

- **M14.15** Canonicalization MUST therefore move out of SQL and into Rust,
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
  above pinned by unit tests (`crates/fhir-sqlite-map/src/canon.rs`). It is
  deliberately in the map crate rather than the store crate: it is
  dialect-neutral and depends on nothing but `serde_json`.

  *Consequence, and an argument for doing this upstream.* Once the canonical
  form is Rust-side, a history chain becomes portable: the same resource
  yields the same preimage in PostgreSQL, SQLite, and MySQL, so a database can
  be migrated between dialects and re-verified. Under the status quo it cannot
  be. This is a breaking change to existing PostgreSQL chains, but the
  machinery to absorb it already exists — `ALGORITHMS` is a versioned array,
  and `resign_history` / `chain-resign` exist to re-sign.

## Namespaces, atomicity, and concurrency

- **M14.16** Each FHIR version's schema MUST occupy its own SQLite database
  file, attached under the version name (`ATTACH '…/r5.sqlite' AS "r5"`). This
  preserves the `"{schema}"."{table}"` qualified-name shape used throughout
  the store layer almost verbatim.
- **M14.17** Schema installation MUST run in a single transaction. SQLite's
  DDL is transactional, so the staging-schema-plus-rename dance the PostgreSQL
  original needs (`DROP SCHEMA … CASCADE`, `ALTER SCHEMA … RENAME TO`, adopted
  because single-transaction DDL exhausts PostgreSQL's lock budget) MUST NOT be
  carried over. Neither MUST `max_locks_per_transaction` tuning.
- **M14.18** `pg_tables` and `information_schema.schemata` introspection MUST
  be replaced by `sqlite_schema`.
- **M14.19** `SELECT pg_advisory_xact_lock($1)` and `SELECT … FOR UPDATE`
  MUST be replaced by `BEGIN IMMEDIATE`, which acquires the write lock up
  front. SQLite admits one writer at a time, so the serialization those
  constructs bought for conditional create/delete is structural here.
- **M14.20** The database MUST run in WAL mode with a configured
  `busy_timeout`. `snapshot()`'s `REPEATABLE READ READ ONLY` MUST become a
  deferred read transaction, which in WAL mode observes a stable snapshot for
  its duration.
- **M14.21** Session GUCs have no SQLite equivalent. `SET LOCAL
  fhir_sqlite.erasure = 'on'` MUST be replaced by a one-row temporary table
  that the append-only trigger consults. `SET LOCAL search_path` and
  `plan_cache_mode = force_generic_plan` MUST be dropped; the EXPLAIN audit
  MUST be rewritten against `EXPLAIN QUERY PLAN`.
- **M14.22** The append-only history guard MUST be reimplemented as SQLite
  `BEFORE UPDATE` / `BEFORE DELETE` triggers using
  `SELECT RAISE(ABORT, '…')`, with the erasure escape conditioned on M14.21's
  flag table. plpgsql, `TG_OP`, and `current_setting` have no equivalents.

## Driver and concurrency model

- **M14.23** The port SHOULD use `rusqlite` behind `deadpool-sqlite`, which
  preserves the existing `deadpool` pooling shape. Decision D5's reasoning for
  rejecting `sqlx` — that the SQL is generated and dynamic, so compile-time
  checked queries buy nothing — applies unchanged.
- **M14.24** Placeholders MUST become `?`. The PostgreSQL original's
  positional `$n` and its "everything crosses the wire as text with explicit
  casts" convention MUST be replaced by native typed binding; the 65535-
  parameter chunking in `insert_shredded` MUST be retuned to
  `SQLITE_MAX_VARIABLE_NUMBER` (32766 on modern builds, 999 on older ones).
- **M14.25** `ON CONFLICT DO NOTHING` and `EXISTS (SELECT 1 …)` carry over
  unchanged. `LEFT JOIN LATERAL` MUST be rewritten as a correlated subquery.
  `DISTINCT ON` MUST be rewritten as a window function or `GROUP BY`.
  `unnest($1::text[])` MUST be rewritten as a values list or `json_each`.
  `ILIKE` MUST become `LIKE` (SQLite's `LIKE` is ASCII-case-insensitive by
  default) — but note this is why M14.4's Rust-side folding matters: correct
  non-ASCII case handling comes from the folded column, not from `LIKE`.
- **M14.26** Transport encryption has no meaning for a local file. The
  `SslPolicy` surface, `PGSSLMODE`, and the TLS-only CI job MUST be replaced
  by at-rest protections (file permissions, and optionally SQLCipher). The
  spec MUST NOT continue to claim transport guarantees it cannot make.

## Testing

- **M14.27** `*_TEST_DB` MUST become a path to a scratch database file, and
  the live suite MUST become unconditional: SQLite needs no server, so the
  ~2,400 lines that currently self-skip without a database SHOULD always run.
  This is a net gain in coverage over the PostgreSQL original.
- **M14.28** `concurrency.rs` MUST be rewritten rather than ported. It asserts
  multi-writer behaviour that SQLite does not offer; the correct assertions are
  that writers serialize and that `busy_timeout` is honoured.
- **M14.29** The `--test-threads=1` constraint (adopted because concurrent
  schema installs exhaust PostgreSQL's lock budget) MUST NOT be carried over;
  per-test database files parallelize freely.

## Upgrade

*Added when `upgrade` landed here, closing this port's quarter of **F-15**.*

- **M14.30** `init` MUST record the **map asset itself** in `fhir_sqlite_meta`,
  gzipped and hex-coded under `map_asset`, alongside `map_checksum` and
  `fhir_version`.

  This is what makes an upgrade possible at all: an upgrade diffs the installed
  map against the current one, and a checksum says only *that* something
  changed, never *what*. The encoding is hex rather than base64 to match
  `fhir-postgresql` byte for byte.

  An install predating this MUST be refused with a message distinguishing it
  from "not installed" — the remedies differ, `init` versus a reload.

- **M14.31** The whole upgrade MUST apply in **one transaction**. SQLite's DDL
  is transactional and its write lock is single-holder, so the chunking the
  PostgreSQL original needs to stay inside a lock budget has no purpose here and
  would be actively worse: an upgrade failing at chunk 7 of 20 leaves a schema
  that is neither the old one nor the new one.

- **M14.32** The history audit envelope MUST be **diffed against
  `pragma_table_info`**, not reconciled. SQLite has no
  `ADD COLUMN IF NOT EXISTS`, so applying those statements unconditionally —
  which is what PostgreSQL does, and what `ddl::history_audit_columns` warns
  against — fails with `duplicate column name` on the second run and on every
  fresh install, where `create_table` already emitted the envelope.

  The table-valued `pragma_table_info(?)` MUST be used rather than the `PRAGMA`
  statement form, which cannot be schema-qualified and would silently report on
  `main` — a different FHIR release's tables on a multi-version database.

- **M14.33** A failed `DROP COLUMN` MUST be reported with its cause. SQLite
  refuses to drop a column that is indexed, part of the primary key, or named in
  a trigger — all of which the generated schema uses — and reports every one as
  a bare `SQLITE_ERROR`. An operator who passed `--allow-destructive` and got
  "SQL logic error" cannot tell that from a bug in this port.

- **M14.34** The `_norm` backfill (`L13`, `L14`) MUST run as **part of** the
  upgrade, not as a step afterwards. An upgrade that adds a folded column
  without populating it leaves every existing row NULL, and since every
  non-`:exact` string search compares that column, those resources silently stop
  matching their own values.

  It MUST fold distinct *values* rather than rows, in bounded batches, one
  transaction per batch, selecting only rows still NULL — which is what makes it
  resumable. Batching matters more here than on PostgreSQL: a SQLite writer
  holds the single write lock for the length of its transaction.
