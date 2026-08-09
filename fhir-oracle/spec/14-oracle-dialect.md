# 14. Oracle dialect

**Status: proposed; the type mapping, namespace, and store decisions below are
now live-verified, transport security and install atomicity are not.** A
draft for review, not ratified (`X15.9`). It MUST NOT be cited as evidence
for a conformance level — the [conformance matrix](../../spec/databases/conformance-matrix.md)
is that document.

This annex records where the Oracle port departs from the
[monorepo core](../../spec/databases/index.md). Requirements are numbered `M14.x` and use
RFC 2119 keywords.

> **This file was rewritten.** It previously contained the `fhir-mysql` annex
> with three lines changed — titled "14. MySQL dialect", declaring the target as
> "MySQL 8.0 or later, InnoDB, `utf8mb4`", carrying a section headed
> "Relationship to fhir-mariadb", and containing the word "Oracle" only in the
> three substituted crate names
> ([`audit.md`](../../spec/databases/audit.md) **F-16**).
>
> **Requirement numbering restarts here.** The `M14.x` ids in the previous file
> were MySQL's requirements wearing this port's name. `C0.5` makes ids
> permanent, so those numbers are **withdrawn, not reused**: no `M14.x` below
> means what the same number meant in the copied file. Any citation of an
> `M14.x` in `fhir-oracle` predating this rewrite is void, and should be traced
> to the MySQL annex it actually came from.

> ## This port now has a live-verified store
>
> This annex is still mostly a **decision list**, not a specification of
> settled behaviour — most entries below say what had to be decided and why
> the obvious answer was wrong. But as of 2026-08-04 the decisions in it have
> been executed by a real store against a real database, not just read.
>
> Writing it that way is the requirement, not a shortcut. `X15.6` treats silence
> as a defect, because silence and having-not-considered-it are identical on the
> page — and the previous file's confident MySQL answers were considerably worse
> than an honest blank.
>
> - **`ddl.rs` is an Oracle emitter, and has been executed** (**F-08** closed,
>   2026-08-03). The full R5 schema — 158 resources, 9,636 statements — installs
>   on Oracle AI Database 26ai Free with 0 invalid objects and 0 unindexable
>   search targets. Its eleven `#[ignore]`d MySQL-asserting tests have been
>   replaced with Oracle ones (`M14.25` discharged); the crate has 47 tests and
>   0 ignored.
>
>   Three defects were found by *running* it and could not have been found by
>   reading: `ORA-02438` on an inline `Bool` CHECK (`M14.23e`), the append-only
>   DELETE guard **failing open** on Oracle's empty-string-is-NULL rule
>   (`M14.29a`), and 453 unindexable reference targets that turned out to be a
>   shared-core defect (**F-50**).
> - **There is a store, and it has connected to a database** (**F-68**,
>   superseding **F-66**'s "compiles but never connected"). Oracle Instant
>   Client for macOS arm64 turned out to be a direct, no-login download;
>   installed, `crates/fhir-oracle-store` connected to a live
>   `gvenzl/oracle-free:23-slim-faststart` and its `init`/`put`/`get`/
>   `delete`/`history`/`vread`/`verify_audit`/`purge`/`log_access`/`search`
>   surface was run against it in `tests/oracle_store.rs`: **7 of 7 tests
>   pass, 0 ignored.** Getting there found and fixed five real defects — see
>   `M14.5`, `M14.19`, `M14.34`, and `audit.md` **F-68** for the account.
>   `upgrade`/`backfill_norm` followed on 2026-08-09 (**F-15**'s last port,
>   **F-47** step 1): `tests/upgrade.rs`, 9 more live tests, and three new
>   requirements, `M14.35`–`M14.37` below. Not done: `R4.5` has no working
>   mechanism (`M14.19`, regressed from "presumed" to "confirmed absent"),
>   no `concurrency.rs` verifies `H5.4` under contention, and no
>   `redaction.rs`.
> - **There are no map tests** — `crates/fhir-oracle-map/tests/` does not exist.
>   The unit tests in `ddl.rs` are what exists.
>
> Conformance level: **Store** (`C0.8`), not Reference — see the [conformance
> matrix](../../spec/databases/conformance-matrix.md) for the row-by-row claim.

## What does not change

- **M14.1** The pure-Rust core — `model.rs`, `shred.rs`, `reconstruct.rs`,
  `value.rs`, `fold.rs`, `canon.rs`, `error.rs`, and all of `gen/` — MUST NOT
  differ from the other ports (`X15.1`). It operates on Rust types and never
  emits SQL, which is why this port's round-trip engine is already correct even
  though its DDL is not.

## Engine floor — decided

- **M14.2** The engine floor is **Oracle Database 12.2** (`S1.4`).

  This is the one item that could not be left open, because the generator has
  already assumed an answer. Oracle identifiers were **30 bytes before 12.2**
  and 128 after, so the shared 63-byte budget (`G2.4`, `X15.3`) is legal on
  12.2+ and silently truncating below it — collapsing distinct table names into
  one, which is precisely the collision `G2.4` exists to make impossible.

  The port had inherited the constant without inheriting a reason
  ([`audit.md`](../../spec/databases/audit.md) **F-09**). Declaring 12.2 makes the
  inherited budget sound.

- **M14.3** `init` MUST verify the server version and refuse below the floor.
  A silent truncation of generated identifiers is not a failure a deployment
  would notice until two resource types shared a table.

- **M14.4** Requiring **23ai** instead would buy a native `BOOLEAN` (`M14.8`)
  and nothing else this port needs. That trade is **not** taken: 23ai is far
  newer than the estate this port exists to serve, and `NUMBER(1)` with a
  `CHECK` is an adequate substitute.

## Namespaces — to decide

- **M14.5** Oracle conflates *user* and *schema*, so `S1.2`'s three independent
  namespaces (`r5`, `r4`, `r3`) have no single obvious mapping. Two candidates,
  and the annex MUST choose one before any DDL is written:

  | Option | Cost |
  |---|---|
  | Three users, one per version | Three sets of grants; cross-version queries need explicit qualification; `init` needs `CREATE USER` privilege |
  | One user, name-prefixed tables | Spends prefix bytes from the 63-byte budget (`M14.2`) on every identifier, which is the budget that is already tightest here |

  **Decided: three users, one per version.** The prefix option spends bytes from
  the identifier budget on *every* identifier, and `M14.2` already calls that
  budget the tightest constraint this port has — 63 characters against a 128-byte
  limit, chosen to stay safe at the 12.2 floor. Spending 3 of them on `r5_` to
  avoid a `CREATE USER` grant trades a permanent, schema-wide cost for a
  one-time deployment one.

  The cost is accepted and MUST be documented rather than discovered: `init`
  needs `CREATE USER` privilege, cross-version queries need explicit
  qualification, and three sets of grants have to be managed. A deployment that
  cannot grant `CREATE USER` cannot install this port, and that is a real
  limitation to state in the README before anyone meets it at 2am.

  **Live-verified, 2026-08-04, with a correction this requirement did not
  originally state: the user MUST be created unquoted, and the schema name
  bound in `RelMap` MUST be uppercase.** Oracle folds an *unquoted*
  `CREATE USER r5 ...` to uppercase (`R5`) for both authentication and
  session identity (`SELECT USER FROM DUAL`). A first attempt created the
  user quoted (`CREATE USER "r5" IDENTIFIED BY ...`) to preserve the
  lowercase spelling this requirement's examples use — login as `"r5"`
  succeeded, but the session's *resolved* identity was still uppercase
  `"R5"`, and every DDL/DML statement qualified as `"r5".*` then failed
  `ORA-01031: insufficient privileges` against a session that was really
  `"R5"`. The working configuration is the unquoted, naturally-uppercase
  user with a matching uppercase `RelMap.schema` — the opposite convention
  from `r5`/`r4`/`r3` on every other port, and now what `scripts/db.sh` and
  `tests/oracle_store.rs` both use (**F-68**).

## Type mapping — to decide

- **M14.6** `col_sql` MUST be rewritten. The current bindings are MySQL's and
  are not merely suboptimal — `TEXT`, `TINYINT`, `DATETIME`, and
  `utf8mb4_0900_bin` do not exist in Oracle, so the emitted DDL does not parse.

  The intended shape, none of it yet implemented or verified:

  | `ColTy` | Intended Oracle binding | Open question |
  |---|---|---|
  | `Bool` | `NUMBER(1)` + `CHECK (c IN (0,1))` | or require 23ai — resolved against by `M14.4` |
  | `Int` | `NUMBER(10)` | |
  | `BigInt` | `NUMBER(19)` | |
  | `Numeric` | `VARCHAR2` | see `M14.7` |
  | `Text` | `VARCHAR2(4000 CHAR)` / `CLOB` | see `M14.9` — the hard one |
  | `TextC` | `VARCHAR2(n CHAR)` + binary collation | see `M14.10` |
  | `Date` | `DATE` | Oracle's `DATE` carries a time component |
  | `Timestamptz` | `TIMESTAMP(6)` | with or without time zone — see `M14.11` |
  | `Jsonb` | `CLOB` | never the `JSON` type — see `M14.12` |

- **M14.7** `Numeric` MUST NOT be `NUMBER` (`M3.6a`). Oracle's `NUMBER` is
  decimal and high-precision, which makes it tempting, and it still normalizes:
  `1.50` is stored and returned as `1.5`. `M3.6` requires the original textual
  precision survive round-trip, so `Numeric` binds to a character type here as
  it does on every engine but PostgreSQL, with range search served by a derived
  sort column.

- **M14.8** `Bool` binds to `NUMBER(1)` with a `CHECK` constraint. Oracle had no
  boolean column type before 23ai, and per `M14.4` this port does not require
  23ai.

- **M14.9** **The `VARCHAR2`/`CLOB` boundary is this port's hardest problem. It
  gated `ddl.rs` and is now settled** — see the end of this requirement.

  `VARCHAR2` maxes at **4000 bytes** — 32767 with extended types, which is a
  database-level setting a generated schema cannot assume. Anything longer must
  be a `CLOB`, and **a `CLOB` cannot be indexed, cannot be compared with `=`,
  and cannot participate in a key** the way character data can.

  This is a sharper version of the constraint the SQL Server port hit with
  `NVARCHAR(MAX)` (`M14.16` there), and sharper in the direction that matters:
  SQL Server's `NVARCHAR(MAX)` still compares with `=`, so those searches are
  correct and merely scan. An Oracle `CLOB` does not, so the same design would
  make some searches *not work at all* rather than work slowly.

  A FHIR `string` has no length bound in the specification, so this cannot be
  resolved by declaring one.

  **Settled.** [Unbounded string search](../../spec/databases/unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) (`U1`–`U10`, `P6.9`) is now
  normative: a text column this engine cannot index or compare as bound gets a
  **bounded adjunct** (`<col>_idx`) and a **checksum adjunct** (`<col>_h`) in
  the generated map. Both, not either — a bounded adjunct cannot answer equality
  and a checksum cannot answer a prefix.

  It is still a **map** change and not a DDL one, so it lands in `model.rs` and
  `gen/src`, which are shared verbatim across all six ports (`X15.1`). This port
  MUST materialize both adjuncts (`U9`) and MUST record the bound *n* here
  (`U10`) before claiming `P6.4a`.

  Oracle is the reason both adjuncts are required rather than one: a `CLOB`
  answers neither `=` nor an index, where SQL Server's `NVARCHAR(MAX)` at least
  answers `=`. `U6`'s confirming comparison therefore has to use
  `DBMS_LOB.COMPARE` on this engine.

- **M14.10** `TextC` MUST provide a binary, non-padding comparison (`M3.6b`).
  Oracle's collation story differs from every other target here, and `CHAR`
  semantics are blank-padded, which is exactly the PAD SPACE behaviour `M3.6b`
  forbids: under it `'Smith' = 'Smith '`, which widens `:exact` and weakens key
  identity. `VARCHAR2` does not pad. The specific collation and `NLS_SORT`
  settings are undecided.

- **M14.11** `Timestamptz` binds to `TIMESTAMP(6)`. Whether `WITH TIME ZONE` is
  used is undecided; every value is normalized to UTC in Rust before binding, so
  the SQL Server port's argument (`M14.10` there) applies — an offset column
  would store a zero offset and invite the belief that local times are
  preserved.

- **M14.12** `Jsonb` MUST NOT be Oracle's `JSON` type (`M3.6c`). It
  re-normalizes, so the bytes read back would differ from the bytes the hash
  chain signed and every chain would fail verification. `CLOB` is the intended
  binding.

## The `ords` column — decided

- **M14.13** `ords` MUST hold the shared text image (`M3.4b`, `X15.5`), as on
  every other port. **Decided and executed: `RAW(255)`** — `ddl.rs` emits it
  directly (`"ords" RAW(255) NOT NULL`); `VARBINARY`, the SQL Server answer,
  does not exist in Oracle, and `RAW` is Oracle's binary type, compared and
  indexed like any fixed column, never treated as a LOB.
- **M14.14** Whichever is chosen, `M3.4a`'s three value-domain properties MUST
  survive: negative ordinals verbatim, `{}` storable and distinct from null, and
  unbounded depth.

  Oracle's treatment of the empty string deserves specific attention here, and
  is the reason this is called out rather than assumed: Oracle traditionally
  treats `''` as `NULL`, and `ords` is a `NOT NULL` primary-key column in which
  the empty *path* is frequent. The stored image of the empty path is `{}` —
  two characters, not the empty string — so the collision does not arise, but a
  future "optimization" that stored the empty path as `''` would break the
  primary key on this engine and on no other.

## Idempotence and triggers — to decide

- **M14.15** Oracle has **no `IF NOT EXISTS`** on anything. Idempotence MUST be
  a PL/SQL wrapper swallowing ORA-00955 (`name is already used`):

  ```sql
  BEGIN EXECUTE IMMEDIATE '…';
  EXCEPTION WHEN OTHERS THEN IF SQLCODE != -955 THEN RAISE; END IF; END;
  ```

  which makes **every** DDL statement a PL/SQL block. That has a consequence
  worth stating: `G2.5`'s "effectively atomic" install cannot lean on
  statement-level idempotence the way the other ports do, and how it is achieved
  here is undecided (`M14.18`).

- **M14.16** Append-only triggers (`M3.17`) MUST raise
  `RAISE_APPLICATION_ERROR(-20001, …)` naming `M3.17`.

- **M14.17** The erasure escape (`M3.18`) has **no session-variable
  equivalent**. PostgreSQL uses `SET LOCAL`, SQL Server uses `SESSION_CONTEXT`,
  MySQL uses a session variable; Oracle's analogue is `SYS_CONTEXT` with an
  **application context**, which must be created up front as its own database
  object with its own trusted package. That is a heavier dependency than any
  other port's and MUST be specified, not improvised.

## Store requirements

- **M14.18** Install atomicity at scale (`G2.5`) — see `M14.15`. Still
  undecided; the live schema install (`M14.23a`) was a single hand-run
  script, not tested for partial-failure behavior.
- **M14.19** Snapshot isolation (`R4.5`). **Tried live, 2026-08-04, and
  rejected: `SET TRANSACTION READ ONLY` does not work on this engine for
  this port's shape of session.** The candidate this requirement named as
  "the likely answer" fails every read with `ORA-01466: unable to read data
  - table definition has changed` on any session that has ever executed
  DDL — reproduced independently with a minimal 3-statement probe (`CREATE
  TABLE` + commit, then on the *same session* `SET TRANSACTION READ ONLY` +
  `SELECT`), with no application logic involved, confirming this is a
  genuine Oracle session-level behavior and not a bug in how the store
  called it. Every session that runs `init` (which is `CREATE TABLE`-heavy)
  is poisoned for this technique for its lifetime.

  The call was removed from `get` rather than shipped broken (see `oracle.rs`
  module doc, `audit.md` **F-68**). **`R4.5` is therefore an open,
  unresolved requirement on this port** — worse than merely undecided, since
  the one candidate mechanism named here is now known not to work. A
  dedicated Oracle session per read-transaction that has never run DDL, a
  separate read-only connection pool, or `DBMS_FLASHBACK`/flashback query
  (`AS OF SCN`/`AS OF TIMESTAMP`) are the remaining candidates and are
  **untried**.
- **M14.20** Write serialization for `version_id` and the chain append
  (`H5.4`). **Implemented, 2026-08-04: `SELECT … FOR UPDATE`** on the base
  row, held until commit/rollback, mirroring the pattern in `put`/`delete`
  (`oracle.rs`). This is exercised by every live test in
  `tests/oracle_store.rs` in the sense that each test's single-writer
  sequence passes, but **no test races concurrent writers** the way
  `fhir-mssql`'s and `fhir-mysql`'s `concurrency.rs` do — the mechanism is
  present and plausible, not contention-verified.
- **M14.21** Paging. `OFFSET … FETCH` exists from 12c, so it is available at the
  `M14.2` floor. Placeholders are `:1`, `:2`, …. Implemented in
  `oracle_search.rs::search_page`; not yet exercised by a test that requests
  a second page with a cursor.
- **M14.22** Transport security (`O10.7`) — Oracle Net encryption and/or TLS,
  and which is the documented production setting. Still undecided; the live
  container this pass connects to over a plain local Docker/Podman port with
  no encryption configured either way.
- **M14.23** **A driver has not been chosen.** This was blocked on a practical
  question that also decides whether live verification can be promised at all:
  **whether an Oracle Database Free image runs on arm64.**

  *Partly answered, 2026-08-01, and the answer has a caveat worth keeping.*

  An arm64 image **exists and pulls**: both
  `container-registry.oracle.com/database/free` and `docker.io/gvenzl/oracle-free`
  publish `linux/arm64` in their manifests, and the latter pulls as a genuine
  arm64 image (2.34 GB) under Podman on Apple silicon. So the architecture is
  not the obstacle it was assumed to be.

  It did **not start** on the default Podman machine (2 GiB RAM), failing with
  `ORA-03113: end-of-file on communication channel` after allocating a 1.6 GB
  SGA, and again with `INIT_SGA_SIZE=768`. Oracle Free wants more memory than a
  default developer VM gives it. Running it therefore requires resizing the
  machine — `podman machine set --memory 6144` or similar — which is a host
  change this project MUST document rather than assume.

  What follows for `O10.12`: a `scripts/db.sh` comparable to the others' is
  possible, and MUST state the memory requirement and fail with that message
  rather than with `ORA-03113`, which names the wrong problem. Until an Oracle
  has actually been started and a schema installed against it, this port MUST
  NOT claim live verification.

## Testing

- **M14.23a** **Measured, 2026-08-02: the image pulls, and the instance dies.**
  `M14.23` recorded that an arm64 image exists and pulls. It is now cached
  locally (`docker.io/gvenzl/oracle-free`, 2.34 GB) and was actually started, so
  the remaining question — whether it *runs* here — has an answer rather than an
  assumption.

  It does not, at the memory available:

  ```text
  CONTAINER: WARNING: The container has less than 2 GB memory available to run
  Oracle Database Free.
  ORA-03113: end-of-file on communication channel
  ```

  Exit code 41. The container gets far enough to start the listener — "Listening
  Endpoints Summary … The command completed successfully" — and then the
  instance itself dies. The host is a `podman machine` with **1.9 GiB** total,
  which the OS shares with the database.

  That is a useful failure to have observed rather than predicted, because it
  distinguishes two things the earlier note could not: the image is not the
  problem, and the architecture is not the problem. **Memory is**, and it is a
  host setting rather than anything in this repository.

  **Superseded within the hour: an Oracle has now run.** Raising the host VM to
  6 GiB was the whole fix. The database reaches `DATABASE IS READY TO USE!` and
  answers SQL:

  ```text
  Oracle AI Database 26ai Free Release 23.26.2.0.0
  ```

- **M14.23b** **Four assumptions this annex rested on are now measured, not
  inferred.** Every one was previously taken from documentation.

  | Claim | Where it was asserted | Measured on 26ai Free |
  | --- | --- | --- |
  | `VARCHAR2` maxes at 4000 bytes unless extended types are on | `M14.9` | **confirmed** — `max_string_size = STANDARD`, and `varchar2(4000 char)` creates |
  | Identifiers are 128 bytes on 12.2+ | `M14.9`, `ddl.rs` header | **confirmed** — a 128-byte name creates; 129 gives `ORA-00972: ... exceeds the maximum length of 128 bytes`. `MAX_IDENT = 63` is therefore safe and conservative |
  | A `CLOB` cannot be `=` compared | the [unbounded string search](../../spec/databases/unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) premise table | **confirmed** — `ORA-22848: cannot use CLOB type as comparison key` |
  | A `CLOB` cannot be indexed | same | **confirmed** — `ORA-02327: cannot create index on expression with data type LOB` |

  The last two matter beyond this port: they are the premise of `U1`–`U10`, the
  reason the checksum adjunct exists at all, and the sharpest cell in that
  section's engine table. That premise is no longer a citation.

- **M14.23c** **`BOOLEAN` exists on 26ai — which confirms `M14.4`'s decision
  rather than reopening it.**

  Measured: `create table t (b boolean)` succeeds and
  `user_tab_columns.data_type` reads `BOOLEAN`.

  *An earlier revision of this requirement read that finding backwards.* It said
  the port "must now decide its engine floor", and attributed that job to
  `M14.5`. Both were wrong, and the annex already said so: **`M14.2` declares
  the floor as Oracle 12.2**, `M14.5` is about namespaces, and **`M14.4`
  explicitly considered requiring 23ai for a native `BOOLEAN` and rejected it**
  — 23ai buys "nothing else this port needs".

  A measurement on 26ai cannot change what a 12.2 floor permits. `BOOLEAN`
  arrived in 23ai, so a schema targeting 12.2 must not emit it however new the
  server in front of you happens to be. The finding confirms `M14.6`'s
  `NUMBER(1)` + `CHECK (c IN (0,1))` is *necessary*, not that it is avoidable.

  That substitute is itself now verified rather than assumed: `NUMBER(1) CHECK
  (c IN (0,1))` accepts `0` and `1` and rejects `2` with `ORA-02290: check
  constraint violated`. `M14.6`'s `Bool` row has been executed.

  Until an Oracle has run **this port's own DDL**, it MUST NOT claim any level
  above Scaffold, however Oracle-shaped that DDL looks (`C0.9`: a level is
  justified by tests that ran). A started engine is a prerequisite, not the
  evidence.

- **M14.23d** **`M14.6`'s type table is now executed, and one row of it is
  misleading as written.**

  All nine bindings create on Oracle 26ai, in one table, alongside the `U1`
  adjuncts and `ords`:

  | `ColTy` | Emitted | Oracle reports |
  | --- | --- | --- |
  | `Bool` | `NUMBER(1) CHECK (c IN (0,1))` | `NUMBER` — accepts 0/1, rejects 2 with `ORA-02290` |
  | `Int` / `BigInt` | `NUMBER(10)` / `NUMBER(19)` | `NUMBER` |
  | `Numeric` | `VARCHAR2(64 CHAR)` | `VARCHAR2(256)` |
  | `Text` | `VARCHAR2(4000 CHAR)` | `VARCHAR2(4000)` — **see below** |
  | `TextC` / `<col>_idx` | `VARCHAR2(450 CHAR)` | `VARCHAR2(1800)` |
  | `Date` | `DATE` | `DATE` |
  | `Timestamptz` | `TIMESTAMP(6)` | `TIMESTAMP(6)` |
  | `Jsonb` | `CLOB` | `CLOB` |
  | `<col>_h` | `RAW(32)` | `RAW(32)` |

  Note the second column against the third: `VARCHAR2(n CHAR)` allocates
  **n × 4 bytes** under `AL32UTF8`, which is why 64 becomes 256 and 450 becomes
  1800.

  **`VARCHAR2(4000 CHAR)` does not hold 4000 characters. It holds 4000 bytes.**
  `NLS_CHARACTERSET` is `AL32UTF8` and the `STANDARD` ceiling is a *byte*
  ceiling, so the declaration is silently capped: as few as **1000** characters
  for 4-byte codepoints. Verified — a genuine 3000-character value built in
  PL/SQL is refused:

  ```text
  ORA-12899: value too large for column "…"."A" (actual: 6000, maximum: 4000)
  ```

  `M14.9` states the byte ceiling correctly, but `M14.6`'s intended binding
  `VARCHAR2(4000 CHAR)` reads as though 4000 FHIR characters fit. They do not.
  The emitter MUST reckon in bytes: the safe character bound for arbitrary
  Unicode is **1000**, and everything above it is `CLOB` — which makes the
  `U1`–`U10` adjuncts load-bearing for far more columns than a 4000-character
  reading would suggest.

  *Method note, because it nearly produced the opposite finding.* Inserting
  `rpad(unistr('\00e9'), 3000, …)` appeared to store 2000 characters silently,
  which reads as data loss and would violate `R4.2`. It is not: `rpad` returns a
  SQL `VARCHAR2`, itself capped at 4000 bytes, so the truncation happened in the
  expression and never reached the column. Building the value in PL/SQL — where
  `VARCHAR2` reaches 32767 — showed the column rejecting it properly. The column
  is well-behaved; the probe was not.

- **M14.23e** **`M14.8`'s CHECK cannot live where `M14.6` implies it does.**

  `M14.8` says `Bool` binds to `NUMBER(1)` with a `CHECK` constraint, and
  `M14.6`'s table writes that as one cell — which reads as though `col_sql` can
  return it. It cannot. `col_sql` is handed a `ColTy` and nothing else, and a
  `CHECK` must name the column it constrains:

  ```text
  ORA-02438: Column check constraint cannot reference other columns
  ```

  Both correct placements were verified — a table-level
  `CONSTRAINT … CHECK (deceased IN (0,1))` and a column-level
  `CHECK (deceased IN (0,1))` naming its own column — and both reject `2` with
  `ORA-02290` while accepting `0` and `1`.

  So the constraint belongs to `create_table`, the only emitter that knows the
  column name. `col_sql` returns the bare type.

  **Until `create_table` emits it, a `Bool` column on this port is an
  unconstrained `NUMBER(1)` and this port MUST NOT claim `M14.8`.** An
  unconstrained `NUMBER(1)` accepts `7`, and a boolean column that accepts `7`
  is worse than one that is documented as absent.

  This is what executing DDL buys over reading it: ten of the eleven bindings
  were right, and the one that was wrong was wrong in a way no amount of
  re-reading the annex would have surfaced.

- **M14.23f** **The fixed-shape columns have no type decision, and the obvious
  one does not work.** `M14.6`'s table maps `ColTy` values. The `Ext`, `Deep`,
  and system tables also carry columns that are *not* `ColTy`-driven — `path`,
  `leaf`, `url`, `v_text`, `v_kind`, `ords`, `key_hash` — and the annex never
  says what they bind to on Oracle.

  Translating the MySQL `TEXT` for them naively gives `CLOB`, and the table
  creates. Then search stops working:

  ```text
  select count(*) from pext where path = 'Patient.name';
  ORA-22848: cannot use CLOB type as comparison key
  ```

  `path` is the column extension search filters on, so a `CLOB` binding makes
  the extension tables writable and unsearchable — the failure mode `M14.9`
  describes, arriving through a column `M14.9` does not cover.

  Each needs deciding on its own evidence, and they do not all go the same way:

  | Column | Bounded in practice? | Likely binding |
  |---|---|---|
  | `path` | yes — a FHIR element path | `VARCHAR2`, so it can be compared and indexed |
  | `v_kind` | yes — one character | `VARCHAR2(1 CHAR)`, never `CHAR` (`M3.6b`: no PAD SPACE) |
  | `ords` | yes — the text image (`M14.13`) | `RAW(255)`, decided and executed |
  | `key_hash` | yes — 32 bytes | `RAW(32)` |
  | `url`, `leaf`, `v_text` | **no** | `CLOB` plus `U1` adjuncts wherever search touches them |

  The last row is the one that matters: it means the adjunct rules are
  load-bearing for the extension tables too, not only for `ColTy::Text`.

  **Answered by the core, 2026-08-02.** `U1a` restates the trigger as *(a search
  reaches this column, this dialect cannot index or compare it as bound)* rather
  than as a property of the FHIR or SQL type — which is exactly why these
  columns were missed. `U11` requires the generator to walk every
  search-reachable column, naming the extension and deep tables specifically.
  `U12` says a fixed-shape column that is **bounded in practice** — `path`,
  `v_kind` — SHOULD be bound to an indexable type instead of given adjuncts,
  which is the cheaper answer here. `U13` forbids a bounded adjunct over opaque
  bytes.

  So this port's bindings follow: `path` and `v_kind` to `VARCHAR2` under `U12`;
  `url`, `leaf` and `v_text` to `CLOB` with adjuncts under `U11`; `ords` to
  `RAW(255)` (`M14.13`, decided and executed). What remained open was the
  generator work `U11` now requires, which is shared-core and lands in all
  six ports at once.

- **M14.23e** **`M14.8`'s CHECK cannot live where `M14.6` implies it does.**

  `M14.8` says `Bool` binds to `NUMBER(1)` with a `CHECK` constraint, and
  `M14.6`'s table writes that as one cell — which reads as though `col_sql` can
  return it. It cannot. `col_sql` is handed a `ColTy` and nothing else, and a
  `CHECK` must name the column it constrains:

  ```text
  ORA-02438: Column check constraint cannot reference other columns
  ```

  Both correct placements were verified — a table-level
  `CONSTRAINT … CHECK (deceased IN (0,1))` and a column-level
  `CHECK (deceased IN (0,1))` naming its own column — and both reject `2` with
  `ORA-02290` while accepting `0` and `1`.

  So the constraint belongs to `create_table`, the only emitter that knows the
  column name. `col_sql` returns the bare type.

  **Until `create_table` emits it, a `Bool` column on this port is an
  unconstrained `NUMBER(1)` and this port MUST NOT claim `M14.8`.** An
  unconstrained `NUMBER(1)` accepts `7`, and a boolean column that accepts `7`
  is worse than one that is documented as absent.

  This is what executing DDL buys over reading it: ten of the eleven bindings
  were right, and the one that was wrong was wrong in a way no amount of
  re-reading the annex would have surfaced.

- **M14.24** This port MUST NOT provision a substitute engine in CI
  (`O10.12`, `C0.10`). Until this revision it provisioned `mysql:8.4` and
  invoked `--test mysql_ddl`, a target that does not exist in this package —
  the job failed on `error: no test target named mysql_ddl`, and no Oracle was
  involved anywhere (**F-06**).

  Its live-database gate has now been **removed** rather than repointed. There
  is nothing to point it at: no Oracle DDL, no driver, no store, and no map
  tests. A pipeline that starts *some* database for a port that cannot talk to
  one is theatre, and a removed gate is at least honestly absent.

- **M14.26** **`U10` record: which columns get adjuncts, and what the bound
  is.** Every column a `string` search parameter targets, and only those, via
  `add_adjunct_columns` gated on `ddl::TEXT_ADJUNCTS`, which is `true` here.
  The bound *n* is **450 characters**, matching `col_sql`'s `VARCHAR2(450
  CHAR)` for `ColTy::TextIdx` and `shred.rs`'s `ADJUNCT_BOUND`. Declared in
  `CHAR` rather than byte semantics so the bound means characters on any
  `NLS_LENGTH_SEMANTICS` setting.

  The checksum adjunct is **`RAW(32)`** — SHA-256's raw bytes, per `U4a`. It
  is not hex text; an earlier revision of this annex said it was, and the SQL
  Server annex records the reversal at `M14.33`.

  `RAW` rather than `BLOB` matters here: a `BLOB` would reintroduce exactly the
  problem the checksum exists to solve, since Oracle will not `=` compare a LOB.
  `RAW(32)` compares and indexes normally, so the equality probe `U6` describes
  is an ordinary index seek, and only the confirming comparison against the
  source `CLOB` needs `DBMS_LOB.COMPARE`.

  This port needs both adjuncts more than its sibling does. `NVARCHAR(MAX)` on
  SQL Server at least answers `=`, so only the index is missing there; a `CLOB`
  answers no comparison at all, so without `<col>_h` an equality search here
  **fails** rather than scans. `U6`'s confirming comparison must then use
  `DBMS_LOB.COMPARE`, which is the one place the confirmation is not a plain
  `=`.

- **M14.27** ~~The adjunct columns are the only Oracle-correct types in
  `ddl.rs`.~~ **Superseded 2026-08-02**: the whole emitter is now Oracle, and
  **F-08** is closed. Retained because ids are permanent (`C0.5`) and because
  the reasoning it recorded — write the ported parts correctly rather than leave
  a placeholder that looks finished — is what made the rest of the port a
  substitution rather than a rewrite.

- **M14.28** `ddl_in` MUST NOT emit `CREATE USER`. `M14.5` binds a version
  namespace to an Oracle user, but provisioning those three accounts is a
  deployment act, not a schema detail, and before 18c a `CREATE USER` must
  carry a password — which would then appear in generated DDL, in logs, and in
  `V$SQL`. `NO AUTHENTICATION` would avoid that and is above this port's 12.2
  floor (`M14.2`).

  The three users are therefore a **documented prerequisite**. Every statement
  qualifies its object with the schema name and assumes the account exists with
  a quota. A deployment that cannot create them cannot install this port, and
  the README MUST say so before anyone meets it at 2am.

  - **M14.28a** `fhir_oracle_meta.key` is `VARCHAR2(191 CHAR)`. 191 is a MySQL
    number — it is what fits a `utf8mb4` index there — and has no Oracle
    meaning. It stands anyway, because changing it would put the two ports'
    metadata tables out of step for no benefit.

- **M14.29** The erasure declaration that permits a history `DELETE` (`M3.18`)
  travels in `CLIENT_INFO`, read by the trigger as
  `SYS_CONTEXT('USERENV', 'CLIENT_INFO')` and set by
  `DBMS_APPLICATION_INFO.SET_CLIENT_INFO`. This amends `M3.18`'s assumption of
  a session variable, which Oracle does not have.

  An application context is the better design and is rejected on deployability:
  it needs `CREATE ANY CONTEXT`, and a port that cannot install without a DBA is
  a port that gets installed as `SYSTEM`.

  The cost MUST be stated wherever this is used: `CLIENT_INFO` is a
  general-purpose field that monitoring tools and connection pools also write. A
  store MUST set it immediately before the delete and clear it immediately
  after, within the same transaction, and MUST NOT rely on a pool to have
  cleared it. This is a weaker guarantee than the MySQL original's.

  - **M14.29a** The comparison MUST use a non-empty sentinel:
    `NVL(SYS_CONTEXT(…), 'unset') != 'fhir_oracle_erasure=on'`. Oracle treats
    the empty string as NULL, so the direct translation of MySQL's
    `COALESCE(@var, '')` yields `NULL != 'x'` → NULL, the `ELSIF` never fires,
    and **the guard fails open**. This was written, installed, and observed
    letting an ordinary `DELETE` through with no error. Any change to this
    trigger MUST be re-verified by executing a forbidden `DELETE`, not by
    reading it.

- **M14.30** The `rid` lookup index on `Ext`/`Deep` MUST be a separate
  `CREATE INDEX`. Oracle has no inline index clause inside `CREATE TABLE`, where
  MySQL uses `KEY`. `create_table` therefore returns a table and nothing else,
  and `ddl_in` emits the index.

- **M14.31** Objects that must survive re-application — `schema_wide_objects`
  and the metadata table — MUST be wrapped in a PL/SQL block that swallows
  **only** `ORA-00955` and `ORA-01408`. `IF NOT EXISTS` arrived in 23ai, above
  the floor. An `EXCEPTION WHEN OTHERS THEN NULL` MUST NOT be used: it converts
  a genuine syntax error into a silent success, which is the exact failure this
  annex exists to prevent. Verified both ways — re-running is silent, and an
  invalid datatype still raises `ORA-00902`.

- **M14.32** Where an index would name a column Oracle cannot index, the
  emitter MUST substitute the `U1` adjunct — the bounded `<col>_idx` where one
  exists, otherwise the digest `<col>_h`, which serves equality exactly as
  `U2a` intends — and MUST emit **no index at all** when neither exists, rather
  than a partial key that would answer a different question.

  Every such omission MUST be enumerable: `search_index_gaps` returns them, so
  the limit is countable rather than inferred from an absent index. It is what
  exposed **F-50**, and it currently returns **0** for R5.

- **M14.25** The eleven `#[ignore]`d MySQL-asserting tests in `ddl.rs` MUST stay
  ignored and MUST stay tracked in `tasks.md` (`T11.14`). They are the record of
  what has to be replaced.

- **M14.34** Three store-level binding rules, each found live by
  `fhir-oracle-store` connecting to a real Oracle for the first time
  (**F-68**), not decidable by reading the driver or the annex alone:

  1. `insert_row` (and any future helper that builds an `INSERT`/`UPDATE`)
     MUST take one already-schema-qualified target identifier, never a bare
     table name plus a separate schema argument. Passing both produced
     `"R5"."R5"."patient_history"` and `ORA-00926: Missing VALUES or SET
     keyword` — Oracle does not reject a doubly-qualified identifier the way
     a syntax error would suggest; it reads as a malformed statement instead,
     which is a slower error to trace back to its cause.
  2. `Timestamptz` and `Date` values MUST bind through a typed
     (`chrono::NaiveDateTime`/`NaiveDate`) parameter, never a plain string
     left to Oracle's implicit conversion. Implicit conversion reads the
     session's `NLS_TIMESTAMP_FORMAT`, not ISO 8601, and rejects an
     ISO 8601 string with `ORA-01843: An invalid month was specified` unless
     the session happens to be configured to expect it — which a generated
     schema cannot assume. `M14.11` names the column type; this requirement
     is the bind-side companion it did not yet state.
  3. A `Bool` (`NUMBER(1)`) value used as a search predicate MUST bind as
     `0`/`1`, never as the string `"true"`/`"false"`. Oracle refuses the
     implicit string-to-number conversion SQL Server and MySQL both allow:
     `ORA-01722: unable to convert string value containing 't' to a number`.
     `oracle_search.rs`'s `target_pred` MUST resolve the target column's
     `ColTy` before choosing a bind kind for a `Token` target — see that
     file's module doc.

## Upgrade and backfill — decided (2026-08-09)

`OracleStore::upgrade` and `backfill_norm` exist and are live-verified —
`tests/upgrade.rs`, 9 tests against `gvenzl/oracle-free:23-slim-faststart`,
closing this port's share of **F-15** (the last port) and **F-47** step 1.
Three requirements are this engine's own:

- **M14.35** **The upgrade is resumable, not transactional.** Oracle has no
  transactional DDL — every DDL statement implicitly commits — so a failed
  upgrade leaves everything before the failure applied, and that cannot be
  prevented. What is required instead: every statement the upgrade applies
  MUST tolerate having already run, so that the recovery for a partial
  upgrade is simply running `upgrade` again. Oracle's `CREATE TABLE`,
  `CREATE INDEX`, and `ALTER TABLE … ADD` have no `IF NOT EXISTS`, so each
  statement is wrapped in a PL/SQL block that swallows exactly the
  already-applied codes — `ORA-00955` (name in use), `ORA-01430` (column
  exists), `ORA-01408` (column list already indexed), and for destructive
  statements `ORA-00942`/`ORA-00904` (already gone) — and re-raises
  everything else. `a_second_upgrade_is_a_no_op` verifies the property from
  the outside. This is the third answer to the same problem: the SQL Server
  annex's `M14.35` is one transaction, the MySQL annex's `M14.35` is
  reported-partial; this port is partial-but-rerunnable.

- **M14.36** **A meta value longer than the string bind limit MUST be
  chunked.** The map asset `upgrade` diffs against is ~1 MB of hex, and
  binding it as a single string parameter fails with `ORA-01461: can bind a
  LONG value only for insert into a LONG column` — even though the target
  column is a `CLOB`, the *bind* is what overflows. Values past the limit
  are stored as `<key>.<i>` rows of at most 3,000 characters, with the base
  key holding a `chunks:N` sentinel; reads reassemble, and a rewrite deletes
  stale chunk rows first. Values that fit bind directly. Found live: `init`
  hit this the moment it first tried to store the asset.

- **M14.37** **The backfill pages by ROWID keyset, not by value.** The
  source column of a fold is a `CLOB`, and a `CLOB` can be neither
  `DISTINCT`ed nor `=`-compared (`ORA-00932`/`ORA-22848`), so the
  values-based loop the other five ports share cannot run here.
  `backfill_norm` instead selects `WHERE dst IS NULL AND src IS NOT NULL`
  ordered by `ROWID` in bounded batches, updates each row by its `ROWID`,
  and commits per batch — resumable, verified by
  `the_backfill_is_resumable`. A fold whose result is empty MUST be skipped
  rather than written: `''` is NULL on this engine (`M14.29a`'s root
  cause), so writing it would leave `dst` NULL and the row eligible on
  every pass, forever.

## The `path` binding — decided (2026-08-09, corrected 2026-08-10)

- **M14.38** **`path` binds to `VARCHAR2(path_bound CHAR)`; the
  conversion cannot be in-place.** Decided 2026-08-09 (`U12a`, **F-47**
  step 2), corrected 2026-08-10; the code lands with F-47 steps 3 and 5,
  and until it does the current `CLOB` stands and this port does not claim
  `U12` for `path`.

  (The correction: this requirement first said `v_kind` was also a `CLOB`
  needing `VARCHAR2(1 CHAR)` — repeating F-47's table instead of reading
  the emitted DDL. `ddl.rs` has emitted `v_kind` as `CHAR(1 CHAR)` since
  F-08's rebuild: one character, indexable, and `M3.6b`'s PAD SPACE
  hazard does not bite a length-one column compared against length-one
  values. `v_kind` needs no conversion and is out of this migration.)

  `path` is `CLOB` today — the binding `M14.23f` already argued against,
  since a `CLOB` compares with nothing (`ORA-22848`) and `path` is a
  structural locator the store filters by exact value. The decided type
  reads the map's recorded bound (`U12a`) rather than a constant —
  `VARCHAR2(path_bound CHAR)`, `VARCHAR2` rather than `CHAR` because the
  values vary in length, which is where padding *would* bite (`M3.6b`).

  Oracle cannot `ALTER TABLE … MODIFY` a `CLOB` into a `VARCHAR2`. The
  conversion is add-column, copy, drop, rename — four statements, each
  implicitly committing, because there is no transactional DDL (`M14.35`).
  So the half-applied story is stated here before the code exists, as the
  migration schedule requires: every step MUST tolerate having already run
  or already been superseded, the same rule as `M14.35`, so that the
  recovery for an upgrade interrupted anywhere in the sequence is running
  `upgrade` again; and the copy MUST fail loudly on any row whose `path`
  exceeds the bound rather than truncate — a truncated `path`
  reconstructs the wrong resource shape (`L4`, via `U12a`).

---

Part of the [fhir-oracle specification](index.md), which is part of the
[fhir-databases specification](../../spec/databases/index.md).
