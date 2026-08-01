//! Converting a resource from one FHIR release to another, saying what was
//! lost.
//!
//! The releases do not share model types (spec 12, R12.4), and there is no
//! `From`/`Into` between them: a conversion that compiled silently would be a
//! conversion whose losses were invisible, and in a health record that is worse
//! than one that fails. What this module offers instead is an **explicit**
//! conversion of the wire form, which hands back a [`LossReport`] naming
//! everything it changed or discarded.
//!
//! ```
//! use fhir::convert;
//! use fhir::r5::R5;
//! use serde_json::json;
//!
//! let patient = json!({
//!     "resourceType": "Patient",
//!     "id": "example",
//!     "active": true,
//! });
//!
//! // Converting a release to itself is the degenerate case, and it is lossless.
//! let out = convert::between::<R5, R5>(&patient);
//! assert!(out.report.is_lossless(), "{}", out.report);
//! assert_eq!(out.value, patient);
//! ```
//!
//! Across releases it is not lossless, and the report is how you find out what
//! it cost. `Patient.animal` existed in R3 and was removed in R4:
//!
//! ```ignore
//! // Requires `--features "r3 r4"`; see tests/convert_releases.rs.
//! use fhir::{convert, r3::R3, r4::R4};
//! use serde_json::json;
//!
//! let r3_patient = json!({
//!     "resourceType": "Patient",
//!     "animal": { "species": { "text": "canine" } },
//! });
//!
//! let out = convert::between::<R3, R4>(&r3_patient);
//! assert!(out.value.get("animal").is_none());
//! assert!(out.report.iter().any(|l| l.path == "Patient.animal"));
//! ```
//!
//! # What it does and does not do
//!
//! It is a **structural** conversion, driven by both releases' generated
//! element tables. It knows what each release's elements are, which types a
//! `value[x]` admits, what repeats and what does not, and what each release
//! requires. It does not know that one release's element was *renamed* into
//! another's — that is a semantic remapping, and guessing at one would be
//! precisely the silent mangling the separate types exist to prevent. Renamed
//! elements are reported as [`LossKind::ElementRemoved`], which is the truthful
//! answer: this layer did not carry them over.
//!
//! Because it is driven by the tables rather than by hand-written rules, every
//! pair of modelled releases is convertible in both directions, including
//! through R2 and R6.

pub use ::fhir_core::convert::{Converted, Loss, LossKind, LossReport};

use ::fhir_core::release::Release;
use ::serde_json::Value;

/// Convert a resource's JSON from release `S` to release `T`.
///
/// Name the releases by their marker types — [`r5::R5`](crate::r5::R5) and
/// friends — and the element tables come from them:
///
/// ```
/// use fhir::convert;
/// use fhir::r5::R5;
///
/// let obs = serde_json::json!({
///     "resourceType": "Observation",
///     "status": "final",
///     "code": { "text": "body weight" },
/// });
///
/// let out = convert::between::<R5, R5>(&obs);
/// assert!(out.report.is_lossless(), "{}", out.report);
/// ```
///
/// The returned [`Converted::value`] holds only what `T` accepts, so
/// deserializing it into `T`'s model succeeds where feeding it the source's
/// JSON directly would have failed — or, worse, quietly succeeded with fields
/// missing.
#[must_use]
pub fn between<S: Release, T: Release>(value: &Value) -> Converted {
    ::fhir_core::convert::resource(S::elements(), T::elements(), value)
}

/// Convert from release `S` to release `T`, or refuse.
///
/// [`between`] always produces a document and tells you what it cost. This
/// produces one only when it cost nothing, which is the right shape for callers
/// who would rather reject a document than forward a lossy version of it — in a
/// clinical exchange, a dropped element is a dropped fact, and the receiver has
/// no way to know it was ever there.
///
/// ```
/// use fhir::convert;
/// use fhir::r5::R5;
///
/// let obs = serde_json::json!({
///     "resourceType": "Observation",
///     "status": "final",
///     "code": { "text": "body weight" },
/// });
///
/// let value = convert::strict::<R5, R5>(&obs).expect("a release can represent its own documents");
/// assert_eq!(value["status"], "final");
/// ```
///
/// The error carries the whole [`LossReport`], so a caller that rejects a
/// document can still say precisely why:
///
/// ```
/// # use fhir::convert;
/// # use fhir::r5::R5;
/// let not_a_resource = serde_json::json!({ "status": "final" });
///
/// match convert::strict::<R5, R5>(&not_a_resource) {
///     Ok(_) => panic!("it has no resourceType"),
///     Err(report) => assert!(!report.is_lossless()),
/// }
/// ```
///
/// # Errors
///
/// The [`LossReport`], whenever the conversion was not lossless. See
/// [`Converted::strict`] for why warnings count as well as discarded data.
pub fn strict<S: Release, T: Release>(value: &Value) -> Result<Value, LossReport> {
    between::<S, T>(value).strict()
}

/// Convert a typed resource from release `S` into release `T`'s JSON.
///
/// The same conversion as [`between`], starting from a value of the source
/// release's model rather than from JSON you already have.
///
/// It takes the release's [`Resource`](fhir_core::release::Release::Resource)
/// enum rather than a bare resource struct, and must: `resourceType` comes from
/// that enum's serde tag, so a `Patient` on its own does not say it is a
/// `Patient` and there would be nothing for the converter to key on.
///
/// ```
/// use fhir::convert;
/// use fhir::r5::{R5, resources::{Patient, Resource}, types};
///
/// let patient = Resource::Patient(Box::new(Patient {
///     id: Some(types::String("example".to_string())),
///     ..Default::default()
/// }));
///
/// let out = convert::from_typed::<R5, R5>(&patient);
/// assert_eq!(out.value["id"], "example");
/// assert!(out.report.is_lossless(), "{}", out.report);
/// ```
#[must_use]
pub fn from_typed<S: Release, T: Release>(resource: &S::Resource) -> Converted {
    let value = ::serde_json::to_value(resource).unwrap_or(Value::Null);
    between::<S, T>(&value)
}
