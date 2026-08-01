//! FHIR Release 5 (R5).
//!
//! This module holds the entire FHIR R5 implementation. The pieces you use
//! day to day are:
//!
//! - [`resources`] — the 158 R5 resources plus the polymorphic
//!   [`Resource`](resources::Resource) enum.
//! - [`types`] — the ~50 complex datatypes and 21 primitive newtypes.
//! - [`codes`] — FHIR `CodeSystem`s as type-safe enums.
//! - [`validate`] — the [`Validate`](validate::Validate) trait and the
//!   primitive-format constraints.
//!
//! The remaining modules ([`parse`], [`abstract_types`], [`properties`],
//! [`resource`], [`todo`]) support the code generator that produces the model
//! from the official specification JSON, and are not needed for consuming the
//! data model.
//!
//! See the [crate-level guide](crate) for a task-oriented walkthrough.

// The README's examples are compiled and run as doctests, so they cannot
// drift from the model they document. `cfg(doctest)` means this costs
// nothing in a normal build and adds nothing to the rendered docs — it
// exists only so `cargo test --doc` sees the code blocks in ../README.md.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDoctests;

/// FHIR R5 classes - the complete set - TODO figure out where these go.
pub mod abstract_types;

// TODO move anywhere better
pub mod properties;
pub mod resource;
pub mod todo;

/// FHIR R5 datatypes
// Every path in this crate's generated and hand-written code is written
// `crate::r5::…`, because it used to be a module of the parent crate.
// Aliasing the crate to its own name keeps all ~2,800 of them resolving, and
// keeps the derive macros — which default to `crate::r5` — working unchanged.
pub use crate as r5;

pub mod types;

/// FHIR R5 resources
pub mod resources;

/// FHIR R5 code systems as type-safe enums
pub mod codes;

/// Lightweight FHIR R5 validation
pub mod validate;

/// Per-element metadata extracted from the FHIR R5 specification (cardinality,
/// bindings, choice types, reference targets, summary membership).
pub mod meta;

/// Support types for `value[x]` choice elements (see `spec/11-choice-types.md`).
pub mod choice;

/// Parsing and precision-aware comparison for the date/time primitives.
pub mod temporal;

/// The [`Coded`](coded::Coded) wrapper for `required`-binding coded fields.
pub mod coded;

/// Ergonomic extension accessors ([`ExtensionExt`](extension_ext::ExtensionExt)).
pub mod extension_ext;

/// Support for the generated `#[derive(Builder)]` builders.
pub mod builder;

/// Utilities for `Bundle`s: iteration, paging, and transaction/batch building.
pub mod bundle_util;

/// Summary serialization (the FHIR `_summary=true` view).
pub mod summary;

/// FHIR XML serialization (feature `xml`).
#[cfg(feature = "xml")]
pub mod xml;

/// Common imports for working with FHIR R5.
pub mod prelude;

/// An async FHIR R5 REST client (feature `client`).
#[cfg(feature = "client")]
pub mod client;

/// The FHIR R5 release, as a type.
///
/// Marker for release-parameterized code such as
/// [`ReleaseClient`](::fhir_core::client::ReleaseClient); see [`::fhir_core::release`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct R5;

impl ::fhir_core::release::Release for R5 {
    const LABEL: &'static str = "R5";
    const VERSION: &'static str = "5.0.0";

    fn elements() -> &'static [::fhir_core::meta::ElementMeta] {
        meta::elements()
    }
    type Resource = resources::Resource;
    type Bundle = resources::Bundle;
    type CapabilityStatement = resources::CapabilityStatement;
    type OperationOutcome = resources::OperationOutcome;

    fn next_link(bundle: &Self::Bundle) -> Option<String> {
        // R5 binds `Bundle.link.relation` to the IANA link-relations value
        // set, so it is a `Coded<E>` here where R3 and R4 have a plain
        // string. `Coded::code()` gives the wire form either way.
        bundle
            .link
            .iter()
            .find(|l| l.relation.code() == "next")
            .map(|l| l.url.0.clone())
    }
}
