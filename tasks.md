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
- [x] **Resolve the AI_STATEMENT.md duplication** — done 2026-08-26: the
      root is the source; the spec-directory copy is now a pointer that
      records the resolution (the root carries everything the draft had
      plus the §8 failure register; the draft's one unique section, "Rules
      for contributors", lives near-verbatim in CONTRIBUTING.md §"Using AI
      tools" — verified by reading both before deleting anything). The
      draft's text stays in git history.
- [ ] **Commit the professionalization pass** (9 modified files, 20+
      untracked documents, the two staged spec renames) — run
      `scripts/check-shared-core.sh`, `check-trademarks.sh`, and
      `check-doc-examples.sh` first. Until this lands, a GitHub visitor sees
      none of it. Ask before pushing.
- [ ] Delete `fhir-tmp-stash/` (contains only a `.DS_Store`).

### Security and supply chain

- [x] **Widen `fhir-security.yml`'s `cargo deny` beyond `fhir/**`** — done
      2026-08-26: a nine-workspace matrix, each against its own `deny.toml`,
      path-triggered on any workspace plus the weekly cron. Verified
      locally first: all nine green, with `fhir-mssql`'s green resting on
      its deny.toml's dated F-67 ignores (`M14.34`) — the acceptance stays
      visible and F-67 stays open, pending the owner decision below.
- [ ] **Get an owner decision on F-67** (High, open since it was filed:
      mssql TLS advisories in published crates). Every outreach prerequisite
      chain passes through it (`PM-4`).
- [x] Create git tags and GitHub releases for the published versions
      (`PM-70`) — done 2026-08-26: sixteen annotated tags at `e28964e`
      pushed, sixteen releases with `TG1.8`-compliant notes, `fhir 4.1.0`
      marked latest, `releases.atom` live; NEWS.md, MAINTAINERS.md,
      CHANGELOG.md, and the outreach register updated in the same change.
- [ ] Sign commits and tags going forward; record the posture change in
      MAINTAINERS.md (which now says tags and releases exist but nothing is
      signed).
- [ ] Decide the publishing shape: crates.io Trusted Publishing from CI vs
      documented laptop publishing; the per-port `publish.yml` workflows are
      currently inert either way.
- [x] Enable GitHub private vulnerability reporting, dependabot, and add
      `.github/ISSUE_TEMPLATE/` — done 2026-08-26, secret scanning too:
      PVR flipped via the API and verified enabled; dependabot alerts and
      automated security fixes enabled, with `.github/dependabot.yml`
      registering every lockfile but capping cargo version-update PRs at
      zero — its first hour on default limits opened 47 major-bump PRs,
      each triggering a port's full live-database CI; all 47 closed with
      the reasoning, which also lives in the file (security advisories
      still open PRs; `fhir-security.yml` fails the build on any known
      advisory). Bug-report and wrong-claim templates carry the
      never-paste-patient-data rule and a stated response expectation;
      SECURITY.md names the Security tab as a private channel alongside
      email.

### Compliance — licensing and trademarks

- [x] Widen the trademark gate beyond root + `help/**` — first tranche done
      2026-08-26: `doc/` (12 files), `fhir-store/` and `fhir-loco/`
      top-level pages, and the six ports' `README.md` are in the script's
      scope and compliant (23 files fixed mechanically; the six README
      edits identical modulo engine name). Corrected while sweeping: the
      six port READMEs claimed "MIT OR Apache-2.0" where the manifests
      carry the five-way expression.
- [x] Widen the trademark gate's remaining tranche — done 2026-08-26, and
      further than planned: the scope is now **tree-wide** (every markdown
      file, 216 more fixed mechanically) with three named structural
      exemptions (`fhir/fhir.md` generated transcript, mdbook `SUMMARY.md`
      manifests, `.github/` issue templates), each reasoned in the script.
      All seven mdbooks build with the footers; all 24 doc examples still
      compile. The sweep touched the README/LICENSE cargo packages into
      every model-family crate, so the remaining 13 published-matched
      crates were bumped (fhir-core 3.2.1, fhir-derive-macros 1.5.1,
      fhir-r2–r6 4.1.1, the five reservations 0.0.2) — all 34 crates now
      sit one unpublished version ahead, one publish pass covers
      everything.
- [x] Replace the ports' `LICENSE-APACHE` header-boilerplate files — done
      2026-08-26: all six are now copies of `LICENSES/Apache-2.0.txt` (§4 of
      Apache-2.0 requires recipients be given the License, and a header
      notice is not it); LICENSE.md's two statements about them updated in
      the same change.
- [x] Set `CITATION.cff`'s `license` field — done 2026-08-26: the five SPDX
      identifiers as a CFF list (CFF takes identifiers, not OR-expressions;
      the choice semantics live in LICENSE.md, which the file's comment
      points at). Matches the `snomed-rust` convention.

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

- [x] **Restore audit-register hygiene** — done 2026-08-26 in the F-93
      filing commit: the F-90 summary row now opens "**fixed** — closed in
      full 2026-08-12", matching its own narrative, and with that the
      table's open set (F-51, F-67) agrees with the intro's list again. The
      cell records its own correction, per F-73's rule.
- [ ] Serve as the family canon: when siblings re-sync
      `spec/special-files-for-public-repos/` and the trademark spec from
      here, keep this repo's copies accurate first (see the in-flight
      items).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
