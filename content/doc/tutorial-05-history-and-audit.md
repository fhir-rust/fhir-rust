# Tutorial 5 — history and audit

Everything in this tutorial exists because the data is protected health
information, and because "the perimeter handles security" is not an answer to
"who looked at this patient".

Normative references: [`spec/05`](../spec/databases/05-versioning-and-history.md),
[`spec/03` M3.15–M3.18](../spec/databases/03-storage-model.md),
[`spec/12`](../spec/databases/12-trust-principal-and-audit.md).

## Versions

Every create, update, and delete increments `version_id` and appends a history
row (`H5.1`):

```rust
store.put(&patient, &Audit::cli()).await?;              // v1, op = C
store.put(&updated, &Audit::cli()).await?;              // v2, op = U
store.delete("Patient", "example", &Audit::cli()).await?; // v3, op = D

store.get("Patient", "example").await?;      // None — base rows are gone
store.vread("Patient", "example", 2).await?; // Some(...) — history remains
store.history("Patient", "example").await?;  // all three entries
```

Delete is **soft at the interface**: base and child rows go, history stays
readable (`H5.2`). A deleted id's history is still the record of what existed.

`vread` serves from history; `get` reconstructs the current version from the
relational tables. The two paths agreeing is a test-suite invariant, not a
runtime check (`H5.3`).

## Attribution

There is no default attribution, deliberately (`PR12.3a`):

```text
Audit::principal("dr.jones@clinic.example", "header:X-Fhir-Principal")
Audit::cli()
Audit::unattributed()

// with optional context
Audit::principal(actor, source)
    .with_client(Some("10.0.0.5".into()))
    .with_request_id(Some(req_id))
    .with_reason(Some("treatment".into()))
```

These land in the history row's audit envelope — `actor`, `actor_source`,
`client`, `request_id`, `reason` — written by the **same statement** that
appends the row, in the same transaction as the data change (`M3.15`).

That transactional coupling is the requirement. An audit record that can be lost
independently of the change it describes is not an audit record; it is a hint.

`actor_source` is what lets an auditor distinguish "the perimeter asserted this
identity" from "nobody did". An API that accepted a write with no attribution
and recorded something plausible would turn a deployment mistake into a
permanent false record — and a false attribution survives review in a way a
missing one does not.

## Disclosure logging

Regulators audit reads before writes. A store that records only mutations cannot
answer "who looked at this patient", so every read appends an access record
(`PR12.5`):

<!-- not compiled: `AccessRecord`'s fields are elided with `/* … */`; see the struct for the seven it needs. -->
```rust,ignore
store.log_access(&AccessRecord { /* … */ }).await?;
```

Three modes (`PR12.6`), and the default is the slow one on purpose:

| Mode | Behaviour |
| --- | --- |
| `sync` | the record commits before the result is returned — **the default** |
| `async` | batched through a bounded queue with a flush interval |
| `off` | permitted only when explicitly allowed, and logged loudly at startup |

`sync` is the default because the failure it prevents cannot be repaired
afterwards: a disclosure with no record is indistinguishable, later, from a
disclosure that never happened. A deployment that needs the throughput opts into
`async` knowingly; the reverse default would make every deployment silently
accept a loss window it never chose.

**In every mode, a disclosure that cannot be recorded fails closed.** The read
is refused, never served unlogged. A saturated queue is an error to the caller,
not a dropped record.

Four counters, and the distinction between them is the point:

| Counter | Means |
| --- | --- |
| `enqueued`, `written` | the healthy path |
| `refused` | reads turned away to keep the log honest — the system working under strain |
| `lost` | records the writer could not commit *after* the data was served — **an incident** |

Non-zero `lost` means disclosures happened that the log does not show. Queue
depth is derived from these rather than tracked separately, so it can never
report a value the counters contradict.

## The hash chain

Each history row carries a digest over its canonical serialization concatenated
with the previous version's digest, per resource id (`M3.16`).

```rust
let breaks = store.verify_audit().await?;
for b in &breaks {
    println!("{} {} v{} broke under {}", b.rtype, b.id, b.version_id, b.algorithm);
}
```

### Two algorithms, two families

SHA-256 **and** SHA3-256 (`M3.16a`). The point is family diversity, not digest
length: MD5 and SHA-1 both fell to the same line of cryptanalysis, and both are
Merkle–Damgård. SHA-256 is Merkle–Damgård; SHA-3 is a sponge. A clinical record
may be retained for decades — longer than anyone can promise a single
construction will stand.

Both are FIPS-approved (180-4, 202), and verification reports each separately
rather than reducing them to one verdict, so a reader can rely on whichever
their regime recognises.

### What the unkeyed chain actually buys

Stated honestly, because the temptation to over-claim here is strong:

**It detects careless or unaware modification** — a migration, a stray `UPDATE`,
a row restored from the wrong backup. **It supports an external witness**: a
chain head recorded off-box makes truncation and wholesale rewriting detectable
even against an attacker who can recompute digests.

**It does not stop an informed attacker with write access.** The digests are
unkeyed over a published pre-image, so anyone who can write the row can compute
a correct digest for what they wrote.

### The keyed tag, which is the actual fix

`HMAC-SHA-256` over the same pre-image, stored as `<key-id>:<hex>` (`M3.16b`).
The key lives in the process and **never in the database**. That is the whole
idea: a key stored where the attacker already has write access protects nothing.

Rules that are easy to get wrong:

- **Use a file, not an environment variable.** Environment is visible in
  `/proc/<pid>/environ`, survives into crash dumps, is reported by
  orchestrators, and is inherited by every child process. A file is none of
  those, and is what Kubernetes secrets and systemd credentials already produce.
- **A key file readable by group or other is refused**, not warned about. A
  warning is read once at startup; the file stays readable for the life of the
  deployment.
- **Generate with `chain-key-new`**, never `openssl rand -hex 32 > key` — the
  shell applies the umask (commonly `022`, producing a file that must be
  refused) and leaves the secret world-readable in the window before `chmod`.
- **The key id travels with the tag**, so rotation is additive. Retired keys stay
  loadable. Without the id, rotating would invalidate all history at once —
  indistinguishable from mass tampering.
- **Only a tag mismatch is a finding.** A missing tag, a tag naming a key this
  process does not hold, and a malformed tag are each reported as what they are.
  Reporting a key-distribution problem as forgery burns an incident response.

### The checkpoint, which catches what the tag cannot

A MAC proves a row was not rewritten. It says nothing about a row that is
**gone** — and a chain missing its most recent version verifies perfectly,
because nothing left behind refers to what was removed.

> **`fhir-postgresql` only**, and this tutorial's other examples are written
> against `fhir-sqlite`. On any port but PostgreSQL the line below does not
> compile.

```rust,ignore
let checkpoint = store.chain_witness().await?;
```

One value covering every chain head in the namespace, such that it changes if
any chain gains a version, loses one, or has its head altered (`M3.16c`).
Checkpoints are also emitted as INFO lines on a dedicated `audit_checkpoint`
log target — they carry no PHI, only counts and digests, so they can be retained
far longer than ordinary application logs and shipped somewhere patient data
must not go.

**The value is only a witness if it lands somewhere the database cannot reach.**
Logs shipped off-host qualify; logs written to a table in the same database, or
to a disk the same compromised account can rewrite, do not. The library cannot
enforce that, and does not pretend to.

### Never backfill a chain

`init --upgrade` adds new digest columns but does **not** backfill them
(`M3.16e`). The rows are recoverable and the digests could be computed — but a
chain assembled after the fact attests only that the rows look consistent *now*,
which is exactly what an attacker who rewrote them would produce. Verification
reports the new chain as beginning where its first digest appears.

Manufacturing evidence is worse than admitting its absence.

## Append-only, enforced by the database

`init` emits a `BEFORE UPDATE OR DELETE` trigger on every history table that
raises an exception (`M3.17`). The documentation also describes the
`REVOKE UPDATE, DELETE` grants a deployment applies to the application role.

Both, not either. A revocable grant and an in-database guard fail differently,
and the point is defence that survives an application bug.

## Erasure

GDPR Art. 17 is the one sanctioned deletion, and it is explicit (`M3.18`):

```rust
// The reason travels on the Audit, not as a separate argument: it is recorded
// on the tombstone the same way it is recorded on every other write, so an
// erasure cannot be attributed differently from a normal change.
let audit = Audit::cli().with_reason(Some("GDPR Art.17 request #4711".into()));
store.purge("Patient", "example", &audit).await?;
```

History rows are removed and replaced by a **tombstone** recording who purged
what, when, why, and the chain it terminated — so an erased record leaves a
*verifiable hole* rather than a silent one. It requires an explicit erasure
acknowledgement, logs at warn level, and emits a checkpoint immediately
afterwards, which is what separates a recorded intentional removal from the
unrecorded kind.

## Availability by port

| | pg | sqlite | mysql | mariadb |
| --- | :-: | :-: | :-: | :-: |
| history, vread, audit envelope | • | • | • | • |
| `verify_audit` | • | • | • | • |
| `purge` | • | • | • | • |
| `emit_checkpoint` | • | • | — | — |
| `chain_witness` | • | — | — | — |
| `resign_history` | • | — | — | — |

`fhir-mssql` and `fhir-oracle` have no store at all. And the audit **tests** —
`audit.rs`, `redaction.rs`, `concurrency.rs` — exist only in
`fhir-postgresql`, so the guarantees on this page are `?` rather than `•`
elsewhere in the [conformance matrix](../spec/databases/conformance-matrix.md). Shared
code is not evidence that this port runs it.

One further caveat specific to PostgreSQL: it still derives its chain pre-image
in SQL (`(($1::text)::jsonb)::text`), so a PostgreSQL chain cannot be verified
by any other port ([`audit.md`](../spec/databases/audit.md) **F-07**).

## Next

- [Tutorial 6 — porting to a new database](tutorial-06-porting.md)
- [The trust boundary](trust-boundary.md) — the one table version of all this
