//! GuidanceResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
//!
//!
//!
//! The formal response to a guidance request
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for GuidanceResponse Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::guidance_response::GuidanceResponse;
/// use fhir::r3::types;
///
/// let value = GuidanceResponse {
///     request_id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `requestId` is the name this serializes to on the wire.
/// assert_eq!(json["requestId"], ::serde_json::json!("pat-1"));
///
/// let back: GuidanceResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "GuidanceResponseDe")]
#[fhir_version("r3")]
pub struct GuidanceResponse {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The id of the request associated with this response, if any
    pub request_id: Option<types::Id>,
    /// Primitive extension sibling for [`request_id`](Self::request_id) (FHIR `_requestId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requestId")]
    pub request_id_ext: Option<types::Element>,

    /// Business identifier
    pub identifier: Option<types::Identifier>,

    /// A reference to a knowledge module
    pub module: types::Reference<crate::r3::resources::ServiceDefinition>,

    /// success | data-requested | data-required | in-progress | failure |
    /// entered-in-error
    pub status: crate::coded::Coded<crate::r3::codes::GuidanceResponseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Patient the request was performed for
    pub subject: Option<types::Reference>,

    /// Encounter or Episode during which the response was returned
    pub context: Option<types::Reference>,

    /// When the guidance response was processed
    pub occurrence_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`occurrence_date_time`](Self::occurrence_date_time) (FHIR `_occurrenceDateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_occurrenceDateTime")]
    pub occurrence_date_time_ext: Option<types::Element>,

    /// Device returning the guidance
    pub performer: Option<types::Reference<crate::r3::resources::Device>>,

    /// Reason for the response
    /// The `GuidanceResponse.reason[x]` choice element (0..1); see [`GuidanceResponseReason`].
    #[serde(flatten)]
    pub reason: Option<GuidanceResponseReason>,

    /// Additional notes about the response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Messages resulting from the evaluation of the artifact or artifacts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evaluation_message: Vec<types::Reference<crate::r3::resources::OperationOutcome>>,

    /// The output parameters of the evaluation, if any
    pub output_parameters: Option<types::Reference<crate::r3::resources::Parameters>>,

    /// Proposed actions, if any
    pub result: Option<types::Reference>,

    /// Additional required data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_requirement: Vec<types::DataRequirement>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuidanceResponseDe {
    id: Option<types::Id>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r3::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    request_id: Option<types::Id>,
    #[serde(rename = "_requestId")]
    request_id_ext: Option<types::Element>,
    identifier: Option<types::Identifier>,
    module: types::Reference<crate::r3::resources::ServiceDefinition>,
    status: crate::coded::Coded<crate::r3::codes::GuidanceResponseStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    subject: Option<types::Reference>,
    context: Option<types::Reference>,
    occurrence_date_time: Option<types::DateTime>,
    #[serde(rename = "_occurrenceDateTime")]
    occurrence_date_time_ext: Option<types::Element>,
    performer: Option<types::Reference<crate::r3::resources::Device>>,
    #[serde(flatten)]
    reason: crate::r3::choice::Slot<GuidanceResponseReason>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    evaluation_message: Vec<types::Reference<crate::r3::resources::OperationOutcome>>,
    output_parameters: Option<types::Reference<crate::r3::resources::Parameters>>,
    result: Option<types::Reference>,
    #[serde(default)]
    data_requirement: Vec<types::DataRequirement>,
}

impl ::core::convert::From<GuidanceResponseDe> for GuidanceResponse {
    fn from(v: GuidanceResponseDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            request_id: v.request_id,
            request_id_ext: v.request_id_ext,
            identifier: v.identifier,
            module: v.module,
            status: v.status,
            status_ext: v.status_ext,
            subject: v.subject,
            context: v.context,
            occurrence_date_time: v.occurrence_date_time,
            occurrence_date_time_ext: v.occurrence_date_time_ext,
            performer: v.performer,
            reason: v.reason.0,
            note: v.note,
            evaluation_message: v.evaluation_message,
            output_parameters: v.output_parameters,
            result: v.result,
            data_requirement: v.data_requirement,
        }
    }
}

/// The `GuidanceResponse.reason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum GuidanceResponseReason {
    /// `reasonCodeableConcept` variant.
    #[fhir("reasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonReference` variant.
    #[fhir("reasonReference")]
    Reference(Box<types::Reference>),
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
