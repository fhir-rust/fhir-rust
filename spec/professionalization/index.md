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

Assessed 2026-08-26, mid-landing: the professionalization pass is being
committed piecewise today, so "in the working tree" below means authored,
verified, and not yet committed at the time of writing.

- **Rule 1 — partly.** `plan.md` and `tasks.md` exist at the root, authored
  2026-08-26, in the working tree. Their checkbox discipline is stated in
  both files' preambles.
- **Rule 2 — partly.** All 19 canonical files exist (root and
  `LICENSES/`), most in the working tree; the known countable inaccuracies
  (LICENSE.md's 33-vs-34 manifest count, the duplicated AI_STATEMENT.md,
  the stale status line in `spec/special-files-for-public-repos/index.md`)
  are open items in `tasks.md` §In flight.
- **Rule 3 — met as a process.** The gaps SECURITY.md and MAINTAINERS.md
  declare (no tags, unsigned commits, laptop publishing, F-67) each map to
  a `tasks.md` item or a `plan.md` open decision.
- **Rule 4 — partly.** Eleven workflows cover all four families
  (`fhir-ci.yml`, one `<port>-ci.yml` per port, `fhir-store-ci.yml`,
  `fhir-loco-ci.yml`, `fhir-security.yml`, `gates.yml`). The trademarks
  gate is wired into `gates.yml` in the working tree; `cargo deny` on push
  is paths-filtered to `fhir/**` (open item, `tasks.md` §Security).
- **Rule 5 — met at its declared scope.** `scripts/check-trademarks.sh`
  passes on root `*.md` + `help/**` (run 2026-08-26). The four families'
  own documentation is not yet covered — recorded in
  `spec/hl7-trademarks-fair-use/index.md` and `tasks.md` §Compliance.
- **Rule 6 — met.** PHI.md committed 2026-08-26, corrected against the
  conformance matrix before landing.
- **Rule 7 — met.** CODE_OF_CONDUCT.md committed 2026-08-26, claims
  verified against AI_STATEMENT.md §8, AGENTS.md rule 5, MAINTAINERS.md.
- **Rule 8 — open.** This repository's copies are the canon for the
  special-files list and the trademark spec; the siblings have drifted and
  re-sync from here (`tasks.md` §Audit and harmonization).
- **Rule 9 — met.** `help/outreach/index.md` gates promotion on
  `PM-70`–`PM-75`, which are exactly the open supply-chain items.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
