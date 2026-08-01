//! PlanDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PlanDefinition
//!
//!
//!
//! The definition of a plan for a series of actions, independent of any
//! specific patient or context
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for PlanDefinition Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinition;
/// use fhir::r3::types;
///
/// let value = PlanDefinition {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: PlanDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinition {
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

    /// Logical URI to reference this plan definition (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the plan definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the plan definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this plan definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this plan definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// order-set | protocol | eca-rule
    pub r#type: Option<types::CodeableConcept>,

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

    /// Natural language description of the plan definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Why this plan definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the asset
    pub usage: Option<types::String>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

    /// When the plan definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the plan definition was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the plan definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for plan definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// E.g. Education, Treatment, Assessment, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// A content contributor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<types::Contributor>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Related artifacts for the asset
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Logic used by the plan definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Reference>,

    /// What the plan is trying to accomplish
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<PlanDefinitionGoal>,

    /// Action defined by the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<PlanDefinitionAction>,
}

/// An action to be taken as part of the plan.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinitionAction;
/// use fhir::r3::types;
///
/// let value = PlanDefinitionAction {
///     text_equivalent: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `textEquivalent` is the name this serializes to on the wire.
/// assert_eq!(json["textEquivalent"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinitionAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User-visible label for the action (e.g. 1. or A.)
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Short description of the action
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Static text equivalent of the action, used if the dynamic aspects
    /// cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::String>,
    /// Primitive extension sibling for [`text_equivalent`](Self::text_equivalent) (FHIR `_textEquivalent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_textEquivalent")]
    pub text_equivalent_ext: Option<types::Element>,

    /// Code representing the meaning of the action or sub-actions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Why the action should be performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Supporting documentation for the intended performer of the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documentation: Vec<types::RelatedArtifact>,

    /// What goals this action supports
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal_id: Vec<types::Id>,
    /// Primitive extension sibling for [`goal_id`](Self::goal_id) (FHIR `_goalId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_goalId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal_id_ext: Vec<Option<types::Element>>,

    /// When the action should be triggered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trigger_definition: Vec<types::TriggerDefinition>,

    /// Whether or not the action is applicable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<PlanDefinitionActionCondition>,

    /// Input data requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<types::DataRequirement>,

    /// Output data definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<types::DataRequirement>,

    /// Relationship to another action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_action: Vec<PlanDefinitionActionRelatedAction>,

    /// When the action should take place
    /// The `PlanDefinition.action.timing[x]` choice element (0..1); see [`PlanDefinitionActionTiming`].
    #[serde(flatten)]
    pub timing: Option<PlanDefinitionActionTiming>,

    /// Who should participate in the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<PlanDefinitionActionParticipant>,

    /// create | update | remove | fire-event
    pub r#type: Option<types::Coding>,

    /// visual-group | logical-group | sentence-group
    pub grouping_behavior: Option<crate::coded::Coded<crate::r3::codes::ActionGroupingBehavior>>,
    /// Primitive extension sibling for [`grouping_behavior`](Self::grouping_behavior) (FHIR `_groupingBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupingBehavior")]
    pub grouping_behavior_ext: Option<types::Element>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior: Option<crate::coded::Coded<crate::r3::codes::ActionSelectionBehavior>>,
    /// Primitive extension sibling for [`selection_behavior`](Self::selection_behavior) (FHIR `_selectionBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_selectionBehavior")]
    pub selection_behavior_ext: Option<types::Element>,

    /// must | could | must-unless-documented
    pub required_behavior: Option<crate::coded::Coded<crate::r3::codes::ActionRequiredBehavior>>,
    /// Primitive extension sibling for [`required_behavior`](Self::required_behavior) (FHIR `_requiredBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requiredBehavior")]
    pub required_behavior_ext: Option<types::Element>,

    /// yes | no
    pub precheck_behavior: Option<crate::coded::Coded<crate::r3::codes::ActionPrecheckBehavior>>,
    /// Primitive extension sibling for [`precheck_behavior`](Self::precheck_behavior) (FHIR `_precheckBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precheckBehavior")]
    pub precheck_behavior_ext: Option<types::Element>,

    /// single | multiple
    pub cardinality_behavior:
        Option<crate::coded::Coded<crate::r3::codes::ActionCardinalityBehavior>>,
    /// Primitive extension sibling for [`cardinality_behavior`](Self::cardinality_behavior) (FHIR `_cardinalityBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cardinalityBehavior")]
    pub cardinality_behavior_ext: Option<types::Element>,

    /// Description of the activity to be performed
    pub definition: Option<types::Reference>,

    /// Transform to apply the template
    pub transform: Option<types::Reference>,

    /// Dynamic aspects of the definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic_value: Vec<PlanDefinitionActionDynamicValue>,

    /// A sub-action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<PlanDefinitionAction>,
}

/// An expression that describes applicability criteria, or start/stop
/// conditions for the action.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinitionActionCondition;
/// use fhir::r3::types;
///
/// let value = PlanDefinitionActionCondition {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinitionActionCondition {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// applicability | start | stop
    pub kind: crate::coded::Coded<crate::r3::codes::ActionConditionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Natural language description of the condition
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Language of the expression
    pub language: Option<types::String>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Boolean-valued expression
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,
}

/// Customizations that should be applied to the statically defined resource.
/// For example, if the dosage of a medication must be computed based on the
/// patient's weight, a customization would be used to specify an expression
/// that calculated the weight, and the path on the resource that would contain
/// the result.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinitionActionDynamicValue;
/// use fhir::r3::types;
///
/// let value = PlanDefinitionActionDynamicValue {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionDynamicValue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinitionActionDynamicValue {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Natural language description of the dynamic value
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The path to the element to be set dynamically
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Language of the expression
    pub language: Option<types::String>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// An expression that provides the dynamic value for the customization
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,
}

/// Indicates who should participate in performing the action described.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinitionActionParticipant;
/// use fhir::r3::types;
///
/// let value = PlanDefinitionActionParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinitionActionParticipant {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// patient | practitioner | related-person
    pub r#type: crate::coded::Coded<crate::r3::codes::ActionParticipantType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// E.g. Nurse, Surgeon, Parent, etc
    pub role: Option<types::CodeableConcept>,
}

/// A relationship to another action such as "before" or "30-60 minutes after
/// start of".
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinitionActionRelatedAction;
/// use fhir::r3::types;
///
/// let value = PlanDefinitionActionRelatedAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionRelatedAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinitionActionRelatedAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What action is this related to
    pub action_id: types::Id,
    /// Primitive extension sibling for [`action_id`](Self::action_id) (FHIR `_actionId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actionId")]
    pub action_id_ext: Option<types::Element>,

    /// before-start | before | before-end | concurrent-with-start | concurrent
    /// | concurrent-with-end | after-start | after | after-end
    pub relationship: crate::coded::Coded<crate::r3::codes::ActionRelationshipType>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// Time offset for the relationship
    /// The `PlanDefinition.action.relatedAction.offset[x]` choice element (0..1); see [`PlanDefinitionActionRelatedActionOffset`].
    #[serde(flatten)]
    pub offset: Option<PlanDefinitionActionRelatedActionOffset>,
}

/// Goals that describe what the activities within the plan are intended to
/// achieve. For example, weight loss, restoring an activity of daily living,
/// obtaining herd immunity via immunization, meeting a process improvement
/// objective, etc.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinitionGoal;
/// use fhir::r3::types;
///
/// let value = PlanDefinitionGoal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionGoal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinitionGoal {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// E.g. Treatment, dietary, behavioral, etc
    pub category: Option<types::CodeableConcept>,

    /// Code or text describing the goal
    pub description: types::CodeableConcept,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<types::CodeableConcept>,

    /// When goal pursuit begins
    pub start: Option<types::CodeableConcept>,

    /// What does the goal address
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::CodeableConcept>,

    /// Supporting documentation for the goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documentation: Vec<types::RelatedArtifact>,

    /// Target outcome for the goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<PlanDefinitionGoalTarget>,
}

/// Indicates what should be done and within what timeframe.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::plan_definition::PlanDefinitionGoalTarget;
/// use fhir::r3::types;
///
/// let value = PlanDefinitionGoalTarget {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionGoalTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PlanDefinitionGoalTarget {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The parameter whose value is to be tracked
    pub measure: Option<types::CodeableConcept>,

    /// The target value to be achieved
    /// The `PlanDefinition.goal.target.detail[x]` choice element (0..1); see [`PlanDefinitionGoalTargetDetail`].
    #[serde(flatten)]
    pub detail: Option<PlanDefinitionGoalTargetDetail>,

    /// Reach goal within
    pub due: Option<types::Duration>,
}

/// The `PlanDefinition.action.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionTiming {
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
    /// `timingDuration` variant.
    #[fhir("timingDuration")]
    Duration(Box<types::Duration>),
    /// `timingRange` variant.
    #[fhir("timingRange")]
    Range(Box<types::Range>),
    /// `timingTiming` variant.
    #[fhir("timingTiming")]
    Timing(Box<types::Timing>),
}

/// The `PlanDefinition.action.relatedAction.offset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionRelatedActionOffset {
    /// `offsetDuration` variant.
    #[fhir("offsetDuration")]
    Duration(Box<types::Duration>),
    /// `offsetRange` variant.
    #[fhir("offsetRange")]
    Range(Box<types::Range>),
}

/// The `PlanDefinition.goal.target.detail[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionGoalTargetDetail {
    /// `detailQuantity` variant.
    #[fhir("detailQuantity")]
    Quantity(Box<types::Quantity>),
    /// `detailRange` variant.
    #[fhir("detailRange")]
    Range(Box<types::Range>),
    /// `detailCodeableConcept` variant.
    #[fhir("detailCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PlanDefinition;

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
