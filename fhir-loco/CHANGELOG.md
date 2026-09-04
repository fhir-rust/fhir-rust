# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file did not exist before 2026-08-29. This crate is the FHIR® REST
surface — it inherited the name `fhir-store` briefly before being renamed
(**F-37**), and had no changelog under either name.

## 0.3.4 — 2026-09-04

`spec/index.md`'s "honest summary" paragraph was corrected in a repo-wide
documentation audit: it had drifted stale, still naming `SV2.14`, `SV2.15`,
`SV3.11`, and `SV4.3` as unmet a month after each was met (and now
`SV2.19`, shipped in 0.3.3, moves the same way if restated as-is). Fixed by
no longer restating a status that can drift a second time — the paragraph
now points at each id's own status line instead. Accurate against the
source tree, but the already-published 0.3.3 artifact still carried the
stale paragraph, since crates.io versions are immutable.
`check-published-match.sh` caught it on the next hosted run (O10.11),
exactly as designed. No source change beyond `spec/index.md` and this
entry.

## 0.3.3 — 2026-09-03

**Conditional delete** (`SV2.19`): `DELETE /{version}/{rtype}?params` is
now served — a routing gap on top of a store capability that already
existed on both mountable backends (`fhir-postgresql`, `fhir-sqlite`),
proposed in `tasks.md`'s capability roadmap and closed the same day. No
match and single match both answer `204` (deletion is idempotent, the
same rule instance-level `DELETE` already followed); more than one match
answers `412` with the same reason shape `SV2.14`'s conditional create
uses; no criteria at all is refused with `400` rather than treated as
"delete the type's one resource if it has exactly one" — silently
allowing that turns a missing query parameter into a request that can
delete an entire type. The `CapabilityStatement` declares
`conditionalDelete: "single"`. Verified end to end against both mounted
backends: `conditional_delete_serves_all_outcomes` (SQLite, all three
store outcomes plus the no-criteria refusal) and a new slice in
`tests/pg_backend.rs` against a live PostgreSQL 18 container — not
assumed from the shared code path, since the postgres backend's own
`FHIR_LOCO_TEST_PG=1` suite self-skips without it (the exact trap
`spec/databases/audit.md`'s `T11.12` names). `cargo fmt`/`clippy -D
warnings` clean.

## 0.3.2 — 2026-09-02

`rstest` 0.25 → 0.26 (already reflected in `Cargo.lock`; the manifest
requirement was fixed to match in a prior commit that missed bumping this
crate's own version). `AGENTS.md` and `CLAUDE.md` added since 0.3.1
published. Released as a patch because 0.6.0/0.3.1-era `fhir-postgresql`
and `fhir-sqlite` path dependencies moved and this crate embeds their
build graph — `O10.11` requires the published version to match its
source, and this is exactly the gap `check-published-match.sh` closed the
same day (F-98/F-102) — this release is verified against the *fixed*
gate, not the one that missed it. `cargo fmt`/`clippy -D warnings` clean;
full test suite (41 tests across `config`, request/response, home, and
the Postgres backend) green.

## 0.3.1 — 2026-08-29

Companion release for `fhir-sqlite` 0.6.1 and `fhir-postgresql` 0.6.1
(`sha2`/`sha3` dependency bumps): this crate's own `Cargo.lock` needed
regenerating too, since it embeds `fhir-sqlite-store`/`-map` and
`fhir-postgresql-store`/`-map` as path dependencies and therefore shares
their build graph. Dependabot cannot see this — its updates are scoped to
one directory and it has no way to know a *sibling* workspace's lockfile
also needs regenerating. `cargo check --locked` now passes (it failed
before this release, exactly the gap this patch closes); the request
suite (in-process against `fhir-sqlite`, no server) re-run, all green. No
source change beyond the regenerated lockfile.

## 0.3.0 — 2026-08-29

**MSRV declared for the first time: 1.96.** `RV1.4` was unmet here until now
— the six ports promised 1.90 and CI built on exactly that toolchain, but
this crate's own floor had never been measured. Measured against loco-rs
and axum with `cargo +1.96 check --all-targets --locked` before being
written down, per `RV1.5` (current `RV1.1` ceiling, spec
`spec/rust-msrv-n-minus-2/`). CI gained a matching `msrv` job
(`.github/workflows/fhir-loco-ci.yml`).

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
