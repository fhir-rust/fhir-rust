//! Release-independent core for the FHIR crates.
//!
//! Everything here is the same for R3, R4, and R5: one `Decimal`, one
//! `Validate` trait, one `Coded<E>`, one date/time parser, one XML reader,
//! one REST client generic over [`release::Release`]. The release crates
//! depend on this one and re-export its items under their own paths, so
//! `fhir::r5::validate::Validate` and `fhir::r4::validate::Validate` are the
//! *same* trait and a value from either release satisfies it.
//!
//! That sharing is the point. A `Validate` defined per release would make
//! `impl Validate for T` non-portable across releases, and a `Decimal`
//! generated three times would be three incompatible types for one FHIR
//! primitive.
//!
//! This crate is not intended to be depended on directly; use `fhir`.

// The doc comments here link to release items (`crate::r4::codes`) that live
// in the release crates and are not visible from here. The links are correct
// for readers of the `fhir` facade, where these modules are re-exported, so
// they are kept rather than stripped.
#![allow(rustdoc::broken_intra_doc_links)]
// FHIR prose contains bare specification URLs and `value[x]` notation.
#![allow(rustdoc::bare_urls)]

/// The FHIR `decimal` primitive, preserving the precision it was given.
pub mod decimal;

/// The [`Validate`](validate::Validate) trait and [`ValidationIssue`](validate::ValidationIssue).
pub mod validate;

/// The [`Coded<E>`](coded::Coded) wrapper for `required`-binding coded fields.
pub mod coded;

/// Support for the generated `#[derive(Builder)]` builders.
pub mod builder;

/// Per-element metadata types, queryable by FHIR path.
pub mod meta;

/// Parsing and precision-aware comparison for the date/time primitives.
pub mod temporal;

/// `_summary` element filtering.
pub mod summary;

/// The [`Release`](release::Release) trait each release crate implements.
pub mod release;

/// Converting a resource between releases, with an explicit loss report.
pub mod convert;

/// Shared helpers.
pub mod util;

/// XML reading and writing (feature `xml`).
#[cfg(feature = "xml")]
pub mod xml;

/// An async FHIR REST client, generic over the release (feature `client`).
#[cfg(feature = "client")]
pub mod client;
