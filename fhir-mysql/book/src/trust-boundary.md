# The trust boundary

fhir-mysql is a **library**, not a system, and not a server (`C0.17`,
`C0.18`). It cannot make a deployment compliant, and it must not be the reason
a deployment cannot be. This chapter states what the `fhir-mysql-store` crate
itself guarantees through its Rust API, and what anything built on top of it —
your own binary, or [`fhir-loco`](../../../fhir-loco/) — has to provide
instead. Earlier versions of this chapter described HTTP endpoints, CLI
commands, and flags that do not exist in this crate; those are corrected below
rather than merely flagged (audit **F-56**).

## What fhir-mysql-store guarantees

| Property | How | Spec |
| --- | --- | --- |
| Writes are transactional and versioned | one transaction per `put`, monotonic `version_id` under `SELECT … FOR UPDATE`, append-only history | R4.4, H5.4 |
| Reads see one consistent snapshot | `get`/`vread`/`history` run inside a transaction, not as separate statements (fixed under **F-21**, after a live torn-read was reproduced) | R4.5 |
| Every change records who made it | the audit envelope (`actor`, `actor_source`, `client`, `request_id`, `reason`) is written in the same statement as the history row | M3.15 |
| Every read can be recorded | `log_access`/`log_access_batch` write a disclosure row — but nothing calls them automatically; the caller decides when a read counts as a disclosure and must call them itself | PR12.5 |
| History cannot be quietly rewritten | `BEFORE UPDATE`/`BEFORE DELETE` triggers (`SIGNAL SQLSTATE '45000'`) refuse any write to a `*_history` table outside a declared erasure | M3.17 |
| History has a tamper-evident chain | SHA-256 **and** SHA3-256 per row, `verify_audit()` recomputes both and reports the first mismatch | M3.16 |
| Erasure leaves a verifiable hole | `purge` deletes a resource and its history and leaves a tombstone naming who, when, why, and the chain's last hash | M3.18 |
| PHI is encrypted in transit to the database | `FHIR_MYSQL_SSL_MODE` defaults to `VERIFY_IDENTITY`; `PREFERRED` is refused rather than silently downgraded | O10.7 |

**Not guaranteed, and not present in this crate at all** — check the
[conformance matrix](../../../spec/databases/conformance-matrix.md) before
assuming otherwise: optimistic concurrency (no `ETag`/`If-Match`, no
`put_audited`, no `expected_version`), conditional create or delete,
multi-operation transactions (`transact_audited`), `_include`/`_revinclude`
search expansion, an external chain witness or checkpoint (`M3.16c` is `—` for
this port — unlike `fhir-postgresql`, which has one, and `fhir-sqlite`, which
emits one but has no witness to compare it against), and `resign_history`
(`M3.16d` is likewise `—` here). None of these are HTTP concerns this book can
wave at a future `fhir-loco` chapter — they are gaps in the library itself.

## What the deployment must provide

fhir-mysql does **not** do these, and nothing that calls it should assume
otherwise:

| Obligation | Why it is not here |
| --- | --- |
| **Authentication** | This crate never sees a request; whatever sits in front of it (your own code, or `fhir-loco`) must establish identity and pass an `Audit` describing it. |
| **Authorization** | There is no scope check, no compartment restriction, no `meta.security` label enforcement, and no consent evaluation anywhere in `fhir-mysql-store`. Any caller with a `MySqlStore` handle can read and write anything the map describes. |
| **TLS to clients** | Out of scope entirely — this crate only speaks to MySQL, never to an HTTP client. |
| **Rate limiting, request shedding** | Not implemented here at any layer. A caller issuing unbounded concurrent `put`s gets unbounded concurrent connections up to the pool's own limit, then queues. |
| **Backup and retention** | Your engine's own tooling — `mysqldump`, binary-log replication, binlog replay for point-in-time recovery. `fhir-mysql` guarantees a consistent snapshot is a valid store; it does not schedule, verify, or retain one. See [Operations](operations.md#backup). |
| **Key management and at-rest encryption** | Filesystem, volume, or cloud-provider encryption. This crate stores no secrets and manages no keys of its own — the one exception, the chain-signing key, is supplied by the caller (below), never read from disk by this crate. |

## What neither this crate nor any layer above it provides yet

Stated rather than implied, so nobody discovers it during an audit:

- **Terminology validation.** Required-binding `CHECK` constraints only. No
  value-set expansion, no SNOMED/LOINC/ICD membership checks.
- **Profile conformance.** Base-specification structure only — not US Core,
  IPS, or any implementation guide.
- **FHIRPath invariants.** The `fhir` crate enforces a handful of the several
  hundred FHIRPath invariants in the specification, not all of them.
- **Referential integrity across resources.** FHIR permits dangling
  references and so does fhir-mysql (`M3.10`).

## Verifying the audit trail

There is no `verify-audit` command. `store.verify_audit()` recomputes every
resource's chain and returns `Vec<ChainBreak>` — empty means "nothing in
history has been altered since it was written":

```rust,ignore
let breaks = store.verify_audit().await?;
for b in &breaks {
    eprintln!("{}/{} v{}: {} broke ({})", b.rtype, b.id, b.version_id, b.algorithm, b.detail);
}
```

Rows written before the audit columns existed carry no stored hash and are
skipped rather than reported — treating them as tampering would be false, and
would train an operator to ignore the report. A deliberate erasure (below) is
likewise reported as what it is, not as a break.

### What each layer proves

Two layers exist here, not three — there is no chain witness in this port
(`M3.16c` = `—`), so a wholesale deletion of a resource's entire chain,
performed by someone who can also disable the append-only trigger, leaves
nothing behind to notice it happened.

| Layer | Stops | Does not stop |
| --- | --- | --- |
| SHA-256 + SHA3-256 digests, always computed | Careless or unaware modification: a migration, a stray `UPDATE` after the trigger is disabled, a row restored from the wrong backup. Two design families, so one line of cryptanalysis cannot take both. | An attacker who knows the pre-image format — it is public, and the digests are unkeyed, so anyone with write access can recompute matching ones. |
| `HMAC-SHA-256` tag, present **only if the caller supplies a signing key** | Forgery — producing a valid tag needs a key that lives in the application process, never in the database, so SQL write access alone is not enough. Live-verified: `verify_audit_accepts_a_clean_chain_and_catches_tampering` flips a byte and confirms all three signals (`sha256`, `sha3-256`, `hmac-sha256`) object. | A row being **deleted** rather than altered — there is nothing to check on a row that is gone, and no witness recorded off-box to notice its absence. |

**This crate does not read a chain key from the environment.** The
`FHIR_SQLITE_CHAIN_KEY`-named `KeyRing::from_env()` in the shared `chain.rs`
module exists (it is `fhir-postgresql` that calls it), but nothing in
`fhir-mysql-store` calls it. A signing key is wired in explicitly:

```rust,ignore
use fhir_mysql_store::chain::{ChainKey, KeyRing};

let key = ChainKey::from_hex("k1", &std::env::var("MY_CHAIN_KEY_HEX")?)
    .map_err(|e| /* ... */)?;
let store = MySqlStore::connect(dsn, map)
    .await?
    .with_chain_keys(KeyRing::new(vec![key]));
```

Whatever environment-variable convention a deployment wants — `FHIR_MYSQL_CHAIN_KEY`
is a reasonable one to adopt — is the caller's to define and read; this crate
only exposes the constructor. The key must never be granted to the MySQL
account itself: a key stored where the attacker already has write access
protects nothing. `ChainKey::from_hex` takes a 32-byte-minimum hex key and an
id; `KeyRing::new` can hold a retired key alongside the active one so rotation
does not invalidate history already tagged with the old id — `verify_audit`
reports a tag naming a key you no longer hold as *unverifiable*, distinctly
from a mismatch, so key rotation does not read as tampering.

## Erasure versus append-only history

GDPR Article 17 says a record must be removable. Everything above says history
must not be. These genuinely conflict, and fhir-mysql resolves it in one
direction, explicitly, through `store.purge`:

```rust,ignore
let report = store.purge("Patient", "1234", &audit).await?;
assert!(report.existed);
```

The resource and every historical version are deleted, and a **tombstone**
takes their place: `history()` afterward returns exactly one entry, `op ==
'X'`, `resource: None`, continuing the version numbering rather than
restarting it. So an erased record leaves a *verifiable hole* — an auditor can
still see that a chain existed and was deliberately terminated — rather than a
gap indistinguishable from a resource that never existed. `purge` on an
unknown id is a no-op (`existed: false`), not an error, and `verify_audit`
afterward reports no break: a lawful erasure is not tampering.
(`purge_erases_history_and_leaves_a_verifiable_hole` is the test this is drawn
from, live against MySQL 8.4.)

Two limits to state before anyone relies on this:

- **The database is not the estate.** Backups, replicas, binlogs, and any
  downstream system that consumed the resource still hold it until they age
  out. Promising erasure means having a plan for all of them; `purge` is one
  step in that plan, not the plan.
- **The guard is against accident, not against a privileged application.** The
  append-only trigger permits `DELETE` on a `*_history` table only while the
  session variable `@fhir_mysql_erasure` is `'on'`, which is how `purge` works
  internally — so any code with a live connection and write access could do
  the same thing deliberately. The trigger stops ordinary code, migrations,
  and stray statements; the tombstone and the access log are what make a
  deliberate erasure accountable rather than invisible. `purge` holds one
  connection for the whole operation specifically because the session
  variable is per-connection: setting it on a pooled connection and deleting
  on another would leave the flag set for whatever request runs next on that
  connection.

## Grants

The append-only trigger is enforcement the application cannot bypass by
accident. Belt and braces, restrict the application's MySQL account too —
adjust database and table names to whichever FHIR version you run:

```sql
REVOKE UPDATE, DELETE ON r5.* FROM 'fhir_mysql_app'@'%';
GRANT SELECT, INSERT ON r5.`patient_history` TO 'fhir_mysql_app'@'%';
-- repeat the GRANT for every `*_history` table the map generates
```

With both in place, rewriting history requires an account with `SUPER` or
`TRIGGER` privilege deliberately disabling the trigger first — an act that is
itself visible in the server's own log
(`verify_audit_accepts_a_clean_chain_and_catches_tampering` exercises exactly
this path: drop the trigger, tamper, then show `verify_audit` catches it).
