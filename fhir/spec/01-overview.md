# 01 — Overview

## Purpose

`fhir` is a Rust crate that provides the **FHIR®** data model as strongly-typed,
`serde`-serializable Rust, together with a spec-driven code generator that
derives that model from the official FHIR specification JSON.

Five releases are modelled, each complete and independent: **R6
(6.0.0-ballot3, a draft)** under `fhir::r6`, **R5 (5.0.0)** under `fhir::r5`,
**R4 (4.0.1)** under `fhir::r4`, **R3 (3.0.2, also published as STU3)** under
`fhir::r3`, and **R2 (DSTU2, 1.0.2)** under `fhir::r2`. How they coexist — and
why they are separate types rather than one — is defined in
[spec 12](12-fhir-releases.md). Every other spec in this directory applies to
each release in turn; where they say "R5" for concreteness, read "the release".

FHIR (Fast Healthcare Interoperability Resources) is the HL7® standard for
representing and exchanging electronic health records.

## Scope

In scope, for each modelled release:

- The FHIR **primitive datatypes** (spec 02).
- The FHIR **complex datatypes** (spec 03).
- The FHIR **resources** (spec 04).
- The FHIR **code systems** as enums (spec 05).
- **JSON serialization** to/from canonical FHIR JSON (spec 06).
- **Structural validation** of the model (spec 07).
- The **code generator** that produces the model from spec JSON (spec 08).
- **Multiple releases** side by side (spec 12).

Out of scope (for now; see each spec's Future work):

- FHIR Turtle representation. (XML is supported behind the `xml` feature.)
- A running FHIR REST server. (A client is supported behind the `client`
  feature.)
- FHIRPath evaluation and full invariant (constraint) checking.
- FHIR releases other than R2 through R6 and R4B (modelled 2026-08-10), and
  DSTU1, which was a trial nothing shipped against and is not modelled.
- *Automatic* conversion between releases. Conversion is available, but only
  as an explicit call that hands back a report of everything it changed or
  discarded (spec 14); it is deliberately not something the compiler will do
  for you via `From`/`Into`.

## Goals

1. **Correctness.** Types mirror their release's StructureDefinitions; JSON
   round-trips losslessly for supported representations. A release's types
   accept exactly what that release permits — no more, so invalid data is
   caught, and no less, so valid data is never dropped.
2. **Ergonomics.** Idiomatic Rust: `Default`, `Clone`, `PartialEq`, `serde`,
   and rich rustdoc with runnable examples.
3. **Uniformity.** Every type is built the same way, so the model is
   predictable and machine-generable.
4. **Spec-driven.** The specifications in this directory define behaviour; the
   implementation follows and is verified against them.
5. **Leanness.** Few dependencies; fast compile and serialization.

## Non-goals

- Hiding FHIR's shape behind a "friendlier" abstraction. This crate exposes
  FHIR faithfully.
- Papering over the differences between FHIR releases. Where the releases
  disagree, the types disagree too.
- Runtime reflection or dynamic typing beyond the `Resource` enum (which also
  types `contained`, since T47) and the `serde_json::Value` used for the
  remaining polymorphic slots (`Bundle.entry.resource`,
  `Parameters.parameter.resource`).

## Crate identity

- Package and crate name: **`fhir`**. Import as `use fhir::…`.
- Cargo **workspace**: the `fhir` facade, `fhir-core` (release-independent
  machinery), one `fhir-rN` crate per modelled release (2–6),
  `fhir-derive-macros` (proc-macros), and the name-reservation crates
  (`fhir-r1`, `fhir-r7` … `fhir-r10`). See spec 12,
  R12.1a.
- Library **and** binary: the library is the model + generator API; the binary
  runs the generator. A library target is required so doctests execute.
- Each modelled release is a cargo feature (spec 12); `r5` is the default.

## Success definition

The crate meets this overview when:

- Each modelled release covers its own primitive types, complex datatypes,
  resources, and code systems in full.
- Every value round-trips through JSON.
- The green gate passes (build, tests, doctests, pedantic clippy) for every
  release and every supported feature combination.

## Acceptance criteria

1. `cargo build` succeeds for the workspace.
2. The public API exposes `fhir::<release>::{types, resources, codes, validate,
   meta}` for every enabled release.
3. `cargo test` and `cargo clippy --all-targets` are clean, for the default
   features and with every release enabled.
4. Each downstream spec (02–12) has its own acceptance criteria met.

## Trademarks

HL7®, and FHIR® are the registered trademarks of Health Level Seven
International and their use of these trademarks does not constitute an
endorsement by HL7.
