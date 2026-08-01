# Operations

## Deployment posture

fhir-mssql handles PHI. The server binds loopback by default and implements
**no authentication** by design — the deployment perimeter (reverse
proxy, service mesh, or SMART-on-FHIR gateway) owns identity and
authorization. TLS terminates either at that perimeter or in-process via
the `tls` build feature (`--tls-cert`/`--tls-key`, rustls).

## Health, metrics, logs

- `/health` — liveness; `/ready` — database connectivity.
- `/metrics` — Prometheus text: request totals, response classes, and
  `fhir_mssql_request_latency_seconds`, a histogram over the default 1ms–10s
  buckets. It is a histogram rather than a running total because a mean
  cannot distinguish "every request took 40ms" from "99% took 5ms and 1%
  took 4 seconds"; `histogram_quantile` answers p99 from these buckets.
- Every request gets an `X-Request-Id` (propagated when supplied) and one
  tracing line with method, path, and status. Resource content is never
  logged.

## Timeouts and load shedding

Server-side `statement_timeout` defaults to 30 s
(`FHIR_MSSQL_STATEMENT_TIMEOUT_MS`); pool waits are bounded at 2 s, and
exhaustion answers **503 + Retry-After** instead of queueing unboundedly.
`fhir-mssql serve` shuts down gracefully on SIGINT/SIGTERM, draining any queued
disclosure records before it exits.

Every edge ceiling is a flag, because the right value depends on the
deployment:

| Flag | Default | Bounds |
| --- | --- | --- |
| `--request-timeout` | 60 s | Wall clock for one request |
| `--max-concurrent` | 256 | Requests in flight before shedding |
| `--max-body-mb` | 32 | Request body size |
| `--max-count` | 1000 | `_count`, whatever the client asks |
| `--max-included` | 1000 | `_include`/`_revinclude` expansion; truncation is reported in the bundle |
| `--pool-size` | 16 | Database connections. Overrides `FHIR_MSSQL_POOL_SIZE` |

`--max-concurrent` sits deliberately above the pool size, so pool exhaustion
stays the usual back-pressure signal and the concurrency limit is the
backstop behind it.

## Install and upgrade

`fhir-mssql init` installs under a staging schema in chunked transactions and
renames it into place atomically — no `max_locks_per_transaction` tuning
required. It records the map checksum and the map itself; re-running is a
no-op, and a mismatched artifact is refused.

`fhir-mssql init --upgrade` diffs the installed map against the current
assets: new tables, columns, and indexes apply automatically; anything
destructive (dropped tables or columns) is refused without
`--allow-destructive`; column type changes always demand a manual
migration. `fhir-mssql drop --yes` removes a version schema in lock-safe
chunks.

An upgrade that adds folded search columns also **backfills** them before
it returns, and reports how many values it folded. This is not optional
tidying: string search compares the folded column, so an upgrade that left
it NULL would answer searches with fewer results and no error at all. The
backfill folds distinct values in batches and is resumable — if it is
interrupted, rerunning `--upgrade` continues where it stopped.

## Backup

A fhir-mssql store is plain PostgreSQL: `pg_dump`, physical replication, and
point-in-time recovery all apply unchanged, and any consistent snapshot
is a valid store.
