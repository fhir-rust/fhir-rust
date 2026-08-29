# Operations

There is no `fhir-mssql serve`, no health endpoint, no `/metrics`, and no
CLI flag anywhere in this workspace (`C0.17`, `C0.18`). Everything below
describes what `MsSqlStore` itself does when a caller embeds it in their own
process — deployment posture, HTTP timeouts, and load shedding are a
server's job, and the server is a separate crate,
[`fhir-loco`](../../../fhir-loco), out of scope for this book.

## Connecting and pooling

`MsSqlStore::connect(dsn, map)` parses an ADO connection string with
`mssql::Config::from_ado_string`, builds a `bb8` pool around it
(`pool.rs` — `mssql` ships no pool of its own, unlike `mysql_async` or
`tokio-postgres`), and **borrows and immediately releases one connection**
before returning. That last step exists because `bb8::Pool::builder().build`
does not itself open a connection: without it, `connect` against a dead port
returned `Ok` and the failure only surfaced on whatever request happened to
run first — a worse diagnostic than a `connect`-time error, found by running
it live (**F-65**).

Pool size is `bb8`'s default; nothing here exposes a way to configure it —
there is no `--pool-size` flag or `FHIR_MSSQL_POOL_SIZE` variable, because
there is no CLI to carry one. A caller that needs a specific pool size has
to build the `bb8::Pool` itself, which `MsSqlStore::connect` does not
currently allow.

## Install

`store.init(checksum)` applies the generated DDL (`fhir_mssql_map::ddl::ddl`)
statement by statement and records `checksum` in `[fhir_mssql_meta]`. It is
**idempotent but not atomic**: T-SQL has no transactional DDL, so a
`CREATE TABLE` commits immediately, and a failed install can leave a partial
schema behind — the same limitation MySQL and MariaDB have, not the staged
schema/atomic rename PostgreSQL supports. `init` reports how far it got
(`Result<usize, StoreError>`, the count of the last statement it reached)
rather than pretending an atomicity it does not have. Idempotence itself
comes from `IF NOT EXISTS (SELECT … FROM sys.objects)` guards around each
statement (`M14.17`) and `CREATE OR ALTER TRIGGER` for the append-only
triggers (`M14.19`) — there is no `IF NOT EXISTS` on `CREATE TABLE` in
T-SQL, so the DDL emitter has to spell idempotence out by hand.
`store.drop_schema()` removes a version's schema; the test suite calls it at
the start of nearly every test rather than assuming a clean database.

**There is no `upgrade`.** The map crate emits upgrade DDL — `ALTER TABLE …
ADD` for new columns, correctly spelled without a `COLUMN` keyword
(`M14.32`, fixing **F-25**, where the emitter had carried MySQL's `ADD
COLUMN` syntax that SQL Server's parser rejects outright) and with a
`DEFAULT` on every added `NOT NULL` column (`M14.33`, fixing **F-26**, since
SQL Server refuses a `NOT NULL` column with no default on a table that
already has rows) — but no method on `MsSqlStore` calls it. That DDL is
unit-tested and has never been executed against a live server, because
nothing exists to execute it with. Do not read "the DDL is correct" as "this
port has an upgrade path": the conformance matrix's `—` in that row means
exactly that no store calls it. `backfill_norm` does not exist either.

## Session-state hygiene on a pooled connection

Two operations set connection-scoped state that a later, unrelated caller
on the same pooled connection must never inherit, and both are reset on
every exit path, not just the happy one:

- **`get`** issues `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` before its
  read (`M14.25`, see [The trust boundary](trust-boundary.md) for why), and
  resets it to `READ COMMITTED` before the connection returns to the pool —
  including when the read loop itself fails partway through, which an
  earlier version of `get` did not guard and would have left an open
  `SNAPSHOT`-isolation transaction on the connection the next borrower
  received.
- **`purge`** sets `SESSION_CONTEXT('fhir_mssql_erasure', 'on')` to pass the
  append-only trigger's erasure escape hatch (`M14.21`), and clears it again
  — inline after the delete, and again in a best-effort cleanup after the
  transaction resolves either way — so a later caller's ordinary `DELETE`
  cannot silently ride through on a flag a previous, unrelated `purge` left
  set.

## Write serialization

`put` and `delete` take `SELECT … WITH (UPDLOCK, ROWLOCK) WHERE [id] = @P1`
on the base row before reading the chain tip, holding the lock until
commit or rollback (`M14.26`, `H5.4`). A second writer for the same
resource id blocks on that `SELECT` rather than racing the tip read.
`tests/concurrency.rs`'s `racing_writers_get_distinct_versions_and_a_verifiable_chain`
confirms 8 of 8 racing writers get distinct, consecutive versions and a
chain that still verifies afterward.

## Logging ceiling

`mssql` itself logs full TDS row payloads — including the raw resource
JSON — at `TRACE`, from its own connection and token-decoding internals,
entirely outside this store's control. `redaction.rs`'s first case failed
immediately when run at `TRACE`; it was not this crate's code that leaked
the marker planted in the test. **`DEBUG` is the ceiling this store can
actually promise; `TRACE` MUST NOT be enabled against this port for a
deployment carrying PHI.**

## Transport security

`connect` negotiates TLS during login regardless of whether the rest of the
connection is otherwise plaintext — SQL Server's own login handshake
requires it (`M14.24`). `TrustServerCertificate=false` in the DSN
(`mssql`'s verifying `TrustConfig::Default`) measurably **rejects** a
self-signed certificate; `=true` accepts it — `tests/ssl_live.rs` proves the
mechanism is not a no-op. The certificate-parsing code in that same
dependency chain used to carry three unpatched CVEs plus one
unmaintained-crate advisory, reaching the shipping `fhir-mssql-store` crate
rather than only a dev-dependency (`M14.34`, **F-67**) — `native-tls` was
tried as a swap and fails the TLS handshake outright against
`azure-sql-edge` on this host. Resolved 2026-08-29 by switching the driver
from `tiberius` (0.12.3, its last release) to `mssql`, a fork carrying the
fixes forward: `rustls-webpki 0.103.15` now, none of the four advisories
remain in the tree. **This port now claims `O10.7` satisfied.**

## Backup

A fhir-mssql deployment's backup story is plain SQL Server: native `BACKUP
DATABASE`, log shipping, availability groups, and point-in-time restore
from a log chain — not `pg_dump` or PostgreSQL PITR, which this chapter
claimed for every port until 2026-08-03 (**F-56**). This crate does not
implement any of it; there is no `backup`/`restore` method on `MsSqlStore`
and none is planned here. A consistent database is a valid store to restore
from — the append-only history table and the hash chain reconstruct the
same way after a restore as before one — but scheduling, retention, and
executing a restore are the operator's and the engine's job, not this
library's.

## Running the live suite

```sh
scripts/db.sh up      # SQL Server 2022 (or FHIR_MSSQL_IMAGE=…/azure-sql-edge on arm64)
scripts/db.sh test    # up, then the live suite
scripts/db.sh client  # an interactive mssql-based client inside the container
scripts/db.sh down
```

`--test-threads=1` is required for `cargo test -p fhir-mssql-store`: each
live test installs and drops its own schema, and running them concurrently
against one small container deadlocked SQL Server (error 1205) during
schema teardown under the default thread-per-core runner — a container-load
artifact, not a bug in any single test.
