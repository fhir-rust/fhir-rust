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
- [x] 34 crates published to crates.io (2026-08-22); audit register at 100
      findings with reproducible evidence per finding (one open: F-98).

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
- [x] **Commit the professionalization pass** — done 2026-08-26: landed,
      pushed, and hosted-green (see `plan.md`'s end-of-day state). Stale by
      the time this session picked the file back up; `git status` against
      `origin/main` is clean.
- [x] Delete `fhir-tmp-stash/` — done 2026-08-27: it was untracked (one
      `.DS_Store`, nothing `git rm` would have touched), so removed directly
      from disk. Nothing to commit for this one.

### Security and supply chain

- [x] **Widen `fhir-security.yml`'s `cargo deny` beyond `fhir/**`** — done
      2026-08-26: a nine-workspace matrix, each against its own `deny.toml`,
      path-triggered on any workspace plus the weekly cron. Verified
      locally first: all nine green, with `fhir-mssql`'s green resting on
      its deny.toml's dated F-67 ignores (`M14.34`) — the acceptance stays
      visible.
- [x] **Get an owner decision on F-67** — done 2026-08-28: **accept the risk
      formally, keep shipping on upstream `tiberius`, document it loudly.**
      Reached after investigating three alternatives, each priced before
      being set aside — a from-scratch driver (~3–4.5 months, and worse on
      the trust axis than the flawed 6-year incumbent), a fork carrying the
      one upstream fix that exists (`prisma/tiberius#419`; 1–2 weeks plus an
      open-ended maintenance tail, and unusable by a published crate as a
      git dependency regardless), and one newer alternative crate (`ms-tds`,
      disqualified on sight — its own description advertises
      offensive/exploitation tooling). Full account: `M14.34` in
      `fhir-mssql/spec/14-mssql-dialect.md`. `deny.toml`'s ignores, the audit
      register, and every document naming this risk now say so. Unblocks
      `PM-4`.
      **Superseded 2026-08-29: actually resolved, not just accepted.** The
      owner published `mssql` (github.com/joelparkerhenderson/mssql-rust), a
      `tiberius` fork maintained to carry the fixes forward — switching to
      it clears all four advisories (`rustls-webpki 0.103.15` now, none of
      the four packages remain in the tree). **F-67 closed** — but **F-98**
      was filed the same day (found bumping `sha2`/`sha3`; see
      `spec/databases/audit.md`), so the register is not empty.
- [x] Create git tags and GitHub releases for the published versions
      (`PM-70`) — done 2026-08-26: sixteen annotated tags at `e28964e`
      pushed, sixteen releases with `TG1.8`-compliant notes, `fhir 4.1.0`
      marked latest, `releases.atom` live; NEWS.md, MAINTAINERS.md,
      CHANGELOG.md, and the outreach register updated in the same change.
- [x] Sign commits and tags going forward — done 2026-08-27: SSH commit/tag
      signing configured `--local` to this repository (not global, so it
      cannot silently activate elsewhere) with a passphrase-protected
      ed25519 key held only on the maintainer's machine. The public key is
      committed at `.github/jph-code-signing.pub` for self-contained
      verification rather than relying on an external profile. Verified the
      configuration is structurally correct by attempting a real signed
      commit before the key was loaded in the agent: it failed cleanly on
      the passphrase prompt (`fatal: failed to write commit object`) rather
      than silently succeeding unsigned or hanging, and left no stray
      commit. Every commit and tag before 2026-08-27 stays unsigned
      (MAINTAINERS.md records the cutover and does not rewrite history).
- [x] Decide the publishing shape — decided by the owner 2026-08-26:
      **documented laptop publishing stays, permanently**, because GitHub is
      not reliable enough to hold the publish path (decided hours after an
      Actions major outage). Executed in the same change: the six inert
      per-port `publish.yml` workflows deleted per MAINTAINERS.md's own
      rule, the six `doc/ci.md` rows and `agents/release.md` updated, and
      MAINTAINERS.md's token row corrected — verified via the API that
      GitHub stores no registry secret and no `crates-io` environment
      exists (the row had claimed one); the only credential is the
      maintainer machine's `~/.cargo/credentials.toml`.
- [x] Enable GitHub private vulnerability reporting, dependabot, and add
      `.github/ISSUE_TEMPLATE/` — done 2026-08-26, secret scanning too:
      PVR flipped via the API and verified enabled; dependabot alerts and
      automated security fixes enabled, with `.github/dependabot.yml`
      registering every lockfile but capping cargo version-update PRs at
      zero — its first hour on default limits opened 47 major-bump PRs,
      each triggering a port's full live-database CI; all 47 closed with
      the reasoning, which also lives in the file (security advisories
      still open PRs regardless of the cap; `fhir-security.yml` fails the
      build on any known advisory).
      **Posture changed 2026-08-29 per `spec/dependabot/`:** the zero cap
      is lifted — each ecosystem now uses GitHub's default limit (5 open
      PRs) per directory instead. Repo-level alerts and automated security
      fixes were already on; re-verified via the API the same day
      (`vulnerability-alerts` 204, `automated-security-fixes` `enabled:
      true`) before writing this down. Bug-report and wrong-claim templates
      carry the never-paste-patient-data rule and a stated response
      expectation; SECURITY.md names the Security tab as a private channel
      alongside
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
      sat one unpublished version ahead — published in full 2026-08-26,
      34 matched at the gate.
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
      **Re-verified 2026-08-27, still current** — `fhir-oracle` remains the
      only port missing `redaction.rs`/`concurrency.rs` (confirmed by file
      presence in all six `tests/`), `fhir-postgresql` remains the only port
      with `audit.rs`/`chain_portability.rs`, and the conformance matrix
      still shows five of six below Reference. No edit needed.

### Outreach

- [ ] Was blocked on `PM-70`–`PM-75`. Tags/releases, signing, the F-67
      decision, the GPL-scanner note, and the licensing fixes are all now
      done — F-67 itself resolved outright 2026-08-29 (driver switch, see
      above), not merely accepted.
      **Still genuinely incomplete: `PM-72` benchmarks** — one port of six has
      a real harness, and the JSONB-vs-relational comparison the whole pitch
      rests on has never been run. That is what remains before
      `help/outreach/index.md` phase 1 executes. The claims register governs
      every public sentence regardless.

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
