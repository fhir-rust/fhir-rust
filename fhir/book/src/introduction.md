# Introduction

`fhir` is a Rust implementation of the **HL7® FHIR®** data model, together with a
spec-driven code generator that produces it from the official FHIR
specification JSON. Five releases are modelled, each a complete, independent
crate: **R2 (1.0.2, DSTU2)**, **R3 (3.0.2, STU3)**, **R4 (4.0.1)**, **R5
(5.0.0)**, and **R6 (6.0.0-ballot3)** — R6 is an unpublished ballot draft, off
by default and outside the crate's semver promise. This guide uses **R5**
(`fhir::r5`, the default feature) throughout; R4 and R3 work identically by
changing one path segment — see [FHIR releases](fhir-releases.md).

Fast Healthcare Interoperability Resources (FHIR, pronounced "fire") is the HL7
standard for exchanging electronic health records. This crate lets you **build,
parse, validate, and round-trip** FHIR resources in idiomatic Rust with `serde`.

## What you get

For each release:

- **Every resource** (Patient, Observation, Encounter, …) as a Rust struct,
  plus a polymorphic `Resource` enum tagged by `resourceType` — 158 in R5,
  146 in R4, 117 in R3.
- **Every complex datatype** and **primitive newtype**, serializing
  transparently to its JSON form.
- **Code systems as type-safe enums** — 265 in R2, 386 in R3, 486 in R4, 442 in
  R5, 459 in R6; `required`-binding fields are typed as those enums via
  [`Coded`](terminology-and-codes.md).
- **`value[x]` choice elements as enums** — exactly one type at compile time.
- **Recursive validation** (`Validate`): primitive formats, cardinality,
  required-binding membership, and a subset of FHIR invariants.
- **Ergonomics**: builders, a prelude, extension helpers, Bundle utilities,
  summary serialization, and an async REST client (feature `client`).

## How to read this guide

Each chapter is task-oriented and standalone. Start with
[Getting started](getting-started.md), then work through the
[Tutorial](tutorial.md), which carries one small record through every stage —
construct, validate, serialize, bundle, summarize, read back. After that, read
[FHIR releases](fhir-releases.md) if you need a release other than R5, or
several at once, and dip into whichever topic you need.

Every line of the tutorial is also a runnable program
(`cargo run --example tutorial`), so it cannot drift from the crate.

Examples throughout use R5 paths. Every one of them works for R4 by changing
`r5` to `r4`.
For the full API, run `cargo doc --open`; for the normative rules, see the
[`spec/`](https://github.com/fhir-rust/fhir-rust/tree/main/spec)
directory in the repository.

> FHIR® is a registered trademark of Health Level Seven International. This crate
> is not affiliated with or endorsed by HL7.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
