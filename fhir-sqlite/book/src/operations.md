# Operations

This chapter used to describe a `fhir-sqlite serve` process — health checks,
`/metrics`, request timeouts, load shedding, TLS flags. None of that exists
in this crate. There is no server here (`C0.17`, `C0.18`); if you are
deploying behind [`fhir-loco`](../../../fhir-loco/), its own operations
documentation covers that surface. This chapter covers what a `SqliteStore`
itself does, because that is what you are actually operating when you embed
this library.

## What `open` sets, and why it matters operationally

`SqliteStore::open` sets four pragmas on every connection, and each is a
behavioural guarantee, not a performance tweak (see
[Getting started](getting-started.md)):

| Pragma | Value | What it changes operationally |
| --- | --- | --- |
| `foreign_keys` | `ON` | A resource rewrite deletes its base row and relies on `ON DELETE CASCADE` to clear children. Off, a rewrite orphans rows silently, and reconstruction later reports them as an unconsumed-row integrity error rather than failing where the mistake was made. |
| `journal_mode` | `WAL` | Readers do not block the writer and vice versa — this is what makes a live backup (below) and a snapshot read (`R4.5`) possible at all. |
| `busy_timeout` | `30000` ms | A second writer blocks and waits, rather than failing immediately with `SQLITE_BUSY`. Relevant because SQLite admits exactly one writer. |
| `synchronous` | `FULL` | Every commit is durable against a power loss, not just an OS crash. This store holds a hash chain over history; a torn write is not a trade this library makes for you. |

## One writer, structurally

SQLite admits a single writer. `put`, `delete`, `purge`, `init`, and
`upgrade` each open `BEGIN IMMEDIATE`, which takes the write lock up front —
this is the port's replacement for PostgreSQL's advisory locks and `SELECT …
FOR UPDATE` (`M14.19`), and it is why `conditional_create_audited` and
`conditional_delete_audited` hold an in-process `write_gate` mutex across
their search-then-write: on a single `SqliteStore` handle, nothing else can
observe or race the gap between the search and the write.

That guarantee is **per-process**, not automatic across two handles on the
same file: `busy_timeout` is what keeps two separate `SqliteStore`s (two
processes, or two connections in one process) from erroring outright, not
what serializes them logically — the second simply waits for the first
`BEGIN IMMEDIATE` to release. Concurrent-writer safety is exercised directly
in `tests/concurrency.rs`, which deliberately opens a **second** connection
on the same file rather than sharing one handle, because two tasks sharing
one `SqliteStore` are already serialized by an in-process mutex and could
never demonstrate a race either way (`T11.10`).

## Install and upgrade

There is no `fhir-sqlite init` or `fhir-sqlite init --upgrade` command
(`C0.17`). The equivalents are `SqliteStore::init` and `SqliteStore::upgrade`:

```rust,ignore
store.init("clinic-2026-08").await?;                       // first install
// … later, after regenerating the map …
let report = store.upgrade("clinic-2026-09", false).await?; // allow_destructive = false
println!("{} additive, {} destructive, {} values folded",
    report.additive, report.destructive, report.folded);
```

`init` installs the whole generated schema in **one transaction**, records
the checksum you pass, and — since this port closed its quarter of audit
finding **F-15** — also records the **map asset itself**, gzipped and
hex-encoded, in `fhir_sqlite_meta` (`M14.30`). That stored copy is what makes
`upgrade` possible at all: it diffs the *installed* map against the map you
open the store with, rather than only knowing that something changed.

`upgrade` differs from the PostgreSQL original in three ways forced by the
engine, not chosen for parity:

1. **The whole upgrade is one transaction** (`M14.31`). SQLite's DDL is
   transactional and there is one write lock to budget, so there is no
   PostgreSQL-style chunking — an upgrade either lands complete or not at
   all, never half of twenty chunks.
2. **The audit envelope is diffed, not reapplied unconditionally.** SQLite
   has no `ADD COLUMN IF NOT EXISTS`; the statements are filtered against
   `pragma_table_info` first, or a second `upgrade` on an already-upgraded
   database fails with `duplicate column name` (`M14.32`).
3. **A refused `DROP COLUMN` names its cause.** SQLite refuses to drop a
   column that is indexed, part of the primary key, or named in a trigger —
   all of which this schema uses — and reports every one as a bare
   `SQLITE_ERROR`. This port rewrites that message to say which column and
   why, because "SQL logic error" tells an operator who passed
   `allow_destructive = true` nothing they can act on.

Destructive changes (dropped tables or columns) are refused unless
`allow_destructive` is `true`; a column **type** change always refuses —
that needs a manual migration regardless (`O10.4a`, `L12`).

**The folded-search-column backfill is part of `upgrade`, not a step after
it.** An upgrade that added a `_norm` column without populating it would
leave every existing row `NULL`, and every non-`:exact` string search
compares that column — so unbackfilled rows would silently stop matching
their own values, with no error anywhere (`M14.34`). `backfill_norm` also
exists standalone, for backfilling without a schema change, batched
(1,000 distinct values per transaction) so an interrupted run resumes rather
than reapplies from nothing:

```rust,ignore
let folded = store.backfill_norm().await?;
```

**An install predating this port's `map_asset` recording cannot be
upgraded.** `upgrade` refuses it with a message naming that specifically —
"installed schema predates upgrade support" — distinct from "not installed",
because the remedies differ: a fresh `init` versus a full reload. There is no
way to infer the old map from the installed schema alone; guessing wrong
there corrupts data rather than merely failing loudly.

To remove a version's schema entirely:

```rust,ignore
store.drop_schema().await?;
```

Because the whole schema is one file, `drop_schema` detaches it and deletes
that file — a single unlink, not `DROP SCHEMA … CASCADE` — so it cannot
half-succeed the way a multi-statement drop can.

## `transact_audited` refuses, on purpose

`transact_audited` (applying a FHIR® transaction Bundle) returns
`StoreError::Unsupported` unconditionally — it is not a stub someone forgot,
it is a decision. A FHIR transaction Bundle is atomic by definition: a caller
submitting one is saying these writes only make sense together. The tempting
shortcut is applying each operation through the ordinary write path and
undoing the earlier ones if a later one fails — a compensating unwind, not
atomicity — and it is weaker in two ways that matter for a system holding
PHI: a reader between operations can observe a half-applied bundle, and a
process that dies mid-unwind leaves the partial state permanently. Shipping
that under the name `transact` would claim a guarantee the code does not
provide, in the one subsystem whose entire purpose is being trustworthy.

Doing it properly needs one `BEGIN IMMEDIATE` held across every operation in
the bundle, which needs `put` and `delete` split so their bodies can run
inside a transaction the caller already opened rather than one they start
themselves. That refactor is tracked (`tasks.md`, T64), not forgotten — and
until it lands, refusing is the honest answer.

## Backup

A `fhir-sqlite` store is one file per FHIR version (plus the main file
`open` was given). Back it up the way you would back up any SQLite database —
this crate implements no backup mechanism of its own:

- **Simplest:** copy the file(s) while the database is closed.
- **Live, consistent:** SQLite's own [online backup
  API](https://www.sqlite.org/backup.html) or `VACUUM INTO 'backup.sqlite'`,
  either of which produces a consistent snapshot while writers continue,
  because `journal_mode=WAL` means readers are never blocked by a writer.

There is no `pg_dump`, no physical replication, and no write-ahead-log
point-in-time recovery here — those are PostgreSQL features this book
claimed for every port until 2026-08-03 (audit **F-56**). The PHI is at rest
in a small number of plain files; what protects it is filesystem permissions
and disk encryption, which is the deployment's responsibility and the
`O10.7` obligation an embedded engine has in place of transport encryption
(see [The trust boundary](trust-boundary.md)).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
