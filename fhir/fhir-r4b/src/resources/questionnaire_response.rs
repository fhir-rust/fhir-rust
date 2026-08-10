//! QuestionnaireResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
//!
//! Version: 4.3.0
//!
//! A structured set of questions and their answers
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A structured set of questions and their answers. The questions are ordered
/// and grouped into coherent subsets, corresponding to the structure of the
/// grouping of the questionnaire being responded to.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire_response::QuestionnaireResponse;
/// use fhir::r4b::types;
///
/// let value = QuestionnaireResponse {
///     questionnaire: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `questionnaire` is the name this serializes to on the wire.
/// assert_eq!(json["questionnaire"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: QuestionnaireResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct QuestionnaireResponse {
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

    /// Unique id for this set of answers
    pub identifier: Option<types::Identifier>,

    /// Request fulfilled by this QuestionnaireResponse
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Form being answered
    pub questionnaire: Option<types::Canonical>,
    /// Primitive extension sibling for [`questionnaire`](Self::questionnaire) (FHIR `_questionnaire`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_questionnaire")]
    pub questionnaire_ext: Option<types::Element>,

    /// in-progress | completed | amended | entered-in-error | stopped
    pub status: crate::coded::Coded<crate::r4b::codes::QuestionnaireAnswersStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The subject of the questions
    pub subject: Option<types::Reference>,

    /// Encounter created as part of
    pub encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,

    /// Date the answers were gathered
    pub authored: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored`](Self::authored) (FHIR `_authored`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authored")]
    pub authored_ext: Option<types::Element>,

    /// Person who received and recorded the answers
    pub author: Option<types::Reference>,

    /// The person who answered the questions
    pub source: Option<types::Reference>,

    /// Groups and questions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireResponseItem>,
}

/// A group or question item from the original questionnaire for which answers
/// are provided.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire_response::QuestionnaireResponseItem;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// ElementDefinition - details for the item
    pub definition: Option<types::Uri>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Name for group or question text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// The response(s) to the question
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answer: Vec<QuestionnaireResponseItemAnswer>,

    /// Nested questionnaire response items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireResponseItem>,
}

/// The respondent's answer(s) to the question.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire_response::QuestionnaireResponseItemAnswer;
/// use fhir::r4b::types;
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
#[serde(from = "QuestionnaireResponseItemAnswerDe")]
#[fhir_version("r4b")]
pub struct QuestionnaireResponseItemAnswer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Single-valued answer to the question
    /// The `QuestionnaireResponse.item.answer.value[x]` choice element (0..1); see [`QuestionnaireResponseItemAnswerValue`].
    #[serde(flatten)]
    pub value: Option<QuestionnaireResponseItemAnswerValue>,

    /// Nested groups and questions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireResponseItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireResponseItemAnswerDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r4b::choice::Slot<QuestionnaireResponseItemAnswerValue>,
    #[serde(default)]
    item: Vec<QuestionnaireResponseItem>,
}

impl ::core::convert::From<QuestionnaireResponseItemAnswerDe> for QuestionnaireResponseItemAnswer {
    fn from(v: QuestionnaireResponseItemAnswerDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
            item: v.item,
        }
    }
}

/// The `QuestionnaireResponse.item.answer.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireResponseItemAnswerValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r4b::choice::Primitive<types::Decimal>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r4b::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r4b::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r4b::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r4b::choice::Primitive<types::String>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r4b::choice::Primitive<types::Uri>),
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
