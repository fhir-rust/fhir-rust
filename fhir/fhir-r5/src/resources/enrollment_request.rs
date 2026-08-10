//! EnrollmentRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EnrollmentRequest
//!
//! Version: 5.0.0
//!
//! EnrollmentRequest Resource: This resource provides the insurance enrollment details to the insurer regarding a specified coverage.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// EnrollmentRequest resource.
///
/// This resource provides the insurance enrollment details to the insurer
/// regarding a specified coverage. It is used to request that an insurer enroll
/// a candidate subject under a particular coverage, and is typically exchanged
/// between a provider and an insurer as part of the eligibility and enrollment
/// workflow in FHIR R5. The request identifies the candidate to be enrolled,
/// the responsible provider submitting the request, the insurer that is
/// expected to act on it, and the coverage under which enrollment is sought,
/// and it is normally answered with a corresponding `EnrollmentResponse`
/// indicating whether the request was accepted, rejected, or requires
/// further review.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient), which is commonly
///   referenced as the enrollment candidate.
/// - [`Reference`](crate::r5::types::Reference), used to point to the
///   insurer, provider, candidate, and coverage participants.
/// - `EnrollmentResponse`, the typical reply resource to this request.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::enrollment_request::EnrollmentRequest;
/// use fhir::r5::types;
///
/// let value = EnrollmentRequest {
///     created: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `created` is the name this serializes to on the wire.
/// assert_eq!(json["created"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: EnrollmentRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentRequest {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier assigned to this enrollment request by the submitter or receiver.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The current status of the request: active | cancelled | draft | entered-in-error.
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::FmStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The date this enrollment request was created.
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// The insurer that is to be notified and expected to act on this enrollment request.
    pub insurer: Option<types::Reference<crate::r5::resources::Organization>>,

    /// The practitioner or organization who is responsible for submitting the request.
    pub provider: Option<types::Reference>,

    /// A reference to the subject (typically a [`Patient`](crate::r5::resources::patient::Patient)) to be enrolled.
    pub candidate: Option<types::Reference<crate::r5::resources::Patient>>,

    /// A reference to the specific coverage under which enrollment is to be applied.
    pub coverage: Option<types::Reference<crate::r5::resources::Coverage>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EnrollmentRequest;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
