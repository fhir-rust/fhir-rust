# Security policy

This software is intended to store clinical data. Please report anything that
looks like a security defect, including one you are unsure about.

## Reporting a vulnerability

Two private channels, either is fine:

- **GitHub private vulnerability reporting** — the repository's [Security
  tab](https://github.com/fhir-rust/fhir-rust/security/advisories/new)
  (enabled 2026-08-26). It keeps the report, the discussion, and any
  eventual advisory in one place.
- **Email [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)**
  with `SECURITY` in the subject.

Either way, include what you found, how to reproduce it, which crate and
version, and — if you are willing — what you think the impact is.

Do not open a public issue for a vulnerability that is not already public.

**What you can expect**, stated as commitments this project can actually keep:

| | |
| --- | --- |
| Acknowledgement | within **7 days** |
| An assessment — is it a defect, what is the severity, what happens next | within **30 days** |
| Credit | your name or handle in the fix's changelog entry, unless you ask otherwise |
| Embargo | as long as you need to coordinate, within reason; we will agree a date rather than impose one |

**If you get no acknowledgement within 30 days, publish.** That is not a
threat, it is the policy: this project has [one
maintainer](MAINTAINERS.md), and a report that vanishes into an unmonitored
inbox helps nobody. A silence longer than the window above means the private
route has failed, and disclosure becomes your call.

There is no bug bounty. There is no legal entity behind this project to
prosecute anyone; good-faith security research on your own copy of this software
is welcome.

## What is in scope

- Any of the 34 published crates.
- Loss of the integrity guarantees: the tamper-evident chain (`M3.16`–`M3.16e`),
  append-only history (`M3.17`), erasure with a tombstone (`M3.18`).
- **PHI in a log at default level** — `O10.2` forbids it and `T11.7` tests it. A
  reproducible case where a patient identifier reaches a log is a security
  defect here, not a cosmetic one.
- Loss of transport encryption to the database (`O10.7`), or a configuration in
  which it silently degrades.
- Anything in the shredding or reconstruction path that could return one
  patient's data in response to a request for another's.
- SQL injection via any path — search parameters, identifiers, or the generated
  schema.
- In `fhir-loco`: authentication bypass, or a route that discloses a resource to
  an unauthenticated request (`SV3.x`).

## What is not a vulnerability here

These are stated in [`PHI.md`](PHI.md) and [`doc/trust-boundary.md`](doc/trust-boundary.md)
as deliberate boundaries, not oversights. Reports of them are welcome as
*documentation* feedback, but they will not be treated as vulnerabilities:

- **The libraries do not authenticate or authorize.** They record the identity
  you supply. Scopes, compartments, consent and `meta.security` enforcement are
  the perimeter's job by design (`PR12.8`).
- **The unkeyed hash chain does not stop an informed attacker with database
  write access.** The digests are unkeyed over a published pre-image. This is
  stated narrowly in the trust boundary, and the keyed variant plus an off-box
  checkpoint is the answer.
- **Terminology is not validated** (`V9.4`).
- **A missing profile/IG validation** (`V9.1` is structural only).

## Known open issues

Honesty here is part of the policy, and this section is not empty:

- **F-67 (High, open — accepted).** Four TLS advisories reach the shipping
  `fhir-mssql-store` through its driver stack, and `native-tls` fails the
  handshake on at least one host. **Formally accepted 2026-08-28**, after
  investigating and pricing a driver replacement and finding none viable
  without either an unbounded maintenance tail or a build worse on the trust
  axis than the incumbent — full account in `M14.34`
  (`fhir-mssql/spec/14-mssql-dialect.md`). It is recorded in full in the
  [audit register](spec/databases/audit.md) and named in
  [`INSTALL.md`](INSTALL.md) and [`PHI.md`](PHI.md). Run `cargo audit` and
  weigh it against your own requirements before depending on `fhir-mssql` —
  the project's decision is not a substitute for yours.

The [audit register](spec/databases/audit.md) is the live list. It records every
known divergence between specification, documentation and code, with the command
that demonstrates each — including the ones that are not flattering.

## Supported versions

**Pre-release.** Only the most recent published version of each crate is
supported, and there is no long-term-support branch, no backport policy, and no
security-only release line. A published version on crates.io is immutable: it
can be yanked by its owner but never replaced.

If you depend on this in a clinical setting, pin a version and keep a fork you
can build. [`MAINTAINERS.md`](MAINTAINERS.md) explains why that advice is
serious rather than boilerplate.

## What this project does to find these itself

| | |
| --- | --- |
| `cargo deny` — advisories, licences, bans, sources | `.github/workflows/fhir-security.yml`, on every push |
| Shared-core divergence gate | `scripts/check-shared-core.sh`, in `gates.yml` |
| Published-version verification | `scripts/check-published-match.sh` |
| Live database suites against real engines | one CI job per port, per engine |
| Fuzzing of the search-SQL and shred paths | per-port `fuzz/` crates |
| `forbid(unsafe_code)` | all 13 crates of the model family |

What none of that proves is in [`AI_STATEMENT.md`](AI_STATEMENT.md) §12 and in
the conformance matrix's `?` cells.
