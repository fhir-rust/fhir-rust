# Operations

Everything below is `Store` methods called from Rust
(`crates/fhir-postgresql-store/src/lib.rs`). There is no server process, no
health endpoint, and no metrics exporter in this crate — see the
[introduction](introduction.md)'s banner. A deployment that wants those wraps
`Store` in its own service, or uses [`fhir-loco`](../../../fhir-loco/).

## Connection posture

`Store::connect` reads the standard `PG*` environment variables (or an
explicit DSN via `pg_config`). Two things are set unconditionally, not left
to the caller to remember:

- **`statement_timeout`** — 30 s by default
  (`FHIR_POSTGRESQL_STATEMENT_TIMEOUT_MS` to change it), sent as `-c
  statement_timeout=…` at connect time. An unbounded statement is an
  unbounded hold on both the `REPEATABLE READ` snapshot a read takes
  (`M14.15`) and the row lock a write takes (`M14.16`), so this is not
  optional hardening — it bounds two other guarantees.
- **TLS** — `PGSSLMODE` defaults to `require`, which verifies the server
  certificate *and* hostname (`M14.27`, **F-17**, fixed 2026-08-03). That is
  stricter than libpq's own `require`, which encrypts without verifying
  anything. Set `PGSSLROOTCERT` if your server uses a private CA;
  `PGSSLMODE=disable` only if you mean it.

The connection pool (`deadpool`) defaults to 16 connections
(`FHIR_POSTGRESQL_POOL_SIZE`, or `Store::connect_full`'s explicit
argument overrides both) and a 2-second acquire timeout — exhaustion returns
`StoreError::Pool` rather than queueing forever. There is no `--max-concurrent`
or admission control in this crate; that is a service-layer concern.

## Install and upgrade

```rust
store.init("r5-baseline").await?;                 // Ok(true): installed
store.init("r5-baseline").await?;                 // Ok(false): already current, no-op
```

`init` stages the generated DDL under a temporary schema in chunked
transactions (200 statements per transaction) and renames it into place with
one `ALTER SCHEMA … RENAME TO …` (`M14.14`). A single transaction for the
whole install does not work — creating 7,355 tables at once exhausts
`max_locks_per_transaction`, a server-wide setting a tenant often cannot
raise — so the staged-schema route gives the same observable atomicity a
different way. `init` refuses to touch a schema that was installed from a
different map (a different `checksum` string), and a failed install leaves
only the orphaned staging schema, which the next `init` call drops before
retrying.

```rust
let report = store.upgrade("r5-v2", allow_destructive).await?;
println!("{} additive, {} destructive, {} values backfilled",
    report.additive, report.destructive, report.folded);
```

`upgrade` diffs the map stored at install time against the map this `Store`
was built with: new tables, new columns, and new search indexes apply
automatically. Dropped tables or columns are refused unless
`allow_destructive` is `true` — the error text says "rerun with
`--allow-destructive`", a holdover from when this had a CLI front end; there
is none today, so read it as "call `upgrade` again passing `true`." **A
column's type changing is never applied automatically**, destructive or not:
`upgrade` returns `StoreError::Other` naming the column and both types, on
the reasoning that a type change needs a real data migration, not a
generated `ALTER COLUMN`.

An upgrade that adds a folded search column also **backfills** it before
returning, and `report.folded` says how many distinct values it folded. This
is not optional tidying: `M14.20`'s search predicates compare the folded
column, so a newly-added column left `NULL` would make existing rows stop
matching searches that used to find them, with no error anywhere. The
backfill folds distinct values (a surname repeats across patients) in
batches of 1,000 and only touches rows still `NULL`, so an interrupted
upgrade resumes correctly on retry rather than re-doing finished work.

`store.drop_schema().await?` removes the version schema — tables in batches
of 50, then the schema itself — for the same lock-budget reason `init`
stages its writes.

## The hash chain: verifying, witnessing, keying, re-signing

Four operations, each answering a different question about tamper-evidence
(`M3.16`; see [the trust boundary](trust-boundary.md) for what each layer
does and does not prove):

```rust
let breaks = store.verify_audit().await?;      // Vec<ChainBreak>, empty = clean
let witness = store.chain_witness().await?;     // one digest over every chain head
store.emit_checkpoint("startup").await;         // logs `witness` on the audit_checkpoint target
let signed = store.resign_history(&audit, "key rotation 2026-08").await?;
```

- **`verify_audit`** recomputes every resource's SHA-256 and SHA-3-256 chain
  from the stored bytes — canonicalized in Rust through the shared `canon.rs`
  since **F-07**, the same function the writer used, so writer and verifier
  agree by construction. A tombstone (`op = 'X'`) terminates a chain rather
  than breaking it. A row written before the audit columns existed reports as
  the point history begins, not as tampering.
- **`chain_witness`** is a single digest over `(resource type, id, last
  version, its two digests)` for every chain in the schema — deterministic
  over unchanged history, so a value recorded yesterday and compared to
  today's exposes truncation or wholesale deletion that a per-row check
  cannot: a chain missing its last version still verifies, because nothing
  left behind refers to what is gone.
- **`emit_checkpoint(reason)`** computes the witness and logs it as one
  `tracing::info!` line on the `audit_checkpoint` target — call it after
  startup and after any erasure (`purge` already does the latter for you).
  The line carries only counts and digests, no PHI, so it can be retained
  and routed separately from ordinary application logs.
- **`resign_history`** counter-signs every row under the current signing key
  without touching the original `row_mac` (`M3.16d`). It verifies the whole
  chain first and refuses to run if anything fails — re-signing unverified
  history would launder a forged row into the new key's authority, which is
  the one thing this must never do.

## Keying the chain

Unkeyed, the two digests detect careless modification but not a forger who
knows the pre-image format — anyone with SQL write access can compute a
matching digest for what they wrote. A keyed HMAC closes that, but the key
must live outside the database.

**The environment variable is `FHIR_SQLITE_CHAIN_KEY`, not
`FHIR_POSTGRESQL_CHAIN_KEY`, even on this engine.** The keying code lives in
the shared, engine-agnostic `fhir-store` crate
(`fhir_store::chain::KeyRing::from_env`), and its variable names were never
adapted per port. Setting `FHIR_POSTGRESQL_CHAIN_KEY` compiles, looks correct,
and does nothing — `Store::chain_key_id()` will report `None`. This is a
real, verified gap in this codebase, not a documentation choice; there is no
tracked audit finding for it as of this writing. Two ways to actually key a
`Store`:

```rust
// Either set the (misleadingly-named) real variable before connecting:
// FHIR_SQLITE_CHAIN_KEY=<64 hex chars>  FHIR_SQLITE_CHAIN_KEY_ID=k1

// Or, more robust — bypass the environment entirely:
use fhir_postgresql_store::chain::KeyRing;
let store = Store::connect(cfg, map).await?
    .with_chain_keys(KeyRing::from_files(
        Some(("k1", "/run/secrets/chain-key-k1".as_ref())),
        &[],   // retired keys, if any, verify but never sign
    ).map_err(/* ... */)?);
```

`with_chain_keys` also sidesteps a real hazard with the environment-variable
route: it is process-global, so mutating it while other code in the same
process is connecting is not thread-safe (`crates/fhir-postgresql-store/tests/audit.rs`
says so explicitly, and structures its own key-rotation test around calling
`with_chain_keys` directly rather than the environment for exactly this
reason).

## Erasure

```rust
let report = store.purge("Patient", "1234", &audit).await?;
// PurgeReport { versions_erased, existed }
```

`purge` is the one sanctioned exception to append-only history (`M3.18`,
GDPR Art. 17): it deletes the resource and every historical version inside a
transaction that sets `fhir_postgresql.erasure = 'on'` — the one condition
the append-only trigger (below) allows a `DELETE` under — and writes a
tombstone in its place recording who erased it, when, why, and the hash both
chains ended on. `verify_audit` treats a tombstone as a recorded erasure, not
a break. `purge` calls `emit_checkpoint("after-erasure")` itself, so the one
sanctioned deletion is always followed by a witness an external log can
compare against.

Two limits worth stating plainly: the database is not the estate — backups,
replicas, and WAL archives still hold the data until they age out, so a
deployment promising erasure needs a plan for all of them, not just this
call. And the guard is against *accident*, not against the application: the
trigger permits a `DELETE` inside a transaction that sets the session
variable, which is exactly how `purge` itself works, so application code
with a direct SQL connection could do the same thing. The trigger stops
ordinary code, migrations, and stray statements; the tombstone and the
access log are what make a *deliberate* erasure accountable rather than
prevented.

## Append-only history

`init` emits `fhir_postgresql_history_is_append_only()` and a `BEFORE UPDATE
OR DELETE … FOR EACH ROW` trigger on every history table (`M14.19`). A plain
`UPDATE` or `DELETE` against a `*_history` table raises an exception naming
`M3.17`; escaping it — as `purge` does, deliberately — requires setting
`fhir_postgresql.erasure` first. Disabling the trigger outright
(`ALTER TABLE … DISABLE TRIGGER`) is the only other way through, and that is
a DBA act that leaves its own trace in the server log.

## Backup

A `fhir-postgresql` database is plain PostgreSQL. `pg_dump`, physical
replication, and point-in-time recovery all apply unchanged, and any
consistent snapshot is a valid store (`M14.28`) — nothing here needs a
special backup tool or a quiesce step beyond what PostgreSQL itself
requires.
