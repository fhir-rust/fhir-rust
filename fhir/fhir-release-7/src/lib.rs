//! Reserved for a future FHIR Release 7 data model.
//!
//! **There is no FHIR R7 specification.** At the time of writing, R6 is in
//! ballot (6.0.0-ballot3) and is the newest release HL7 has published in any
//! form. R7 is simply the number that would come next; HL7 has not announced
//! it, and nothing here should be read as implying otherwise. This crate holds the name so that the model can be published here
//! when — and if — such a release exists, and so the crate layout of this
//! workspace stays predictable: one crate per release, all siblings of
//! [`fhir-core`](https://crates.io/crates/fhir-core).
//!
//! It deliberately contains no types. A placeholder `Resource` or `Patient`
//! would be a guess about a specification nobody has written, and a wrong
//! guess is worse than an absent one for anything that touches clinical data.
//!
//! # Adding the model when the specification lands
//!
//! The path is short, because the generator already knows how to do this —
//! R6 was added this way. See `doc/adding-a-release.md` in the repository.
//! In outline:
//!
//! 1. Put the official definition bundles in
//!    `doc/fhir-specifications/r7/fhir-definitions-json/`.
//! 2. Add `Version::R7` to `src/codegen/version.rs` (module, label, version
//!    string, spec URL, parse tokens).
//! 3. Add `"r7"` to `KNOWN_VERSIONS` in `fhir-derive-macros`.
//! 4. Run `cargo run -- r7`, which writes the model into `fhir-release-7/src`.
//! 5. Copy the per-release support modules from the nearest release and
//!    adapt whatever that release actually changed.
//!
//! Step 5 is the only part that needs judgement. Going from R5 to R6, it was
//! a single function: `Bundle.link.relation` became a coded value where it
//! had been a bare string.

// Nothing is exported yet, and that is the point. `#![no_std]` keeps the
// crate honest: it cannot accidentally acquire a dependency or an API before
// there is a specification to model.
#![no_std]
