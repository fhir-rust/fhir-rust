# Plan — `fhir` Rust monorepo

Goal: a production-grade, spec-driven Rust monorepo for the HL7® FHIR®
standard — the `fhir/` model family, six `fhir-<engine>/` database ports
governed by one normative core, `fhir-store/` (shared library), and
`fhir-loco/` (HTTP surface); 34 crates on crates.io — professionalized for
its real audience: healthcare professionals and the engineers who serve them,
worldwide, in settings where a wrong claim has clinical cost.

Method: **specification-driven development, with an adversarial audit
culture.** Normative behavior lives in `spec/` and `fhir/spec/`; the findings
register (`spec/databases/audit.md`, 92 findings and counting) exists because
this repository's main failure mode has been confident text that nothing
substantiates. Engineering status is read from the
[conformance matrix](spec/databases/conformance-matrix.md) and the dialect
annexes — never from a checklist. This file holds only what no spec does: the
repository-level professionalization plan. Execution items live in
[`tasks.md`](tasks.md), where a `[x]` means verified, not intended.

## Where the repository stands (verified 2026-08-26, end of day)

All 34 publishable crates are on crates.io at their current source versions
(republished in full 2026-08-26; `check-published-match.sh`: 34 matched,
0 mismatched), every independently-versioned unit is tagged and carries a
GitHub Release (`spec/git-tags-name-published-versions/`), and the whole
professionalization pass is committed, pushed, and hosted-green. The
canonical special-files set exists in full; trademark fair use is specified,
mechanically gated in CI **tree-wide** — every markdown page and the nine
crate roots' rustdoc; the security surfaces (private vulnerability
reporting, dependabot alerts and security fixes, secret scanning, a
nine-workspace `cargo deny` matrix) are on; and publishing is, by owner
decision, permanently a documented laptop step. Commits and tags remain
signed from 2026-08-27 onward, local-repo SSH signing with the public key
committed at `.github/jph-code-signing.pub`; commits and tags before that
date stay unsigned permanently (MAINTAINERS.md).

## Workstreams — professionalization (2026-08 onward)

Six workstreams, shared with the sibling repositories (`hl7-rust`,
`er7-rust`, `snomed-rust`, `openehr-rust`) so the family converges on one
posture. Open items for each are in `tasks.md`.

1. **Governance.** The document set is complete and candid (single
   maintainer, no escalation path beyond the maintainer, forking as the
   continuity guarantee). Remaining work is accuracy, not existence: the
   duplicated and divergent AI_STATEMENT.md (the LICENSE.md counts and the
   special-files status line were fixed 2026-08-26 — see `tasks.md`). In a
   repository whose
   code-of-conduct makes overstating capability a conduct matter, the
   governance files must survive their own standard.

2. **Compliance — licensing and trademarks.** Root-level compliance is done
   and gated. Two frontiers remain: scope (the trademark check covers root
   `*.md` + `help/**` only — `doc/`, `fhir/`, the six ports' READMEs and
   books, `fhir-store/`, `fhir-loco/` are ungated and non-compliant), and
   the ports' `LICENSE-APACHE` files, which are header boilerplate rather
   than the license text — stated as outstanding work inside LICENSE.md
   itself.

3. **Security and supply chain.** `cargo deny` runs weekly and on push — but
   paths-filtered to `fhir/**`, so the six ports and `fhir-loco` were not
   scanned on push when **F-67 (High: mssql TLS advisories)** was filed
   there; F-67 itself closed 2026-08-29 (driver switched from `tiberius` to
   `mssql`). The publish path is a laptop with a long-lived registry token; the
   per-port `publish.yml` workflows are inert. No tags, no signing, no SBOM,
   no Trusted Publishing, no private-vulnerability-reporting config, no
   dependabot, no issue templates.

4. **Privacy and patient data.** PHI.md exists and is the family's model: a
   privacy-officer Q&A that names its own limits (no certification, no known
   deployment, no Inferno run; redaction/concurrency tests in five of six
   ports — `fhir-oracle` has neither — and the dedicated audit-chain suites
   only in `fhir-postgresql`, corrected 2026-08-26 against the tree).
   Keeping it true as ports advance is the work.

5. **Outreach.** `help/outreach/index.md` is thorough and self-gating: its
   prerequisites (`PM-70`–`PM-75`: tags/releases, signing, F-67 decision,
   GPL-scanner note, licensing fixes) are all now closed — F-67 accepted
   2026-08-28, then resolved outright 2026-08-29 (driver switch). What still
   gates phase 1 is `PM-72`:
   benchmarks exist for one port of six, and the JSONB-vs-relational
   comparison the whole pitch rests on has never been run.

6. **Audit and harmonization.** The audit register is the family's reference
   implementation, and it currently fails its own hygiene rule (F-73's
   failure mode): the intro's open list says F-51 and F-67, while the F-90
   row reads "open …" yet ends "Closed in full 2026-08-12". This repository
   also holds the canonical special-files list and trademark spec that the
   siblings have drifted from; harmonization means the siblings re-sync from
   here, and this repo keeps its canon accurate.

## Open decisions (awaiting a call, not code)

- ~~**F-67 (High).**~~ Decided 2026-08-28: **accept the risk formally, keep
  shipping on upstream `tiberius`, document it loudly.** The oldest open
  decision in the repo, closed after investigating and pricing three
  alternatives — a from-scratch driver, a fork of the one upstream fix that
  exists (`prisma/tiberius#419`), and one newer alternative crate (`ms-tds`,
  disqualified on sight) — none viable without either an unbounded
  maintenance tail or a cost worse than the flawed incumbent. Full account:
  `M14.34` in `fhir-mssql/spec/14-mssql-dialect.md`.
  **Superseded 2026-08-29: resolved outright**, not just accepted — the
  owner published `mssql`, a `tiberius` fork maintained to carry the fixes
  forward. The advisories no longer exist in the dependency tree.
- ~~**F-51 (Medium).**~~ Fixed 2026-08-29: `tests/oracle_ddl.rs`
  (`fhir-oracle-map`) installs a sampled schema live, on the model of
  `fhir-mssql`'s `mssql_ddl.rs`. The "driver decision" this bullet expected
  turned out to already be decided by evidence this repository had produced
  itself: `fhir-oracle-store` (F-68) already proved the `oracle` crate +
  Instant Client works live. With `F-67` closed the same day, the audit
  register has no open finding left.
- ~~**Publishing shape.**~~ Decided 2026-08-26: **documented laptop
  publishing, permanently** — the owner's judgment that GitHub is not
  reliable enough to hold the publish path, made hours after an Actions
  major outage stalled every hosted run. The six inert `publish.yml`
  workflows are deleted; `spec/publishing.md` is the process; the residual
  (one machine, one person, a long-lived local token) is recorded in
  MAINTAINERS.md. (`PM-70`'s tags and releases exist as of the same day.)

## Non-goals (for now)

- No new engines, no new families, until the six workstreams close.
- No conformance claims beyond what the matrix shows; "the code is shared
  from a port where it works" remains `?`, not `•`.

## Risks & watch items

- The uncommitted pass is the single biggest risk: a large, coherent body of
  governance work that currently has zero external effect and could rot
  against the moving tree.
- Countable claims (crate counts, coverage lists, status lines) are this
  repository's known failure mode — every number in the new documents needs
  the same verification discipline the audit register applies to code.
- The trademark gate widening (`doc/`, families) multiplies by the shared
  six-port core: a disclaimer edit in one port's book is six edits (rule 2).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
