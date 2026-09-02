# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file did not exist before 2026-08-29. This crate is the FHIR® REST
surface — it inherited the name `fhir-store` briefly before being renamed
(**F-37**), and had no changelog under either name.

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
