# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file did not exist before 2026-08-29: this crate was extracted from
~860 duplicated lines across the six database ports (**F-45**) and had never
had a changelog of its own. It starts here rather than being backfilled,
since the extraction predates this file and reconstructing that history
would be guessing at dates this crate itself has no record of.

## Unreleased

**MSRV declared for the first time: 1.96.** `RV1.4` was unmet here until now
— the six ports promised 1.90 and CI built on exactly that toolchain, but
this crate's own floor had never been measured. Measured against 1.96 (the
current `RV1.1` ceiling, spec `spec/rust-msrv-n-minus-2/`) with `cargo +1.96
check --all-targets --workspace --locked` before being written down, per
`RV1.5`. CI gained a matching `msrv` job (`.github/workflows/fhir-store-ci.yml`).
