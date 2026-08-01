# 14. PostgreSQL dialect

**Status: proposed.** A draft for review, not yet ratified (`X15.9`), so it MUST
NOT be cited as evidence for a conformance level until it is.

This annex records where the PostgreSQL port departs from the
[monorepo core](../../spec/index.md), and — as importantly — where it does not.
Requirements are numbered `M14.x` and use RFC 2119 keywords.

## Why this annex exists at all

`fhir-postgresql` is the original. Sections 1–13 were written against PostgreSQL
and *were* the specification, so for most of this project's life there was
nothing for an annex to differ from — which is why it is the only port that had
none (`audit.md` **F-14**).

Consolidating the core removed that exemption rather than granting it. The core
is now stated in abstract column types and engine-neutral mechanisms, so
PostgreSQL's `jsonb`, its `smallint[]`, its `COLLATE "C"`, and its staged-schema
install are bindings like any other port's, and they need writing down for the
same reason: a reader must be able to tell a decision from an inheritance.

Two of the entries below are genuine departures rather than bindings —
`M14.13` (`jsonb`, which violates `M3.6c`) and `M14.14` (the TLS default). Both
were invisible while this port defined the spec.

## What does not change

- **M14.1** The pure-Rust core — `model.rs`, `shred.rs`, `reconstruct.rs`,
  `value.rs`, `fold.rs`, `error.rs`, and all of `gen/` — MUST NOT differ from
  the other ports (`X15.1`). It operates on Rust types and never emits SQL.
- **M14.2** `PG_MAX_IDENT = 63` and `SPLIT_WIDTH = 150` are this engine's real
  limits and are simultaneously the monorepo's shared budget (`X15.3`,
  `G2.4`, `G2.6`). PostgreSQL is where those numbers come from: 63 is its
  identifier limit and 150 is far below its 1600-column limit. Every other port
  inherits them because they are the tightest, so a generated name is legal
  everywhere.

## Engine floor and namespaces

- **M14.3** The engine floor is **PostgreSQL 18** (`S1.4`). Older servers are
  unsupported.
- **M14.4** Each FHIR version installs into its own **schema** — `r5`, `r4`,
  `r3` (`S1.2`) — created by `CREATE SCHEMA`. A database MAY host any subset.

## Type mapping

- **M14.5** `col_sql` binds `ColTy` (`M3.6`) as follows.

  | `ColTy` | PostgreSQL |
  |---|---|
  | `Bool` | `boolean` |
  | `Int` | `integer` |
  | `BigInt` | `bigint` |
  | `Numeric` | `numeric` |
  | `Text` | `text` |
  | `TextC` | `text COLLATE "C"` |
  | `Date` | `date` |
  | `Timestamptz` | `timestamptz` |
  | `Jsonb` | `jsonb` — **departs from `M3.6c`, see `M14.13`** |

- **M14.6** `Numeric` binds to `numeric`, and this port is the **only** one that
  can (`M3.6a`). PostgreSQL's `numeric` is arbitrary-precision *and* preserves
  declared scale per value, so `1.50` reads back as `1.50`. Every other engine's
  decimal type carries a fixed declared scale and would return
  `1.500000000000000000000000000000`, which is why they bind `Numeric` to text.

  This is not a free win. Where `numeric` cannot preserve the lexical form —
  trailing zeros beyond its scale — the shredder records the original text in
  the primitive-extension channel, because round-trip fidelity (`R4.2`) is the
  invariant and the column is only the fast path.

- **M14.7** `TextC` binds to `text COLLATE "C"`, which is binary and does not
  pad, satisfying `M3.6b` directly. The other ports' `utf8mb4_0900_bin`,
  `utf8mb4_nopad_bin`, `Latin1_General_100_BIN2`, and `COLLATE BINARY` are each
  that engine's spelling of this property.

- **M14.8** `Date` and `Timestamptz` bind to native `date` and `timestamptz` for
  the **derived `_sort` columns only**. The stored value remains `text`
  (`M3.6`), because FHIR partial dates are not representable natively without
  inventing precision.

## The `ords` column

- **M14.9** `ords` binds to `smallint[]`, and this is the **only** port with a
  native array type. `M3.4a`'s three value-domain properties hold natively:
  negative ordinals are ordinary `smallint`s, the empty array is a valid
  distinct value, and array length is unbounded.
- **M14.10** The stored image MUST nonetheless remain interchangeable with the
  other ports' text image (`M3.4b`, `X15.5`). It already is: values cross the
  wire as text with explicit casts in both directions — `($n::text)::smallint[]`
  on insert, `SELECT "ords"::text` on read — so `fmt_ords` and `parse_ords` are
  shared unmodified and a PostgreSQL database compares value-for-value against
  any other port's.
- **M14.11** PostgreSQL-only query idioms MUST be documented as such. `ords[1] =
  1` — subscripting to match any descendant of the first instance — works here
  and on no other port, where `ords` is `TEXT`. The book teaches it; it MUST say
  that `ords LIKE '{1,%'` is the portable form (`M14.9` in the SQLite annex
  records the other side of this).

## Canonical JSON and the hash chain

- **M14.12** The canonical form the hash chain commits to MUST be computed in
  Rust, by one function shared between the writer and the verifier (`X15.2`).

  *Amended when this was resolved; it read "Departure, unresolved" until then.*

  This port derived it in SQL — `(($1::text)::jsonb)::text`, passed to
  `chain::preimage` — and was consequently the only map crate without
  `canon.rs`. The 0.4.0 release had moved the *digest* out of the database for
  the reason `M3.16b` gives; the *pre-image* had not moved, and the argument
  applies to it just as well. The bytes signed were whatever `jsonb` produced
  when it reordered keys and rewrote number spellings — a form defined by a
  PostgreSQL version rather than by this specification — so `X15.11` failed: a
  chain written here could not be verified anywhere else.

  `canon.rs` is now ported in, byte-identical to the other five, and both the
  writer and `verify_audit` derive the pre-image through it. Writer and verifier
  MUST canonicalize the **same stored bytes**, so that they agree by
  construction rather than by both calling the same SQL cast.

  This was a **chain format change**. Rows written under the old form do not
  verify under the new one, and the pre-image carries no version marker to
  dispatch on. The project elected to treat itself as having no installed base
  rather than carry a dual-format verifier or add a per-row marker; **the
  migration for an existing database is a reload**, and its old rows report as
  breaks until then. Tracked as [`audit.md`](../../spec/audit.md) **F-07**.

- **M14.13** **Departure.** `M3.6c` requires `Jsonb` bind to a type that does
  not re-normalize JSON. `jsonb` re-normalizes by definition — that is what it
  is for. The binding is retained rather than changed because history rows and
  contained resources are read back through the same engine that wrote them, so
  within this port the round-trip is stable; but it is the mechanism behind
  `M14.12`, and fixing `M14.12` in Rust is what makes the column type stop
  mattering.

  A future port of this binding to `text` would be a data migration, not a code
  change, and MUST be planned as one.

  *Since `M14.12` was resolved, this departure no longer touches the chain.* The
  pre-image is canonicalized in Rust from the stored bytes, so how `jsonb`
  chooses to render them is no longer something a digest depends on. What
  remains is the narrower `M3.6c` point: `jsonb` can still alter a value on the
  way **in** — `1e2` is stored as `100` — so what this port signs is the value
  as stored, not necessarily the value as submitted. That is a real difference
  from the ports binding `text`, and it is why this stays a recorded departure
  rather than being closed with `M14.12`.

## Install, atomicity, and concurrency

- **M14.14** `init` MUST stage the install under a temporary schema
  (`<schema>__init`) in chunked transactions and rename it into place in a
  single statement (`G2.5`).

  PostgreSQL has transactional DDL, so the obvious implementation is one
  transaction — and it does not work: creating 7,355 tables in one transaction
  exhausts `max_locks_per_transaction`, which is a server-wide setting a tenant
  often cannot raise. The staged schema gives the same observable atomicity by a
  different route. A failed init leaves only the staging schema, which the next
  init removes. Schema drops are likewise chunked.

- **M14.15** Multi-statement reads MUST run in one
  `REPEATABLE READ READ ONLY` transaction (`R4.5`). `READ COMMITTED` — the
  server default — re-reads on every statement and is exactly the torn-resource
  failure the requirement describes.
- **M14.16** `version_id` assignment and the chain append MUST be serialized per
  resource id by `SELECT "version_id" … FOR UPDATE` on the base row, taken
  before the history append (`H5.4`). This is also what preserves the property
  that moving the digest computation out of SQL would otherwise have cost: the
  read of the previous digest cannot race the insert.
- **M14.17** The hashed timestamp MUST be the database's own `now()`, read in
  the same transaction and written back verbatim, so the value hashed is the
  value stored.
- **M14.18** Every connection MUST carry a `statement_timeout`, set through
  `options` at connect time. An unbounded statement is an unbounded hold on both
  the snapshot of `M14.15` and the row lock of `M14.16`.

## Append-only history

- **M14.19** `init` MUST emit `fhir_postgresql_history_is_append_only()` and a
  `BEFORE UPDATE OR DELETE … FOR EACH ROW` trigger on every history table,
  raising an exception naming `M3.17`. Escaping it requires
  `ALTER TABLE … DISABLE TRIGGER`, a deliberate DBA act that leaves its own
  trace.

## Search

- **M14.20** No SQL folding function is used. Folding happens in Rust
  (`P6.6`, `L1`): the shredder fills the materialized `_norm` column on write,
  the search term is folded before binding, and every index is on a plain
  materialized column rather than a function expression.
- **M14.21** `ddl.rs` MUST NOT define or emit a SQL folding function (`L3`).

  *Amended when this was fixed.* It previously carried `NORM_FN` and
  `norm_function`, producing `fhir_postgresql_norm(text)` — a residue of the
  pre-`P6.6` design that **nothing called**: not the store, not any index, not
  any generated predicate. Both are now deleted, and a unit test asserts
  `schema_wide_objects` emits no folding function (**F-18**).

  The earlier wording of this requirement said the function was emitted "into
  the schema". It was not — `schema_wide_objects` never included it, so no
  database ever had one. It was dead `pub` API, reachable only by a caller who
  went looking. The prohibition stands either way: a folding function reachable
  from the schema invites a query written against it, which would introduce the
  second definition of string equality `L1` exists to prevent.

  (The SQLite annex's `M14.4` already observed this about the PostgreSQL
  original. It is recorded here as this port's own item rather than left as a
  remark in a sibling's file.)

- **M14.22** The `unaccent` extension is **not** required, and no extension is.
  An earlier design depended on it and needed an `IMMUTABLE` wrapper, an
  expression index the planner would not use against a bound parameter, and a
  deployment-time check for an extension managed-PostgreSQL tenants often cannot
  install. `pgcrypto` is likewise not required, because nothing hashes in SQL
  (`M3.16b`).
- **M14.23** Prefix search MUST be emitted as a range predicate over the `_norm`
  column, never `LIKE $1 || '%'` (`P6.6a`). PostgreSQL extracts a prefix from a
  *constant* pattern only, so the `LIKE` form degrades to a sequential scan in
  the generic plan while looking correct in any hand-run `EXPLAIN` with a
  literal.

## Driver and transport

- **M14.24** The driver is `tokio-postgres` with a `deadpool` pool, not `sqlx`
  (plan decision D5): the SQL here is generated and dynamic, so compile-time
  checked queries buy nothing, while `tokio-postgres` gives pipelining and
  binary-format parameters.
- **M14.25** Values cross the wire as **text with explicit casts**
  (`($n::text)::numeric`, `($n::text)::smallint[]`), in both directions. This is
  what keeps the engine's lexical-fidelity guarantees — decimal scale, partial
  dates — intact end to end.
- **M14.26** TLS is `rustls`, configured from `PGSSLMODE` or the DSN, with
  `PGSSLROOTCERT` for the trust anchor (`O10.7`).

- **M14.27** **Departure.** `O10.7` requires that a port default to verifying
  the server certificate. `SslPolicy` has three effective modes and defaults to
  `Prefer`, which does not verify:

  | `sslmode` input | Effective mode | Verifies certificate? |
  |---|---|---|
  | `disable` | `Disable` | — |
  | `prefer`, `allow` | `Prefer` **(default)** | no |
  | `require`, `verify-ca`, `verify-full` | `Require` | yes, certificate **and** hostname |

  Two things about that table are deliberate and one is not.

  Deliberate: `require` here is **stricter than libpq's**, which encrypts
  without validating anything and therefore does not survive an active attacker.
  Collapsing `verify-ca` into full verification is also deliberate — stricter
  than asked, and the safe direction to err.

  Not deliberate: the **default**. `Prefer` matches libpq, which is why it was
  chosen, but libpq's default is a compatibility decision for a general-purpose
  client and this is a component that carries PHI on every connection. A
  deployment that sets nothing gets an unverified connection and no warning.

  Until the default changes, deployments MUST set `PGSSLMODE=verify-full`
  explicitly, and the documentation MUST say so rather than describing
  `verify-full` as "the production setting" as though it were in force.

## Backup

- **M14.28** Backup and restore are PostgreSQL's own — `pg_dump` and PITR
  (`O10.6`). A consistent snapshot is always a valid store; the documentation
  MUST state that invariant and MUST name PostgreSQL's mechanism rather than
  another engine's.

---

Part of the [fhir-postgresql specification](index.md), which is part of the
[fhir-databases specification](../../spec/index.md).
