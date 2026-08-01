//! # FHIR for Rust
//!
//! `fhir` is a Rust implementation of the **HL7 FHIR®** data model, together
//! with a code generator that produces it from the official FHIR specification
//! JSON files. Three releases are modelled: **R5** (5.0.0) under [`r5`],
//! **R4** (4.0.1) under [`r4`], and **R3** (3.0.2, also called STU3) under
//! [`r3`].
//!
//! Fast Healthcare Interoperability Resources (FHIR) is the HL7 standard for
//! exchanging electronic health records. For each release this crate gives you:
//!
//! - **Every resource** (Patient, Observation, Encounter, …) as a Rust
//!   `struct` under [`r5::resources`] / [`r4::resources`] / [`r3::resources`],
//!   each round-tripping to and from the canonical FHIR JSON via `serde`.
//! - **Every datatype** (Period, HumanName, CodeableConcept, …) under
//!   [`r5::types`] / [`r4::types`] / [`r3::types`], including the primitive
//!   newtypes ([`Code`](r5::types::Code), [`Id`](r5::types::Id),
//!   [`DateTime`](r5::types::DateTime), …).
//! - **Type-safe code systems** — 400+ FHIR `CodeSystem`s as Rust enums under
//!   [`r5::codes`] / [`r4::codes`] / [`r3::codes`].
//! - **A polymorphic `Resource` enum** tagged by `resourceType`, for reading a
//!   resource whose type you do not know ahead of time.
//! - **Lightweight validation** via the [`Validate`](validate::Validate) trait
//!   and `#[derive(Validate)]`.
//! - **A code generator** under [`codegen`] that reads the specification JSON
//!   for a release and emits its Rust model.
//!
//! FHIR® is a registered trademark of Health Level Seven International. This
//! crate is not affiliated with or endorsed by HL7.
//!
//! ## Installation
//!
//! Each release is a complete model of ~135,000 lines, so they are cargo
//! features and you compile only what you use. `r5` is on by default:
//!
//! ```toml
//! [dependencies]
//! fhir = "1"                                                # R5 only
//! # fhir = { version = "1", features = ["r3", "r4"] }       # every release
//! # fhir = { version = "1", default-features = false, features = ["r3"] }  # R3 only
//! serde_json = "1" # or any other serde data format
//! ```
//!
//! ## Choosing a release
//!
//! An R3, R4 and R5 `Patient` are three different Rust types, and that is
//! deliberate. The releases disagree in ways that quietly corrupt data if
//! conflated. `Observation.value[x]` allows eleven types in R3 and eleven in
//! R4 — but not the same eleven, since R3 permits `Attachment` and not
//! `integer` and R4 reversed both. A resource's `id` is typed `id` in R3 and
//! `string` afterwards. `MedicationRequest.medication[x]` is a choice element
//! in R4 but a `CodeableReference` in R5. R3 has no `canonical` or `url`
//! primitive; R4 has no `integer64` or `CodeableReference`. Convert between
//! releases explicitly, through JSON, deciding what to do with whatever does
//! not carry over — see the `r4_and_r5_side_by_side` example.
//!
//! What the releases *do* share lives in the crate root and is re-exported from
//! both: the [`Validate`](validate::Validate) trait, [`Coded<E>`](coded::Coded),
//! [`BuilderError`](builder::BuilderError), the [`meta`] table types, date/time
//! parsing in [`temporal`], and the REST [`client`], which is generic over a
//! [`Release`](release::Release).
//!
//! The examples below use R5. Every one of them works for another release by
//! changing `r5` in the import path.
//!
//! ## Design in one paragraph
//!
//! Everything in the data model derives [`serde::Serialize`] and
//! [`serde::Deserialize`], so you work with the crate almost entirely through
//! `serde_json`. Resources and complex datatypes are plain structs; FHIR
//! *primitive* types (`code`, `id`, `dateTime`, …) are thin newtypes such as
//! `Code(String)` that serialize *transparently* to their underlying JSON
//! value. Optional elements are `Option<T>`, repeating elements are `Vec<T>`,
//! and FHIR's `value[x]` choice elements are a generated enum per element. See
//! [Cardinality and choice types](#cardinality-and-choice-types) below.
//!
//! ## Quick start
//!
//! Build a `Patient`, serialize it to canonical FHIR JSON, and parse it back:
//!
//! ```
//! use fhir::r5::resources::Patient;
//! use fhir::r5::coded::Coded;
//! use fhir::r5::codes::AdministrativeGender;
//! use fhir::r5::types::{Boolean, HumanName, String as FhirString};
//!
//! let patient = Patient {
//!     id: Some(FhirString("pat-1".to_string())),
//!     active: Some(Boolean(true)),
//!     gender: Some(Coded::Known(AdministrativeGender::Male)),
//!     name: vec![HumanName {
//!         family: Some(FhirString("Chalmers".to_string())),
//!         given: vec![FhirString("Peter".to_string())],
//!         ..Default::default()
//!     }],
//!     ..Default::default()
//! };
//!
//! // Serialize to canonical FHIR JSON. `None` fields are omitted.
//! let json = serde_json::to_string_pretty(&patient).unwrap();
//! assert!(json.contains("\"family\": \"Chalmers\""));
//!
//! // Parse it back — a perfect round trip.
//! let parsed: Patient = serde_json::from_str(&json).unwrap();
//! assert_eq!(parsed, patient);
//! ```
//!
//! ## Primitives are transparent newtypes
//!
//! A FHIR primitive is a wrapper around a Rust value, so its JSON form is just
//! that value — no wrapper object:
//!
//! ```
//! use fhir::r5::types::{Boolean, Code, Integer64};
//!
//! assert_eq!(serde_json::to_value(Code("final".to_string())).unwrap(), "final");
//! assert_eq!(serde_json::to_value(Boolean(true)).unwrap(), true);
//! // `integer64` is serialized as a JSON *string* per the FHIR spec:
//! assert_eq!(serde_json::to_value(Integer64(9_000_000_000)).unwrap(), "9000000000");
//! ```
//!
//! ## Validation
//!
//! The [`Validate`](r5::validate::Validate) trait reports every problem it
//! finds as a [`ValidationIssue`](r5::validate::ValidationIssue). Primitive
//! types check their FHIR regex constraints; `#[derive(Validate)]` makes
//! complex types and resources validate recursively, prefixing each nested
//! issue's `path` with the field name.
//!
//! ```
//! use fhir::r5::resources::Patient;
//! use fhir::r5::types::{Id, Uri};
//! use fhir::r5::validate::Validate;
//!
//! // Primitive format checks:
//! assert!(Id("patient-1".to_string()).is_valid());
//! assert!(!Id("has spaces".to_string()).is_valid());
//!
//! // Recursive validation of a whole resource:
//! let mut patient = Patient::default();
//! assert!(patient.validate().is_empty());
//!
//! // A `uri` may not be surrounded by whitespace.
//! patient.implicit_rules = Some(Uri(" http://bad ".to_string()));
//! let issues = patient.validate();
//! assert_eq!(issues.len(), 1);
//! // The path is prefixed with the field name, then the primitive's own label.
//! assert_eq!(issues[0].path, "implicit_rules.uri");
//! ```
//!
//! ### What validation does not cover
//!
//! "Validation" here means *structural* validation — element existence,
//! types, cardinality, primitive lexical rules, choice exclusivity, required
//! bindings — plus three FHIR invariants (`ext-1`, `dom-2`, `dom-4`) of the
//! 314 that R5 states. It does **not** check FHIRPath invariants (the other
//! 311 keys, including `ele-1`), profile conformance (US Core, IPS, or any
//! implementation guide), terminology membership beyond the required-binding
//! enums, or whether a `Reference` resolves to anything.
//!
//! A resource this crate calls valid may still be rejected by a conformant
//! FHIR server. `spec/10-invariants-coverage.md` enumerates every unenforced
//! rule rather than leaving the gap implicit.
//!
//! ## Code systems as enums
//!
//! Coded values are available as type-safe enums that serialize to their
//! canonical FHIR code strings:
//!
//! ```
//! use fhir::r5::codes::AdministrativeGender;
//!
//! let gender = AdministrativeGender::Female;
//! assert_eq!(serde_json::to_value(&gender).unwrap(), "female");
//!
//! let parsed: AdministrativeGender = serde_json::from_value("male".into()).unwrap();
//! assert_eq!(parsed, AdministrativeGender::Male);
//! ```
//!
//! ## Reading a resource of unknown type
//!
//! When you receive JSON but do not know its `resourceType`, deserialize into
//! the [`Resource`](r5::resources::Resource) enum. It is tagged by
//! `resourceType`, so serde picks the right variant for you:
//!
//! ```
//! use fhir::r5::resources::Resource;
//!
//! let json = serde_json::json!({
//!     "resourceType": "Patient",
//!     "id": "pat-1",
//!     "active": true
//! });
//!
//! match serde_json::from_value(json).unwrap() {
//!     Resource::Patient(patient) => {
//!         assert_eq!(patient.id.unwrap().0, "pat-1");
//!     }
//!     other => panic!("expected a Patient, got {other:?}"),
//! }
//! ```
//!
//! ## Cardinality and choice types
//!
//! FHIR element cardinality maps to Rust types as follows:
//!
//! | FHIR cardinality | Rust type        |
//! |------------------|------------------|
//! | `0..1`           | `Option<T>`      |
//! | `1..1`           | `T`              |
//! | `0..*`           | `Vec<T>`         |
//! | `1..*`           | `Vec1<T>`        |
//!
//! A `value[x]` *choice* element becomes one generated enum with a variant per
//! allowed type — for example `Observation.value` is `Option<ObservationValue>`
//! with variants `Quantity`, `String`, `Boolean`, … so exactly one is set.
//! Required-binding coded fields are their [`r5::codes`] enum wrapped in
//! [`Coded`](r5::coded::Coded). Every resource and datatype has a
//! `Type::builder()`, and [`prelude`] re-exports the common items.
//!
//! ## More examples
//!
//! Runnable programs live in the `examples/` directory. Run one with, e.g.:
//!
//! ```sh
//! cargo run --example build_patient
//! cargo run --example validate_resource
//! cargo run --example read_bundle
//! cargo run --example primitive_extensions
//! cargo run --example operation_outcome
//! cargo run --example extensions
//! cargo run --example transaction_bundle
//! ```
//!
//! ## Crate layout
//!
//! Per release (`r5` shown; `r4` is identical in shape):
//!
//! - [`r5::resources`] — the 158 R5 resources (146 in R4, 117 in R3), plus the
//!   [`Resource`](r5::resources::Resource) enum.
//! - [`r5::types`] — the complex datatypes and primitive newtypes.
//! - [`r5::codes`] — FHIR `CodeSystem`s as enums.
//! - [`r5::validate`] — that release's primitive-format constraints.
//! - [`r5::meta`] — per-element specification metadata (cardinality, bindings,
//!   choice types, reference targets).
//!
//! Shared across releases:
//!
//! - [`validate`] — the [`Validate`](validate::Validate) trait and [`ValidationIssue`](validate::ValidationIssue).
//! - [`coded`], [`builder`], [`meta`], [`temporal`], [`summary`] — the machinery each release binds to its own types.
//! - [`release`] — the [`Release`](release::Release) trait, for code generic over a release.
//! - [`client`] — the async REST client (feature `client`).
//! - [`codegen`] — the generator that turns specification JSON into a release model.

// Enable Clippy's pedantic lint group across the whole crate.
// A data model has no business dereferencing raw pointers (spec R13.14).
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
// This crate deliberately nests each concept in a directory beside a
// same-named module file (e.g. `resource/resource.rs`), so allow the
// resulting module-inception pattern crate-wide.
#![allow(clippy::module_inception)]
// The type/field doc comments are long-form FHIR prose (mentioning terms such
// as ValueSet, ConceptMap, JSON, XHTML, etc.); backticking every such term
// would harm readability without adding value, so allow these doc lints.
#![allow(clippy::doc_markdown)]
#![allow(clippy::doc_link_with_quotes)]
// The generated/parse modules deliberately glob-import their sibling namespace
// (`use crate::r5::parse::…::*;`) as a convenience; keep that pattern.
#![allow(clippy::wildcard_imports)]
// These fire only on the work-in-progress code-generator helpers (which unwrap
// freely and return owned values). Documenting every panic/`Result`/`#[must_use]`
// on WIP internals is premature churn; revisit once the generator API stabilizes.
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
// The datatype/resource doc comments are long-form FHIR prose that contains
// bare specification URLs (e.g. `http://hl7.org/fhir/...`) and FHIR choice
// notation such as `value[x]`. rustdoc would otherwise treat these as bare
// URLs or (broken) intra-doc links. Backticking/wrapping every occurrence
// across ~200 modules would not improve the docs, so allow these rustdoc lints.
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::broken_intra_doc_links)]

#[cfg(feature = "client")]
pub mod client {
    //! An async FHIR REST client (feature `client`).
    //!
    //! The client itself is release-independent and lives in `fhir-core`,
    //! generic over [`Release`](crate::release::Release). The R5 aliases below
    //! cannot: they name a concrete release model, which the core crate has
    //! no knowledge of.
    pub use ::fhir_core::client::*;

    /// An async FHIR R5 REST client.
    #[cfg(feature = "r5")]
    pub type Client = ReleaseClient<crate::r5::R5>;

    /// An error from a FHIR R5 REST interaction.
    #[cfg(feature = "r5")]
    pub type ClientError = ReleaseClientError<crate::r5::R5>;
}

/// Common imports for working with FHIR R5 (feature `r5`).
#[cfg(feature = "r5")]
pub mod prelude;

/// The FHIR R6 draft model (feature `r6`).
///
/// **R6 is in ballot.** This model is generated from 6.0.0-ballot3 and will
/// change as the ballot does; it is not covered by this crate's semver
/// promise.
#[cfg(feature = "r6")]
pub use ::fhir_release_6 as r6;

#[cfg(feature = "r5")]
pub mod r5 {
    //! The FHIR R5 model (feature `r5`).
    //!
    //! The model itself lives in the `fhir-r5` crate and is re-exported here
    //! wholesale, so `fhir::r5::types::Patient` is unchanged. `parse` stays
    //! in this crate: it is work-in-progress generator machinery that reads
    //! this repository's specification files through
    //! [`DEFINITIONS_DIR`](crate::DEFINITIONS_DIR), which a published model
    //! crate has no business carrying.
    pub use ::fhir_release_5::*;

    /// Work-in-progress helpers that parse the FHIR specification.
    pub mod parse;
}

/// The FHIR Release 4 (4.0.1) model (feature `r4`).
#[cfg(feature = "r4")]
pub use ::fhir_release_4 as r4;

/// The FHIR Release 3 (3.0.2, also called STU3) model (feature `r3`).
#[cfg(feature = "r3")]
pub use ::fhir_release_3 as r3;

/// The FHIR DSTU2 (1.0.2) model (feature `r2`).
///
/// Retired by HL7 in favour of R3, and modelled because deployed systems
/// still speak it.
#[cfg(feature = "r2")]
pub use ::fhir_release_2 as r2;

pub use ::fhir_core::util;

// ---- Release-independent core ----------------------------------------------
//
// These modules hold the parts of the model that do not change between FHIR
// releases, so that one implementation serves every release and a value from
// any release satisfies the same traits. Each release module re-exports them
// (e.g. `r5::validate::Validate` is `validate::Validate`).

/// The [`Validate`](validate::Validate) trait and [`ValidationIssue`](validate::ValidationIssue),
/// shared by every FHIR release.
pub use ::fhir_core::validate;

/// The [`Coded<E>`](coded::Coded) wrapper for `required`-binding coded fields.
pub use ::fhir_core::coded;

/// Support for the generated `#[derive(Builder)]` builders.
pub use ::fhir_core::builder;

/// The shape of the per-element specification metadata each release generates.
pub use ::fhir_core::meta;

/// Parsing and precision-aware comparison for the FHIR date/time primitives.
pub use ::fhir_core::temporal;

/// The FHIR `decimal` primitive, whose lexical precision is clinical data.
///
/// Shared rather than generated per release: `decimal` means the same thing
/// in R3, R4 and R5, and one implementation is one place to be correct.
pub use ::fhir_core::decimal;

/// Pruning a resource to the FHIR `_summary=true` view.
pub use ::fhir_core::summary;

/// FHIR XML serialization (feature `xml`).
#[cfg(feature = "xml")]
pub use ::fhir_core::xml;

/// Naming a FHIR release in generic code (the [`Release`](release::Release) trait).
pub use ::fhir_core::release;

/// Converting a resource between releases, with an explicit loss report.
pub mod convert;

/// The spec-JSON to Rust code generator, parameterized by FHIR release.
pub mod codegen;

/// Absolute path to the directory holding the FHIR R5 specification JSON files
/// that ship with this crate.
///
/// This names R5 specifically, for the R5-only code that predates multi-release
/// support. For a directory chosen by release, use
/// [`Version::definitions_dir`](codegen::Version::definitions_dir).
pub static DEFINITIONS_DIR: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("doc")
            .join("fhir-specifications")
            .join("r5")
            .join("fhir-definitions-json")
    });

/// Literate programming: a block of generated Rust source code, as a string.
pub type SourceCodeString = String;
