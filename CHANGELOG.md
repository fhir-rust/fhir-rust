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

**Tags exist as of 2026-08-26; GitHub releases still do not.** Sixteen
annotated tags name the 2026-08-22 publication, each with a GitHub Release
(see the Unreleased entry and `spec/git-tags-name-published-versions/`), so
`releases.atom` is a live feed — `PM-70` closed. Dates below are commit
dates, and the only published artefacts are the crates.io versions named in
the 2026-08-22 entry.

History before 2026-08-01 belongs to the separate projects this monorepo was
assembled from, and lives in the per-family changelogs above.

## Unreleased

Repository-level documents added: `CITATION.cff`, `CODEOWNERS`,
`MAINTAINERS.md`, `AI_STATEMENT.md`, `INSTALL.md`, `COMPARISONS.md`,
`BENCHMARKS.md`, `NEWS.md`, `PHI.md`, `SECURITY.md`, `CONTRIBUTING.md`,
`CODE_OF_CONDUCT.md`, `RFC.md`, `GOVERNANCE.md`, this file, SPDX information in `LICENSE.md`,
`LICENSES/` with the full text of all five licence options, `.github/FUNDING.yml`,
and `help/outreach/index.md`.

Code changes, each version-bumping the crates it touched — by the end of
2026-08-26, **all 34 publishable crates sit one unpublished version ahead**
(six ports 0.5.1, `fhir-store`/`fhir-loco` 0.2.1, `fhir` 4.1.1,
`fhir-core` 3.2.1, `fhir-derive-macros` 1.5.1, `fhir-r2`–`fhir-r6` 4.1.1,
the five name reservations 0.0.2), so one publish pass covers everything;
`scripts/check-published-match.sh` reports all 34 as not yet published,
with its vacuous-OK banner saying exactly that:

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
