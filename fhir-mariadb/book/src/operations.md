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
then records the map checksum, the FHIR® version, and the map asset itself
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

A consistent MariaDB snapshot is a valid store. A logical drill can use
`mariadb-dump --single-transaction --routines --triggers --hex-blob`. Keep the
audit keys outside the dump, restore with the `mariadb` client, then reopen with
the matching map and verify the installed checksum, current and earlier
resources, disclosures, search results, and keyed audit chain.

MariaDB's dump qualifies trigger DDL with the original database name. Restore
into that same name after dropping or renaming the damaged database; importing
beside a still-live source under a different default database can attempt to
recreate triggers against the source and fail with `ERROR 1359` (trigger
already exists). A 2026-11-19 drill verified this procedure on MariaDB 11.4.13
with a keyed R5 Patient slice (F-104).

For production recovery objectives, use binary-log backup and replay, or a
consistent filesystem/volume snapshot taken while the server is quiesced.
This is not PostgreSQL WAL PITR. Nothing in this crate schedules or retains a
backup; that remains the deployment's job (see the [trust
boundary](trust-boundary.md)).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
