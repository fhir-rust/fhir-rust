# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This file did not exist before 2026-08-29. This crate is the FHIR® REST
surface — it inherited the name `fhir-store` briefly before being renamed
(**F-37**), and had no changelog under either name.

## Unreleased

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
