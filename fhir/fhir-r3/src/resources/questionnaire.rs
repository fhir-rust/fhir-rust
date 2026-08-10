//! Questionnaire
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
//!
//!
//!
//! A structured set of questions
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Questionnaire Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::questionnaire::Questionnaire;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Logical URI to reference this questionnaire (globally unique)
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

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r3::codes::PublicationStatus>,
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

    /// Date this was last changed
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

    /// Natural language description of the questionnaire
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Why this questionnaire is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

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

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for questionnaire (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Concept that represents the overall questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// Resource that can be subject of QuestionnaireResponse
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_type: Vec<crate::coded::Coded<crate::r3::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`subject_type`](Self::subject_type) (FHIR `_subjectType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subjectType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_type_ext: Vec<Option<types::Element>>,

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
/// use fhir::r3::resources::questionnaire::QuestionnaireItem;
/// use fhir::r3::types;
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
#[serde(from = "QuestionnaireItemDe")]
#[fhir_version("r3")]
pub struct QuestionnaireItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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
    pub r#type: crate::coded::Coded<crate::r3::codes::ItemType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Only allow data when
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enable_when: Vec<QuestionnaireItemEnableWhen>,

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
    pub options: Option<types::Reference<crate::r3::resources::ValueSet>>,

    /// Permitted answer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub option: Vec<QuestionnaireItemOption>,

    /// Default value when item is first rendered
    /// The `Questionnaire.item.initial[x]` choice element (0..1); see [`QuestionnaireItemInitial`].
    #[serde(flatten)]
    pub initial: Option<QuestionnaireItemInitial>,

    /// Nested questionnaire items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    link_id: types::String,
    #[serde(rename = "_linkId")]
    link_id_ext: Option<types::Element>,
    definition: Option<types::Uri>,
    #[serde(rename = "_definition")]
    definition_ext: Option<types::Element>,
    #[serde(default)]
    code: Vec<types::Coding>,
    prefix: Option<types::String>,
    #[serde(rename = "_prefix")]
    prefix_ext: Option<types::Element>,
    text: Option<types::String>,
    #[serde(rename = "_text")]
    text_ext: Option<types::Element>,
    r#type: crate::coded::Coded<crate::r3::codes::ItemType>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    #[serde(default)]
    enable_when: Vec<QuestionnaireItemEnableWhen>,
    required: Option<types::Boolean>,
    #[serde(rename = "_required")]
    required_ext: Option<types::Element>,
    repeats: Option<types::Boolean>,
    #[serde(rename = "_repeats")]
    repeats_ext: Option<types::Element>,
    read_only: Option<types::Boolean>,
    #[serde(rename = "_readOnly")]
    read_only_ext: Option<types::Element>,
    max_length: Option<types::Integer>,
    #[serde(rename = "_maxLength")]
    max_length_ext: Option<types::Element>,
    options: Option<types::Reference<crate::r3::resources::ValueSet>>,
    #[serde(default)]
    option: Vec<QuestionnaireItemOption>,
    #[serde(flatten)]
    initial: crate::r3::choice::Slot<QuestionnaireItemInitial>,
    #[serde(default)]
    item: Vec<QuestionnaireItem>,
}

impl ::core::convert::From<QuestionnaireItemDe> for QuestionnaireItem {
    fn from(v: QuestionnaireItemDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            link_id: v.link_id,
            link_id_ext: v.link_id_ext,
            definition: v.definition,
            definition_ext: v.definition_ext,
            code: v.code,
            prefix: v.prefix,
            prefix_ext: v.prefix_ext,
            text: v.text,
            text_ext: v.text_ext,
            r#type: v.r#type,
            type_ext: v.type_ext,
            enable_when: v.enable_when,
            required: v.required,
            required_ext: v.required_ext,
            repeats: v.repeats,
            repeats_ext: v.repeats_ext,
            read_only: v.read_only,
            read_only_ext: v.read_only_ext,
            max_length: v.max_length,
            max_length_ext: v.max_length_ext,
            options: v.options,
            option: v.option,
            initial: v.initial.0,
            item: v.item,
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
/// use fhir::r3::resources::questionnaire::QuestionnaireItemEnableWhen;
/// use fhir::r3::types;
///
/// let value = QuestionnaireItemEnableWhen {
///     has_answer: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `hasAnswer` is the name this serializes to on the wire.
/// assert_eq!(json["hasAnswer"], ::serde_json::json!(true));
///
/// let back: QuestionnaireItemEnableWhen = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "QuestionnaireItemEnableWhenDe")]
#[fhir_version("r3")]
pub struct QuestionnaireItemEnableWhen {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Question that determines whether item is enabled
    pub question: types::String,
    /// Primitive extension sibling for [`question`](Self::question) (FHIR `_question`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_question")]
    pub question_ext: Option<types::Element>,

    /// Enable when answered or not
    pub has_answer: Option<types::Boolean>,
    /// Primitive extension sibling for [`has_answer`](Self::has_answer) (FHIR `_hasAnswer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hasAnswer")]
    pub has_answer_ext: Option<types::Element>,

    /// Value question must have
    /// The `Questionnaire.item.enableWhen.answer[x]` choice element (0..1); see [`QuestionnaireItemEnableWhenAnswer`].
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
    has_answer: Option<types::Boolean>,
    #[serde(rename = "_hasAnswer")]
    has_answer_ext: Option<types::Element>,
    #[serde(flatten)]
    answer: crate::r3::choice::Slot<QuestionnaireItemEnableWhenAnswer>,
}

impl ::core::convert::From<QuestionnaireItemEnableWhenDe> for QuestionnaireItemEnableWhen {
    fn from(v: QuestionnaireItemEnableWhenDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            question: v.question,
            question_ext: v.question_ext,
            has_answer: v.has_answer,
            has_answer_ext: v.has_answer_ext,
            answer: v.answer.0,
        }
    }
}

/// One of the permitted answers for a "choice" or "open-choice" question.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::questionnaire::QuestionnaireItemOption;
/// use fhir::r3::types;
///
/// let value = QuestionnaireItemOption {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: QuestionnaireItemOption = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "QuestionnaireItemOptionDe")]
#[fhir_version("r3")]
pub struct QuestionnaireItemOption {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Answer value
    /// The `Questionnaire.item.option.value[x]` choice element (1..1); see [`QuestionnaireItemOptionValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<QuestionnaireItemOptionValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireItemOptionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r3::choice::Slot<QuestionnaireItemOptionValue>,
}

impl ::core::convert::From<QuestionnaireItemOptionDe> for QuestionnaireItemOption {
    fn from(v: QuestionnaireItemOptionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
        }
    }
}

/// The `Questionnaire.item.initial[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemInitial {
    /// `initialBoolean` variant.
    #[fhir("initialBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `initialDecimal` variant.
    #[fhir("initialDecimal")]
    Decimal(crate::r3::choice::Primitive<types::Decimal>),
    /// `initialInteger` variant.
    #[fhir("initialInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
    /// `initialDate` variant.
    #[fhir("initialDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `initialDateTime` variant.
    #[fhir("initialDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `initialTime` variant.
    #[fhir("initialTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `initialString` variant.
    #[fhir("initialString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `initialUri` variant.
    #[fhir("initialUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `initialAttachment` variant.
    #[fhir("initialAttachment")]
    Attachment(Box<types::Attachment>),
    /// `initialCoding` variant.
    #[fhir("initialCoding")]
    Coding(Box<types::Coding>),
    /// `initialQuantity` variant.
    #[fhir("initialQuantity")]
    Quantity(Box<types::Quantity>),
    /// `initialReference` variant.
    #[fhir("initialReference")]
    Reference(Box<types::Reference>),
}

/// The `Questionnaire.item.enableWhen.answer[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemEnableWhenAnswer {
    /// `answerBoolean` variant.
    #[fhir("answerBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `answerDecimal` variant.
    #[fhir("answerDecimal")]
    Decimal(crate::r3::choice::Primitive<types::Decimal>),
    /// `answerInteger` variant.
    #[fhir("answerInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
    /// `answerDate` variant.
    #[fhir("answerDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `answerDateTime` variant.
    #[fhir("answerDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `answerTime` variant.
    #[fhir("answerTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `answerString` variant.
    #[fhir("answerString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `answerUri` variant.
    #[fhir("answerUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `answerAttachment` variant.
    #[fhir("answerAttachment")]
    Attachment(Box<types::Attachment>),
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

/// The `Questionnaire.item.option.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemOptionValue {
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
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
