//! Naming a FHIR release in generic code.
//!
//! The data model is release-specific — an R4 `Patient` and an R5 `Patient` are
//! different Rust types — but code that merely *moves resources around* (the
//! REST client, for example) is not. [`Release`] lets such code name "the
//! `Bundle` of whichever release the caller chose" without duplicating itself
//! per release.
//!
//! Each enabled release module provides a marker type implementing this trait:
//! [`r4::R4`](crate::r4::R4) and [`r5::R5`](crate::r5::R5).
//!
//! ```
//! use fhir::release::Release;
//! use fhir::r5::R5;
//!
//! assert_eq!(R5::LABEL, "R5");
//! assert_eq!(R5::VERSION, "5.0.0");
//!
//! // The associated types name that release's resources.
//! let bundle = <R5 as Release>::Bundle::default();
//! assert!(bundle.entry.is_empty());
//! ```

use ::serde::de::DeserializeOwned;

/// A FHIR release, as a type: a marker naming that release's core resources.
///
/// Implemented by [`r4::R4`](crate::r4::R4) and [`r5::R5`](crate::r5::R5).
pub trait Release {
    /// The release label, e.g. `"R5"`.
    const LABEL: &'static str;

    /// The full FHIR version, e.g. `"5.0.0"`.
    const VERSION: &'static str;

    /// This release's per-element metadata table, sorted by FHIR path.
    ///
    /// The same table [`meta::element`](crate::meta) queries. Exposing it here
    /// is what lets release-agnostic code — the cross-release converter in
    /// [`convert`](crate::convert) — work on a release named only as a type
    /// parameter.
    fn elements() -> &'static [crate::meta::ElementMeta];

    /// The polymorphic resource enum, tagged by `resourceType`.
    ///
    /// `Serialize` is required as well as `DeserializeOwned` because this is
    /// the only type in a release that carries its own `resourceType` — the
    /// individual resource structs do not, the enum's `#[serde(tag)]` supplies
    /// it — and cross-release conversion needs that tag to know what it is
    /// looking at.
    type Resource: DeserializeOwned + ::serde::Serialize + std::fmt::Debug;

    /// The `Bundle` resource.
    type Bundle: DeserializeOwned + std::fmt::Debug + Default;

    /// The `CapabilityStatement` resource returned by `GET [base]/metadata`.
    type CapabilityStatement: DeserializeOwned + std::fmt::Debug;

    /// The `OperationOutcome` resource servers return to describe errors.
    type OperationOutcome: DeserializeOwned + std::fmt::Debug;

    /// The URL of the bundle's `next` link, if it has one.
    ///
    /// Paging is the one piece of "moving resources around" that has to look
    /// *inside* a release's `Bundle`, so the trait exposes just that much
    /// rather than the whole type. Without it, following a searchset across
    /// pages would be written once per release.
    fn next_link(bundle: &Self::Bundle) -> Option<String>;
}
