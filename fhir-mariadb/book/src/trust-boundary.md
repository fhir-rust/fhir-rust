# The trust boundary

fhir-mariadb is a **library**, not a system, and not a server. Earlier
versions of this chapter described HTTP status codes, request headers, and a
`fhir-mariadb serve` command — none of which exist in this crate (`C0.17`,
`C0.18`; audit **F-56**). If a deployment wants an HTTP surface with
authentication, authorization, and a proxy-facing perimeter, that is
[`fhir-loco`](../../../fhir-loco/), a separate crate. This chapter states, in
one place, what calling `fhir-mariadb-store` directly actually guarantees —
verified against `crates/fhir-mariadb-store/src/mariadb.rs` — and what any
caller (`fhir-loco` or your own code) still has to provide, because a
boundary nobody can point at is not a boundary (spec `PR12.8`).

## What fhir-mariadb guarantees

| Property | How | Spec |
| --- | --- | --- |
| Writes are transactional and versioned | `put`/`delete` run in one `mysql_async` transaction; `version_id` increments off the history tip | `R4.4`, `H5.1` |
| Concurrent writers to one resource serialize | `SELECT … FOR UPDATE` on the base row before the tip is read, so N writers queue instead of racing (closed audit **F-24**: 1 of 8 succeeded before this) | `H5.4` |
| Reads see one consistent snapshot | `get` runs its multi-table reconstruction inside one `REPEATABLE READ` transaction (closed audit **F-21**: reads used to tear across statements) | `R4.5` |
| Every change records who made it | the same statement that writes history writes `actor`, `actor_source`, `client`, `request_id`, `reason` alongside it | `M3.15` |
| Every read *can* be recorded | `log_access`/`log_access_batch` insert an `AccessRecord` into `fhir_mariadb_access_log` — see the caveat below | `PR12.5` |
| History cannot be rewritten by ordinary SQL | `BEFORE UPDATE`/`BEFORE DELETE` triggers (`SIGNAL SQLSTATE '45000'`) refuse any statement that does not first set the erasure escape | `M3.16`, `M3.17` |
| History is tamper-evident under two independent digest families | every history row carries `row_hash` (SHA-256) and `row_hash_sha3` (SHA3-256), chained to the previous row; `verify_audit` recomputes and reports the first break | `M3.16` |
| PHI is encrypted in transit to the database by default | `mysql_async` connects with `FHIR_MARIADB_SSL_MODE=VERIFY_IDENTITY` unless overridden; `PREFERRED` is refused rather than silently downgraded (closed audit **F-54**) | `O10.7` |
| Erasure leaves a verifiable hole, not a silent gap | `purge` deletes the resource and its history, then writes one tombstone row (`op = 'X'`) recording who, when, why, and the hash the chain ended on | `M3.18` |

Two of those rows need the caveat spelled out, not left implicit:

**Disclosure logging is opt-in, not automatic.** `get`, `search`, and
`history` do not call `log_access` themselves — nothing in this crate wires
a read to a log entry. A caller that wants `PR12.5` satisfied must call
`log_access`/`log_access_batch` itself, on every read path it cares about.
Silence here is a caller bug, not a library one, but it is easy to have
without noticing.

**Nothing in this crate reads a MariaDB-specific chain-key environment
variable.** Keying the chain (turning the bare SHA-256/SHA3-256 hashes into
HMAC tags no one without the key can forge) is done by building a
`fhir_store::chain::KeyRing` and passing it to
`MariaDbStore::with_chain_keys`. `KeyRing::from_env()` exists and works, but
it is defined once in the shared `fhir-store` crate and reads
`FHIR_SQLITE_CHAIN_KEY` / `FHIR_SQLITE_CHAIN_KEY_ID` literally, regardless of
which port calls it — there is no `FHIR_MARIADB_CHAIN_KEY`. Setting a
variable by that name has no effect and fails silently by doing nothing; if
you want env-based keying for this port, either set the `SQLITE`-named
variables anyway (they are read the same way by every port) or build the
`KeyRing` explicitly with `KeyRing::new`/`KeyRing::from_files` and skip
`from_env` entirely.

## What the deployment (or your calling code) must provide

fhir-mariadb does **not** do these, and a caller that skips them is not safe
to put patient data behind:

| Obligation | Why it is not here |
| --- | --- |
| **Authentication and authorization** | There is no principal, no scope check, no `meta.security` enforcement, and no consent evaluation anywhere in this crate. `Audit::actor` is whatever the caller passes in — `Audit::unattributed()` if nothing is known — and the store trusts it completely. |
| **Optimistic concurrency, `ETag`/`If-Match`, conditional create/delete** | None of it exists. `StoreError::Conflict` and the `CondCreate`/`CondDelete`/`TxOp`/`TxOutcome` types are re-exported from `fhir-store` for API compatibility with ports that do implement them, but nothing in `fhir-mariadb-store` constructs or returns them. A concurrent `put` to the same id is serialized by the `FOR UPDATE` lock above, not rejected — the second writer wins, silently, in write order. |
| **Multi-operation transactions across resources** | `put`/`delete`/`purge` are each one resource, one transaction. There is no `transact`/`transact_audited` here. |
| **A tamper-evidence checkpoint or external witness** | `verify_audit` exists and is complete; there is no `emit_checkpoint`, `chain_witness`, or `resign_history` in this crate. If you need an off-box witness of the chain head, you have to build it yourself from `verify_audit`'s output. |
| **TLS to your own callers** | `FHIR_MARIADB_SSL_MODE`/`FHIR_MARIADB_SSL_CA` secure the link to MariaDB. They say nothing about how your process is reached. |
| **Rate limiting, network isolation, backup scheduling, key management, at-rest encryption** | All deployment-perimeter concerns; see [Operations](operations.md) for what this crate does about backups (nothing beyond "a snapshot of plain MariaDB is a valid store") and what it does not. |

## What neither provides yet

Stated rather than implied, so nobody discovers it during an audit:

- **Terminology validation.** Required-binding `CHECK` constraints only. No
  value-set expansion, no SNOMED/LOINC/ICD membership checks.
- **Profile conformance.** Base-specification structure only — not US Core,
  IPS, or any implementation guide.
- **FHIRPath invariants.** The `fhir` crate enforces three of 314.
- **Referential integrity across resources.** FHIR® permits dangling
  references and so does fhir-mariadb (`M3.10`).

## Configuring TLS to the database

The vocabulary is MariaDB's, not libpq's — this port used to read `PGSSLMODE`
because the text was copied from `fhir-postgresql`, and a deployment that set
that variable believing it took effect would get a silent plaintext link,
which is exactly the failure `O10.7` exists to prevent (audit **F-54**).

```sh
export FHIR_MARIADB_SSL_MODE=VERIFY_IDENTITY   # the default; shown for clarity
export FHIR_MARIADB_SSL_CA=/etc/ssl/mariadb-ca.pem
```

`FHIR_MARIADB_SSL_MODE` takes MariaDB's own four values: `DISABLED`,
`REQUIRED` (encrypts, validates nothing — weaker than it sounds), `VERIFY_CA`,
and `VERIFY_IDENTITY` (the default). `PREFERRED` is a fifth value MariaDB's
own client accepts and this port's driver, `mysql_async`, cannot express —
`SslOpts` makes TLS mandatory or nothing, with no third state — so `PREFERRED`
is a startup error naming the two modes that do exist, rather than a silent
choice of one.

## Verifying the audit trail

```rust,ignore
let breaks = store.verify_audit().await?;
```

Recomputes every resource's SHA-256 and SHA3-256 chains from the stored
history rows and returns every `ChainBreak` found — empty means "nothing in
history has been altered since it was written". A row predating the chain
columns has no stored hash and is skipped rather than reported: calling that
tampering would train an operator to ignore real breaks. There is no CLI
command for this; it is a method a caller invokes.

### What each layer actually proves here

| Layer | Stops | Does not stop | Status in this crate |
| --- | --- | --- | --- |
| SHA-256 + SHA3-256 digests | Careless or unaware modification — a migration, a stray `UPDATE` that got past the trigger, a row restored from the wrong backup. Two design families, so one line of cryptanalysis cannot take both. | An attacker who knows the pre-image format, which is public and unkeyed. | Always on; every history row carries both. |
| HMAC tag (`row_mac`) | Forgery, if the caller wired a signing key in. Producing a valid tag needs a key held only in the calling process, never written to the database. | A row being deleted outright. | Off by default. Requires `with_chain_keys(KeyRing::new(...))` or an equivalent explicit call — see the caveat above about `from_env`'s variable names. |
| `fhir_mariadb_countersign` table | Nothing yet. | — | The table exists, and `verify_audit` reads it as a fallback when a row's own `row_mac` is absent or signed by an unheld key — but **nothing in this crate inserts into it**. It is dead weight until a caller starts writing counter-signatures somewhere the primary history table's own compromise would not also compromise. Do not rely on it. |
| Off-box witness (checkpoint, external log) | Truncation and wholesale deletion, if you build it. | — | Not provided. `verify_audit`'s report is the only primitive; recording and comparing it externally is on you. |

## Erasure versus append-only history

GDPR Article 17 says a record must be removable. Everything above says
history must not be. fhir-mariadb resolves this in one direction, explicitly:

```rust,ignore
let report = store.purge("Patient", "1234", &Audit::principal("ops:jane", "cli")
    .with_reason(Some("art-17 request #4471".into()))).await?;
// report.existed, report.versions_erased
```

The resource and every historical version are deleted, and one tombstone
row (`op = 'X'`, no `resource`) takes their place, chained from the tip that
was erased. `verify_audit` reports a tombstone as a recorded erasure, not a
break — a report that cried wolf on every lawful erasure is one an operator
learns to ignore.

Mechanically: `purge` sets a **session** variable, `@fhir_mariadb_erasure =
'on'`, on the one pooled connection it holds for the whole operation, then
deletes and clears the variable before the connection returns to the pool.
The trigger's escape hatch checks exactly that variable
(`ddl::append_only_triggers`), so a connection that never sets it cannot
delete a history row no matter what SQL it runs — but application code that
*does* set it, deliberately or by a bug that reuses this pattern, can. The
guard is a defence against accident and against ordinary code paths, not
against a determined attacker with equivalent access to this crate's own
internals.

Two limits to state before anyone relies on this:

- **The database is not the estate.** Backups, replicas, binlogs, and any
  downstream system that consumed the resource still hold it until they age
  out. `purge` is one step in an erasure plan, not the whole plan.
- **`REVOKE` is a second line, not the first.** The trigger stops ordinary
  code, migrations, and stray statements from touching history at all,
  independent of grants:

  ```sql
  REVOKE UPDATE, DELETE ON r5.* FROM 'fhir_mariadb_app'@'%';
  GRANT SELECT, INSERT ON r5.patient_history TO 'fhir_mariadb_app'@'%';
  -- and each other `_history` table
  ```

  With both in place, rewriting history requires a superuser deliberately
  disabling a trigger — an act that is itself visible in the server's own
  log, independent of anything this crate records.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
