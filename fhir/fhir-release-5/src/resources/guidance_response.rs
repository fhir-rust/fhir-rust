//! GuidanceResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GuidanceResponse
//!
//! Version: 5.0.0
//!
//! GuidanceResponse Resource: A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A guidance response is the formal response to a guidance request,
/// including any output parameters returned by the evaluation, as well as
/// the description of any proposed actions to be taken. It is typically
/// produced by a clinical decision support service after evaluating a
/// knowledge module (such as a rule, ECA rule, or order set) against a
/// patient's data, and may carry proposed actions, data requirements, or
/// evaluation messages.
///
/// Within the FHIR R5 clinical decision support workflow, a
/// `GuidanceResponse` is generated as the result of invoking a CDS Hooks
/// service, a `$evaluate` operation on a `PlanDefinition`, or an equivalent
/// knowledge artifact. It correlates back to the originating request via
/// `request_identifier`, references the subject and encounter the guidance
/// applies to, and reports its processing outcome through `status`
/// (for example `success`, `data-required`, or `failure`). The response may
/// bundle structured output parameters, proposed follow-up actions as
/// `result` references, evaluation `note`s, and any `data_requirement`s the
/// evaluating module still needs in order to complete its assessment.
///
/// # Related resources
///
/// - The `subject` is typically a [`Patient`](crate::r5::resources::patient::Patient).
/// - The guidance module itself may be identified via `module_codeable_concept`
///   using a [`CodeableConcept`](crate::r5::types::CodeableConcept), or by
///   canonical/absolute URI referencing a `PlanDefinition` or `ActivityDefinition`.
/// - `evaluation_message` and `output_parameters` typically reference an
///   `OperationOutcome` and a `Parameters` resource, respectively.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::guidance_response::GuidanceResponse;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GuidanceResponse {
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

    /// The identifier of the original guidance request that this response correlates to, if any
    pub request_identifier: Option<types::Identifier>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The `GuidanceResponse.module[x]` choice element (0..1); see [`GuidanceResponseModule`].
    #[serde(flatten)]
    pub module: Option<GuidanceResponseModule>,

    /// The processing status of the guidance response: success | data-requested | data-required | in-progress | failure | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::GuidanceResponseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The patient (or other subject) the guidance was requested and evaluated for
    pub subject: Option<types::Reference>,

    /// Encounter during which the response was returned
    pub encounter: Option<types::Reference>,

    /// When the guidance response was processed
    pub occurrence_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`occurrence_date_time`](Self::occurrence_date_time) (FHIR `_occurrenceDateTime`).
    #[serde(rename = "_occurrenceDateTime")]
    pub occurrence_date_time_ext: Option<types::Element>,

    /// Device returning the guidance
    pub performer: Option<types::Reference>,

    /// Why guidance is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Additional notes about the response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Reference to an OperationOutcome containing messages resulting from the evaluation of the artifact or artifacts
    pub evaluation_message: Option<types::Reference>,

    /// Reference to a Parameters resource containing the output parameters of the evaluation, if any
    pub output_parameters: Option<types::Reference>,

    /// Proposed actions resulting from the evaluation, such as RequestGroup resources, if any
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference>,

    /// Additional data that was requested by the evaluating module but was not supplied with the original request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_requirement: Vec<types::DataRequirement>,
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
/// The `GuidanceResponse.module[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum GuidanceResponseModule {
    /// `moduleUri` variant.
    #[fhir("moduleUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `moduleCanonical` variant.
    #[fhir("moduleCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
    /// `moduleCodeableConcept` variant.
    #[fhir("moduleCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}
