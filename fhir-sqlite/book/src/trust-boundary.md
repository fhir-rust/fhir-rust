# The trust boundary

fhir-sqlite is a library, not a system. It cannot make a deployment
compliant, and it must not be the reason a deployment cannot be. This chapter
states, in one place, what `fhir-sqlite` actually guarantees — checked
against `crates/fhir-sqlite-store/src/sqlite.rs` and its tests, not against
what a server built on it might advertise — and what the code around it has
to provide (spec `PR12.8`).

This chapter previously described a `fhir-sqlite serve` process: bind
addresses, a trusted-proxy header scheme, `/metrics`, an `--audit-mode` flag.
None of that exists in this crate (`C0.17`, `C0.18`). If you are building a
server over this library, that surface belongs to it — see
[`fhir-loco`](../../../fhir-loco/) — not to `fhir-sqlite`.

## What `fhir-sqlite` guarantees, unconditionally

These happen inside `put`, `delete`, and `purge` themselves, in the same
`BEGIN IMMEDIATE` transaction as the data change. You cannot call them
without getting this.

| Property | Mechanism | Spec |
| --- | --- | --- |
| Writes are transactional and versioned | one `BEGIN IMMEDIATE` per write; `version_id` comes from the history tip, not the base row, so it is correct even for a resource that was deleted and recreated | `H5.4`, `R4.4` |
| History is append-only | `BEFORE UPDATE`/`BEFORE DELETE` triggers on every `*_history` table `RAISE(ABORT, …)` unless a one-row erasure-flag table exists for the duration of the transaction (`M14.22`) | `M3.17` |
| Every change records who made it | the same `INSERT` that appends the history row carries `actor`, `actor_source`, `client`, `request_id`, `reason` from the `Audit` you passed | `M3.15` |
| History cannot be quietly rewritten | a two-family hash chain per resource, SHA-256 **and** SHA3-256, each linking to the previous row's digest; `verify_audit()` recomputes both and reports a `ChainBreak` naming the resource, version, and which algorithm disagreed | `M3.16`, `M3.16a` |
| A multi-statement read sees one snapshot | `get` opens a **deferred** (read-only) transaction before its first `SELECT`, pinning a WAL snapshot for the whole reconstruction. This was not theoretical: before it existed, a reader on a second connection observed one resource's `patient_name` from version 8 next to its `patient_telecom` from version 12 — a torn read, found and fixed as audit **F-21** | `R4.5` |
| Optimistic concurrency | `put_audited(resource, expected_version, audit)` checks the stored version first and returns `StoreError::Conflict { expected, found }` on a mismatch, rather than writing over it — the caller decides how that becomes an HTTP 412, since this library does not speak HTTP | `D11` |
| Conditional writes cannot race each other | `conditional_create_audited`/`conditional_delete_audited` hold an in-process lock across their search-then-write, and SQLite's single-writer lock (`BEGIN IMMEDIATE`) extends that guarantee across processes too | `A7.10` |
| Erasure leaves a verifiable hole, not a gap | `purge` deletes the base row and every history row for one resource, then inserts a single `'X'`-op tombstone whose `prev_hash` records the chain tip that was erased — proof something was there and was deliberately removed, distinct from a resource that never existed | `M3.18` |

## What `fhir-sqlite` provides but does not do for you

These are real, tested capabilities — but calling them is **your**
responsibility, not something `get`/`search`/`put` triggers on your behalf.
Getting this distinction wrong is the easiest way to end up with a boundary
that looks stronger than it is.

- **Disclosure logging is opt-in per call.** `log_access(&AccessRecord)`
  writes one row to `fhir_sqlite_access_log`, and `log_access_batch` writes
  several — but **`get` and `search` do not call it themselves.** A `Patient`
  read through `store.get(...)` alone leaves no disclosure row. If your
  application (or the server layer in front of this library) needs "every
  read is recorded" to be true, it must call `log_access` itself, on every
  read path, including reads that found nothing — `outcome: "not-found"` is
  still a disclosure attempt worth recording, and the test suite records it
  that way.
- **Keyed (HMAC) chain tags are opt-in per store.** Unkeyed, the SHA-256/SHA3-256
  chain stops *careless* modification — anyone who can write SQL can also
  recompute an unkeyed digest for what they wrote. A keyed chain stops
  *forgery*: producing a valid tag needs a key that is never written to the
  database. Load one and attach it before opening writes:

  ```rust,ignore
  use fhir_sqlite_store::chain::KeyRing;

  // Reads FHIR_SQLITE_CHAIN_KEY (hex) / FHIR_SQLITE_CHAIN_KEY_ID, and
  // FHIR_SQLITE_CHAIN_KEYS_RETIRED (id=hex,id=hex,…) for keys that still
  // verify but no longer sign.
  let keys = KeyRing::from_env()?;
  let store = SqliteStore::open("clinic.sqlite", map).await?
      .with_chain_keys(keys);
  ```

  Rotation is additive: each tag records which key id signed it
  (`k1:9f86d0…`), so retiring a signing key does not invalidate rows it
  already signed, as long as that key id is still listed as retired. Drop it
  entirely and those rows become **unverifiable**, which `verify_audit`
  reports as exactly that — not as tampering. This is confirmed by
  `rows_signed_with_an_unheld_key_are_not_called_tampering` in the test
  suite: a verifier holding the wrong key must say "I cannot check this,"
  never "this was altered."

- **Chain verification is on demand, not continuous.** `verify_audit()`
  recomputes every resource's chain when you call it; nothing runs it on a
  schedule. `emit_checkpoint(reason)` calls `verify_audit` and logs one
  `tracing` line on the `audit_checkpoint` target — `INFO … chain checkpoint:
  verified` when clean, `ERROR` with a break count or an error otherwise. It
  is a log line, not a table row, and deliberately so: a checkpoint's value
  comes from living somewhere the database itself cannot rewrite. Nothing in
  this crate calls `emit_checkpoint` on an interval; if you want that, your
  application schedules it.

## What each layer of the chain actually stops

Conflating these is how a deployment ends up believing it has protection it
does not.

| Layer | Stops | Does not stop |
| --- | --- | --- |
| SHA-256 + SHA3-256 digests (unkeyed) | Careless or unaware modification: a migration, a stray `UPDATE`, a row restored from the wrong backup. Two unrelated design families (Merkle–Damgård vs. sponge), so one cryptanalytic advance cannot take both. | An attacker with SQL write access who knows the pre-image format — it is public and the digests are unkeyed, so they can recompute a matching one. |
| `HMAC-SHA-256` tag (opt-in, see above) | Forgery: producing a valid tag needs the key, which is never written to the database. | A row being **deleted** outright — a tag proves nothing about a row that is simply gone. |
| An external witness | Truncation and wholesale deletion — a chain missing its last version verifies perfectly, because nothing left behind refers to what was removed. | **Nothing here does this yet.** There is no `chain_witness` function in this port (`M3.16c`: sqlite emits a checkpoint but has no dedicated witness digest), and no `resign_history` (`M3.16d`). Both are `—` in the [conformance matrix](../../../spec/databases/conformance-matrix.md). `emit_checkpoint`'s log line is the closest thing available today, and it only helps if it ships somewhere the database cannot reach. |

## What this library does not do at all

- **Authentication and authorization.** There is no perimeter concept inside
  a library — no header parsing, no proxy trust, no scope check. `Audit`
  simply records whatever actor string you construct it with:
  `Audit::unattributed()` (recorded as `"unauthenticated"`, deliberately, so
  "we do not know who did this" is itself visible rather than blank),
  `Audit::cli()` (`"cli:$USER"`), or `Audit::principal(actor, source)` for an
  identity your own code has already verified. Verifying that identity is
  entirely your application's job.
- **Transport encryption.** There is no connection to encrypt — a store is a
  file path (`O10.7` does not apply; see [Operations](operations.md) for what
  replaces it: file permissions and, if you need it, disk or SQLCipher
  encryption).
- **Rate limiting, request shedding, or any request-shaped concept at all.**
  This crate has no notion of a request. `write_gate` serializes conditional
  writes; it is not a throttle.
- **Terminology validation.** Required-binding `CHECK` constraints only — no
  value-set expansion, no SNOMED/LOINC/ICD membership checks.
- **Profile conformance.** Base-specification structure only, not US Core,
  IPS, or any implementation guide.
- **Referential integrity across resources.** FHIR® permits a dangling
  reference and so does this store (`M3.10`) — `subject_ref_id` is not a
  foreign key to another resource type's table.

## Two limits on erasure, stated plainly

- **The database is not the estate.** Backups, replicas, and anything
  downstream that already consumed a resource still hold it until they age
  out on their own schedule. `purge` is one step in an erasure plan, not the
  plan.
- **The append-only trigger guards against accident and ordinary code, not
  against a deliberate attacker with direct database access.** `purge`'s
  escape hatch is a one-row table the trigger checks for; any SQL running
  with the same privileges as this library could insert that row and delete
  history too. What makes a *legitimate* erasure accountable is not the
  trigger — it is that `purge` always leaves the tombstone and always runs
  under an `Audit`, so a deliberate bypass leaves no matching tombstone and
  is the anomaly `verify_audit` and a routine review would surface.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
