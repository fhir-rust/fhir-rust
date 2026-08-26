# PHI, privacy, and what this software does with patient data

**Plain-language answers for a privacy officer, a security reviewer, or anyone
filling in a vendor questionnaire.** It cites the normative sources rather than
restating them, so it cannot drift from them: the requirement table is
[`doc/trust-boundary.md`](doc/trust-boundary.md) (required by `PR12.8`), and the
regulation-to-requirement-to-evidence mapping is
[`spec/databases/13-compliance-mapping.md`](spec/databases/13-compliance-mapping.md).

**Status: pre-release.** Nothing here is certified by anyone, and no deployment
of this software is known to exist. See the last section before relying on any
of it.

## The short answers

| Question | Answer |
| --- | --- |
| Does this software send data anywhere? | **No.** It opens no network connection except to the database you configure. |
| Does it phone home, or collect telemetry or analytics? | **No.** There is no such code in the repository. |
| Does it embed or call an AI model? | **No.** See [`AI_STATEMENT.md`](AI_STATEMENT.md) §1. |
| Does it hold PHI? | Only in the database *you* run, in tables *you* created. The libraries hold data in memory for the duration of a call. |
| Does it write PHI to logs? | **Not at default level** — `O10.2`, and `T11.7` tests it. |
| Does it record who accessed a record? | **Yes**, deliberately — and that record contains identifiers. See below. |
| Can I erase a patient? | Yes — `M3.18`, erasure with a tombstone, mapped to GDPR Art. 17. |
| Is the database connection encrypted? | **Verifying by default on PostgreSQL, MySQL, MariaDB, and SQL Server** — `O10.7` (but read **F-67** for SQL Server). SQLite has no connection; Oracle's transport security is undecided (`M14.22`). |
| Is it a medical device? | **No**, and it cannot make your deployment compliant. See "What this is not". |
| Who do I contact? | [`SECURITY.md`](SECURITY.md) for anything sensitive; [`MAINTAINERS.md`](MAINTAINERS.md) otherwise. |

## What the software is

Four families, and their PHI posture genuinely differs:

| Family | What it is | PHI posture |
| --- | --- | --- |
| [`fhir/`](fhir/) — the model | FHIR® resources as Rust types | **Touches no database and performs no I/O at all.** It parses and serializes values you hand it. If PHI passes through it, that is your program's memory, not a store. |
| [`fhir-store/`](fhir-store/) — the persistence core | audit chain, attribution and disclosure record types | Links no driver and opens no socket. It defines the shapes; it does not move data. |
| `fhir-<engine>/` — the six database ports | shred resources into relational tables in **your** database | The only family that stores anything. It connects to one database, the one whose DSN you supply. |
| [`fhir-loco/`](fhir-loco/) — the HTTP surface | a FHIR REST API over one store | Accepts network connections. This is the only component that listens on a socket, and the only one with an authentication story. |

## What it does *not* do

This is the honest core of the document, and it is lifted from the
[trust boundary](doc/trust-boundary.md)'s right-hand column. Your deployment
must provide every one of these:

- **Authentication.** The six libraries do not authenticate. They take an
  identity you supply and record it. (`fhir-loco` is different — it verifies a
  PASETO v4.public token on every request and refuses to boot without an issuer
  public key. Even there, only *authentication* is provided.)
- **Authorization.** Nothing here evaluates whether a principal *may* see what
  it asked for. This includes SMART on FHIR scopes, compartments, consent
  evaluation, and `meta.security` label enforcement. The store records that a
  read happened; it does not decide whether it should have.
- **Terminology validation.** A `required` binding is checked against the
  literal list of codes the generator extracted (`M3.7`). No value set is
  expanded, no subsumption computed, no `$validate-code` performed (`V9.4`).
- **Profile and implementation-guide conformance.** Structural validation only
  (`V9.1`).
- **Rate limiting, TLS termination for your clients, log retention and review,
  and key custody.**

## What it does do, that a reviewer needs to know about

**It records identities, by design.** This is a feature answering HIPAA
§164.312(b), and it has a privacy consequence worth stating plainly:

- **Attribution on every write** (`M3.15`, `PR12.4`) — who acted.
- **Disclosure logging on every read** (`PR12.5`, `PR12.6`) — who saw what.

So the audit tables contain identifiers for the *users* of the system, alongside
references to the resources they touched. Treat those tables as sensitive: they
are a record of clinical access, and their retention, review and erasure are
your deployment's responsibility.

There is **no default attribution** (`PR12.3a`). You pass an `Audit` value, and
`actor_source` records which kind — a perimeter assertion, a CLI action, or
explicitly unattributed. Passing `unattributed()` is legitimate; it just shows
up in the record as what it is.

**It maintains a tamper-evident history chain** (`M3.16`–`M3.16e`), and the
[trust boundary](doc/trust-boundary.md) states narrowly what that does and does
not achieve — unkeyed it detects careless modification and supports an external
witness; it does not stop an informed attacker with write access, and a keyed
chain only helps if the key lives somewhere that attacker does not. A chain
missing its most recent version verifies perfectly, which is why the off-box
checkpoint matters.

## Regulatory framing

- **HIPAA.** [§13](spec/databases/13-compliance-mapping.md) maps §164.312(b)
  audit controls, §164.312(c) integrity, §164.312(e) transmission security and
  §164.502 minimum necessary to numbered requirements and to the tests that
  evidence them.
- **GDPR.** Art. 17 erasure (`M3.18`), Art. 30 records of processing
  (`PR12.5`, `PR12.7`), Art. 32 security of processing (`O10.7`, `O10.8`,
  `O10.10`).
- **IEC 62304.** The specification-to-tasks-to-test traceability is the
  lifecycle evidence; the [conformance matrix](spec/databases/conformance-matrix.md)
  is where it is recorded.

**These are components, not certified systems.** §13 puts it in the form this
project uses, and it is the sentence to quote back to anyone who reads more into
the table than it says:

> They cannot make a deployment compliant, but they must not be the reason a
> deployment cannot be.

Whether a *deployment* meets an obligation depends on the perimeter, the
operating procedures, and the evidence retained — none of which a library
provides.

**Not a medical device.** These libraries store, transfer and retrieve records.
A downstream integrator who gives their product a medical purpose brings *their*
product into scope; that classification is theirs to make.

## Development data

No patient data, no personally identifiable health information, and no customer
data exists anywhere in this repository — not in source, not in fixtures, not in
CI. Test data comes from the example resources published inside the official
FHIR specification packages, which are modelling artifacts rather than records
about people. This is a structural property you can check against the tree.

## Known limits that bear on this document

Stated here so you find them from this page rather than from an audit:

- **`fhir-mssql` carries four open TLS advisories** that reach the shipping
  crate through its driver stack — **F-67**, still open, and the only **High**
  finding currently in the [audit register](spec/databases/audit.md). If
  encrypted transport to SQL Server matters to you, read that finding before
  depending on the port.
- **No Inferno run has ever been performed** against `fhir-loco`, so
  ONC/HTI FHIR conformance and Bulk Data are recorded as *partly* satisfied in
  §13 rather than as met.
- **Test depth is uneven.** Redaction and concurrency tests exist in five of
  the six ports; `fhir-oracle` has neither, and no working `R4.5` snapshot
  mechanism. The dedicated audit-chain suites (`audit.rs`,
  `chain_portability.rs`) exist only in `fhir-postgresql`, though every
  port's live suite exercises `verify_audit`. Where the conformance matrix
  shows `?`, the code is shared from a port where it passes and nothing
  tests it there.
- **Verification depth varies by port.** Five of six sit below Reference level
  (`C0.8`). The [conformance matrix](spec/databases/conformance-matrix.md) is
  the only document in this repository that distinguishes them, and step 4 of
  §13's audit procedure — check the matrix for the port you are actually
  deploying — is the step that gets skipped.
- **Nothing is signed.** No commit or tag carries a signature, so provenance of
  a given change cannot be cryptographically verified.

## If you are filling in a questionnaire

Cite this file for the posture, [`doc/trust-boundary.md`](doc/trust-boundary.md)
for the requirement-level table, §13 for the regulation mapping, and the
[conformance matrix](spec/databases/conformance-matrix.md) for what your
specific port has been shown to do. If a question has no answer in those four,
ask — an unanswered question is more useful to this project than a guessed one.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
