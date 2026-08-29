# Changelog

All notable changes to the **repository as a whole** are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Individual crates are versioned independently under
[Semantic Versioning](https://semver.org/spec/v2.0.0.html), and each family
keeps its own changelog with the detail this one summarises:

| Family | Changelog |
| --- | --- |
| Model | [`fhir/CHANGELOG.md`](fhir/CHANGELOG.md) |
| Databases | [postgresql](fhir-postgresql/CHANGELOG.md) · [sqlite](fhir-sqlite/CHANGELOG.md) · [mysql](fhir-mysql/CHANGELOG.md) · [mariadb](fhir-mariadb/CHANGELOG.md) · [mssql](fhir-mssql/CHANGELOG.md) · [oracle](fhir-oracle/CHANGELOG.md) |

**Tags and GitHub releases both exist as of 2026-08-26.** Every
independently-versioned unit is tagged and carries a GitHub Release —
`spec/git-tags-name-published-versions/index.md` — so `releases.atom` is a
live feed and `PM-70` is closed. Commits and tags are signed from 2026-08-27
onward; everything before that date is unsigned and stays that way
(`MAINTAINERS.md`). Dates below are commit dates.

History before 2026-08-01 belongs to the separate projects this monorepo was
assembled from, and lives in the per-family changelogs above.

## 2026-08-29 — F-51 fixed: fhir-oracle's DDL install is now a live test

`fhir-oracle`'s Schema-level claim (`C0.8`, `C0.9`) rested on a hand-run
`podman exec ... sqlplus` transcript, not a test that runs. The finding
named a real-sounding blocker -- "a live test needs an Oracle driver
decision" -- that turned out to already be decided: **F-68** had already
proved `fhir-oracle-store` connecting live via the `oracle` crate and Oracle
Instant Client, months earlier. No architectural choice remained, only the
mechanical work of giving `fhir-oracle-map` (which had never depended on a
driver) its own copy of that same proven path.

`crates/fhir-oracle-map/tests/oracle_ddl.rs`, on the model of
`fhir-mssql`'s `mssql_ddl.rs`: installs a sampled schema (`Patient`,
`Observation`) live and counts tables and triggers afterward rather than
trust a lack of errors. One genuine Oracle-specific complication SQL
Server's model does not have: Oracle unifies user and schema (`M14.5`), so
installing into a fresh schema means creating a fresh database user, which
needs a SYSTEM-level connection no regular test login holds -- the test
connects twice, mirroring in Rust what this port's own `scripts/db.sh` and
CI workflow already do in shell for the version users the *store's* tests
use, with its own dedicated, non-colliding user.

Verified live before being trusted: run twice consecutively against a real
`gvenzl/oracle-free` container (166 statements, 105 tables, 2 triggers,
identically both times -- the idempotency check `mssql_ddl.rs`'s own
history warns is not free); the skip and fail-loud paths both confirmed
against a genuinely unreachable connect string; `--release` and
`cargo clippy -- -D warnings` clean. Wired into `fhir-oracle-ci.yml`.

**What this does not establish, stated rather than left implicit:** the
full ~9,636-statement R5 install remains hand-verified only (**F-08**); and
two behaviours -- the append-only trigger actually refusing a forbidden
`UPDATE`/`DELETE` (`M14.29`) and the `Bool` CHECK actually rejecting `2`
(`M14.8`) -- are untouched by this fix and confirmed, not assumed, absent
from `fhir-oracle-store`'s own live suite too. Both are genuinely untested
and worth their own finding.

`F-67` is now the sole open finding in the audit register.

## 2026-08-29 — a self-assessment re-verified rather than trusted

`spec/professionalization/index.md`'s rule-by-rule assessment was dated
2026-08-26, "mid-landing", and had not been touched since -- three days in
which most of what it called open actually closed. Re-verified each rule
against the tree directly rather than repeating the old text:

- Rule 3: tags (60), signing (active since 2026-08-27), and F-67 (decided
  2026-08-28) are no longer gaps -- the rule moves from "met as a process"
  to "met".
- Rule 4: `fhir-security.yml`'s `cargo deny` trigger was claimed still
  paths-filtered to `fhir/**`. Checked the workflow file directly: it was
  not -- both its `push` and `pull_request` triggers already list all nine
  workspace paths, fixed 2026-08-26 in the same pass this note failed to
  reflect.
- Rule 9: `help/outreach/index.md`'s `PM-70`-`PM-75` are five-sixths done,
  not "exactly the open items" -- only `PM-72` (benchmarks) remains from
  that set.

Found while re-verifying, not assumed: `LICENSE.md` line 32 still said the
SPDX expression was declared in "33" manifests, contradicting the correct
"34" two paragraphs above it in the same file (41 total manifests measured
fresh, 7 `publish = false`, 34 publishable). And
`help/outreach/index.md`'s own `PM-75` sequencing-table row still said
"raise the decision now" for a GPL call the owner had already made and
that the same file's own `PM-75` write-up already recorded as decided --
an inconsistency within one document, not between two.

## 2026-08-28 — Trusted Publishing: checked, and reconciled with an earlier decision

`spec/trusted-publishing/index.md` stated a family-wide intent to adopt it
once production-ready across every forge and destination. Checked against
each registry's own documentation rather than assumed: crates.io Trusted
Publishing is GA on GitHub Actions and available on GitLab.com CI (not
self-hosted); Codeberg/Forgejo has none yet, upstream. That alone would
leave the condition unmet for this repository's three mirrors.

But a more specific, already-recorded decision governs here regardless:
the owner ruled out publishing from CI entirely on 2026-08-26, on GitHub
Actions reliability grounds, not on Trusted Publishing's own readiness.
Trusted Publishing has no CI workflow to attach to under that model, so the
two facts don't compete — the CI decision settles it either way. Recorded
in the spec doc's own status section, cross-referenced from
`spec/publishing.md`, `MAINTAINERS.md`, `README.md`, `index.md`, and
`spec/index.md`.

Corrected while touring those routing tables: `README.md`, `index.md`, and
`spec/index.md` all still described `spec/publishing.md` as "what blocks"
publication and named 33 crates — stale since all 34 published 2026-08-22.

## 2026-08-28 — F-67 decided: accept the risk formally

The oldest open decision in the repository closed. Investigated and priced
three alternatives before accepting: a from-scratch TDS driver (~3–4.5
months, and worse on the trust axis than the flawed 6-year incumbent), a
fork carrying the one upstream fix that exists
([`prisma/tiberius#419`](https://github.com/prisma/tiberius/pull/419) —
clean and CI-green on its own fork, but no maintainer review in 3+ months,
and unusable by a published crate regardless since cargo forbids a `git`
dependency in one), and one newer alternative crate (`ms-tds`, disqualified
on sight — its own description advertises offensive/exploitation tooling
alongside its driver code). Full account with real numbers:
[`M14.34`](fhir-mssql/spec/14-mssql-dialect.md).

Decision: keep shipping `fhir-mssql-store` on upstream `tiberius`, document
the risk loudly rather than quietly. `deny.toml`'s four ignores, the
[audit register](spec/databases/audit.md)'s F-67 row, and every document
that names this risk to a reader — `SECURITY.md`, `PHI.md`, `INSTALL.md`,
`RFC.md`, `plan.md`, `tasks.md`, `help/outreach/index.md` — now say so
consistently. `PM-4` (outreach's mssql-naming prerequisite) is satisfied by
the documented-statement branch it always offered; `PM-72` (benchmarks) is
now the only thing still gating outreach phase 1.

## 2026-08-28 — funding channels, checked rather than assumed

`spec/free-open-source-funding/` executed: GitHub Sponsors was already live
(verified via GitHub's own API, not assumed), so `.github/FUNDING.yml` now
declares it alongside Patreon, Ko-fi and PayPal. Open Collective is not
set up — checked against its own API, no collective exists at either
`joelparkerhenderson` or `fhir-rust` — and creating one needs the
maintainer's own sign-in, so it stays a stated open item in
[`CONTRIBUTING.md`](CONTRIBUTING.md), [`NEWS.md`](NEWS.md), and the spec
itself rather than being invented or left silent.

## 2026-08-27 — commit and tag signing begins

Every commit and tag before this date is unsigned and permanently stays that
way; history is not rewritten. From this date, commits and tags in this
repository are signed with an SSH key, configured **locally to this
repository only** — signing does not silently activate in a clone or a
sibling project. The public key is committed at
[`.github/jph-code-signing.pub`](.github/jph-code-signing.pub) for
self-contained verification. Full posture, what this does and does not prove,
and the verification command: [`MAINTAINERS.md`](MAINTAINERS.md).

Also: two `tasks.md` checkboxes corrected from stale to actually-done, and
`fhir-tmp-stash/` (an untracked stray `.DS_Store`) deleted.

## 2026-08-26 — the description-disclaimer release (second of the day)

Every publishable crate's Cargo.toml `description` — the string crates.io
renders as the crate's page — now carries the HL7® trademark fair-use
disclaimer verbatim, with ® on the first use of each word mark, in the
canonical three-part shape ending "This project is an independent work."
`scripts/check-trademarks.sh` gained a section that walks every `[package]`
manifest and fails unless each publishable description complies (fuzz
crates are `publish = false` and are skipped by that field);
`spec/hl7-trademarks-fair-use/` records the coverage.

A manifest change is a source change, so `O10.11` demands the published
versions move with it: all 34 crates republished — six ports 0.5.2,
`fhir-store` and `fhir-loco` 0.2.2, `fhir` 4.1.2, `fhir-r2`–`fhir-r6`
4.1.2, `fhir-core` 3.2.2, `fhir-derive-macros` 1.5.2, the five name
reservations 0.0.3. No code changed; the descriptions are the whole
release.

## 2026-08-26 — the professionalization release: all 34 crates republished

Everything below shipped to crates.io on 2026-08-26 in one pass, as the
documented laptop step the owner decided that day is permanent.
`scripts/check-published-match.sh` reports **34 matched, 0 mismatched** —
every published version is byte-identical to its source. Versions: six ports
0.5.1; `fhir-store` and `fhir-loco` 0.2.1; `fhir`, `fhir-r2`–`fhir-r6` 4.1.1;
`fhir-core` 3.2.1; `fhir-derive-macros` 1.5.1; the five name reservations
0.0.2. With this pass, `fhir-derive-macros` and the reservations acquire
their first tags (`TG1.10`'s promise, kept).

Repository-level documents added: `CITATION.cff`, `CODEOWNERS`,
`MAINTAINERS.md`, `AI_STATEMENT.md`, `INSTALL.md`, `COMPARISONS.md`,
`BENCHMARKS.md`, `NEWS.md`, `PHI.md`, `SECURITY.md`, `CONTRIBUTING.md`,
`CODE_OF_CONDUCT.md`, `RFC.md`, `GOVERNANCE.md`, this file, SPDX information in `LICENSE.md`,
`LICENSES/` with the full text of all five licence options, `.github/FUNDING.yml`,
and `help/outreach/index.md`.

Code changes in this release, each of which version-bumped the crates it
touched (which is why all 34 moved at once):

- `#![forbid(unsafe_code)]` at every crate root in the repository, gated by
  `scripts/check-forbid-unsafe.sh` in `gates.yml`.
- A Trademarks section in every top-level crate's rustdoc, so the disclaimer
  reaches docs.rs readers, gated with the same trademark check.
- The trademark gate went tree-wide: every markdown file in the repository
  (216 fixed in the final sweep — books, specs, the audit register, the
  families' own pages), with three named structural exemptions reasoned in
  the script. The model-family bump above exists because the sweep touched
  `fhir/README.md` and `fhir/LICENSE.md`, which cargo packages into every
  model crate — the published artifacts get the disclaimer at the next
  publish.
- `fhir-loco`: `clippy::result_large_err` allowed on the one loco-convention
  handler, with the reason stated (`loco_rs::Error` is the framework's type).
- `fhir-sqlite`: the torn-read concurrency test's reader/writer overlap is
  now structural (barrier + done flag) rather than probabilistic — its first
  hosted run proved the old shape could finish reading before the writer
  started, failing its own T11.12 guard on a slow runner.
- mysql/mariadb: the env-mutating SSL default test moved to an integration
  test, which `forbid(unsafe_code)` requires.

Decided: the five-way licence expression stays as it is, and the reasoning is
recorded in `LICENSE.md`.

Trademarks: every public document at the root and under `help/` now carries the
registration mark on first use of an HL7® word mark and the required disclaimer,
per `spec/hl7-trademarks-fair-use/`. `scripts/check-trademarks.sh` gates it in
`gates.yml` — the assurance that spec asks for.

**The 2026-08-22 publication is tagged and released.** Sixteen annotated tags
at `e28964e`, one per independently-versioned unit: six ports (a port's three
crates share one `version.workspace`, so one tag names all three),
`fhir-store`, `fhir-loco`, `fhir`, `fhir-core`, and the six release crates
published at 4.1.0. The convention is now normative and cross-family —
`spec/git-tags-name-published-versions/` (`TG1.x`) — and gated by
`scripts/check-tags.sh`. Each tag carries a GitHub Release (2026-08-26) whose
note states the retroactive-tag caveat and cites the conformance matrix
rather than claiming beyond it (`TG1.7`, `TG1.8`); `releases.atom` is live.

Corrected while tagging: `README.md` understated the model crate's coverage,
saying R3/R4/R5 were modelled in code with R2/R6 "in spec" and that R6 was
`publish = false`. All six releases are modelled in code and published —
`fhir-r2` is 108k lines and `fhir-r6` is 248k, the largest in the tree. R6 is
off by default because it is generated from a draft ballot, which is the true
caveat. The crates that really contain no types are the name reservations
`fhir-r1` and `fhir-r7`–`fhir-r10`. Also corrected in `COMPARISONS.md` and the
outreach claims register.

Six published crates deliberately have no tag: `fhir-derive-macros` and the five
name-reservation release crates went out before this repository existed, so
there is no commit here to point at (`TG1.10`).

**`#![forbid(unsafe_code)]` in every published crate.** The model family has
declared it since 2026-08-06 (`R13.14`, T39); the 18 database-port crates,
`fhir-store` and `fhir-loco` now do too, so all **34** published crates forbid
unsafe code. `forbid` rather than `deny` is the point — `deny` can be lifted by
an `#[allow]` anywhere inside the crate, so it records an intention, while
`forbid` cannot be lifted at all and records a guarantee.

The seven `publish = false` fuzz crates declare it too, so all **54 crate
roots** in the repository forbid unsafe code — 34 published crates plus 20
cargo-fuzz targets, each of which is its own `[[bin]]` and therefore its own
root. The fuzz crates were expected to be an exemption, on the assumption that
libfuzzer-sys's `fuzz_target!` macro could not expand under `forbid`. Checked
rather than assumed, and the assumption was wrong: all seven fuzz workspaces
build clean with it. There is no exemption.

Verified by `cargo check --all-targets --locked` in all nine workspaces and
`cargo +nightly check --all-targets` in all seven fuzz workspaces, and gated by
`scripts/check-forbid-unsafe.sh` in `gates.yml`.

Two unit tests moved to make it possible. `fhir-mysql-store` and
`fhir-mariadb-store` each tested their SSL default with a `#[cfg(test)] mod
tests` block that calls `std::env::remove_var`, which is `unsafe` in edition
2024 — and a unit test compiles as part of its crate, so `forbid` would have
broken it. Each is now an integration test in `tests/ssl_default.rs`, which is a
separate crate and therefore unaffected. This is the arrangement
`fhir-postgresql` already had (`tests/ssl_default.rs`, **F-17**), so the three
SSL-default tests are now consistent across the ports that have one. No test was
weakened or deleted, and neither still needs a database.

Corrected: `doc/trust-boundary.md` still said `fhir-mssql` and `fhir-oracle`
had no store, which stopped being true when **F-65** and **F-68** closed.

## 2026-08-22 — all 34 crates published to crates.io

The repository's stated publishing goal was reached. Every package the gate
enumerates now exists on crates.io at the version its source claims, and
`scripts/check-published-match.sh` reports `34 matched, 0 mismatched, 0
skipped`.

- **Published:** the 18 database-port crates at `0.5.0`, `fhir-store` `0.2.0`,
  `fhir-loco` `0.2.0`, `fhir` `4.1.0`, `fhir-core` `3.2.0`, and
  `fhir-r2`/`r3`/`r4`/`r4b`/`r5`/`r6` at `4.1.0`.
- **Changed:** `serde_json` gains the `float_roundtrip` feature alongside
  `arbitrary_precision` across every family. The two are opposite halves of one
  precision guarantee: `arbitrary_precision` covers JSON → `Number` → JSON,
  `float_roundtrip` covers `f64` → JSON → `f64`. Seven crates were version-bumped
  rather than edited, because a published version is immutable (`O10.11`).
- **Not established by any of this:** publication is not a conformance claim,
  and the `O10.4c` re-shred it shipped was verified on one developer machine
  rather than in hosted CI. The
  [conformance matrix](spec/databases/conformance-matrix.md) remains the
  document that says what each port has been shown to do.

Details: [`spec/publishing.md`](spec/publishing.md).

## 2026-08-21 – 2026-08-22 — the `O10.4c` re-shred migration, all six ports

Live-verified per port against a real server: sqlite, then postgresql, then
mysql and mariadb, then mssql and oracle. Alongside it, every live test in the
five server ports learned to find its own server, and a live job that finds
none now fails rather than skipping (`T11.12`, `T11.13`).

Also: the cross-family rules that the agents directory is lowercase (`AG1`) and
that the MSRV is current-minus-three (`RV1`).

## 2026-08-10 – 2026-08-12 — the row-size defect, and CI that actually runs

- **F-90.** The full R3/R4/R5 schemas did not install on stock MySQL 8.4 or
  MariaDB 11.4: InnoDB's create-time row-size check rejected the widest
  generated tables. Fixed at the shared generator with a byte-aware force-split
  (`G2.6a`), which changed table shapes in all six ports; assets and fixtures
  were regenerated and a budget test now gates them. Live-verified on both
  engines, then closed in full with the `O10.4b` moved-column guard on all six.
- **F-91, F-92.** Three test suites were found never to have really run. The
  first hosted runs of the store suites are what found them.
- **F-06.** `fhir-oracle`'s live-database CI job was restored with a real engine
  (`gvenzl/oracle-free`) and was green on its first hosted execution.

## 2026-08-06 – 2026-08-10 — the model crate's breaking work, and `fhir-loco` grows up

- `fhir`: `contained` typed as the `Resource` enum, reference fields typed by
  `targetProfile`, `PrimVec` for repeating primitives, `forbid(unsafe_code)` in
  all 13 crates, a property-based round-trip suite, and R4B modelled as
  `fhir-r4b` with `fhir-release-N` renamed `fhir-rN`. Two of these are
  breaking.
- `fhir-loco`: conditional create, an admin plane on a separate listener, an
  enforced TLS posture, `_include`/`_revinclude`, type- and system-level
  `_history`, system-level async Bulk Data `$export`, and multi-port wiring —
  closing `F-58`.
- **CI consolidated to the repository root**, which is where GitHub actually
  reads workflows from (`F-49`, `F-84`).

## 2026-08-03 – 2026-08-05 — the honesty pass

The findings that gave this repository its documentation discipline were found
and fixed in this window. Among them:

- **F-64** — every non-PostgreSQL port's `doc/benchmarks.md` presented
  `fhir-postgresql`'s measured numbers as its own, including a live round trip
  and a bulk-load benchmark for two ports that had no store at all. Corrected in
  all five; see [`BENCHMARKS.md`](BENCHMARKS.md).
- **F-56** — every port's book described another engine's tooling.
- **F-58** — `fhir-loco` had no specification. It now has one (`SV1.x`–`SV4.x`).
- **F-71, F-70, F-87, F-86** — real defects, including `active=true` silently
  matching nothing in sqlite and a chain env-var name that was simply wrong.
- The repository gates were committed so they can run (`F-49`, `F-10`, `F-60`).

## 2026-08-01 – 2026-08-02 — the monorepo

Seven previously separate projects were assembled into one repository with one
remote: the model crate `fhir`, six database ports, and the shared
specification. Then `fhir-loco` (the HTTP surface) and `fhir-store` (the
engine-agnostic persistence core, extracted from ~860 lines duplicated across
the six ports) were added.

Generated map assets moved into the map crates, so a consumer needs no
specification packages at build time.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
