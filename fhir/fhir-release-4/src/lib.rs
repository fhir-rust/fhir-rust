//! FHIR Release 4 (R4).
//!
//! This module holds the FHIR R4 (4.0.1) implementation, laid out exactly like
//! [`crate::r5`] so that porting between releases is a matter of changing one
//! path segment. The pieces you use day to day are:
//!
//! - [`resources`] — the 146 R4 resources plus the polymorphic
//!   [`Resource`](resources::Resource) enum.
//! - [`types`] — the 43 complex datatypes and 20 primitive newtypes.
//! - [`codes`] — FHIR `CodeSystem`s as type-safe enums.
//! - [`validate`] — the R4 primitive-format constraints, over the shared
//!   [`Validate`](crate::validate::Validate) trait.
//!
//! Unlike [`crate::r5`], every model module here is **generated** from the
//! official R4 definition JSON by [`crate::codegen`]; regenerate with
//! `cargo run -- r4` rather than editing `fhir-release-4/src/types` or `fhir-release-4/src/resources`
//! by hand.
//!
//! # R4 is not R5
//!
//! The releases are deliberately separate types, because they disagree in ways
//! that silently corrupt data if conflated. For example `Observation.value[x]`
//! allows 11 types in R4 and 13 in R5; `MedicationRequest.medication[x]` is a
//! choice in R4 but a `CodeableReference` in R5; and R4 has no `integer64`,
//! `CodeableReference`, or `RatioRange` datatype at all. Convert between them
//! explicitly, through JSON, rather than assuming they interoperate.
//!
//! See the [crate-level guide](crate) for a task-oriented walkthrough.

// The README's examples are compiled and run as doctests, so they cannot
// drift from the model they document. `cfg(doctest)` means this costs
// nothing in a normal build and adds nothing to the rendered docs — it
// exists only so `cargo test --doc` sees the code blocks in ../README.md.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;

/// FHIR R4 datatypes.
// Every path in this crate's generated code is written `crate::r4::…`,
// because it used to be a module of the parent crate. Aliasing the crate to
// its own name keeps all ~1,300 of them resolving, and keeps the derive
// macros — which emit `crate::r4` from `#[fhir_version("r4")]` — working
// without a change.
pub use crate as r4;

pub mod types;

/// FHIR R4 resources.
pub mod resources;

/// FHIR R4 code systems as type-safe enums.
pub mod codes;

/// Lightweight FHIR R4 validation.
pub mod validate;

/// Per-element metadata extracted from the FHIR R4 specification (cardinality,
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

/// Common imports for working with FHIR R4.
pub mod prelude;

/// An async FHIR R4 REST client (feature `client`).
#[cfg(feature = "client")]
pub mod client;

/// FHIR XML serialization (feature `xml`).
#[cfg(feature = "xml")]
pub mod xml;

/// The FHIR R4 release, as a type.
///
/// Marker for release-parameterized code such as
/// [`ReleaseClient`](::fhir_core::client::ReleaseClient); see [`::fhir_core::release`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct R4;

impl ::fhir_core::release::Release for R4 {
    const LABEL: &'static str = "R4";
    const VERSION: &'static str = "4.0.1";

    fn elements() -> &'static [::fhir_core::meta::ElementMeta] {
        meta::elements()
    }
    type Resource = resources::Resource;
    type Bundle = resources::Bundle;
    type CapabilityStatement = resources::CapabilityStatement;
    type OperationOutcome = resources::OperationOutcome;

    fn next_link(bundle: &Self::Bundle) -> Option<String> {
        bundle
            .link
            .iter()
            .find(|l| l.relation.0 == "next")
            .map(|l| l.url.0.clone())
    }
}
