# Professionalization

This specification defines what "professional" means for this repository and
binds the maintainer as much as any contributor. The audience is healthcare
professionals and the engineers who serve them, worldwide, in production use;
the standing constraint is that a wrong claim in this domain has clinical
cost. Rationale and current execution state live in [`plan.md`](../../plan.md)
and [`tasks.md`](../../tasks.md); this file holds the rules.

## Rules

1. **Plans are files, and a checked box is a verified fact.** `plan.md` and
   `tasks.md` exist at the repository root. A `[x]` means the work was done
   and verified, with the evidence named — never that it is intended,
   assumed, or inherited from a sibling repository.
2. **The special files exist and stay accurate.** The canonical list is
   [`spec/special-files-for-public-repos/`](../special-files-for-public-repos/index.md).
   Every countable claim in those files (crate counts, test counts, coverage
   lists, "X is enabled/disabled") is measured before it is written and
   re-verified when cited.
3. **Self-declared gaps are promises.** A gap named in SECURITY.md,
   MAINTAINERS.md, or AI_STATEMENT.md ("no CI", "unsigned commits") is either
   closed or consciously accepted in `tasks.md` — and the declaring document
   is updated in the same change that closes it.
4. **CI enforces what documents claim.** Every check a document says this
   repository runs (tests, clippy, fmt, MSRV, trademark rules, doc gates)
   runs in CI on every push. A laptop-only check is a claim, not a guarantee.
5. **Trademark discipline.** The marks are **HL7®** and **FHIR®** (and
   CDA®, which this repository's prose does not use), owned by Health Level
   Seven International. The normative terms are
   [`spec/hl7-trademarks-fair-use/`](../hl7-trademarks-fair-use/index.md):
   the registration mark follows the first prose use of a word mark on each
   page; any page using a mark carries the disclaimer verbatim; places of
   prominence say "HL7® FHIR® standard". Enforcement is
   `scripts/check-trademarks.sh` — root `*.md` plus `help/**`, code and
   link targets masked — run by the `trademarks` job in
   `.github/workflows/gates.yml`. Documents outside the script's scope,
   this one included, comply anyway.
6. **Patient data is addressed in plain language.** `PHI.md` at the root
   states what the software does and does not do with patient data, for a
   reader who is a privacy officer, not a Rust programmer. It never claims
   compliance or certification.
7. **Conduct has a document and a path.** `CODE_OF_CONDUCT.md` at the root
   (Contributor Covenant 2.1 plus this family's claim-accuracy clause:
   overstating what the software does is a conduct matter, not only a bug).
8. **Harmonization runs through the family.** The sibling repositories
   (`hl7-rust`, `er7-rust`, `snomed-rust`, `openehr-rust`) share these
   rules, the special-files list, and the six workstreams (governance;
   compliance — licensing and trademarks; security and supply chain;
   privacy and patient data; outreach; audit and harmonization).
   Conventions sync from the repository that owns the canonical copy —
   for the special-files list and the trademark spec, that is this one —
   rather than drifting independently.
9. **Outreach is gated.** No promotion while a rule above is unmet for the
   surface being promoted; `help/outreach/index.md` names the prerequisites.

## Status in this repository

Assessed 2026-08-26, mid-landing; **re-verified 2026-08-29 against the tree
as it now stands, not assumed from the original assessment.** Most of what
that first pass called "partly" or "open item" closed in the three days
between — recorded here rather than left to read as still true.

- **Rule 1 — met.** `plan.md` and `tasks.md` exist at the root, committed,
  with checkbox discipline stated in both files' preambles and exercised
  continuously since (F-67's decision, the funding and trusted-publishing
  work, and this re-verification itself all landed as dated `tasks.md`
  entries).
- **Rule 2 — met.** All canonical files exist and are committed. The three
  countable inaccuracies the original assessment named are each closed,
  checked directly rather than trusted: `LICENSE.md`'s manifest count now
  reads **34** everywhere in the file (line 32 still said 33 until this
  re-verification caught it — measured fresh: 41 total `[package]`
  manifests, 7 `publish = false`, 34 publishable); `AI_STATEMENT.md` has one
  source, the root, with the `spec/special-files-for-public-repos/` copy a
  pointer rather than a divergent draft; and that same file's own status
  section is current, not stale, as of 2026-08-26.
- **Rule 3 — met.** The gaps this rule is about are no longer gaps: 60
  annotated tags exist (`spec/git-tags-name-published-versions/`), commit and
  tag signing has been active since 2026-08-27 (verified:
  `git log -1 --format=%G?` returns `G`), laptop publishing is a stated
  permanent decision rather than an open gap, and **F-67 was decided
  2026-08-28** — accept the risk formally, documented in `M14.34` and
  everywhere the risk is named to a reader. Nothing here is an open item any
  more; the record of how each closed is what `tasks.md` and the audit
  register hold.
- **Rule 4 — met.** Eleven workflows cover all four families
  (`fhir-ci.yml`, one `<port>-ci.yml` per port, `fhir-store-ci.yml`,
  `fhir-loco-ci.yml`, `fhir-security.yml`, `gates.yml`). The trademarks gate
  runs in `gates.yml`. `cargo deny` on push is **not** paths-filtered to
  `fhir/**` — checked directly against `.github/workflows/fhir-security.yml`
  rather than trusted from the prior assessment: both its `push` and
  `pull_request` triggers list all nine workspace paths, and its job matrix
  runs `cargo deny` against each workspace's own `deny.toml` independently.
- **Rule 5 — met at its declared scope.** `scripts/check-trademarks.sh`
  passes on root `*.md` + `help/**` and on the nine top-level crate roots'
  rustdoc (run 2026-08-26). The four families' own markdown is not yet
  covered — recorded in `spec/hl7-trademarks-fair-use/index.md` and
  `tasks.md` §Compliance.
- **Rule 6 — met.** PHI.md committed 2026-08-26, corrected against the
  conformance matrix before landing.
- **Rule 7 — met.** CODE_OF_CONDUCT.md committed 2026-08-26, claims
  verified against AI_STATEMENT.md §8, AGENTS.md rule 5, MAINTAINERS.md.
- **Rule 8 — open.** This repository's copies are the canon for the
  special-files list and the trademark spec; the siblings have drifted and
  re-sync from here (`tasks.md` §Audit and harmonization).
- **Rule 9 — met, and narrower than when this rule was written.**
  `help/outreach/index.md` gated promotion on `PM-70`–`PM-75`; checked
  directly against that file rather than repeated from memory, five of
  those six (`PM-70`, `71`, `73`, `74`, `75`) are done, and the sixth
  (`PM-72`, benchmarks — one port of six has a real harness, and the
  JSONB-vs-relational comparison the whole pitch rests on has never been
  run) is now the only thing gating outreach phase 1, alongside `PM-76`
  (a machine-readable conformance statement), which was never in the
  original six.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
