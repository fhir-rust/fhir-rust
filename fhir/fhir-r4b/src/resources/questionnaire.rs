//! Questionnaire
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
//!
//! Version: 4.3.0
//!
//! A structured set of questions
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A structured set of questions intended to guide the collection of answers
/// from end-users. Questionnaires provide detailed control over order,
/// presentation, phraseology and grouping to allow coherent, consistent data
/// collection.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire::Questionnaire;
/// use fhir::r4b::types;
///
/// let value = Questionnaire {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: Questionnaire = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct Questionnaire {
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

    /// Canonical identifier for this questionnaire, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the questionnaire
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this questionnaire (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this questionnaire (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub derived_from: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_ext: Vec<Option<types::Element>>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Resource that can be subject of QuestionnaireResponse
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub subject_type: ::fhir_core::PrimVec<crate::coded::Coded<crate::r4b::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`subject_type`](Self::subject_type) (FHIR `_subjectType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subjectType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_type_ext: Vec<Option<types::Element>>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the questionnaire
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for questionnaire (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this questionnaire is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// When the questionnaire was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the questionnaire was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the questionnaire is expected to be used
    pub effective_period: Option<types::Period>,

    /// Concept that represents the overall questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// Questions and sections within the Questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireItem>,
}

/// A particular question, question grouping or display text that is part of
/// the questionnaire.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire::QuestionnaireItem;
/// use fhir::r4b::types;
///
/// let value = QuestionnaireItem {
///     read_only: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `readOnly` is the name this serializes to on the wire.
/// assert_eq!(json["readOnly"], ::serde_json::json!(true));
///
/// let back: QuestionnaireItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct QuestionnaireItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for item in questionnaire
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

    /// Corresponding concept for this item in a terminology
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// E.g. "1(a)", "2.5.3"
    pub prefix: Option<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prefix")]
    pub prefix_ext: Option<types::Element>,

    /// Primary text for the item
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// group | display | boolean | decimal | integer | date | dateTime +
    pub r#type: crate::coded::Coded<crate::r4b::codes::ItemType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Only allow data when
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enable_when: Vec<QuestionnaireItemEnableWhen>,

    /// all | any
    pub enable_behavior:
        Option<crate::coded::Coded<crate::r4b::codes::QuestionnaireEnableBehavior>>,
    /// Primitive extension sibling for [`enable_behavior`](Self::enable_behavior) (FHIR `_enableBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_enableBehavior")]
    pub enable_behavior_ext: Option<types::Element>,

    /// Whether the item must be included in data results
    pub required: Option<types::Boolean>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// Whether the item may repeat
    pub repeats: Option<types::Boolean>,
    /// Primitive extension sibling for [`repeats`](Self::repeats) (FHIR `_repeats`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_repeats")]
    pub repeats_ext: Option<types::Element>,

    /// Don't allow human editing
    pub read_only: Option<types::Boolean>,
    /// Primitive extension sibling for [`read_only`](Self::read_only) (FHIR `_readOnly`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_readOnly")]
    pub read_only_ext: Option<types::Element>,

    /// No more than this many characters
    pub max_length: Option<types::Integer>,
    /// Primitive extension sibling for [`max_length`](Self::max_length) (FHIR `_maxLength`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_maxLength")]
    pub max_length_ext: Option<types::Element>,

    /// Valueset containing permitted answers
    pub answer_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`answer_value_set`](Self::answer_value_set) (FHIR `_answerValueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_answerValueSet")]
    pub answer_value_set_ext: Option<types::Element>,

    /// Permitted answer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answer_option: Vec<QuestionnaireItemAnswerOption>,

    /// Initial value(s) when item is first rendered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub initial: Vec<QuestionnaireItemInitial>,

    /// Nested questionnaire items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireItem>,
}

/// One of the permitted answers for a "choice" or "open-choice" question.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire::QuestionnaireItemAnswerOption;
/// use fhir::r4b::types;
///
/// let value = QuestionnaireItemAnswerOption {
///     initial_selected: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `initialSelected` is the name this serializes to on the wire.
/// assert_eq!(json["initialSelected"], ::serde_json::json!(true));
///
/// let back: QuestionnaireItemAnswerOption = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "QuestionnaireItemAnswerOptionDe")]
#[fhir_version("r4b")]
pub struct QuestionnaireItemAnswerOption {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Answer value
    /// The `Questionnaire.item.answerOption.value[x]` choice element (1..1); see [`QuestionnaireItemAnswerOptionValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<QuestionnaireItemAnswerOptionValue>,

    /// Whether option is selected by default
    pub initial_selected: Option<types::Boolean>,
    /// Primitive extension sibling for [`initial_selected`](Self::initial_selected) (FHIR `_initialSelected`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_initialSelected")]
    pub initial_selected_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireItemAnswerOptionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r4b::choice::Slot<QuestionnaireItemAnswerOptionValue>,
    initial_selected: Option<types::Boolean>,
    #[serde(rename = "_initialSelected")]
    initial_selected_ext: Option<types::Element>,
}

impl ::core::convert::From<QuestionnaireItemAnswerOptionDe> for QuestionnaireItemAnswerOption {
    fn from(v: QuestionnaireItemAnswerOptionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
            initial_selected: v.initial_selected,
            initial_selected_ext: v.initial_selected_ext,
        }
    }
}

/// A constraint indicating that this item should only be enabled
/// (displayed/allow answers to be captured) when the specified condition is
/// true.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire::QuestionnaireItemEnableWhen;
/// use fhir::r4b::types;
///
/// let value = QuestionnaireItemEnableWhen {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireItemEnableWhen = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "QuestionnaireItemEnableWhenDe")]
#[fhir_version("r4b")]
pub struct QuestionnaireItemEnableWhen {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Question that determines whether item is enabled
    pub question: types::String,
    /// Primitive extension sibling for [`question`](Self::question) (FHIR `_question`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_question")]
    pub question_ext: Option<types::Element>,

    /// exists | = | != | > | < | >= | <=
    pub operator: crate::coded::Coded<crate::r4b::codes::QuestionnaireEnableOperator>,
    /// Primitive extension sibling for [`operator`](Self::operator) (FHIR `_operator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_operator")]
    pub operator_ext: Option<types::Element>,

    /// Value for question comparison based on operator
    /// The `Questionnaire.item.enableWhen.answer[x]` choice element (1..1); see [`QuestionnaireItemEnableWhenAnswer`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub answer: Option<QuestionnaireItemEnableWhenAnswer>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireItemEnableWhenDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    question: types::String,
    #[serde(rename = "_question")]
    question_ext: Option<types::Element>,
    operator: crate::coded::Coded<crate::r4b::codes::QuestionnaireEnableOperator>,
    #[serde(rename = "_operator")]
    operator_ext: Option<types::Element>,
    #[serde(flatten)]
    answer: crate::r4b::choice::Slot<QuestionnaireItemEnableWhenAnswer>,
}

impl ::core::convert::From<QuestionnaireItemEnableWhenDe> for QuestionnaireItemEnableWhen {
    fn from(v: QuestionnaireItemEnableWhenDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            question: v.question,
            question_ext: v.question_ext,
            operator: v.operator,
            operator_ext: v.operator_ext,
            answer: v.answer.0,
        }
    }
}

/// One or more values that should be pre-populated in the answer when
/// initially rendering the questionnaire for user input.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::questionnaire::QuestionnaireItemInitial;
/// use fhir::r4b::types;
///
/// let value = QuestionnaireItemInitial {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireItemInitial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "QuestionnaireItemInitialDe")]
#[fhir_version("r4b")]
pub struct QuestionnaireItemInitial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Actual value for initializing the question
    /// The `Questionnaire.item.initial.value[x]` choice element (1..1); see [`QuestionnaireItemInitialValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<QuestionnaireItemInitialValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireItemInitialDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r4b::choice::Slot<QuestionnaireItemInitialValue>,
}

impl ::core::convert::From<QuestionnaireItemInitialDe> for QuestionnaireItemInitial {
    fn from(v: QuestionnaireItemInitialDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
        }
    }
}

/// The `Questionnaire.item.answerOption.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemAnswerOptionValue {
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r4b::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r4b::choice::Primitive<types::Date>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r4b::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r4b::choice::Primitive<types::String>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}

/// The `Questionnaire.item.enableWhen.answer[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemEnableWhenAnswer {
    /// `answerBoolean` variant.
    #[fhir("answerBoolean")]
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `answerDecimal` variant.
    #[fhir("answerDecimal")]
    Decimal(crate::r4b::choice::Primitive<types::Decimal>),
    /// `answerInteger` variant.
    #[fhir("answerInteger")]
    Integer(crate::r4b::choice::Primitive<types::Integer>),
    /// `answerDate` variant.
    #[fhir("answerDate")]
    Date(crate::r4b::choice::Primitive<types::Date>),
    /// `answerDateTime` variant.
    #[fhir("answerDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `answerTime` variant.
    #[fhir("answerTime")]
    Time(crate::r4b::choice::Primitive<types::Time>),
    /// `answerString` variant.
    #[fhir("answerString")]
    String(crate::r4b::choice::Primitive<types::String>),
    /// `answerCoding` variant.
    #[fhir("answerCoding")]
    Coding(Box<types::Coding>),
    /// `answerQuantity` variant.
    #[fhir("answerQuantity")]
    Quantity(Box<types::Quantity>),
    /// `answerReference` variant.
    #[fhir("answerReference")]
    Reference(Box<types::Reference>),
}

/// The `Questionnaire.item.initial.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemInitialValue {
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
