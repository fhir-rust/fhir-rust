//! EnrollmentResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EnrollmentResponse
//!
//! Version: 5.0.0
//!
//! EnrollmentResponse Resource: This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource provides enrollment and plan details from the processing of an
/// EnrollmentRequest resource.
///
/// EnrollmentResponse is the payer's reply to an `EnrollmentRequest`. It
/// communicates the outcome of processing a request to enroll a patient into a
/// coverage or plan, along with any disposition message describing the result.
/// It references the originating request, the responsible insurer organization,
/// and the provider that submitted the request. This resource is part of the
/// FHIR financial module.
///
/// # See also
///
/// - `EnrollmentRequest`, the originating request that this resource answers.
/// - [`Reference`](crate::r5::types::Reference), used to point to the
///   originating request, the insurer organization, and the requesting
///   provider.
/// - [`Patient`](crate::r5::resources::patient::Patient), which is typically
///   the subject of the coverage referenced by the originating request.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::enrollment_response::EnrollmentResponse;
/// use fhir::r5::types;
///
/// let value = EnrollmentResponse {
///     disposition: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `disposition` is the name this serializes to on the wire.
/// assert_eq!(json["disposition"], ::serde_json::json!("abc"));
///
/// let back: EnrollmentResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentResponse {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The status of the resource instance itself: active | cancelled | draft | entered-in-error
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::FmStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reference to the originating `EnrollmentRequest` that this response answers
    pub request: Option<types::Reference>,

    /// The processing outcome of the enrollment request: queued | complete | error | partial
    pub outcome: Option<crate::r5::coded::Coded<crate::r5::codes::EnrollmentOutcome>>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`).
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// A human-readable description of the outcome of processing the request
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`).
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// The date on which this enrollment response was created
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Reference to the insurer organization that processed the enrollment request
    pub organization: Option<types::Reference>,

    /// Reference to the practitioner who is responsible for the claim, if applicable
    pub request_provider: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EnrollmentResponse;

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
