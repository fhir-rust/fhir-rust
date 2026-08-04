# The trust boundary

fhir-mssql is a component, not a system. It is a **library** — there is no
server, no CLI, and no network surface of its own (`C0.17`, `C0.18`); a
caller such as `fhir-loco` links it, holds an `MsSqlStore`, and is
responsible for everything an HTTP deployment needs that a Rust struct
cannot provide (identity, TLS to clients, rate limiting). This chapter
states, in one place, what the **store crate itself** guarantees when
called correctly, and what it does not — because a boundary nobody can
point at is not a boundary (spec `PR12.8`), and because several of the
claims this chapter used to make described a server that does not exist in
this workspace.

## What `fhir-mssql-store` guarantees

| Property | How | Spec |
| --- | --- | --- |
| Writes are transactional and versioned | one `BEGIN`/`COMMIT TRANSACTION` per write, monotonic `version_id`, append-only history | R4.4, H5.1 |
| Writers for one resource are serialized | `WITH (UPDLOCK, ROWLOCK)` on the base row before the chain-tip read, held to commit/rollback; 8 of 8 racing writers get distinct consecutive versions, live-verified | H5.4, M14.26 |
| `get` sees one consistent snapshot | `SET TRANSACTION ISOLATION LEVEL SNAPSHOT`, backed by `ALLOW_SNAPSHOT_ISOLATION` on a dedicated database — see below | R4.5, M14.25 |
| Every change records who made it | the audit envelope (`actor`, `actor_source`, `client`, `request_id`, `reason`) is written by the same statement that writes the history row | M3.15 |
| Every read can be recorded | `log_access`/`log_access_batch` write to `[fhir_mssql_access_log]`; **calling them is the caller's responsibility** — `get`/`search` do not call them implicitly | PR12.5 |
| History cannot be quietly rewritten | `CREATE OR ALTER TRIGGER … INSTEAD OF UPDATE/DELETE`, raising `THROW 50000`, on every history table | M3.16, M3.17 |
| Diagnostics do not leak stored content | `StoreError::Unsupported` (client-safe, names only what the caller sent) is a separate variant from `StoreError::Other`/`Db` (operator diagnostics); `redaction.rs` asserts a rejected write never echoes the value | A7.11, T11.7 |
| Nothing is silently dropped on write | the shredder rejects elements the map does not define, naming the path, rather than dropping them | R4.7 |

Two rows a reader of the PostgreSQL original would expect and will not find
here: **there is no optimistic concurrency** (`If-Match`/`ETag` comparison
against an `expected_version`) and **no conditional create/delete**
anywhere in this crate. `put`/`delete` always replace or remove the current
version; a caller wanting compare-and-swap semantics has to build it on top
today.

### `R4.5`, in more detail: the fix took two tries

`get` reads a base table and every child table as separate statements. This
engine's *default* isolation, `READ COMMITTED`, gives each of those
statements its own view of the latest committed data — unlike PostgreSQL's
or MySQL's `REPEATABLE READ` default, a bare transaction wrap around them is
**not** the same claim as snapshot isolation. `tests/concurrency.rs`'s
`reads_never_tear_under_concurrent_writes` reproduced this live: a reader
observed `active` from one concurrent write interleaved with
`name`/`telecom` from another.

The first fix tried, `READ_COMMITTED_SNAPSHOT` at the database level, is
the answer that reads right in the documentation and isn't: run live, it did
**not** stop the torn read, because RCSI gives each *statement* its own
snapshot, not the whole *transaction* one. What actually works is `get`
issuing `SET TRANSACTION ISOLATION LEVEL SNAPSHOT` immediately before
`BEGIN TRANSACTION`, which requires `ALLOW_SNAPSHOT_ISOLATION` enabled at
the database level — and that in turn requires a database this port can run
`ALTER DATABASE` against at all: a DSN that lands in `master` fails, because
SQL Server refuses the option there outright. **The DSN a caller supplies
to `connect` must name a database** (`database=fhir_mssql`; `scripts/db.sh
up` prints one already provisioned with both options enabled).
`SET TRANSACTION ISOLATION LEVEL` is session-, not transaction-scoped, so
`get` resets it to `READ COMMITTED` before the connection goes back to the
pool — see
[Operations](operations.md#session-state-hygiene-on-a-pooled-connection).

## What the audit chain actually proves

Every history row carries a SHA-256 link and a SHA-3-256 link over the same
pre-image (`chain::preimage`/`link`, shared across all six ports, unkeyed
by default) — two independent design families, so one line of
cryptanalysis cannot take both. `store.verify_audit()` recomputes both for
every resource and returns `Vec<ChainBreak>`, one entry per algorithm per
break, never merged.

**Unkeyed, a hash chain proves only that nothing changed by accident.**
Anyone with SQL write access can also compute matching digests, since the
pre-image format is public. Keying it closes that gap:

```rust,ignore
let key = fhir_mssql_store::chain::ChainKey::from_hex("k1", &hex_key)?;
let keys = fhir_mssql_store::chain::KeyRing::new(vec![key]);
let store = MsSqlStore::connect(dsn, map).await?.with_chain_keys(keys);
```

`with_chain_keys` is the whole API surface for this — called once, at
construction. There is **no environment-variable key loading in this
crate**: the shared `fhir-store::chain` module does carry `ChainKey::from_env`/
`KeyRing::from_env` helpers, but they read `FHIR_SQLITE_CHAIN_KEY`-named
variables (literally that name — the module is byte-identical across all
six ports and was written for `fhir-sqlite`), and nothing in
`fhir-mssql-store` calls them. A caller who wants environment-driven keys
has to read that variable and build the `KeyRing` itself.

Keyed rows carry an `HMAC-SHA-256` tag (`[row_mac]`, `<key-id>:<hex>`),
checked by `verify_audit` against a small `[fhir_mssql_countersign]` table.
This check did not exist until this pass found it missing by running the
concurrency suite's tamper test live: `verify_audit` reported the
`sha256`/`sha3-256` breaks from a tampered `actor` column but never the
`hmac-sha256` one, because nothing had ever read `[row_mac]` back
(**F-65**). `tests/concurrency.rs`'s tamper case now asserts detection
across all three signals.

A missing tag is not a finding — rows written before keys were configured
have none. A tag signed by a key this process does not hold is
**unverifiable**, logged as a warning, not reported as tampering: "I cannot
check this" and "this was altered" are different claims. Only a mismatch
under a key this process does hold is a break.

**What this port does not have:** a chain witness, a checkpoint log, or
`resign_history`. `M3.16c`/`M3.16d` are `—` for this port in the
conformance matrix — `chain_witness` and `resign_history` exist on
`fhir-postgresql` only. Do not describe this crate as emitting periodic
checkpoints or off-box witnesses; it does not.

## Erasure versus append-only history

GDPR Article 17 says a record must be removable; everything above says
history must not be. `store.purge(rtype, id, audit)` resolves this in one
direction:

```rust,ignore
let report = store.purge("Patient", "1234", &audit).await?; // PurgeReport { versions_erased, existed }
```

Every history row for the resource is deleted (through the trigger's
`SESSION_CONTEXT`-gated escape hatch, `M14.21` — see
[Operations](operations.md#session-state-hygiene-on-a-pooled-connection)),
the base row is removed if it existed, and one **tombstone** history row is
inserted recording the audit envelope — who, when, why — for the erasure
itself. `versions_erased` is counted from `COUNT(*)` before the delete, not
from the driver's own row-count total: running this live found the
`INSTEAD OF DELETE` trigger's own nested `DELETE` makes
`ExecuteResult::total()` sum every `DONE` token in the batch, doubling the
count if trusted (**F-65**).

`verify_audit` treats a tombstone as a recorded erasure, not a break — a
report that cried wolf on every lawful erasure would be one an operator
learns to ignore, which detects nothing.

Two limits worth stating plainly:

- **The database is not the estate.** Backups, replicas, and any downstream
  system that consumed the resource still hold it until they age out.
  `purge` is one step in an erasure plan, not the plan.
- **The trigger guards against accident, not against the application.**
  `purge`'s own escape hatch is ordinary application-level SQL execution
  with the right session flag set; anything else with database access could
  set the same flag. The trigger stops ordinary code, migrations, and stray
  statements from touching history at all — the tombstone and the access
  log are what make a *deliberate* erasure accountable, not what prevent
  one.

## Grants

The append-only trigger is enforcement the application cannot bypass from
ordinary DML. Belt and braces, restrict the application role too:

```sql
DENY UPDATE, DELETE ON SCHEMA::[r5] TO [fhir_mssql_app];
GRANT SELECT, INSERT ON SCHEMA::[r5] TO [fhir_mssql_app];
```

With both in place, rewriting history requires a principal with
`ALTER`/`CONTROL` on the schema deliberately disabling the trigger — an act
that is itself visible in the server's own audit log, if one is configured
(SQL Server Audit, out of scope for this crate).

## What the deployment must still provide

fhir-mssql-store does **not** do these, and nothing embedding it should
assume otherwise:

| Obligation | Why it is not here |
| --- | --- |
| **Authentication and authorization** | This crate accepts an `Audit` a caller constructs and records it; it verifies nothing about who the caller is and enforces no scope, compartment, or consent check. |
| **TLS to clients, rate limiting, network isolation** | Concerns of whatever HTTP surface sits in front (e.g. `fhir-loco`), not this library. |
| **Encrypted transport to the database** | Wired, but **not satisfiable today** — see `O10.7` below. |
| **Backup and retention** | Your engine's own tooling; see [Operations](operations.md#backup). |
| **Key management and at-rest encryption** | This crate holds a chain-signing key only if a caller passes one to `with_chain_keys`; it manages no other secrets and encrypts nothing at rest. |
| **Terminology validation, profile conformance, FHIRPath invariants beyond what `fhir` enforces, referential integrity across resources** | Out of scope for the storage layer everywhere in this monorepo, not specific to this port. |

### `O10.7`: diagnosed, not satisfied

`connect` negotiates TLS during login (`tiberius`/`rustls`, `M14.24`) —
SQL Server requires it even for an otherwise plaintext connection.
`tests/ssl_live.rs` proves the trust decision is a real mechanism, not a
no-op: `TrustServerCertificate=false` measurably rejects
`azure-sql-edge`'s self-signed certificate, reproducibly, and
`TrustServerCertificate=true` accepts the same certificate. But the
certificate-parsing code in the same dependency chain
(`rustls-webpki 0.101.7`, pinned transitively through `tiberius 0.12.3`'s
`rustls 0.21`) carries three unpatched CVEs plus one unmaintained-crate
advisory, confirmed reaching the shipping `fhir-mssql-store` crate, not
merely a dev-dependency (`M14.34`, **F-67**). `native-tls` was tried as a
replacement and fails the handshake outright on this host. Both facts are
true at once — the mechanism works, and the library underneath it does not
have a fix available today — and this port therefore does **not** claim
`O10.7` satisfied. Whether to accept that risk formally, fund a different
driver, or drop TLS as this port's transport story is an open owner
decision, not a checklist item.
