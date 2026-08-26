# Tasks

Repository-level professionalization checklist; rationale and workstreams
live in [`plan.md`](plan.md). A `[x]` here means the work is **verified
done**, not intended — check items off in the same change that completes
them, with the evidence named.

**This file is not engineering status.** The ports' own `tasks.md` files have
a documented history of false `[x]` claims (**F-27**); read the
[conformance matrix](spec/databases/conformance-matrix.md) and each port's
dialect annex for what works. Nothing here speaks for a port.

## Done (verified 2026-08-26, the state this file starts from)

- [x] All 19 canonical special files exist
      (`spec/special-files-for-public-repos/index.md`), including PHI.md,
      CODE_OF_CONDUCT.md (Contributor Covenant 2.1 + claim-accuracy clause),
      LICENSES/ with all five full texts, and `.github/FUNDING.yml` — most
      still uncommitted (see below).
- [x] Trademark fair use specified (`spec/hl7-trademarks-fair-use/`),
      mechanically checked (`scripts/check-trademarks.sh` passes on root
      `*.md` + `help/**`), and wired into `gates.yml` in the working tree.
- [x] Outreach plan exists (`help/outreach/index.md`, with claims register
      and prerequisites `PM-70`–`PM-75`) and correctly gates promotion on
      supply-chain work.
- [x] 34 crates published to crates.io (2026-08-22); audit register at 92
      findings with reproducible evidence per finding.

## Next up

Grouped by `plan.md` workstream. Order within a group is priority order.

### In flight — land it first

- [x] **Fix the countable inaccuracies before committing** — done
      2026-08-26, in the commit that landed LICENSE.md:
      - LICENSE.md now states the measurement: 41 `[package]` manifests, the
        34 publishable ones all carrying the SPDX expression (16 verbatim,
        18 via `license.workspace = true`), 7 `publish = false` internal
        crates declaring no licence field. Re-measured before editing.
      - LICENSE.md's "For automated tooling" list now points at `LICENSES/`
        for full texts and names the ports' `LICENSE-APACHE` as the header
        boilerplate the earlier section already said it was.
      - `spec/special-files-for-public-repos/index.md`'s status note now
        says all root files meet the trademark rules, gated by
        `scripts/check-trademarks.sh` in `gates.yml` — verified by that
        script passing on every root `*.md`.
- [ ] **Resolve the AI_STATEMENT.md duplication**: root (281 lines) vs
      `spec/special-files-for-public-repos/AI_STATEMENT.md` (257 lines,
      divergent from line 1). One becomes the source; the other becomes a
      pointer or is deleted. The spec file itself says so.
- [ ] **Commit the professionalization pass** (9 modified files, 20+
      untracked documents, the two staged spec renames) — run
      `scripts/check-shared-core.sh`, `check-trademarks.sh`, and
      `check-doc-examples.sh` first. Until this lands, a GitHub visitor sees
      none of it. Ask before pushing.
- [ ] Delete `fhir-tmp-stash/` (contains only a `.DS_Store`).

### Security and supply chain

- [ ] **Widen `fhir-security.yml`'s `cargo deny` beyond `fhir/**`** so the
      six ports and `fhir-loco` — where F-67's TLS advisories live — are
      scanned on push, not only by the weekly cron.
- [ ] **Get an owner decision on F-67** (High, open since it was filed:
      mssql TLS advisories in published crates). Every outreach prerequisite
      chain passes through it (`PM-4`).
- [ ] Create git tags and GitHub releases for the published versions
      (`PM-70`); sign commits and tags going forward; record the posture
      change in MAINTAINERS.md.
- [ ] Decide the publishing shape: crates.io Trusted Publishing from CI vs
      documented laptop publishing; the per-port `publish.yml` workflows are
      currently inert either way.
- [ ] Enable GitHub private vulnerability reporting, dependabot, and add
      `.github/ISSUE_TEMPLATE/`; update SECURITY.md's reporting channel in
      the same change.

### Compliance — licensing and trademarks

- [ ] Widen the trademark gate's scope beyond root + `help/**`: first
      `doc/` (12 files, currently non-compliant), then `fhir/`, then the six
      ports' `README.md`/`book/` and `fhir-store/`/`fhir-loco/` — the port
      sweep is a shared-core-adjacent change (six identical edits; scope it
      per `CLAUDE.md` before starting).
- [ ] Replace the ports' `LICENSE-APACHE` header-boilerplate files with the
      actual license text (named as outstanding work in LICENSE.md).
- [ ] Set `CITATION.cff`'s `license` field to the SPDX expression instead of
      "See license file".

### Governance

- [ ] Nothing beyond the in-flight accuracy items — the document set is
      complete. Revisit if the family adds a convention this repo lacks.

### Privacy and patient data

- [ ] Keep PHI.md's limits section current as ports advance (it names:
      redaction/concurrency tests in five of six ports with `fhir-oracle`
      lacking both, dedicated audit-chain suites only in `fhir-postgresql`,
      five of six ports below Reference level, no Inferno run) — each of
      those is a dated claim that goes stale the day a port closes the gap.
      The first stale version of this very item ("audit/redaction/
      concurrency tests only in `fhir-postgresql`") was caught and corrected
      2026-08-26 while verifying PHI.md against the tree.

### Outreach

- [ ] Blocked on `PM-70`–`PM-75` (tags/releases, signing, F-67, GPL-scanner
      note, licensing fixes) — then execute `help/outreach/index.md` phase 1.
      The claims register governs every public sentence.

### Audit and harmonization

- [ ] **Restore audit-register hygiene** (`spec/databases/audit.md`): the
      F-90 summary row reads "open …" yet ends "Closed in full 2026-08-12",
      and the intro's open list (F-51, F-67, line 79) disagrees with the
      table — the exact drift F-73 was filed for. Reconcile both in one
      change.
- [ ] Serve as the family canon: when siblings re-sync
      `spec/special-files-for-public-repos/` and the trademark spec from
      here, keep this repo's copies accurate first (see the in-flight
      items).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
