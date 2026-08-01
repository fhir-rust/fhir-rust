//! FHIR Release 6 (6.0.0-ballot3).
//!
//! Laid out exactly like [`crate::r5`], so porting between releases is a
//! matter of changing one path segment. The pieces you use day to day:
//!
//! - [`resources`] — the 161 R6 resources plus the polymorphic
//!   [`Resource`](resources::Resource) enum.
//! - [`types`] — the 51 complex datatypes and 21 primitive newtypes.
//! - [`codes`] — FHIR code systems as type-safe enums.
//! - [`validate`] — the R6 primitive-format constraints, over the shared
//!   [`Validate`](crate::validate::Validate) trait.
//!
//! Every model module here is **generated** from the official R6 definition
//! JSON by [`crate::codegen`]; regenerate with `cargo run -- r6` rather than
//! editing `fhir-release-6/src/types` or `fhir-release-6/src/resources` by hand.
//!
//! # R6 is not R5
//!
//! The releases are deliberately separate types, because they disagree in
//! ways that silently corrupt data if conflated.
//!
//! R6 is closest to R5: both admit 13 types in `Observation.value[x]` and
//! both have `integer64`, `CodeableReference` and `RatioRange`. The change
//! that needed hand-written support was `Bundle.link.relation`, which became
//! a coded value where R5 had a bare string.
//!
//! **This is a ballot draft, not a published release.** A later ballot can
//! rename or remove anything here and this crate will follow without a major
//! version bump, so it sits outside the workspace's semver promise.
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

/// FHIR R6 datatypes.
// Every path in this crate's generated code is written `crate::r6::…`,
// because it used to be a module of the parent crate. Aliasing the crate to
// its own name keeps all ~1,000 of them resolving, and keeps the derive
// macros — which emit `crate::r6` from `#[fhir_version("r6")]` — working
// without a change.
pub use crate as r6;

// R6 is a ballot draft. The model is generated from 6.0.0-ballot3 and tracks
// that ballot: names and cardinalities can change between ballots in ways a
// released FHIR version never would. The crate is `publish = false` for this
// reason.

pub mod types;

/// FHIR R6 resources.
pub mod resources;

/// FHIR R6 code systems as type-safe enums.
pub mod codes;

/// Lightweight FHIR R6 validation.
pub mod validate;

/// Per-element metadata extracted from the FHIR R6 specification (cardinality,
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

/// Common imports for working with FHIR R6.
pub mod prelude;

/// An async FHIR R6 REST client (feature `client`).
#[cfg(feature = "client")]
pub mod client;

/// FHIR XML serialization (feature `xml`).
#[cfg(feature = "xml")]
pub mod xml;

/// The FHIR R6 release, as a type.
///
/// Marker for release-parameterized code such as
/// [`ReleaseClient`](::fhir_core::client::ReleaseClient); see [`::fhir_core::release`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct R6;

impl ::fhir_core::release::Release for R6 {
    const LABEL: &'static str = "R6";
    const VERSION: &'static str = "6.0.0-ballot3";

    fn elements() -> &'static [::fhir_core::meta::ElementMeta] {
        meta::elements()
    }
    type Resource = resources::Resource;
    type Bundle = resources::Bundle;
    type CapabilityStatement = resources::CapabilityStatement;
    type OperationOutcome = resources::OperationOutcome;

    fn next_link(bundle: &Self::Bundle) -> Option<String> {
        // R6 types `Bundle.link.relation` as a coded value drawn from the
        // IANA link-relations value set, where R3-R5 carried a bare string.
        // Both arms are checked because `Coded` exists precisely so a code
        // outside the value set still round-trips: a server that sends
        // "next" in a way this build did not parse as `Known` must still be
        // followable.
        bundle
            .link
            .iter()
            .find(|l| match &l.relation {
                coded::Coded::Known(codes::IanaLinkRelations::Next) => true,
                coded::Coded::Unknown(s) => s == "next",
                coded::Coded::Known(_) => false,
            })
            .map(|l| l.url.0.clone())
    }
}
