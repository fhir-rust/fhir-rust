//! QuestionnaireResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
//!
//! Version: 5.0.0
//!
//! QuestionnaireResponse Resource: A structured set of questions and their answers.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A structured set of questions and their answers.
///
/// A QuestionnaireResponse captures the answers gathered when a Questionnaire is
/// completed. The questions are ordered and grouped into coherent subsets that
/// correspond to the grouping structure of the Questionnaire being responded to.
/// Each response records the subject, author, and source of the answers, along
/// with a nested tree of items that hold the individual answers and their child
/// items.
///
/// Clinically and administratively, a QuestionnaireResponse is used to capture
/// structured data collected through forms such as intake assessments, patient
/// reported outcomes, screening tools, surveys, and consent or eligibility
/// questionnaires. It links back to the originating `Questionnaire` definition
/// via a canonical URL and, through its `item` elements, mirrors the grouping
/// and ordering of that definition so that each answer can be traced to the
/// specific question that prompted it. The resource tracks lifecycle state
/// through its `status` element (for example in-progress, completed, or
/// amended), records when the answers were authored, and identifies both who
/// or what recorded the response and who or what actually supplied the
/// answers, which may differ (for example, a clinician recording answers
/// given verbally by a patient).
///
/// See also: the response subject is typically a [`Patient`](crate::r5::resources::patient::Patient)
/// or other resource referenced via `subject`, coded answer values commonly use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) or `Coding`, and the
/// structure of `item` mirrors the corresponding `Questionnaire` resource.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire_response::QuestionnaireResponse;
/// use fhir::r5::types;
///
/// let value = QuestionnaireResponse {
///     authored: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authored` is the name this serializes to on the wire.
/// assert_eq!(json["authored"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: QuestionnaireResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponse {
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

    /// Business identifier for this set of answers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Request fulfilled by this QuestionnaireResponse
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Canonical URL of the `Questionnaire` resource that defines the questions being answered
    pub questionnaire: types::Canonical,
    /// Primitive extension sibling for [`questionnaire`](Self::questionnaire) (FHIR `_questionnaire`).
    #[serde(rename = "_questionnaire")]
    pub questionnaire_ext: Option<types::Element>,

    /// The lifecycle status of this response: in-progress | completed | amended | entered-in-error | stopped
    pub status: crate::r5::coded::Coded<crate::r5::codes::QuestionnaireAnswersStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The [`Patient`](crate::r5::resources::patient::Patient) or other resource the questions are about
    pub subject: Option<types::Reference>,

    /// Encounter the questionnaire response is part of
    pub encounter: Option<types::Reference>,

    /// Date the answers were gathered
    pub authored: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored`](Self::authored) (FHIR `_authored`).
    #[serde(rename = "_authored")]
    pub authored_ext: Option<types::Element>,

    /// The individual or device that received and recorded the answers
    pub author: Option<types::Reference>,

    /// The individual or device that answered the questions
    pub source: Option<types::Reference>,

    /// The top-level tree of groups and questions, mirroring the structure of the source `Questionnaire`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireResponseItem>,
}

/// Groups and questions.
///
/// An item within a QuestionnaireResponse. An item may be a group that contains
/// child items, or a question that carries one or more answers. Each item points
/// back to a specific item in the source Questionnaire via its `linkId`.
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire_response::QuestionnaireResponseItem;
/// use fhir::r5::types;
///
/// let value = QuestionnaireResponseItem {
///     definition: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `definition` is the name this serializes to on the wire.
/// assert_eq!(json["definition"], ::serde_json::json!("http://example.org"));
///
/// let back: QuestionnaireResponseItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponseItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pointer to specific item from Questionnaire
    pub link_id: types::String,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// ElementDefinition - details for the item
    pub definition: Option<types::Uri>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Name for group or question text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// The response(s) to the question
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answer: Vec<QuestionnaireResponseItemAnswer>,

    /// Child items of group item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireResponseItem>,
}

/// The response(s) to the question.
///
/// A single answer to a question item, expressed as one of the supported value
/// types. An answer may itself contain nested child items, allowing complex,
/// grouped responses beneath a single question.
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire_response::QuestionnaireResponseItemAnswer;
/// use fhir::r5::types;
///
/// let value = QuestionnaireResponseItemAnswer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireResponseItemAnswer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponseItemAnswer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `QuestionnaireResponse.item.answer.value[x]` choice element (0..1); see [`QuestionnaireResponseItemAnswerValue`].
    #[serde(flatten)]
    pub value: Option<QuestionnaireResponseItemAnswerValue>,

    /// Child items of question
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireResponseItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = QuestionnaireResponse;

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
/// The `QuestionnaireResponse.item.answer.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireResponseItemAnswerValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}
