//! FHIR Release 3 (STU3, 3.0.2).
//!
//! Laid out exactly like [`crate::r5`], so porting between releases is a
//! matter of changing one path segment. The pieces you use day to day:
//!
//! - [`resources`] — the 117 R3 resources plus the polymorphic
//!   [`Resource`](resources::Resource) enum.
//! - [`types`] — the 36 complex datatypes and 18 primitive newtypes.
//! - [`codes`] — FHIR code systems as type-safe enums.
//! - [`validate`] — the R3 primitive-format constraints, over the shared
//!   [`Validate`](crate::validate::Validate) trait.
//!
//! Every model module here is **generated** from the official R3 definition
//! JSON by [`crate::codegen`]; regenerate with `cargo run -- r3` rather than
//! editing `fhir-release-3/src/types` or `fhir-release-3/src/resources` by hand.
//!
//! # R3 is not R5
//!
//! The releases are deliberately separate types, because they disagree in
//! ways that silently corrupt data if conflated.
//!
//! `Observation.value[x]` admits 11 types here and 13 in R5 — but a matching
//! count is not compatibility: STU3 takes `Attachment` and not `integer`,
//! and R4 reversed both. A resource's `id` is typed `id` here and `string`
//! from R4 onwards, and `Extension.url` is a `uri` here and a `string`
//! afterwards. STU3 has no `canonical` or `url` primitive, and no
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

/// FHIR R3 datatypes.
// Every path in this crate's generated code is written `crate::r3::…`,
// because it used to be a module of the parent crate. Aliasing the crate to
// its own name keeps all ~1,000 of them resolving, and keeps the derive
// macros — which emit `crate::r3` from `#[fhir_version("r3")]` — working
// without a change.
pub use crate as r3;

pub mod types;

/// FHIR R3 resources.
pub mod resources;

/// FHIR R3 code systems as type-safe enums.
pub mod codes;

/// Lightweight FHIR R3 validation.
pub mod validate;

/// Per-element metadata extracted from the FHIR R3 specification (cardinality,
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

/// Common imports for working with FHIR R3.
pub mod prelude;

/// An async FHIR R3 REST client (feature `client`).
#[cfg(feature = "client")]
pub mod client;

/// FHIR XML serialization (feature `xml`).
#[cfg(feature = "xml")]
pub mod xml;

/// The FHIR R3 release, as a type.
///
/// Marker for release-parameterized code such as
/// [`ReleaseClient`](::fhir_core::client::ReleaseClient); see [`::fhir_core::release`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct R3;

impl ::fhir_core::release::Release for R3 {
    const LABEL: &'static str = "R3";
    const VERSION: &'static str = "3.0.2";

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
