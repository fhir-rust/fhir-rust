# Security, PHI, and the audit trail

Governing sections: [`spec/03-storage-model.md`](../spec/03-storage-model.md)
(`M3.15`–`M3.18`), [`spec/12-trust-principal-and-audit.md`](../spec/12-trust-principal-and-audit.md),
[`spec/13-compliance-mapping.md`](../spec/13-compliance-mapping.md).

Everything in these libraries assumes the data is **protected health
information**. That assumption changes which mistakes are recoverable.

## The trust boundary

| These libraries guarantee | The deployment must provide |
| --- | --- |
| Attribution on every write (`M3.15`, `PR12.4`) | Authentication |
| A disclosure record on every read (`PR12.5`) | Authorization, scopes, compartments |
| Tamper-evident history (`M3.16`) | Consent enforcement |
| Erasure with a tombstone (`M3.18`) | TLS termination |
| Append-only history in the database (`M3.17`) | `meta.security` label enforcement |
| No PHI in logs at default level (`O10.2`) | Rate limiting per identity |

Authentication is deliberately outside; **recording who acted is not**. A
perimeter knows the identity and only the store knows which rows were touched,
so neither can answer HIPAA §164.312(b) alone. That split is the reason §12
exists.

## Attribution

Never let an unattributed write look like an attributed one (`PR12.3a`). The
caller constructs the attribution deliberately:

```rust
Audit::principal(actor, source)   // the perimeter asserted an identity
Audit::cli()                      // a local operator ran this
Audit::unattributed()             // nobody did, and the record says so
```

`actor_source` records which. A store API that accepted a write with no audit
argument and recorded something plausible would convert a deployment mistake
into a permanent false record — and a false attribution is worse than a missing
one, because it survives review.

## The hash chain

History is chained under **SHA-256 and SHA3-256** — deliberately two design
families, not two digest lengths (`M3.16a`). MD5 and SHA-1 both fell to the same
line of cryptanalysis and both are Merkle–Damgård; a clinical record may be
retained for decades, longer than anyone can promise one construction will
stand. Both are FIPS-approved.

What the unkeyed chain actually buys, stated so nobody over-claims it: it
detects **careless or unaware modification** — a migration, a stray `UPDATE`, a
row restored from the wrong backup — and it supports an external witness. It
does **not** stop an informed attacker with write access, because the digests
are unkeyed over a published pre-image.

The fix is the keyed tag: `HMAC-SHA-256`, key held by the process and never by
the database. A key stored where the attacker already has write access protects
nothing.

Rules that are easy to get wrong:

- **Compute in the application, never in the database** (`M3.16b`). Also a
  portability requirement (`X15.2`): a chain over one engine's JSON output is
  verifiable by that engine alone. All six ports now derive the pre-image from
  the shared `canon.rs` — PostgreSQL was the last holdout (**F-07**, fixed), and
  `chain_portability.rs` recomputes one of its chains from the exported rows.
- **Only a tag mismatch is a finding.** A missing tag, a tag naming a key this
  process lacks, and a malformed tag are each reported as what they are.
  Reporting a key-distribution problem as forgery burns an incident response.
- **Key ids travel with tags**, so rotation is additive. Without the id,
  rotating would invalidate all history at once — indistinguishable from mass
  tampering.
- **Verification is constant-time.** A timing oracle lets an attacker with write
  access recover a valid tag byte by byte.
- **Never backfill a chain** (`M3.16e`). A chain assembled after the fact
  attests only that rows look consistent *now* — exactly what an attacker who
  rewrote them would produce. Report the chain as beginning where it begins.

## Keys

- At least 32 bytes. A `changeme` placeholder reaching production yields tags an
  attacker reproduces by guessing.
- **A file, not an environment variable.** Environment is visible in
  `/proc/<pid>/environ`, survives into crash dumps, is reported by
  orchestrators, and is inherited by every child. A file is none of those, and
  is what Kubernetes secrets and systemd credentials already produce.
- A key file readable by group or other is **refused**, not warned about. A
  warning is read once at startup; the file stays readable for the life of the
  deployment.
- Zeroize on drop. Freed memory is not scrubbed, and a key otherwise lingers in
  the heap and in any core dump.
- Generate with `chain-key-new`, never `openssl rand > key`: the shell applies
  the umask (commonly `022`, producing a file that must be refused) and leaves
  the secret world-readable in the window before `chmod`.

## What never goes in a log

`O10.2` and `T11.7`. At default level, **no resource content**. Not a name, not
an identifier, not a date of birth, not a free-text note.

Also never: a submitted value echoed back in an error. An error is the one place
a value escapes into a log, a response, and a ticket simultaneously.

The redaction test is the enforcement: a full CRUD + search cycle over a
resource containing a distinctive marker, asserting no log line ever contains
it. Only `fhir-postgresql` has it (`redaction.rs`); porting it is high-value
work.

**The checkpoint is the exception, and deliberately so.** `M3.16c` emits chain
checkpoints on a dedicated `audit_checkpoint` log target precisely because they
carry no PHI — only counts and digests — so they can be retained far longer than
ordinary application logs and shipped somewhere patient data must not go. That
separation is what makes a log-based witness practical.

## Erasure

`purge` (`M3.18`) is the one sanctioned deletion, for GDPR Art. 17. It removes
history rows and leaves a **tombstone** recording who purged what, when, why,
and the chain it terminated — so an erased record leaves a verifiable hole
rather than a silent one. It requires an explicit erasure acknowledgement, logs
at warn, and emits a checkpoint immediately after, which is what separates a
recorded intentional removal from the unrecorded kind.

## Search and injection

Every user-supplied value binds as a parameter (`P6.8`) — including in
`LIMIT`/`OFFSET`, sort direction, and cursor decoding. The only interpolated
fragments are table and column names from the generated relational map, quoted
by the dialect's rule. A value must never be routed through that path.

The `fuzz/seeds/search_sql/injection.txt` corpus is evidence for this, and
`T11.9` requires it be run rather than merely committed.

## Before you touch anything in this area

Read `store/src/chain.rs`'s module header first. It contains the design argument
— why not in the database, what the unkeyed chain does and does not buy, how the
two properties lost by moving out of SQL were recovered — and that argument is
not reconstructible from the code.
