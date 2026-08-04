# The trust boundary

fhir-oracle is a component, not a system. It cannot make a deployment
compliant, and it must not be the reason a deployment cannot be. This
chapter states, in one place, what fhir-oracle guarantees and what the
deployment around it has to provide — because a boundary nobody can point at
is not a boundary (`PR12.8`).

It is also a **library**, not a server (`C0.17`, `C0.18`). Every guarantee
below is something the Rust API does when you call it, not something an HTTP
server enforces on your behalf — there is no `fhir-oracle serve`, no
`--bind`, no `--trust-proxy`. If those concepts matter to your deployment,
they belong to whatever process embeds this store.

## What fhir-oracle guarantees

| Property | How | Spec |
| --- | --- | --- |
| Writes are transactional and versioned | one transaction per write, `SELECT … FOR UPDATE` row lock, monotonic `version_id`, append-only history | `R4.4`, `H5.1`, `H5.4` |
| Every change records who made it | audit envelope columns written by the same statement as the change | `M3.15` |
| Every read *can* be recorded | `log_access` writes a disclosure row to `fhir_oracle_access_log` — calling it is the caller's responsibility, this library does not intercept reads to do it automatically | `PR12.5` |
| History cannot be quietly rewritten | per-resource SHA-256 **and** SHA-3-256 chain, plus a database trigger refusing `UPDATE`/`DELETE` on history outside a declared erasure | `M3.16`, `M3.17` |
| Nothing is silently dropped | unknown elements rejected during shred, not swallowed | `D12` |

## What fhir-oracle does **not** guarantee, and why

| Property | Why not |
| --- | --- |
| **Reads see one consistent snapshot (`R4.5`)** | **Open, confirmed gap.** The one candidate mechanism this port considered, `SET TRANSACTION READ ONLY`, fails outright on this engine (`ORA-01466` on any session that has run DDL) — see [Operations](operations.md). `get` currently reads with no snapshot-isolation protection under concurrent writers at all. |
| Authentication | Identity belongs to whatever calls this library. `Audit::from_principal(...)` records who the caller says did something; it verifies nothing. |
| Authorization | There is no scope check, no compartment restriction, no `meta.security` label enforcement, and no consent evaluation. |
| Transport encryption to the database | Undecided on this engine (`O10.7`, `M14.22`) — the live test suite connects over a plain local port with no encryption configured either way. |
| Terminology validation | Required-binding `CHECK` constraints only. No value-set expansion, no SNOMED/LOINC/ICD membership checks. |
| Profile conformance | Base-specification structure only — not US Core, IPS, or any implementation guide. |
| Referential integrity across resources | FHIR permits dangling references and so does this store (`M3.10`). |

## Attributing a write

Every write takes an `Audit`:

```rust,ignore
use fhir_oracle_store::Audit;

// A write attributed to a principal your own perimeter vouched for —
// this library does not authenticate the string, it only records it.
let audit = Audit::from_principal("dr-who", "header:X-Fhir-Oracle-Principal");

store.put(&patient_json, &audit).await?;
```

`Audit::cli()` attributes a write to whoever is running the current OS
process (`$USER`/`$USERNAME`) — useful for scripts and tests, not for a
deployment with real callers.

## Recording a disclosure

Reads are not audited automatically — call `log_access` yourself, once per
interaction that returned or could have returned patient data:

```rust,ignore
store.log_access(&fhir_oracle_store::AccessRecord {
    audit: Audit::from_principal("dr-who", "header:X-Fhir-Oracle-Principal"),
    interaction: "read".into(),
    rtype: Some("Patient".into()),
    id: Some("example".into()),
    version_id: Some(1),
    outcome: "ok".into(),
    result_count: None,
}).await?;
```

## Verifying the audit trail

```rust,ignore
let breaks = store.verify_audit().await?;
assert!(breaks.is_empty(), "{breaks:?}");
```

It recomputes every resource's chain — both the SHA-256 and SHA-3-256
families — and returns every break it finds; an empty vector means a clean
chain. Rows written before the audit columns existed carry no hash and are
reported as the point a chain begins, not as tampering.

An optional HMAC signing key stops one specific thing a plain hash chain
cannot: without a key, anyone with `INSERT` access to the history tables can
recompute matching digests, so a bare hash chain proves only that nothing
changed *by accident*. Supplying one changes that:

```rust,ignore
use fhir_oracle_store::chain::{ChainKey, KeyRing};

let hex = std::env::var("FHIR_ORACLE_CHAIN_KEY_HEX").expect("set a 32-byte hex key");
let keys = KeyRing::new(vec![ChainKey::from_hex("k1", &hex)?]);
let store = OracleStore::connect(user, password, connect_string, map)
    .await?
    .with_chain_keys(keys);
```

**`KeyRing::from_env()` exists but do not use it here — it silently reads
the wrong variable.** It is shared code (`fhir_store::chain`, identical
across all six ports) and hardcodes `FHIR_SQLITE_CHAIN_KEY`/`_ID`/
`_RETIRED` regardless of which port calls it — there is no
`FHIR_ORACLE_CHAIN_KEY` support in it. Setting a variable by that name
compiles, looks correct, and does nothing; `from_env()` returns an empty
key ring and every chain link is an unkeyed hash. Construct the ring
explicitly, as above, until this is fixed upstream — see `audit.md`
**F-70**.

**What this port does not have**, unlike `fhir-postgresql`: `chain_witness`
(an off-box, recomputable checkpoint you can compare against later) and
`resign_history` (rotating the countersign after a key change). Both exist
only on the reference port today.

## Erasure versus append-only history

GDPR Article 17 says a record must be removable. The guarantee above says
history must not be quietly rewritten. These genuinely conflict, and this
port resolves it in one direction, explicitly:

```rust,ignore
let report = store.purge("Patient", "1234", &audit).await?;
// report.existed, report.versions_erased
```

Every historical version of the resource is deleted, and a **tombstone**
takes their place. `verify_audit` treats a tombstone as a recorded erasure,
not a chain break — a tamper-evidence report that cries wolf on every
lawful erasure is one an operator learns to ignore, at which point it
detects nothing (`purge_erases_history_and_leaves_a_verifiable_hole`, live).

Two limits to state before anyone relies on this:

- **The database is not the estate.** Backups, standbys (Data Guard), and
  any downstream system that consumed the resource still hold it until they
  age out. `purge` is one step in an erasure plan, not the plan.
- **The guard is against accident, not against a determined operator.** The
  append-only trigger permits `DELETE` only inside a window where
  `DBMS_APPLICATION_INFO`'s `CLIENT_INFO` names the erasure — which is how
  `purge` itself works, so application code with direct SQL access could do
  the same. The trigger stops ordinary code, migrations, and stray
  statements from touching history at all; the tombstone and the access log
  are what make a *deliberate* erasure accountable rather than invisible.

## Grants

The append-only trigger is enforcement the application cannot bypass from
ordinary SQL. Belt and braces, restrict the application role too:

```sql
REVOKE UPDATE, DELETE ON "R5"."patient_history" FROM some_readonly_role;
-- and every other <resource>_history table
```

With both in place, rewriting history requires a DBA deliberately disabling
a trigger — an act that is itself visible in the server's own audit trail.
