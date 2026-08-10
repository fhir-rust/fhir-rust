//! QuestionnaireResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/QuestionnaireResponse
//!
//!
//!
//! A structured set of questions and their answers
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for QuestionnaireResponse Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::questionnaire_response::QuestionnaireResponse;
/// use fhir::r2::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct QuestionnaireResponse {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for this set of answers
    pub identifier: Option<types::Identifier>,

    /// Form being answered
    pub questionnaire: Option<types::Reference<crate::r2::resources::Questionnaire>>,

    /// in-progress | completed | amended
    pub status: crate::coded::Coded<crate::r2::codes::QuestionnaireAnswersStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The subject of the questions
    pub subject: Option<types::Reference>,

    /// Person who received and recorded the answers
    pub author: Option<types::Reference>,

    /// Date this version was authored
    pub authored: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored`](Self::authored) (FHIR `_authored`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authored")]
    pub authored_ext: Option<types::Element>,

    /// The person who answered the questions
    pub source: Option<types::Reference>,

    /// Primary encounter during which the answers were collected
    pub encounter: Option<types::Reference<crate::r2::resources::Encounter>>,

    /// Grouped questions
    pub group: Option<QuestionnaireResponseGroup>,
}

/// A group of questions to a possibly similarly grouped set of questions in
/// the questionnaire response.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::questionnaire_response::QuestionnaireResponseGroup;
/// use fhir::r2::types;
///
/// let value = QuestionnaireResponseGroup {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireResponseGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct QuestionnaireResponseGroup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Corresponding group within Questionnaire
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Name for this group
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Additional text for the group
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// The subject this group's answers are about
    pub subject: Option<types::Reference>,

    /// Nested questionnaire response group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<QuestionnaireResponseGroup>,

    /// Questions in this group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub question: Vec<QuestionnaireResponseGroupQuestion>,
}

/// Set of questions within this group. The order of questions within the group
/// is relevant.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::questionnaire_response::QuestionnaireResponseGroupQuestion;
/// use fhir::r2::types;
///
/// let value = QuestionnaireResponseGroupQuestion {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireResponseGroupQuestion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct QuestionnaireResponseGroupQuestion {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Corresponding question within Questionnaire
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Text of the question as it is shown to the user
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// The response(s) to the question
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answer: Vec<QuestionnaireResponseGroupQuestionAnswer>,
}

/// The respondent's answer(s) to the question.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::questionnaire_response::QuestionnaireResponseGroupQuestionAnswer;
/// use fhir::r2::types;
///
/// let value = QuestionnaireResponseGroupQuestionAnswer {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: QuestionnaireResponseGroupQuestionAnswer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "QuestionnaireResponseGroupQuestionAnswerDe")]
#[fhir_version("r2")]
pub struct QuestionnaireResponseGroupQuestionAnswer {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Single-valued answer to the question
    /// The `QuestionnaireResponse.group.question.answer.value[x]` choice element (0..1); see [`QuestionnaireResponseGroupQuestionAnswerValue`].
    #[serde(flatten)]
    pub value: Option<QuestionnaireResponseGroupQuestionAnswerValue>,

    /// Nested questionnaire group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<QuestionnaireResponseGroup>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireResponseGroupQuestionAnswerDe {
    id: Option<types::Id>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r2::choice::Slot<QuestionnaireResponseGroupQuestionAnswerValue>,
    #[serde(default)]
    group: Vec<QuestionnaireResponseGroup>,
}

impl ::core::convert::From<QuestionnaireResponseGroupQuestionAnswerDe>
    for QuestionnaireResponseGroupQuestionAnswer
{
    fn from(v: QuestionnaireResponseGroupQuestionAnswerDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
            group: v.group,
        }
    }
}

/// The `QuestionnaireResponse.group.question.answer.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireResponseGroupQuestionAnswerValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
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
