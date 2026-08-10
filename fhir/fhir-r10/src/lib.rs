//! Reserved for a future FHIR Release 10 data model.
//!
//! **There is no FHIR R10 specification.** R6 is the newest release HL7 has
//! published in any form, and it is still in ballot. R10 is four numbers
//! past anything that exists; HL7 has not announced it, and nothing here
//! should be read as implying otherwise.
//!
//! This crate holds the name so the model can be published here if such a
//! release exists, and so the crate family stays contiguous: one crate per
//! release, all siblings of
//! [`fhir-core`](https://crates.io/crates/fhir-core). A gap in the sequence
//! is an invitation for an unrelated crate to occupy a name in the scheme,
//! which has already happened once to `fhir-r4`.
//!
//! It is also where the two-digit boundary lands. `fhir-r9` and
//! `fhir-r10` sort adjacently by number but not as strings, so tools
//! that order the family lexically will place `-10` before `-2`. Nothing
//! here depends on that ordering, but anyone scripting over the family
//! should sort numerically.
//!
//! It deliberately contains no types. A placeholder `Patient` would be a
//! guess about a specification nobody has written, and a wrong guess is
//! worse than an absent one for anything that touches clinical data.
//!
//! For a real FHIR model today, use [`fhir`](https://crates.io/crates/fhir),
//! which covers DSTU2, STU3, R4 and R5, with R6 as a ballot draft.

// Nothing is exported, and that is the point. `#![no_std]` keeps the crate
// honest: it cannot accidentally acquire a dependency or an API before there
// is a specification to model.
// A data model has no business dereferencing raw pointers (spec R13.14).
#![forbid(unsafe_code)]
#![no_std]
