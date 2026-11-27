# Tasks

Repository-level professionalization checklist; rationale and workstreams
live in [`plan.md`](plan.md). A `[x]` here means the work is **verified
done**, not intended — check items off in the same change that completes
them, with the evidence named.

**This file is not engineering status.** The ports' own `tasks.md` files have
a documented history of false `[x]` claims (**F-27**); read the
[conformance matrix](spec/databases/conformance-matrix.md) and each port's
dialect annex for what works. Nothing here speaks for a port.

## Coverage — 2026-11-26 (macOS arm64, cargo-llvm-cov 0.9.1)

- [x] **`fhir-sqlite-store` above 95%:** **98.92%** lines (4,307/4,354;
      `sqlite.rs` 98.67%, `sqlite_search.rs` 99.71%), full suite serial
      (`sqlite_store` 34, `upgrade` 14, all green; ~60 min instrumented),
      integration-test sources excluded as in `scripts/coverage.sh`. The
      100% item below stays open: 47 lines remain.

- [x] **`fhir-store` line coverage above 95%.** Was 91.73% (366/399); now
      **99.32%** (435/438). Covered: `ChainKey::from_env`/`KeyRing::from_env`
      with an unset prefix, retired-key parsing split into
      `KeyRing::push_retired` (ordering, blank entries, malformed `id=hex`,
      bad hex; mutation-verified), an unreadable key path, and
      `Audit::cli_as`. A decorative loop in
      `digests_are_reproducible_but_the_mac_is_not` (its `guess` was never
      used) became one real assertion. Remaining: line 386, the
      `if let Some` push in `KeyRing::from_env` with a key set, which needs
      env mutation (`unsafe` under `forbid(unsafe_code)`).

## Work in progress handoff — 2026-11-20

- [~] **Raise `fhir-sqlite-store` production line coverage to 100%.** The last
      completed authoritative full-suite measurement is **97.71%**
      (3,671/3,757 lines; search SQL 926/926), produced on macOS arm64 with
      remapped paths and integration-test sources excluded from the denominator
      while all tests still executed. Since that measurement, the worktree adds
      deterministic rusqlite-authorizer fault injection, corrupt stored-map and
      conflicting-DDL upgrade tests, a 1,001-value multi-batch fold backfill,
      no-history and moved-column migration guards, and directly testable lazy
      row decoders for search, references, reconstruction, history, audit, and
      disclosure rows. The 22-test unit suite passes; the expanded 14-test
      upgrade suite passed 13 tests in one serial run, and its sole fixture
      failure (foreign-key enforcement on direct setup rows) was corrected and
      re-run successfully in isolation. The implementation and this evidence
      are committed and pushed as `d89cc3b`. Two subsequent authoritative full
      coverage attempts were interrupted at the user's stop requests; the
      second had not produced
      `/private/tmp/fhir-sqlite-full-post-commit.json`, and its process-status
      check was itself cancelled, so do not infer either a pass or a failure
      from that attempt. Next: first confirm no stale `cargo llvm-cov` process
      remains, then rerun
      `fhir-sqlite/scripts/coverage.sh`, use the new exact uncovered-line list to
      close the remainder, and raise `--fail-under-lines` from 95 to 100 only
      when that command passes. Do not cite 100% before then.

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

### Deployment verification

- [x] **Give Oracle readers a single committed version** (db:R4.5, T11.6,
      Oracle M14.19). On Oracle 26ai Free 23.26.2.0.0, a read immediately
      after schema installation succeeded and all 80 reads paired with
      writes on independent connections returned a whole version. With
      snapshot protection removed, the first pair mixed an old root status
      with a new repeating child. See the
      [matrix](spec/databases/conformance-matrix.md).
- [~] **Rehearse recovery on the mounted stores first** (O10.6, T11.16).
      The SQLite integration test exercised the full bundled R5 map on
      SQLite 3.54.1. It recovered the resource and its earlier version,
      installed checksum, disclosure attribution, both signed entries,
      and the checkpoint kept outside the two snapshot files. A later
      deletion was absent from restored history.
      PostgreSQL 18.7 passed a local full-R5 custom archive restore with
      the same checks, including the retained chain witness. Add a
      repeatable PostgreSQL store test. MySQL 8.4.13 also passed a local
      keyed logical restore over a Patient slice of the released R5 map.
      MariaDB 11.4.13 passed the equivalent keyed Patient-slice drill with
      its native dump and same-name database restore. Azure SQL Edge
      15.0.2000.1574 passed a native checksummed whole-database backup and
      restore with the same assertions. Oracle 26ai Free 23.26.2.0.0 passed
      a Data Pump export/import of the keyed Patient slice, including the
      pre-provisioned-user recovery required by M14.5/M14.28. Add checked-in
      drills for the five server ports, and repeat the SQL Server exercise on
      full SQL Server 2022 before treating it as product conformance.
      Track each engine's results in [F-104](spec/databases/audit.md#f-104).
- [ ] **Exercise HTTP overload at the clinical listener** (SV4.2). Hold
      admitted requests open at a small configured limit. Assert that the
      next request receives `503` and `Retry-After` before a store call.
      After a held request completes or is cancelled, an ordinary request
      must succeed. Record the tested service or perimeter configuration and
      distinguish any deployment mitigation from the service's own status.
      [Production settings](fhir-loco/config/production.yaml)
- [~] **Run multilingual search through each store** (L15a). On 2026-11-18
      SQLite R5 returned all three synthetic IDs for the voiced kana query.
      Exact search distinguished all three spellings, including the decomposed
      form, and each Patient retained its input. Add the full default/exact
      table to the proposed cross-database suite below, run it per engine,
      and mutation-check that using a folded key for exact search fails.
      [Fixtures and expected results](spec/databases/locale-accent-folding.md)
- [ ] **Verify the first regional profile requested by an adopter.** Record
      the implementation-guide package and version, base release, terminology
      versions, validator and configuration. Validate a synthetic accepted
      example and one that violates a named profile constraint. Exercise the
      required interactions on the selected backend and publish the observed
      outcomes and outstanding gaps alongside the support claim.
      [Scope of regional claims](spec/professionalization/index.md#rules)

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

- [x] **Repair the local Rust compatibility check** (2026-11-18, RV1.5).
      `make ci-minimum-supported-rust-version` now calls the renamed
      script, resolves installed patch releases, and checks and tests whole
      workspaces with their lockfiles. A recording Cargo stand-in verified
      both commands in all nine directories using real Rust 1.98.1 and
      1.99.1 toolchains. The persistence core also passed an actual locked
      all-target check on 1.98.1. The full compatibility suite was outside
      this review.
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
      2026-08-26: all six are now copies of `LICENSES/apache-2.0/index.md` (§4 of
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

### Tier 4 — adoption tooling (proposed 2026-11-17; items 1–2 of the source list were not supplied)

Each of these touches a claim the project makes publicly, so the claims
register governs every sentence, and each starts from an artifact that
already exists rather than from nothing.

- [ ] **Migration tools from JSON-based FHIR stores.** Import of existing
      JSON/JSONB-backed resources (NDJSON, Bulk Data export files, a document
      table) into any port via `put`, with a lossless-round-trip check per
      resource (`R4.x`, db family) and a report of anything the shredder
      refuses. Needs a spec section first (`C0.19`–`C0.22`); decide whether it
      is a library function, a `fhir-loco` operation, or a new CLI crate
      (there is no CLI crate today).
- [ ] **Performance comparison against other FHIR implementations.** Blocked
      on `PM-72`: only `fhir-postgresql` has a measured harness and the
      JSONB-vs-relational comparison has never been run. `BENCHMARKS.md`
      (`W16.10`: numbers name what measured them and when) and
      `COMPARISONS.md` (qualitative) are the homes; no number may be
      presented for a port that has not produced it.
- [ ] **Implementation guides for common FHIR use cases.** Task-oriented
      guides (store and search a Patient, history and `vread`, audited
      writes, running `fhir-loco`), written per `agents/documentation.md`;
      each example must be run, not assumed.
- [~] **Compliance documentation showing which FHIR features are
      supported.** *First, hand-derived version written 2026-11-17:
      [`doc/fhir-feature-support.md`](doc/fhir-feature-support.md). Still open:
      generate it, and inventory search modifiers per port (marked `?` there).* Publish a per-feature support statement — resources and
      releases (R3/R4/R5), interactions, search parameters and modifiers,
      operations — derived from the
      [conformance matrix](spec/databases/conformance-matrix.md) and
      `fhir-loco`'s `SV2`, ideally as a generated `CapabilityStatement`
      so it cannot drift from the code. It must report `•`/`~`/`?`/`—`
      honestly per port (e.g. batch/transaction Bundles `501`, `_since` on
      `$export` refused) and is distinct from `13-compliance-mapping.md`,
      which maps regulatory controls, not FHIR features.

Second batch (same date; item 1 of this list was not supplied). Each entry
says what already exists, so none is re-derived from scratch:

- [ ] **Performance benchmarks for each database backend.** This is `PM-72`
      (see Outreach above), not new work: one port of six has a harness.
      Extending it to the other five must respect `W16.10` — dialect-specific
      numbers per engine, never the PostgreSQL figures with a name swapped
      (**F-64**).
- [ ] **Regular security audit.** `SECURITY.md` and the `fhir-security.yml`
      workflow exist; what is missing is a *recurring* human-reviewed audit
      with findings filed in `spec/databases/audit.md`. Needs a cadence and an
      owner decided first (`GOVERNANCE.md`).
- [ ] **Better error messages for FHIR validation failures.** Note the scope
      line: validation is out of scope for `fhir-loco` (`SV1.4`), so this
      means clearer `OperationOutcome`s for the failures the store and
      deserialiser already raise (`09-validation.md`, `SV2.14`), not a
      validator.
- [ ] **Caching layer for frequently accessed resources.** Needs a scope
      decision before design (Tier 3 in kind): a cache over audited,
      versioned clinical data must keep `get`/`vread`/history and the access
      log honest — a cached read still has to be logged, and a stale read after
      `put`/`delete`/`purge` is a correctness bug. Conditional read (Tier 1)
      gives most of the client-side benefit with none of that risk.
- [ ] **Complex FHIR search parameters.** Search is already specified
      (`spec/databases/06-search.md`); the matrix records chained-reference
      search as only partly tested. The work is to enumerate which parameter
      types, modifiers, `_include`/`_revinclude` and chained forms are `•`
      per port, then close the gaps — feeds the compliance documentation item
      above.
- [ ] **Cross-database integration tests.** Per-port live suites exist
      (`fhir-<engine>-ci.yml`) and the shared core is gated byte-for-token by
      `scripts/check-shared-core.sh`; what does not exist is one suite run
      against all six engines asserting identical observable behaviour
      (same resources in, same search results, history and reconstruction
      out). Mutation-verify it (`T11.10`, `L16`).

## Engineering and clinical-safety audits (imported 2026-11-25)

Two external audits against commit `1a00c40`, both now deleted from
`docs/audits/` and recorded here. Non-normative: findings are unverified until
filed in `spec/databases/audit.md`.

### Engineering audit (VS Code contributor workflow)

- [x] **HTTP check runs against PostgreSQL when either side changes** — done
      2026-11-25: `fhir-loco-ci.yml` triggers on `fhir-postgresql/**` and has
      a `postgres-backend` job (PostgreSQL 18 service); `pg_backend` is
      `#[ignore]` and run with `--ignored`, so it fails without a database.
      Still to verify: a hosted run, and the failure with the service
      removed.
- [x] **Shared-core gate discovers generator files** — done 2026-11-25:
      `check-shared-core.sh` compares the union of `gen/src` and `gen/tests`
      paths across all six ports; verified with a PostgreSQL-only file and a
      SQLite-only file. `CONTRIBUTING.md` no longer hardcodes a file count.
- [~] **Focused VS Code opening path** — `CONTRIBUTING.md` table and a
      `persistence.code-workspace` pilot (linked project, tasks, CodeLLDB
      "Debug audit preimage") written 2026-11-25. Unverified interactively:
      Go to Definition on `preimage`, diagnostic click-through, breakpoint
      showing `version_id = 3`, `actor = "clinician"`, cold-index timing,
      Windows/Linux. Do not add a blanket `**/target/**` exclude (it hides
      tracked `fhir/src/r5/parse/all/target/`).
- [ ] Optional: R4-only rust-analyzer preset for `fhir/`
      (`cargo.noDefaultFeatures` + `features = ["r4"]`); measure time to first
      Go to Definition before adding more presets.

### Clinical information safety audit

Reproduced on SQLite and PostgreSQL 18.7, R4 and R5 maps, synthetic data.

- [~] **Patient search must keep the target type.** *Landed 2026-11-25 in all six ports: `TargetKind::Reference` carries `types` from `where(resolve() is T)`, the six store `target_pred`s apply them to bare ids and `Type/id`, maps regenerated (R4, R5). Verified: sqlite end to end (full workspace green), other five lib + assets_current + roundtrip. NOT done: R3 (its expressions carry no restriction; could use SearchParameter `target`), HTTP-level test on both backends, live-engine runs on the other five, mutation check.* `Observation?patient=shared`
      returns an Observation whose subject is `Group/shared`: the generator
      drops the FHIRPath type restriction and bare IDs compare ID alone
      (cf. F-38). Carry permitted target types into compiled search targets
      and bundled maps; `subject` stays broad. Shared generator change in all
      six ports; HTTP test on both backends.
- [~] **Reject or implement `:not` (and other unsupported modifiers).** *Rejection landed 2026-11-25 in all six store `target_pred`s (modifier on a non-string, non-reference target → `Unsupported` → HTTP 400); unit-tested on sqlite only, other five compile-checked; no HTTP-level test or live-engine run yet; `:not` itself still unimplemented.*
      `status:not=entered-in-error` returns exactly the `entered-in-error`
      records. Smallest safe fix: explicit 400 OperationOutcome. If
      implemented, test the complement over repeating codings and missing
      values. Then publish which name/modifier pairs are supported
      (`doc/fhir-feature-support.md`).
- [~] **Quantity equality uses query precision.** *`eq`/`ne` on Number and Quantity widened to the precision interval 2026-11-25 in all six store crates (`precision_bounds`/`numeric_pred`; unit-tested on sqlite, compile-checked and lib-tested elsewhere, no live-engine run except sqlite's own suite). Still to do: live-engine tests per port, and a mutation check; lt/gt/ge/le unchanged; exponent notation keeps exact comparison.* `5.0|ucum|mg` must match
      4.95 <= v < 5.05 (5.04 mg). SQLite casts to REAL, PostgreSQL uses
      numeric `=`; both miss. Tests for `5`, `5.0`, `5.00`, exact endpoints,
      different-unit control. Unit conversion is a separate capability.
- [~] **Enforce minimum cardinality at write (V9.1).** *Landed 2026-11-27 in all
      six ports (see **F-111**); `fhir-loco` returns 422 naming the element.
      Live-verified on all six engines after CI caught
      PostgreSQL's live corpus test (see F-111). NOT done: the read-only
      inventory of existing rows, checks inside contained/inline/spilled
      subtrees.* An Observation without
      `code` (1..1) is accepted. Carry min cardinality in the map, enforce in
      the shared shredder, HTTP 4xx OperationOutcome naming
      `Observation.code`, nothing committed on rejection. Fix the SQLite
      `history_page.rs` fixture that relies on a code-less Observation.
      Provide a read-only inventory for existing databases; never invent a
      code.
- [~] File the four findings in `spec/databases/audit.md`; update the feature
      matrix and the README "Production today" PostgreSQL recommendation;
      revert each fix in isolation to show its test fails. *Filed as
      **F-108**–**F-111** 2026-11-26; F-108–F-110 mutation-verified on
      SQLite (each reverted fix fails
      `patient_search_keeps_its_target_type_and_quantities_use_precision`);
      other five ports not mutation-tested. Matrix rows added and the README
      "Production today" row now points at F-108–F-111 (2026-11-26).*
- [ ] **Clinical acceptance suite** (synthetic, reviewer-signed): corrected
      result is current while `vread` gives the earlier one; allergy
      none-known / unavailable / missing section stay distinct; medication
      order status/intent/negation survive readback and lossy conversion is
      surfaced; same MRN under two assigning systems never merges.
- [ ] **Pilot profile pinning.** One exchange profile with FHIR release, IG
      package and terminology versions pinned; record structural, profile,
      terminology and clinical-review results independently.
- [~] **PHI inventory correction.** *2026-11-26: `PHI.md` names the export
      files, their location, lifetime and the deployer's backup duty;
      `export::start_retention` (called from `before_run`) deletes job
      directories orphaned by a restart (UUID names only) and sweeps on a
      timer every `min(TTL, 60 s)`. Unit test
      `orphaned_export_directories_are_removed_and_nothing_else`,
      mutation-verified (UUID guard, live-job guard). NOT done: an
      end-to-end export that expires across a real process restart.* `PHI.md` says PHI is only in the
      database, but bulk export writes NDJSON files; expiry runs only at
      export kickoff over an in-memory registry, so a restart can orphan
      files. Document location/retention/backups; reconcile on restart and
      on a timer; prove with an export that expires across a restart.
- [~] **Erasure and access wording.** *2026-11-26: `PHI.md`'s answer and a new
      "Erasure" section state purge's one-resource scope, the tombstone, and
      what it does not reach (referencing resources, access log, backups,
      exports, policy; checked against SQLite `purge`). NOT done: the privacy
      lead's verification across reads, `_include` searches, history and
      exports.* "Can I erase a patient?" should state
      that purge acts on one resource type+ID and leaves a tombstone
      (M3.18); wider record disposition, consent and patient-context policy
      belong to the deployer (SMART scopes). Privacy lead verifies reads,
      searches with includes, history and exports.

## Release blocker: PostgreSQL crates diverge from crates.io (2026-11-27)

- [ ] `fhir-postgresql CI` job "No version diverges from crates.io" (O10.11) is red on every `main` run back to 648bf92, before the F-111 work. Published `fhir-postgresql-gen` 0.6.3 differs from the tree (LICENSE.md, plus the F-111 `shred`/`model` changes). Fix: bump the affected crates' versions, changelog them, and publish. Owner release decision; not done. Confirm the other ports' crates are unpublished at their current versions (the SQLite check passes).

## Dependabot and review requests (2026-11-27)

Handled by subagents in separate worktrees, then merged locally and re-verified together; **nothing pushed, no PR commented on, merged or closed** (that is yours to do):

- [x] **#142 fhir-loco 401 carries an OperationOutcome** (maxfunke) — conflicted with the credential gate; ported onto main (`unauthorized()` now builds `outcome()`, keeps `WWW-Authenticate: Bearer`), author's test kept and extended to gated reads, `/metadata` and `$export-status`; mutation-verified. Close #142 in favour of the merged commit, crediting the author.
- [x] **#134 + #135 rusqlite 0.43** — merge only together (both link bundled SQLite; loco also has a rusqlite dev-dependency). Taken as one change in fhir-sqlite and fhir-loco; no source change; same bundled SQLite 3.54.1; full sqlite store suite and loco suite green.
- [x] **#133 jiff 1.0.1 (oracle), #138 (loco)** — taken. **#137 (mssql) is broken as proposed** (mssql 2.1's `jiff` feature implements `FromSql` for jiff 0.2 only, E0277); the working change is mssql driver 2.1 → 3.0.0 together with jiff 1.0. Only oracle, mssql and loco use jiff. Verified: fmt, clippy, MSRV `--locked`, offline suites, and the SQL Server (`azure-sql-edge`) live suite.
- [x] #136 serde_with and #141 README (docs draft) were already closed; nothing to do.
- [ ] **Flaky deadlock in the mssql upgrade tests.** `fhir-mssql-store/tests/upgrade.rs` shares one database across tests; under the default parallel test threads it fails about 1 run in 4–6 with error 1205 ("deadlock victim") in a different test each time, on driver 2.1 **and** 3.0 (baseline `ead4480`: 1 of 6; merged: 1 of 4 and 1 earlier; 0 of 4 with `--test-threads=1`). Not introduced by the bump. CI should run that suite with `--test-threads=1` (the oracle port's guide already says to) or give each test its own database.
- [ ] **Local `cargo deny` reports `chrono` unmaintained** (RUSTSEC-2026-0543, via loco-rs) and two stale `deny.toml` ignores (RUSTSEC-2026-0194/0195 no longer match). CI's log did not show the chrono error; re-check with CI's cargo-deny version before touching `deny.toml`.

## Production and quality audits (imported 2026-11-26)

Two external audits, both deleted from `docs/audits/` and recorded here.
Non-normative until filed in `spec/databases/audit.md` (or the
`fhir-loco` spec). The GPT audit's summary referred to "16 prioritized work
items" in a full report that was never in the file; only its six headline
findings arrived, and all six were **verified against source 2026-11-26** at
`648bf92` (file:line below). Live-engine suites and the HTTP request suite
were not rerun for this triage.

### Production audit (GPT, against `648bf92`) — release blockers

- [~] **P0 — authenticate every FHIR read before store access (SV3.1).**
      *Landed 2026-11-26: `fhir::require_credential` layered over
      `fhir::routes()` and `export::routes()` (401 before any handler; the
      refused attempt is still disclosure-logged). Test
      `every_read_without_a_valid_token_is_refused` walks ten read/export
      paths with no token and a foreign-issuer token; mutation-verified
      (layer removed → `200` on `/r5/Patient/gate-1`). 24 tokenless reads in
      `tests/requests/fhir.rs` and 6 in `tests/pg_backend.rs` now carry a
      token. Full SQLite request suite green (28). NOT run: `pg_backend`
      (no PostgreSQL here; compiles only). **Decided 2026-11-26 (owner): gate
      `/{version}/metadata` too** — SV3.1 unchanged; the test now asserts
      401 for it without a token.*
      `read`, `vread`, `search`, `history`/`type_history`/
      `system_history` (`fhir-loco/src/controllers/fhir.rs`) never gate on
      `auth::audit_from`; only `disclose` calls it, and on failure it logs
      actor `"unauthenticated"` and the handler **returns the resource
      anyway**. Mutations and export are gated; reads are not. Fix: one
      router-level layer (or extractor) that refuses with 401 before any
      handler runs, except `/metadata` if SV3 says so; keep the
      rejected-attempt disclosure. Test: a table over every route in
      `routes()` asserting 401 without a token and with a wrong-key token,
      and fix the existing request test that reads without a credential.
      Mutation-verify by removing the layer (`T11.10`).
- [~] **P0 — no clinical content without a recorded disclosure (PR12.5).**
      *Decided and landed 2026-11-26: fail-closed by default. `disclose`
      returns the 503 OperationOutcome; all eight content-returning call
      sites (read ×2, vread, conditional create, search, history ×2,
      export file) return it. Opt-out `FHIR_LOCO_DISCLOSURE_FAIL_OPEN=true`,
      warned at boot. Test
      `a_read_whose_disclosure_cannot_be_recorded_is_refused` (trigger
      aborts access-log inserts → 503, no body; removed → 200),
      mutation-verified. NOT done: per-site tests beyond read, the opt-out
      path, a `fhir-loco/spec` line (spec files were being edited
      concurrently).*
      `disclose` (`fhir.rs`, "A logging failure is reported but never
      fails the request") and the export file handler send PHI after
      `log_access` fails. That doc comment is a deliberate choice that
      contradicts PR12.5's "every read appends an access record"; decide in
      the spec (fail-closed default, perhaps an explicit opt-out), then make
      `disclose` return `Result` and answer 503 OperationOutcome on
      failure. Test with a store whose access-log insert fails (the SQLite
      authorizer fault injection already exists).
- [~] **P0 for credentialed paging — client leaks the bearer to any origin.**
      *Landed 2026-11-26: `ReleaseClient::same_origin` resolves the link
      against `base_url` and refuses (`ReleaseClientError::Url`, origin only,
      no query echoed) any other scheme/host/port; `search_all` checks the
      budget before each request. Tests in `fhir/tests/client_r5.rs`
      (cross-origin refused and the foreign server gets no request;
      same-origin absolute and relative links followed with the token;
      `max_pages` 2 and 1 issue exactly 2 and 1 requests); both fixes
      mutation-verified. Spec: `R13.9a` added in
      `fhir/spec/13-assurance.md`. Found
      while gating, pre-existing at HEAD: `RUSTDOCFLAGS=-D warnings cargo doc
      -p fhir-core` fails with "redundant explicit link target" under any
      feature set, with no source span (likely macro-generated docs); the
      workspace-wide `cargo doc --no-deps` gate in `fhir/CLAUDE.md` passes,
      so this only bites a per-crate doc build. Also
      the `fhir-core/src/lib.rs` `client` doctest (line 72) cannot compile
      under `-p fhir-core` because the facade's `client` feature is off.*
      `fhir/fhir-core/src/client.rs` `next_page` sends
      `self.http.get(next)` through `send`, which attaches the bearer, for
      whatever URL the server's `next` link names. Require same scheme +
      host + port as `base_url` (else a typed error, no request). Also
      `search_all` calls `next_page` *before* checking `max_pages`, so it
      issues one request past the budget; check first. Tests: a foreign
      `next` host gets no request and no token; `max_pages = 1` issues
      exactly one request.

### Production audit — correctness

*The three SQLite store findings below are filed as **F-105**–**F-107** in
`spec/databases/audit.md` (2026-11-26). The `fhir-loco` and client
findings belong to those families' specs, not the database register.*

- [~] **Read pairs a body with a version fetched separately.**
      *2026-11-26: `fhir-loco` now reads through one
      `AnyStore::get_versioned` (`read`, conditional-create's existing
      branch), and `vread` reads once. PostgreSQL's `get` already returned
      `Got` from one snapshot; `SqliteStore::get_versioned` (and `get`) now
      read `version_id` inside the reconstruction's own transaction
      (`read_current`) instead of a later `status` call. Full SQLite store
      and fhir-loco suites green. Test
      `get_versioned_pairs_a_body_with_its_own_version` (second handle, 150
      racing writes) — the old `get`+`status` fails it 2/2 runs ("generation 0
      paired with version 6"). PostgreSQL has no equivalent test.*
- [x] **`SqliteStore::status` says one transaction, takes none, and hides
      errors.** *Fixed 2026-11-26: deferred `unchecked_transaction` over
      both queries, `.optional().store()?` instead of `.ok()`; test
      `status_reports_a_failed_read_as_an_error_not_unknown` (authorizer
      denies `patient`, then `patient_history`), mutation-verified. Checked the
      other ports: PostgreSQL already uses one snapshot and `query_opt`;
      mysql, mariadb, mssql and oracle have no `status`.* Its comment claims "one read transaction over both tables",
      but the two `query_row`s run on the bare connection, and each `.ok()`
      turns any SQL error into "absent" — so a failing read reports
      `Unknown` (HTTP 404) rather than an error. Wrap in
      `unchecked_transaction` and use `.optional()` (found 2026-11-26). `read` calls
      `store.get` then `store.status`; a concurrent update between them
      yields an older body with a newer ETag/`meta.versionId` (a lost-update
      hazard for `If-Match`). `vread` likewise queries twice. Return body
      and version from one store call (one snapshot).
- [x] **Read replaces stored `meta`.** *Fixed 2026-11-26:
      `stamp_version` merges `versionId` into stored `meta`; test
      `a_read_keeps_stored_meta_and_adds_the_version` (security label kept,
      ETag equals `meta.versionId`), mutation-verified. SQLite backend only;
      PostgreSQL backend not run here.* `read` does
      `obj.insert("meta", {versionId})`, discarding stored
      `meta.security`, `meta.tag`, `meta.profile` — security labels are
      clinical-safety data. Merge `versionId` (and `lastUpdated`) into the
      stored `meta` instead. Test: a resource with a security label reads
      back with it.
- [x] **SQLite `_since` drops same-day history.** *Fixed 2026-11-26:
      `since_cutoff` converts an RFC 3339 instant (offset applied, sub-µs
      rounds up) to the stored form, passes the stored form through, and
      refuses anything else as `Unsupported`. Unit tests (offsets, day
      rollover, leap day, rounding carry, 11 malformed forms) and
      `an_rfc3339_since_earlier_the_same_day_admits_that_days_versions`,
      mutation-verified (unconverted: 0 of 4 versions).* Stored `last_updated` is
      `YYYY-MM-DD HH:MM:SS.ffffff` (space; `render_utc_micros`), but
      `history_page` compares it textually to the caller's RFC 3339
      `_since` (`T`). `' '` < `'T'`, so every version on the cutoff's
      date compares low and is omitted. The `history_page` doc comment's
      "every stored `last_updated` is RFC 3339" is wrong. Normalise
      `_since` to the stored form (UTC, offset applied, six fractional
      digits) before binding; test cutoffs earlier the same day, with an
      offset, and with fractional seconds. Checked 2026-11-26: SQLite only —
      PostgreSQL casts `($1::text)::timestamptz`, and the other four ports
      have no `_since` path.

### Quality-improvement audit (Mistral Medium 3.5) — triage

A generic checklist, not source-verified by its author; several items are
already done or contradict settled decisions. Kept here only where they add
something.

- **Already done / superseded:** coverage measurement and thresholds
  (`fhir-sqlite/scripts/coverage.sh`, `bin/test-coverage`), shared-core gate
  over `gen/tests` (F-48, 2026-11-25), mutation testing as policy (`T11.10`),
  fuzzing (`fhir-sqlite/fuzz`), CI gates (`gates.yml`), comparison page
  (`COMPARISONS.md`), citation (`CITATION.cff`), pagination and `$export`
  (`fhir-loco`), PASETO auth (its "implement SMART" item is SV3.10's
  deliberate non-goal).
- **Rejected:** retry/circuit-breaker/rate-limit layers in the store
  libraries (`C0.17`: no server concerns in a port); translations and
  social-media accounts before the outreach prerequisites (`PM-70`–`PM-75`).
- [ ] Machine-readable conformance: generate the CapabilityStatement's
      supported-search table from the same source as
      `doc/fhir-feature-support.md` so the two cannot drift.
- [ ] Concurrency tests beyond PostgreSQL (row-lock / If-Match races) for
      each server port's live suite.
- [ ] Unit or integration test for the five live server-engine suites in CI
      is already required (`FHIR_<PORT>_REQUIRE_DB=1`); verify each job
      actually ran green on the last push before claiming it.
- [ ] Outreach items (awesome-rust, chat.fhir.org, JOSS) belong to
      `help/outreach/index.md`; add them there only after its prerequisites.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
