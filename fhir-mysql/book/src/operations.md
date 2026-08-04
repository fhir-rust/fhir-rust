# Operations

This chapter used to describe a `fhir-mysql serve` binary — flags, `/health`,
`/metrics`, request shedding. None of that exists in this workspace (`C0.17`,
`C0.18`; audit **F-56**): `fhir-mysql` is a library with three crates and no
server. If you are running behind `fhir-loco` or a binary you wrote yourself,
health checks, metrics, request timeouts, and load shedding are that
process's concerns, not this crate's. What follows is what `fhir-mysql-store`
itself actually does.

## Connecting, and TLS

`MySqlStore::connect(dsn, map)` reads `FHIR_MYSQL_SSL_MODE` (default
`VERIFY_IDENTITY`) and `FHIR_MYSQL_SSL_CA`; `connect_with(dsn, map, mode)`
takes the mode explicitly. Both fail at construction time — not at first
query — if the DSN is malformed or the server is unreachable (`ssl.rs`,
`O10.7`, **F-54**).

These are **not** the PostgreSQL port's `PGSSLMODE`/`PGSSLROOTCERT`: this
crate speaks `mysql_async`, not libpq, and reusing the PostgreSQL names would
invite a deployment to set a variable that silently does nothing. `PREFERRED`
is refused outright, not approximated — `mysql_async` can only make TLS
mandatory or absent, and silently downgrading a request for encryption to a
plaintext link is exactly the failure `O10.7` exists to prevent:

```sh
export FHIR_MYSQL_SSL_MODE=VERIFY_IDENTITY   # the default; DISABLED, REQUIRED, VERIFY_CA also accepted
export FHIR_MYSQL_SSL_CA=/etc/ssl/mysql-ca.pem
```

Live-verified: `VERIFY_IDENTITY` rejects a self-signed certificate against a
real MySQL 8.4 server (`ssl_live.rs`), and `O10.7` defaults to the verifying
mode rather than requiring an operator to opt into it.

## Install and upgrade

`store.init(checksum)` installs the generated schema — every `CREATE TABLE`,
index, and trigger statement, applied one at a time — and records the map
checksum, the FHIR version, and the **map asset itself** (gzipped, hex-coded,
`LONGTEXT` — a `TEXT` column silently truncates a ~2.4 MB R5 asset, `M14.34`)
in `fhir_mysql_meta`. Re-running against an already-installed schema is
refused. Unlike `fhir-postgresql`, there is **no staged install and no atomic
rename**: MySQL's DDL commits implicitly, so there is nothing to roll back.
If a statement fails partway through, `init` reports exactly how many
statements had already been applied, because an operator cleaning up needs to
know whether the database is empty or half-built (`M14.22`, `M14.35`).

```rust,ignore
let n = store.init("r5-2026-08").await?;   // number of DDL statements applied
```

`store.upgrade(checksum, allow_destructive)` diffs the installed map asset
against the current one: new tables, columns, and indexes apply automatically;
dropped tables or columns are refused unless `allow_destructive` is `true`;
a changed column type always refuses — there is no in-place migration path
for that case. The MySQL-specific trap here is that plain `CREATE INDEX` has
no `IF NOT EXISTS`, so blindly re-applying the shared index list a second time
fails with `Duplicate key name` (**F-28**); `upgrade` reconciles it against
`information_schema.statistics` and `information_schema.columns` instead —
and does so *after* the additive statements run, not before, because a table
`create_table` just created already carries the columns a pre-computed filter
would try to add again (`M14.36`).

`upgrade` also runs `backfill_norm` as part of the same call, not as a
separate step, and returns how many distinct values it folded
(`UpgradeReport { additive, destructive, folded }`). This is not optional
tidying: every non-`:exact` string search compares the folded column, so a
schema that gained the column without backfilling it would answer real
searches with fewer results and no error at all (`L13`, `L14`, `M14.37`). The
backfill folds **distinct values**, not rows, in batches of 1,000, and only
rows still `NULL` — which is what makes an interrupted upgrade resumable by
simply calling `upgrade` again.

```rust,ignore
let report = store.upgrade("r5-2026-09", false).await?;
println!("{} additive, {} folded", report.additive, report.folded);
```

`store.drop_schema()` issues one `DROP SCHEMA IF EXISTS` — not chunked, since
MySQL's own DDL already has no cross-table transaction to protect.
`store.table_count()` / `store.trigger_count()` read `information_schema` for
diagnostics, and `store.installed_checksum()` distinguishes "not installed"
from "installed, checksum X" — an install predating **F-15** (no
`map_asset` column) is refused with a message naming that distinction rather
than being mistaken for "not installed" (`M14.33`).

## Bulk writes and `max_allowed_packet`

Multi-row inserts are chunked to stay under MySQL's 65,535-placeholder limit
per statement (`insert_rows`, chunk size derived from column count, capped at
60,000 total placeholders). Nothing in this crate inspects or adjusts the
server's `max_allowed_packet`, which bounds total statement *byte* size
independently of placeholder count (`M14.28`) — a deployment writing very
wide resources should size that server setting itself; the placeholder
chunking does not protect against it.

## Backup

A fhir-mysql store is plain MySQL: `mysqldump` for a logical dump,
binary-log replication, and any consistent snapshot is a valid store.
Point-in-time recovery is binlog replay — not PostgreSQL's WAL-based PITR,
which this chapter described for every port until 2026-08-03 (audit
**F-56**). `fhir-mysql` guarantees that a consistent snapshot is a valid
store; it does not schedule, verify, or retain one.
