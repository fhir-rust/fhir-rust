//! Questionnaire
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
//!
//!
//!
//! A structured set of questions
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Questionnaire Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::questionnaire::Questionnaire;
/// use fhir::r2::types;
///
/// let value = Questionnaire {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: Questionnaire = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Questionnaire {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External identifiers for this questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Logical identifier for this version of Questionnaire
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// draft | published | retired
    pub status: crate::coded::Coded<crate::r2::codes::QuestionnaireStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Date this version was authored
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Organization/individual who designed the questionnaire
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact information of the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Resource that can be subject of QuestionnaireResponse
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_type: Vec<crate::coded::Coded<crate::r2::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`subject_type`](Self::subject_type) (FHIR `_subjectType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subjectType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_type_ext: Vec<Option<types::Element>>,

    /// Grouped questions
    pub group: QuestionnaireGroup,
}

/// A collection of related questions (or further groupings of questions).
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::questionnaire::QuestionnaireGroup;
/// use fhir::r2::types;
///
/// let value = QuestionnaireGroup {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct QuestionnaireGroup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// To link questionnaire with questionnaire response
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Name to be displayed for group
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Concept that represents this section in a questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<types::Coding>,

    /// Additional text for the group
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Whether the group must be included in data results
    pub required: Option<types::Boolean>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// Whether the group may repeat
    pub repeats: Option<types::Boolean>,
    /// Primitive extension sibling for [`repeats`](Self::repeats) (FHIR `_repeats`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_repeats")]
    pub repeats_ext: Option<types::Element>,

    /// Nested questionnaire group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<QuestionnaireGroup>,

    /// Questions in this group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub question: Vec<QuestionnaireGroupQuestion>,
}

/// Set of questions within this group. The order of questions within the group
/// is relevant.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::questionnaire::QuestionnaireGroupQuestion;
/// use fhir::r2::types;
///
/// let value = QuestionnaireGroupQuestion {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireGroupQuestion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct QuestionnaireGroupQuestion {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// To link questionnaire with questionnaire response
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Concept that represents this question on a questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<types::Coding>,

    /// Text of the question as it is shown to the user
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// boolean | decimal | integer | date | dateTime +
    pub r#type: Option<crate::coded::Coded<crate::r2::codes::AnswerFormat>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Whether the question must be answered in data results
    pub required: Option<types::Boolean>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// Whether the question can have multiple answers
    pub repeats: Option<types::Boolean>,
    /// Primitive extension sibling for [`repeats`](Self::repeats) (FHIR `_repeats`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_repeats")]
    pub repeats_ext: Option<types::Element>,

    /// Valueset containing permitted answers
    pub options: Option<types::Reference>,

    /// Permitted answer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option: Vec<types::Coding>,

    /// Nested questionnaire group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<QuestionnaireGroup>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Questionnaire;

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
