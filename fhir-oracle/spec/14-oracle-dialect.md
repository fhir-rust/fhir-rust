# 14. Oracle dialect

**Status: proposed, and mostly undecided.** A draft for review, not ratified
(`X15.9`). It MUST NOT be cited as evidence for a conformance level.

This annex records where the Oracle port departs from the
[monorepo core](../../spec/index.md). Requirements are numbered `M14.x` and use
RFC 2119 keywords.

> **This file was rewritten.** It previously contained the `fhir-mysql` annex
> with three lines changed — titled "14. MySQL dialect", declaring the target as
> "MySQL 8.0 or later, InnoDB, `utf8mb4`", carrying a section headed
> "Relationship to fhir-mariadb", and containing the word "Oracle" only in the
> three substituted crate names
> ([`audit.md`](../../spec/audit.md) **F-16**).
>
> **Requirement numbering restarts here.** The `M14.x` ids in the previous file
> were MySQL's requirements wearing this port's name. `C0.5` makes ids
> permanent, so those numbers are **withdrawn, not reused**: no `M14.x` below
> means what the same number meant in the copied file. Any citation of an
> `M14.x` in `fhir-oracle` predating this rewrite is void, and should be traced
> to the MySQL annex it actually came from.

> ## ⚠ Nothing in this port is Oracle yet
>
> This annex is therefore mostly a **decision list**, not a specification of
> behaviour. Most entries below say what must be decided and why the obvious
> answer is wrong; a handful state decisions that can be made now.
>
> Writing it that way is the requirement, not a shortcut. `X15.6` treats silence
> as a defect, because silence and having-not-considered-it are identical on the
> page — and the previous file's confident MySQL answers were considerably worse
> than an honest blank.
>
> - **`ddl.rs` is the MySQL emitter** (**F-08**). It emits `TEXT`,
>   `TINYINT(1)`, `DATETIME(6)`, `LONGTEXT`, and `COLLATE utf8mb4_0900_bin`,
>   none of which exist in Oracle, and its comments still discuss MySQL's 2038
>   `TIMESTAMP` range. Its eleven MySQL-asserting tests are `#[ignore]`d with
>   that reason attached, so a green suite cannot be mistaken for Oracle
>   conformance.
> - **There is no store**, and no driver in the workspace.
> - **There are no map tests** — `crates/fhir-oracle-map/tests/` does not exist.
>
> The port's `ddl.rs` module header is honest about all of this and is the
> source for most of what follows. Conformance level: **Scaffold** (`C0.8`).

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
  ([`audit.md`](../../spec/audit.md) **F-09**). Declaring 12.2 makes the
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

  The prefix option interacts badly with `M14.2` and is likely wrong, but the
  privilege requirement of the first is a real deployment constraint. Undecided.

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

  **Settled.** [Unbounded string search](../../spec/unbounded-string-search-must-have-bounded-adjunct-and-checksum-adjunct.md) (`U1`–`U10`, `P6.9`) is now
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

## The `ords` column — to decide

- **M14.13** `ords` MUST hold the shared text image (`M3.4b`, `X15.5`), as on
  every other port. The type is undecided between `VARCHAR2(n)` and `RAW(n)`;
  `VARBINARY` — the SQL Server answer — does not exist in Oracle.
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

## Undecided, and required before any store

- **M14.18** Install atomicity at scale (`G2.5`) — see `M14.15`.
- **M14.19** Snapshot isolation (`R4.5`). Oracle's multiversion read consistency
  is strong and `SET TRANSACTION READ ONLY` is the likely answer, but it MUST be
  named rather than assumed.
- **M14.20** Write serialization for `version_id` and the chain append (`H5.4`)
  — presumably `SELECT … FOR UPDATE`, unverified.
- **M14.21** Paging. `OFFSET … FETCH` exists from 12c, so it is available at the
  `M14.2` floor. Placeholders are `:1`, `:2`, ….
- **M14.22** Transport security (`O10.7`) — Oracle Net encryption and/or TLS,
  and which is the documented production setting.
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

- **M14.24** This port MUST NOT provision a substitute engine in CI
  (`O10.12`, `C0.10`). Until this revision it provisioned `mysql:8.4` and
  invoked `--test mysql_ddl`, a target that does not exist in this package —
  the job failed on `error: no test target named mysql_ddl`, and no Oracle was
  involved anywhere (**F-06**).

  Its live-database gate has now been **removed** rather than repointed. There
  is nothing to point it at: no Oracle DDL, no driver, no store, and no map
  tests. A pipeline that starts *some* database for a port that cannot talk to
  one is theatre, and a removed gate is at least honestly absent.

- **M14.25** The eleven `#[ignore]`d MySQL-asserting tests in `ddl.rs` MUST stay
  ignored and MUST stay tracked in `tasks.md` (`T11.14`). They are the record of
  what has to be replaced.

---

Part of the [fhir-oracle specification](index.md), which is part of the
[fhir-databases specification](../../spec/index.md).
