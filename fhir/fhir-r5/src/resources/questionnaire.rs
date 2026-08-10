//! Questionnaire
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Questionnaire
//!
//! Version: 5.0.0
//!
//! Questionnaire Resource: A structured set of questions intended to guide the collection of answers from end-users.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A structured set of questions intended to guide the collection of answers
/// from end-users.
///
/// A Questionnaire is an organized collection of questions, grouped into
/// sections, that provides detailed control over the order, presentation,
/// phraseology, and grouping of those questions so that data can be gathered in
/// a coherent and consistent way. It serves both clinical and administrative
/// purposes, underpinning forms such as patient intake and registration,
/// pre-procedure checklists, patient-reported outcome measures, research case
/// report forms, surveys, and structured clinical assessments. The
/// Questionnaire resource defines only the definition and structure of a form,
/// including question types, allowed answers, validation rules, and conditional
/// display; the actual answers supplied for a particular subject are recorded
/// separately in a `QuestionnaireResponse`. In FHIR R5 the Questionnaire is a
/// canonical, versionable, and publishable metadata resource: it carries a
/// canonical `url`, a business `version`, and a publication `status`, and its
/// items can nest to any depth and be shown or hidden dynamically through
/// enableWhen logic. Answer choices may be enumerated inline or drawn from an
/// external value set for terminology binding.
///
/// Related resources: the captured answers live in a `QuestionnaireResponse`,
/// question items can reference terminology through
/// [`Coding`](crate::r5::types::Coding) and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and the intended
/// subject of a response is often a [`Patient`](crate::r5::resources::patient::Patient).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire::Questionnaire;
/// use fhir::r5::types;
///
/// let value = Questionnaire {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: Questionnaire = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "QuestionnaireDe")]
pub struct Questionnaire {
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

    /// Canonical identifier for this questionnaire, an absolute globally unique URI used to reference the form.
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier for questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the questionnaire
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `Questionnaire.versionAlgorithm[x]` choice element (0..1); see [`QuestionnaireVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<QuestionnaireVersionAlgorithm>,

    /// Name for this questionnaire (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this questionnaire (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Based on Questionnaire
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub derived_from: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`).
    #[serde(rename = "_derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_ext: Vec<Option<types::Element>>,

    /// Publication lifecycle state of this questionnaire: draft, active, retired, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Resource that can be subject of QuestionnaireResponse
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub subject_type: ::fhir_core::PrimVec<types::Code>,

    /// Primitive extension siblings for [`subject_type`](Self::subject_type)
    /// (FHIR `_subjectType`). A *repeating* primitive: the array aligns
    /// element-by-element with `subjectType`, using JSON `null` where a given
    /// entry has no extension, hence `Vec<Option<Element>>`.
    #[serde(rename = "_subjectType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_type_ext: Vec<Option<types::Element>>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the questionnaire
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the questionnaire was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the questionnaire was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the questionnaire is expected to be used
    pub effective_period: Option<types::Period>,

    /// Concept that represents the overall questionnaire
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// Top-level questions and sections that make up the form, each of which may nest further items.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<QuestionnaireItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct QuestionnaireDe {
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
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r5::choice::Slot<QuestionnaireVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    #[serde(default)]
    derived_from: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_derivedFrom")]
    #[serde(default)]
    derived_from_ext: Vec<Option<types::Element>>,
    status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    #[serde(default)]
    subject_type: ::fhir_core::PrimVec<types::Code>,
    #[serde(rename = "_subjectType")]
    #[serde(default)]
    subject_type_ext: Vec<Option<types::Element>>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    approval_date: Option<types::Date>,
    #[serde(rename = "_approvalDate")]
    approval_date_ext: Option<types::Element>,
    last_review_date: Option<types::Date>,
    #[serde(rename = "_lastReviewDate")]
    last_review_date_ext: Option<types::Element>,
    effective_period: Option<types::Period>,
    #[serde(default)]
    code: Vec<types::Coding>,
    #[serde(default)]
    item: Vec<QuestionnaireItem>,
}

impl ::core::convert::From<QuestionnaireDe> for Questionnaire {
    fn from(v: QuestionnaireDe) -> Self {
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
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            derived_from: v.derived_from,
            derived_from_ext: v.derived_from_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            subject_type: v.subject_type,
            subject_type_ext: v.subject_type_ext,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            approval_date: v.approval_date,
            approval_date_ext: v.approval_date_ext,
            last_review_date: v.last_review_date,
            last_review_date_ext: v.last_review_date_ext,
            effective_period: v.effective_period,
            code: v.code,
            item: v.item,
        }
    }
}

/// Questions and sections within the Questionnaire.
///
/// Each item is either a group (containing nested items), a display element, or
/// a question that collects an answer of a particular type.
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire::QuestionnaireItem;
/// use fhir::r5::types;
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
pub struct QuestionnaireItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier for this item within the questionnaire, used to link answers and enableWhen conditions.
    pub link_id: types::String,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// ElementDefinition - details for the item
    pub definition: Option<types::Uri>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Corresponding concept for this item in a terminology
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// E.g. "1(a)", "2.5.3"
    pub prefix: Option<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`).
    #[serde(rename = "_prefix")]
    pub prefix_ext: Option<types::Element>,

    /// Primary text for the item
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Kind of item, such as a grouping, display text, or a question of a specific answer data type.
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ItemType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Only allow data when
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enable_when: Vec<QuestionnaireItemEnableWhen>,

    /// all | any
    pub enable_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::QuestionnaireEnableBehavior>>,
    /// Primitive extension sibling for [`enable_behavior`](Self::enable_behavior) (FHIR `_enableBehavior`).
    #[serde(rename = "_enableBehavior")]
    pub enable_behavior_ext: Option<types::Element>,

    /// hidden | protected
    pub disabled_display:
        Option<crate::r5::coded::Coded<crate::r5::codes::QuestionnaireDisabledDisplay>>,
    /// Primitive extension sibling for [`disabled_display`](Self::disabled_display) (FHIR `_disabledDisplay`).
    #[serde(rename = "_disabledDisplay")]
    pub disabled_display_ext: Option<types::Element>,

    /// Whether the item must be included in data results
    pub required: Option<types::Boolean>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`).
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// Whether the item may repeat
    pub repeats: Option<types::Boolean>,
    /// Primitive extension sibling for [`repeats`](Self::repeats) (FHIR `_repeats`).
    #[serde(rename = "_repeats")]
    pub repeats_ext: Option<types::Element>,

    /// Don't allow human editing
    pub read_only: Option<types::Boolean>,
    /// Primitive extension sibling for [`read_only`](Self::read_only) (FHIR `_readOnly`).
    #[serde(rename = "_readOnly")]
    pub read_only_ext: Option<types::Element>,

    /// No more than these many characters
    pub max_length: Option<types::Integer>,
    /// Primitive extension sibling for [`max_length`](Self::max_length) (FHIR `_maxLength`).
    #[serde(rename = "_maxLength")]
    pub max_length_ext: Option<types::Element>,

    /// optionsOnly | optionsOrType | optionsOrString
    pub answer_constraint:
        Option<crate::r5::coded::Coded<crate::r5::codes::QuestionnaireAnswerConstraint>>,
    /// Primitive extension sibling for [`answer_constraint`](Self::answer_constraint) (FHIR `_answerConstraint`).
    #[serde(rename = "_answerConstraint")]
    pub answer_constraint_ext: Option<types::Element>,

    /// ValueSet containing permitted answers
    pub answer_value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`answer_value_set`](Self::answer_value_set) (FHIR `_answerValueSet`).
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

/// Only allow data when a specified condition is met.
///
/// Defines a question/answer pair that must hold for the enclosing item to be
/// enabled, combined via the item's enableBehavior when multiple are present.
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire::QuestionnaireItemEnableWhen;
/// use fhir::r5::types;
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
pub struct QuestionnaireItemEnableWhen {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The linkId of question that determines whether item is enabled/disabled
    pub question: types::String,
    /// Primitive extension sibling for [`question`](Self::question) (FHIR `_question`).
    #[serde(rename = "_question")]
    pub question_ext: Option<types::Element>,

    /// exists | = | != | > | < | >= | <=
    pub operator: crate::r5::coded::Coded<crate::r5::codes::QuestionnaireEnableOperator>,
    /// Primitive extension sibling for [`operator`](Self::operator) (FHIR `_operator`).
    #[serde(rename = "_operator")]
    pub operator_ext: Option<types::Element>,

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
    operator: crate::r5::coded::Coded<crate::r5::codes::QuestionnaireEnableOperator>,
    #[serde(rename = "_operator")]
    operator_ext: Option<types::Element>,
    #[serde(flatten)]
    answer: crate::r5::choice::Slot<QuestionnaireItemEnableWhenAnswer>,
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

/// Permitted answer.
///
/// One of the discrete answer options offered for a question item, optionally
/// marked as selected by default.
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire::QuestionnaireItemAnswerOption;
/// use fhir::r5::types;
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
pub struct QuestionnaireItemAnswerOption {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `Questionnaire.item.answerOption.value[x]` choice element (0..1); see [`QuestionnaireItemAnswerOptionValue`].
    #[serde(flatten)]
    pub value: Option<QuestionnaireItemAnswerOptionValue>,

    /// Whether option is selected by default
    pub initial_selected: Option<types::Boolean>,
    /// Primitive extension sibling for [`initial_selected`](Self::initial_selected) (FHIR `_initialSelected`).
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
    value: crate::r5::choice::Slot<QuestionnaireItemAnswerOptionValue>,
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

/// Initial value(s) when item is first rendered.
///
/// Supplies the default value for a question item using the appropriate typed
/// value for the item's answer type.
/// # Examples
///
/// ```
/// use fhir::r5::resources::questionnaire::QuestionnaireItemInitial;
/// use fhir::r5::types;
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
pub struct QuestionnaireItemInitial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `Questionnaire.item.initial.value[x]` choice element (0..1); see [`QuestionnaireItemInitialValue`].
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
    value: crate::r5::choice::Slot<QuestionnaireItemInitialValue>,
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
/// The `Questionnaire.item.answerOption.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemAnswerOptionValue {
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}

/// The `Questionnaire.item.enableWhen.answer[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemEnableWhenAnswer {
    /// `answerBoolean` variant.
    #[fhir("answerBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `answerDecimal` variant.
    #[fhir("answerDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `answerInteger` variant.
    #[fhir("answerInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `answerDate` variant.
    #[fhir("answerDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `answerDateTime` variant.
    #[fhir("answerDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `answerTime` variant.
    #[fhir("answerTime")]
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `answerString` variant.
    #[fhir("answerString")]
    String(crate::r5::choice::Primitive<types::String>),
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

/// The `Questionnaire.item.initial.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireItemInitialValue {
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

/// The `Questionnaire.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum QuestionnaireVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
