//! GuidanceResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
//!
//! Version: 4.3.0
//!
//! The formal response to a guidance request
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A guidance response is the formal response to a guidance request, including
/// any output parameters returned by the evaluation, as well as the
/// description of any proposed actions to be taken.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::guidance_response::GuidanceResponse;
/// use fhir::r4b::types;
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
#[serde(from = "GuidanceResponseDe")]
#[fhir_version("r4b")]
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
    pub contained: Vec<crate::r4b::resources::Resource>,

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
    pub status: crate::coded::Coded<crate::r4b::codes::GuidanceResponseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Patient the request was performed for
    pub subject: Option<types::Reference>,

    /// Encounter during which the response was returned
    pub encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,

    /// When the guidance response was processed
    pub occurrence_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`occurrence_date_time`](Self::occurrence_date_time) (FHIR `_occurrenceDateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_occurrenceDateTime")]
    pub occurrence_date_time_ext: Option<types::Element>,

    /// Device returning the guidance
    pub performer: Option<types::Reference<crate::r4b::resources::Device>>,

    /// Why guidance is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Why guidance is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Additional notes about the response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Messages resulting from the evaluation of the artifact or artifacts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evaluation_message: Vec<types::Reference<crate::r4b::resources::OperationOutcome>>,

    /// The output parameters of the evaluation, if any
    pub output_parameters: Option<types::Reference<crate::r4b::resources::Parameters>>,

    /// Proposed actions, if any
    pub result: Option<types::Reference>,

    /// Additional required data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_requirement: Vec<types::DataRequirement>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuidanceResponseDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r4b::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    request_identifier: Option<types::Identifier>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(flatten)]
    module: crate::r4b::choice::Slot<GuidanceResponseModule>,
    status: crate::coded::Coded<crate::r4b::codes::GuidanceResponseStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    subject: Option<types::Reference>,
    encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,
    occurrence_date_time: Option<types::DateTime>,
    #[serde(rename = "_occurrenceDateTime")]
    occurrence_date_time_ext: Option<types::Element>,
    performer: Option<types::Reference<crate::r4b::resources::Device>>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    evaluation_message: Vec<types::Reference<crate::r4b::resources::OperationOutcome>>,
    output_parameters: Option<types::Reference<crate::r4b::resources::Parameters>>,
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
            request_identifier: v.request_identifier,
            identifier: v.identifier,
            module: v.module.0,
            status: v.status,
            status_ext: v.status_ext,
            subject: v.subject,
            encounter: v.encounter,
            occurrence_date_time: v.occurrence_date_time,
            occurrence_date_time_ext: v.occurrence_date_time_ext,
            performer: v.performer,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            note: v.note,
            evaluation_message: v.evaluation_message,
            output_parameters: v.output_parameters,
            result: v.result,
            data_requirement: v.data_requirement,
        }
    }
}

/// The `GuidanceResponse.module[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum GuidanceResponseModule {
    /// `moduleUri` variant.
    #[fhir("moduleUri")]
    Uri(crate::r4b::choice::Primitive<types::Uri>),
    /// `moduleCanonical` variant.
    #[fhir("moduleCanonical")]
    Canonical(crate::r4b::choice::Primitive<types::Canonical>),
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
