# Operations

fhir-mariadb is a **library**. There is no server process to deploy, no port
to bind, no health endpoint, and no CLI (`C0.17`, `C0.18`) — an earlier
version of this chapter described `fhir-mariadb serve`, `/health`,
`/metrics`, and a table of `--flag` options that belong to none of the six
database ports (audit **F-56**). If a deployment wants an HTTP surface, that
is [`fhir-loco`](../../../fhir-loco/), a separate crate; everything below is
what `MariaDbStore` itself does, called from your own process.

## Install

```rust,ignore
let store = MariaDbStore::connect(dsn, map).await?;
let applied = store.init("r5-baseline").await?; // number of DDL statements applied
```

`init` applies the generated DDL directly against the target database and
then records the map checksum, the FHIR version, and the map asset itself
(gzipped, hex-coded) in `fhir_mariadb_meta`. **This is not atomic.** MariaDB's
DDL commits implicitly, so the staged-schema-then-rename dance the
PostgreSQL original uses has no MariaDB equivalent (`M14.22`) — this chapter
used to claim one anyway. If a statement fails partway through, `init`
returns an error naming how many statements had already been applied and
that they remain; there is no rollback. An install that already matches the
recorded checksum is a no-op.

## Upgrade

```rust,ignore
let report = store.upgrade("r5-current-checksum", /* allow_destructive */ false).await?;
```

`upgrade` diffs the installed map (read back from `fhir_mariadb_meta`)
against the map the store was constructed with: new tables, columns, and
indexes apply automatically; destructive changes (dropped tables or columns)
are refused unless `allow_destructive` is `true`; column type changes always
refuse outright and require a manual migration (`O10.4a`, `L12`). An install
that predates `M14.33` — recorded before the map asset itself was stored —
is refused with a message distinguishing it from "not installed": the
remedy is a reload via `init`, not an upgrade.

Two consequences of `M14.22` apply here too, and are this port's own, not
shared with the other five:

- **No transaction covers the upgrade.** A failure partway through leaves a
  schema that is neither the old shape nor the new one, and the error
  reports the count of statements already applied so an operator knows what
  state it is in (`M14.35`).
- **The reconcile step is not naively idempotent.** MariaDB has no
  `CREATE INDEX IF NOT EXISTS` as this port emits it, so re-running the
  access-log index list wholesale fails with `Duplicate key name` on a
  second pass (audit **F-28**). `upgrade` filters that list — and the
  history audit-envelope columns — against `information_schema` first, and
  does so **after** the additive statements run, not before, because a table
  the upgrade just created already carries the new columns (`M14.36`).

`upgrade` also **backfills** any newly folded search column before it
returns, and the report's `folded` count says how many distinct values it
processed. This is not optional tidying: non-`:exact` string search compares
the folded column, so an upgrade that left it `NULL` on existing rows would
silently return fewer results, with no error at all (`M14.37`). The backfill
selects only rows still `NULL`, in bounded batches, so it is resumable if
interrupted — call `upgrade` again and it continues where it stopped.

## Drop

```rust,ignore
store.drop_schema().await?;
```

Drops the database (`DROP DATABASE`, MariaDB's rendering of a "schema" —
`M14.21`) and everything in it. There is no confirmation flag; the caller's
own code is the confirmation.

## Backfill on its own

```rust,ignore
let folded = store.backfill_norm().await?;
```

Runs the same folded-column backfill `upgrade` runs internally, without
touching DDL — useful after a fold-rule change (`L13`, `L14`) that added no
new column. Closes this port's share of audit **F-15**.

## Backup

A fhir-mariadb store is plain MariaDB: `mariadb-dump` for a logical dump,
binary-log replication, and any consistent filesystem or volume snapshot
taken while the server is quiesced is a valid backup. Point-in-time recovery
is binlog replay — not PostgreSQL's WAL-based PITR, which this chapter
described for every port until 2026-08-03 (audit **F-56**). Nothing in this
crate schedules a backup; that is the deployment's job (see the
[trust boundary](trust-boundary.md)).
