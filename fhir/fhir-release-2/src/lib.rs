//! FHIR Release 2 (DSTU2, 1.0.2).
//!
//! Laid out exactly like [`crate::r5`], so porting between releases is a
//! matter of changing one path segment. The pieces you use day to day:
//!
//! - [`resources`] — the 94 R2 resources plus the polymorphic
//!   [`Resource`](resources::Resource) enum.
//! - [`types`] — the 28 complex datatypes and 18 primitive newtypes.
//! - [`codes`] — FHIR code systems as type-safe enums.
//! - [`validate`] — the R2 primitive-format constraints, over the shared
//!   [`Validate`](crate::validate::Validate) trait.
//!
//! Every model module here is **generated** from the official R2 definition
//! JSON by [`crate::codegen`]; regenerate with `cargo run -- r2` rather than
//! editing `fhir-release-2/src/types` or `fhir-release-2/src/resources` by hand.
//!
//! # R2 is not R5
//!
//! The releases are deliberately separate types, because they disagree in
//! ways that silently corrupt data if conflated.
//!
//! `Observation.value[x]` admits 10 types here and 13 in R5. `HumanName
//! .family` repeats in DSTU2 and is single-valued from STU3 onwards, which is
//! the difference most likely to stop a port compiling. Medication ordering
//! is [`MedicationOrder`](resources::medication_order::MedicationOrder), a
//! resource later releases renamed to `MedicationRequest`; server
//! capabilities are `Conformance`, not `CapabilityStatement`. DSTU2 has no
//! `integer64`, `CodeableReference` or `RatioRange` datatype.
//!
//! Convert between releases explicitly, through JSON, rather than assuming
//! they interoperate.
//!
//! See the [crate-level guide](crate) for a task-oriented walkthrough.

// The README's examples are compiled and run as doctests, so they cannot
// drift from the model they document. `cfg(doctest)` means this costs
// nothing in a normal build and adds nothing to the rendered docs — it
// exists only so `cargo test --doc` sees the code blocks in ../README.md.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;

/// FHIR R2 datatypes.
// Every path in this crate's generated code is written `crate::r2::…`,
// because it used to be a module of the parent crate. Aliasing the crate to
// its own name keeps all ~700 of them resolving, and keeps the derive
// macros — which emit `crate::r2` from `#[fhir_version("r2")]` — working
// without a change.
pub use crate as r2;

pub mod types;

/// FHIR R2 resources.
pub mod resources;

/// FHIR R2 code systems as type-safe enums.
pub mod codes;

/// Lightweight FHIR R2 validation.
pub mod validate;

/// Per-element metadata extracted from the FHIR R2 specification (cardinality,
/// bindings, choice types, reference targets, summary membership).
pub mod meta;

/// Support types for `value[x]` choice elements (see `spec/11-choice-types.md`).
pub mod choice;

/// The [`Coded`](coded::Coded) wrapper for `required`-binding coded fields.
pub mod coded;

/// Support for the generated `#[derive(Builder)]` builders.
pub mod builder;

/// Ergonomic extension accessors ([`ExtensionExt`](extension_ext::ExtensionExt)).
pub mod extension_ext;

/// Utilities for `Bundle`s: iteration, paging, and transaction/batch building.
pub mod bundle_util;

/// Summary serialization (the FHIR `_summary=true` view).
pub mod summary;

/// Parsing and precision-aware comparison for the date/time primitives.
pub mod temporal;

/// Common imports for working with FHIR R2.
pub mod prelude;

/// An async FHIR R2 REST client (feature `client`).
#[cfg(feature = "client")]
pub mod client;

/// FHIR XML serialization (feature `xml`).
#[cfg(feature = "xml")]
pub mod xml;

/// The FHIR R2 release, as a type.
///
/// Marker for release-parameterized code such as
/// [`ReleaseClient`](::fhir_core::client::ReleaseClient); see [`::fhir_core::release`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct R2;

impl ::fhir_core::release::Release for R2 {
    const LABEL: &'static str = "R2";
    const VERSION: &'static str = "1.0.2";

    fn elements() -> &'static [::fhir_core::meta::ElementMeta] {
        meta::elements()
    }
    type Resource = resources::Resource;
    type Bundle = resources::Bundle;
    // DSTU2 called it `Conformance`; R3 renamed it `CapabilityStatement`.
    // The associated type is the release-independent name, so a client
    // generic over `Release` works across the rename.
    type CapabilityStatement = resources::Conformance;
    type OperationOutcome = resources::OperationOutcome;

    fn next_link(bundle: &Self::Bundle) -> Option<String> {
        bundle
            .link
            .iter()
            .find(|l| l.relation.0 == "next")
            .map(|l| l.url.0.clone())
    }
}
