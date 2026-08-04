# Operations

This chapter used to describe an HTTP server — health/ready endpoints,
Prometheus metrics, request timeouts, graceful shutdown. **None of that
exists in this crate.** fhir-oracle is a library (`C0.17`, `C0.18`); if you
want those concerns, they belong to whatever process embeds this store, or
to [`fhir-loco`](https://github.com/fhir-rust/fhir-rust/tree/main/fhir-loco)
if you mount a store behind HTTP. What follows is what the library itself
actually does.

## Connecting

`OracleStore::connect(user, password, connect_string, map)` builds a
connection pool via the `oracle` crate's own `oracle::pool::Pool` (Oracle
Instant Client's session pooling, not a hand-rolled one). Every method on
`OracleStore` is `async` but the driver itself is synchronous — it calls
into ODPI-C, which calls into OCI, which blocks the OS thread on network
I/O — so every method wraps its body in `tokio::task::spawn_blocking`.

## Schema install

`store.init(checksum)` runs the generated DDL — 9,636 statements for the
full R5 schema — and records the checksum it installed from;
`installed_checksum()` reads it back. Re-running `init` with the same
checksum is a no-op in the sense that the DDL itself is idempotent (every
statement is a PL/SQL block swallowing `ORA-00955`, since Oracle has no
`CREATE TABLE IF NOT EXISTS`), not in the sense that it diffs anything — see
below.

**There is no `init --upgrade` and no `backfill_norm`.** Both exist on
other ports (PostgreSQL, SQLite, MySQL, MariaDB); this port has neither.
Installing a new schema version currently means a fresh install, not a
migration.

## Reads, writes, and the two open concurrency questions

`put`/`delete` take a row lock (`SELECT … FOR UPDATE` on the base row) before
computing the next `version_id` and appending to history, which is Oracle's
native equivalent of the `WITH (UPDLOCK, ROWLOCK)` hint SQL Server needs for
the same guarantee (`H5.4`, `M14.20`). This is implemented and exercised by
every test in `tests/oracle_store.rs`, but **no test races concurrent
writers against it** — unlike `fhir-mssql`'s and `fhir-mysql`'s
`concurrency.rs`, which do. Treat it as "present and plausible", not
"contention-verified".

**`get` has no snapshot-isolation protection at all, and that is a known,
open gap, not an oversight.** `R4.5` requires a multi-statement read of one
resource's tables to see one consistent snapshot. The candidate this port's
annex named, `SET TRANSACTION READ ONLY`, was tried live and found to fail
outright: `ORA-01466: unable to read data - table definition has changed`,
on **any** session that has ever executed DDL — which includes every session
that has run `init`. This was reproduced independently with a minimal
3-statement probe (`CREATE TABLE` + commit, then on the same session `SET
TRANSACTION READ ONLY` + `SELECT`), confirming it is a genuine Oracle
session-level behavior, not a bug in how the store called it. The call was
removed rather than shipped broken. No replacement mechanism is implemented
yet — see `M14.19` in the [dialect annex](../../spec/14-oracle-dialect.md).

## Erasure

`store.purge(rtype, id, audit)` deletes a resource's history and leaves a
tombstone, the same pattern every port uses. The append-only trigger that
would otherwise refuse any `DELETE`/`UPDATE` on history is bypassed for the
duration of the call via `DBMS_APPLICATION_INFO.SET_CLIENT_INFO` — Oracle's
nearest equivalent to a session variable — set immediately before the
delete and cleared immediately after, on the same connection, never assumed
to reset itself between pool checkouts (`M14.29`).

## Audit

`store.log_access(record)` writes a disclosure row to
`"<schema>"."fhir_oracle_access_log"`; `store.verify_audit()` recomputes the
SHA-256 and SHA-3-256 chains over history and reports any break. Both are
live-tested (`disclosures_are_recorded`,
`history_vread_delete_and_verify_audit`). An optional HMAC signing key can
be supplied via `OracleStore::with_chain_keys`, a builder method — there is
no CLI flag for it, because there is no CLI. There is no `chain_witness` or
`resign_history` on this port (both exist only on `fhir-postgresql`).

## Backup

A fhir-oracle store is plain Oracle underneath: RMAN backups, Data Guard for
a standby, flashback or RMAN point-in-time recovery. None of that is
`pg_dump` or PostgreSQL PITR, which this chapter claimed for every port
until 2026-08-03 (audit **F-56**).

One Oracle-specific note: a version namespace here is a **user** (`M14.5`),
so a backup scope is a user's objects rather than a schema inside a
database — plan RMAN/export scripts accordingly.
