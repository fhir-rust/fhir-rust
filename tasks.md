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
- [x] 34 crates published to crates.io (2026-08-22); audit register at 102
      findings with reproducible evidence per finding (none currently open).

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

## Capability roadmap (proposed, triaged) — 2026-09-03

**This section is different from the rest of the file.** Everything above is
a professionalization item: verified done, or a known gap in governance,
compliance, or hygiene. What follows is **not done, not speced, and not
normative** (`spec/index.md`'s precedence rule, restated in `GOVERNANCE.md`:
nothing in this file decides anything) — it is a candidate list of FHIR®
capabilities this repository does not yet serve, found by reading HL7®'s own
RESTful API description (`hl7.org/fhir/http.html`) against `fhir-loco`'s
spec (`SV1`–`SV4`) and the [conformance matrix](spec/databases/conformance-matrix.md),
rather than by guessing. Pursuing any item means writing the requirement
into the spec first ([`agents/spec-workflow.md`](agents/spec-workflow.md),
`C0.19`–`C0.22`), same as
everything else here — this list is the "should we" input to that step, not
a substitute for it.

Two things HL7 documents that this project has already, deliberately,
decided **not** to build are left off this list rather than listed as gaps:
a validation service and an authorization server (`SV1.4` — both explicitly
out of scope, restated here so nobody re-proposes them without reading why).

### Tier 1 — small-to-medium effort, no store-layer blocker

- [x] **Surface HTTP conditional delete where the store already has it** —
      done 2026-09-03: `DELETE /{version}/{rtype}?params` (`SV2.19`,
      `fhir-loco` 0.3.3). No match and single match both answer `204`
      (idempotent, matching instance-level `DELETE`); multiple matches
      answer `412` with `SV2.14`'s reason shape; no criteria at all is
      refused with `400` rather than treated as "delete the type's one
      resource if it has exactly one". Verified against both mounted
      backends — SQLite by a dedicated outcome-table test, PostgreSQL by a
      live-container run (`FHIR_LOCO_TEST_PG=1`), not assumed from the
      shared code path. `cargo fmt`/`clippy -D warnings` clean.
- **Conditional read** (`If-Modified-Since` / `If-None-Match` on `GET`,
  answering `304`) — a standard, small RESTful capability with no mention
  in `SV2` today; no store change implied, since it only needs the
  `last_updated`/ETag data `get` and `vread` already return.
- **`_elements` and `_summary` search/read parameters** — bandwidth-saving
  partial responses HL7 defines alongside `_count`/`_offset`/`_total`
  (`SV2.12`'s neighbours); adds a projection step after the store read,
  no store change.
- **`_since` on Bulk Data `$export`** — `SV2.15` refuses it by name today
  ("`_type` is the one supported parameter"), which means every export is
  a full dump; incremental export is the single most common real-world use
  of Bulk Data and the store already tracks `last_updated` per resource,
  so this is a filter addition to the existing per-resource-read
  implementation, not new store machinery.
- **Batch Bundles.** `SV2.18` refuses `POST /{version}` batch Bundles with
  `501`, naming the reason plainly: batch is not atomic and "is
  implementable without the [transaction] question" — it is refused only
  because nobody has built it yet. Building it is looping the existing
  per-resource routes over a Bundle's entries and assembling the response
  Bundle; no new store capability, unlike transaction Bundles below.

### Tier 2 — high value, blocked on store-layer work first

- **Bring `fhir-mysql`, `fhir-mariadb`, `fhir-mssql`, and `fhir-oracle` to
  audited-write parity with `fhir-postgresql`/`fhir-sqlite`.** The
  conformance matrix shows `conditional_create`, `conditional_delete`,
  `put_audited`, `delete_audited`, and `history_page` (type/system) all
  `—` on those four ports today — meaning `fhir-loco`'s conditional-create
  endpoint (`SV2.14`) and type/system history endpoints (`SV2.17`) can only
  ever be meaningfully exercised against two of the six backends it could
  mount. This is the single largest lever for making the other four ports'
  **Store → Reference** gap concrete rather than descriptive: it is exactly
  what separates the two levels per `CLAUDE.md`'s own framing, restated for
  each port as its `M14.x` dialect work once someone picks it up.
- **Transaction Bundles.** `SV2.18` refuses these with `501` too, but for a
  named, harder reason: atomicity needs the store's `put`/`delete` to run
  inside one caller-held transaction, and `transact_audited` is `•` on
  `fhir-postgresql` alone (`~` on `fhir-sqlite`, `—` on the other four).
  This is the capability most FHIR clients assume exists (`Bundle` of type
  `transaction` is core to the spec, not an extra); it is also genuinely
  hard here, since compensation-based atomicity was already investigated
  and rejected (a reader could observe a half-applied bundle, and a
  crash mid-unwind would leave it that way permanently, per `SV2.18`'s own
  reasoning). Worth a dedicated design pass, not a quick patch.
- **A real `export` store operation.** `export` is a named library
  operation in `spec/databases/00-conformance.md` (alongside `put`, `get`,
  `purge`) and shows `—` on **all six ports** in the matrix — it does not
  exist anywhere. `fhir-loco`'s current Bulk Data `$export` (`SV2.15`)
  works around this with a sequence of per-resource reads, which its own
  spec text already flags as **not one transaction-time snapshot**. A real
  `export` primitive (snapshot-consistent, streamed rather than read one
  row at a time) would fix that honestly-stated gap and likely outperform
  the current approach on large exports besides.
- **`$everything` and compartment-based export
  (`Patient/$export`, `Group/$export`).** All three need "compartment
  machinery" `SV2.15` says plainly the store does not have. `$everything`
  is one of the most commonly expected FHIR operations in practice
  (a clinician's "give me this patient's whole record" button); bundling
  it with compartment export makes sense because both need the same
  underlying capability — a compartment definition and a way to resolve
  "everything referencing or referenced by resource X" — built once.

### Tier 3 — real capability, needs a scope decision before design

- **Subscriptions** (the R4/R5 notification framework: `Subscription`
  resources, channel types, event delivery, retry). Not mentioned anywhere
  in `fhir-loco`'s spec — not deferred, not refused, simply absent. This is
  a materially different kind of capability from everything above: it
  needs background delivery, a retry/backoff policy, and a new failure
  mode (a subscriber that never acknowledges) that nothing in the current
  request/response model has to reason about. Worth a scope conversation —
  does `fhir-loco` become a service with background workers, or does this
  stay explicitly out of scope alongside validation and authorization? —
  before any `SVx` requirement gets written, not after.
- **JSON Patch** (`PATCH` route, restates HL7's `http.html` patch
  interaction) and conditional patch. No route exists today (`SV2.1`'s
  table has no `PATCH`). Needs a scope decision of its own: is a patch
  audited the same way `put_audited` is, and does partial-update semantics
  fit this crate's "translate, don't decide storage behaviour" principle
  (`SV1.1`) as cleanly as a full-resource `PUT` does?
- **System-level search and delete** (`GET {base}?_type=Type1,Type2`,
  system-level conditional delete). Lower real-world demand than the
  items above — mostly used by bulk tooling and admin scripts rather than
  clinical clients — listed for completeness rather than urgency.
- **Real cursor pagination for `_history`.** `SV2.17` already names this
  as "future work, not an implied promise" rather than a gap discovered
  today; repeated here only so it sits alongside the rest of this list
  instead of only inside one requirement's prose.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
