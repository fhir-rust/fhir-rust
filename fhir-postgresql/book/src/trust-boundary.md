# The trust boundary

fhir-postgresql is a library, not a system, and a narrower one than earlier
versions of this chapter suggested: it is a `Store` you call from Rust, not a
server with endpoints, headers, or flags. It cannot make a deployment
compliant, and it must not be the reason a deployment cannot be. This
chapter states, in one place, what calling `Store` genuinely guarantees, what
is offered as a mechanism the caller must actually invoke, and what the
surrounding application has to provide entirely on its own (spec `PR12.8`).

## What `Store` guarantees unconditionally

These hold on every call, with no setup beyond `Store::connect` — verified
either live (the store's own test suite: `concurrency.rs`, `audit.rs`,
`redaction.rs`) or by reading the code path directly.

| Property | How | Spec |
| --- | --- | --- |
| Writes are transactional and versioned | one transaction per write, monotonic `version_id`, append-only history (`M14.16`) | R4.4, H5.1 |
| Reads see one consistent snapshot | every multi-table read runs in one `REPEATABLE READ READ ONLY` transaction (`M14.15`) | R4.5 |
| Conditional interactions are race-free | criteria hashed into a `pg_advisory_xact_lock`, match and write share one transaction (`conditional_create`/`conditional_delete`) | A7.10 |
| Optimistic concurrency is enforced, including inside a transaction Bundle | `expected_version` checked against the locked row; mismatch is `StoreError::Conflict`, not a silent overwrite; `transact_audited` applies the same check per op | D11, A7.9 |
| Every write records who made it | `put`/`delete` without an explicit `Audit` still write one, as `actor = "unauthenticated"` — the column is never blank | M3.15 |
| History cannot be quietly rewritten | per-resource SHA-256 **and** SHA-3-256 chain, plus a database trigger refusing `UPDATE`/`DELETE` on `*_history` outside a declared erasure | M3.16, M3.17 |
| PHI is encrypted in transit to the database, by default | `PGSSLMODE` defaults to `require` — verifies certificate and hostname (`M14.27`, **F-17**) | O10.7 |
| Client-safe errors are a distinct Rust type from internal diagnostics | `StoreError::Unsupported`/`Conflict` name what the caller sent; `StoreError::Pg`/`Other` may name schema or stored values and are meant for logs, not for echoing back | A7.11 |
| Unrecognized input is rejected, not silently dropped | `shred` returns `ShredError::At{path, msg}` naming the offending element path rather than ignoring it | D12 |

## What `Store` offers but does not do for you

These need the caller to actually call them. Nothing above implies they
happen automatically, and assuming otherwise is the gap this section exists
to close.

- **Disclosure logging (`PR12.5`) is opt-in, not automatic.** `get`,
  `search`, `history`, and `vread` do **not** call `log_access` internally —
  grep `crates/fhir-postgresql-store/src/lib.rs` and `log_access` appears
  exactly once, at its own definition. A caller that wants "every read is
  recorded" has to call `store.log_access(&AccessRecord { .. })` (or
  `log_access_batch`) itself, after every read it wants attested. An earlier
  version of this chapter listed disclosure logging as something `Store`
  guarantees; it does not — it makes the guarantee cheap to build, not free.
- **The hash chain is unkeyed unless you key it**, and the *correct*
  environment variable to do so, even on this engine, is
  `FHIR_SQLITE_CHAIN_KEY` — see [Operations](operations.md#keying-the-chain)
  for why, and for `with_chain_keys` as the more robust route. Unkeyed, the
  chain still detects careless modification; it does not stop a forger who
  has SQL write access and knows the public pre-image format.
- **Erasure (`purge`) is a call you make, not a policy the library enforces
  for you.** See [Operations](operations.md#erasure) for what it does and
  its two hard limits (the database is not the estate; the append-only guard
  stops accidents, not a determined application).
- **Checkpoints are not automatic on a schedule.** `emit_checkpoint(reason)`
  is a call your code makes (`purge` calls it once, for you, after an
  erasure); there is no background timer in this crate.

## What the deployment must provide entirely on its own

Nothing here does these, and an application built on `Store` that skips them
is not safe to put patient data behind:

| Obligation | Why it is not here |
| --- | --- |
| **Authentication** | `Audit::principal(actor, source)` records an identity; it never establishes one. `Store` trusts whatever the caller passes. |
| **Authorization** | There is no scope check, no compartment restriction, no `meta.security` enforcement, and no consent evaluation anywhere in this crate. Any caller with a `Store` value can read and write anything the schema holds. |
| **A network surface at all** | This crate opens a `tokio-postgres` connection to PostgreSQL; it does not listen on a socket. TLS *to clients*, rate limiting per identity, and request-size limits belong to whatever sits in front — `fhir-loco` or your own service. |
| **Wiring disclosure logging to every read** | The mechanism exists (`log_access`); calling it on every `get`/`search`/`history`/`vread` is the caller's job, as above. |
| **Backup scheduling and retention policy** | Plain PostgreSQL (`pg_dump`, PITR) applies unchanged (`M14.28`); nothing here schedules anything. |
| **Key management and at-rest encryption** | Filesystem, volume, or cloud-provider encryption, and wherever you keep the chain-signing key file. `Store` stores no secrets itself. |

## What neither this crate nor `fhir` provides yet

Stated rather than implied, so nobody discovers it during an audit:

- **Terminology validation.** No `CHECK` constraint, enum type, or anything
  else in `ddl.rs` enforces a required value-set binding — `gender` is a
  plain `text` column, not a constrained one. No SNOMED/LOINC/ICD membership
  checks exist anywhere in this port.
- **Profile conformance.** Base-specification structure only — not US Core,
  IPS, or any implementation guide.
- **Referential integrity across resources.** FHIR permits dangling
  references and so does this schema (`M3.10`) — child tables carry no
  foreign key to the resource they reference, only to their own parent row.

## What each layer of the hash chain proves

Three layers, and they stop different things. Conflating them is how a
deployment ends up believing it has protection it does not.

| Layer | Stops | Does not stop |
| --- | --- | --- |
| SHA-256 + SHA3-256 digests | Careless or unaware modification: a migration, a stray `UPDATE`, a row restored from the wrong backup. Two design families, so one line of cryptanalysis cannot take both. | An attacker who knows the pre-image format — it is public, and the digests are unkeyed, so they can recompute them. |
| `HMAC-SHA-256` tag (needs a configured key — see above) | Forgery. Producing a valid tag needs a key held outside the database, never written to it, so SQL write access alone is not enough. | A row being **deleted** wholesale. |
| Chain witness, recorded off-box (`chain_witness`) | Truncation and wholesale deletion. | Anything, if the witness is stored somewhere the same attacker can also reach. |

Without a key, a hash chain proves only that nothing changed *by accident*:
anyone who can write the rows can also write matching digests. The witness
closes a different gap than the tag does — a chain missing its last version
still verifies perfectly on its own, because nothing left behind refers to
what was removed, which is exactly why a value recorded outside the database
matters. See [Operations](operations.md#the-hash-chain-verifying-witnessing-keying-re-signing)
for the calls themselves.

Rotation is additive by design: each tag records the key id that signed it,
so retiring a key does not invalidate history signed under it — until that
key is actually dropped from the `KeyRing`, at which point those rows become
*unverifiable*, which `verify_audit` reports as exactly that, not as
tampering. Reporting a key-distribution gap as a forgery would burn an
incident response on nothing.

## Grants, as a defense-in-depth measure

The append-only trigger is enforcement the calling code cannot bypass without
setting the erasure session variable or disabling the trigger outright —
both are DBA-visible acts. Restricting the connecting role's own SQL grants
is a second, independent layer, at the deployment's discretion — this crate
does not create or manage database roles itself:

```sql
REVOKE UPDATE, DELETE ON ALL TABLES IN SCHEMA r5 FROM your_app_role;
GRANT SELECT, INSERT ON r5.patient_history TO your_app_role;  -- and each *_history table
```

With both in place, rewriting history needs a role with `ALTER TABLE`
privilege deliberately disabling a trigger — an act that is itself visible in
the server log, not merely a bug in application code.
