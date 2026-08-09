//! GuidanceResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
//!
//! Version: 6.0.0-ballot3
//!
//! The formal response to a guidance request
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A guidance response is the formal response to a guidance request, including
/// any output parameters returned by the evaluation, as well as the
/// description of any proposed actions to be taken.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::guidance_response::GuidanceResponse;
/// use fhir::r6::types;
///
/// let value = GuidanceResponse {
///     occurrence_date_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `occurrenceDateTime` is the name this serializes to on the wire.
/// assert_eq!(json["occurrenceDateTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: GuidanceResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GuidanceResponse {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identifier of the request associated with this response, if any
    pub request_identifier: Option<types::Identifier>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// What guidance was requested
    /// The `GuidanceResponse.module[x]` choice element (1..1); see [`GuidanceResponseModule`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub module: Option<GuidanceResponseModule>,

    /// success | data-requested | data-required | in-progress | failure |
    /// entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::GuidanceResponseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Patient the request was performed for
    pub subject: Option<types::Reference>,

    /// Encounter during which the response was returned
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// When the guidance response was processed
    pub occurrence_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`occurrence_date_time`](Self::occurrence_date_time) (FHIR `_occurrenceDateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_occurrenceDateTime")]
    pub occurrence_date_time_ext: Option<types::Element>,

    /// Device returning the guidance
    pub performer: Option<types::Reference<crate::r6::resources::Device>>,

    /// Why guidance is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Additional notes about the response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Messages resulting from the evaluation of the artifact or artifacts
    pub evaluation_message: Option<types::Reference<crate::r6::resources::OperationOutcome>>,

    /// The output parameters of the evaluation, if any
    pub output_parameters: Option<types::Reference<crate::r6::resources::Parameters>>,

    /// Proposed actions, if any
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference>,

    /// Additional required data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_requirement: Vec<types::DataRequirement>,
}

/// The `GuidanceResponse.module[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum GuidanceResponseModule {
    /// `moduleUri` variant.
    #[fhir("moduleUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `moduleCanonical` variant.
    #[fhir("moduleCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `moduleCodeableConcept` variant.
    #[fhir("moduleCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = GuidanceResponse;

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
