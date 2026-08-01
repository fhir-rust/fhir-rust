//! PlanDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PlanDefinition
//!
//! Version: 5.0.0
//!
//! PlanDefinition Resource: This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// PlanDefinition Resource.
///
/// This resource allows for the definition of various types of plans as a
/// sharable, consumable, and executable artifact. The resource is general
/// enough to support the description of a broad range of clinical and
/// non-clinical artifacts such as clinical decision support rules, order sets,
/// protocols, and drug quality specifications.
///
/// In FHIR R5 a PlanDefinition is a definitional resource: it describes the
/// intended structure of care or work rather than any specific instance of it
/// having occurred for a particular subject. Authors compose a plan from a
/// hierarchy of actions, each of which can carry conditions, timing,
/// participants, inputs and outputs, and relationships to other actions,
/// together with goals the plan is meant to achieve. Because the definition is
/// computable, decision support engines and workflow systems can apply a
/// PlanDefinition to a patient's context to generate concrete request
/// resources, such as a CarePlan, RequestOrchestration, or Task, and to drive
/// event-condition-action rules. This makes PlanDefinition a foundational
/// building block for knowledge artifacts that are published, versioned, and
/// shared across organizations.
///
/// Related resources: an ActivityDefinition supplies the reusable detail for
/// individual actions, a Library carries the logic and terminology the plan
/// relies on, and applying a plan commonly targets a subject such as a
/// [`Patient`](crate::r5::resources::patient::Patient). Many fields are typed
/// with shared datatypes such as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinition;
/// use fhir::r5::types;
///
/// let value = PlanDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinition {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this plan definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the plan definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the plan definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `PlanDefinition.versionAlgorithm[x]` choice element (0..1); see [`PlanDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<PlanDefinitionVersionAlgorithm>,

    /// Name for this plan definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this plan definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the plan definition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`).
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// High-level kind of artifact this plan represents, such as order-set, clinical-protocol, eca-rule, or workflow-definition.
    pub r#type: Option<types::CodeableConcept>,

    /// Publication lifecycle state of the plan definition: draft, active, retired, or unknown; this field is required.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// The `PlanDefinition.subject[x]` choice element (0..1); see [`PlanDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<PlanDefinitionSubject>,

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

    /// Natural language description of the plan definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for plan definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this plan definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the plan
    pub usage: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`).
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

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

    /// When the plan definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the plan definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the plan definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Additional documentation, citations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Logic used by the plan definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`).
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// The clinical or business goals the plan is intended to accomplish, against which its actions can be measured.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<PlanDefinitionGoal>,

    /// The actors, such as roles or participant types, that take part in carrying out the plan.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<PlanDefinitionActor>,

    /// The ordered, possibly nested actions that make up the plan and define what should be done and when.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<PlanDefinitionAction>,

    /// The `PlanDefinition.asNeeded[x]` choice element (0..1); see [`PlanDefinitionAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<PlanDefinitionAsNeeded>,
}

/// What the plan is trying to accomplish.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionGoal;
/// use fhir::r5::types;
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
pub struct PlanDefinitionGoal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// E.g. Treatment, dietary, behavioral
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

/// Target outcome for the goal.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionGoalTarget;
/// use fhir::r5::types;
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
pub struct PlanDefinitionGoalTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The parameter whose value is to be tracked
    pub measure: Option<types::CodeableConcept>,

    /// The `PlanDefinition.goal.target.detail[x]` choice element (0..1); see [`PlanDefinitionGoalTargetDetail`].
    #[serde(flatten)]
    pub detail: Option<PlanDefinitionGoalTargetDetail>,

    /// Reach goal within
    pub due: Option<types::Duration>,
}

/// Actors within the plan.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::plan_definition::PlanDefinitionActor;
///
/// let value = PlanDefinitionActor::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PlanDefinitionActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Describes the actor
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who or what can be this actor
    pub option: vec1::Vec1<PlanDefinitionActorOption>,
}

/// Who or what can be this actor.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActorOption;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActorOption {
///     type_canonical: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `typeCanonical` is the name this serializes to on the wire.
/// assert_eq!(json["typeCanonical"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: PlanDefinitionActorOption = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActorOption {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::ActionParticipantType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`).
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,
}

/// Action defined by the plan.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionAction;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionAction {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for the action in the PlanDefinition
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`).
    #[serde(rename = "_prefix")]
    pub prefix_ext: Option<types::Element>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Brief description of the action
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::Markdown>,
    /// Primitive extension sibling for [`text_equivalent`](Self::text_equivalent) (FHIR `_textEquivalent`).
    #[serde(rename = "_textEquivalent")]
    pub text_equivalent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Code representing the meaning of the action or sub-actions
    pub code: Option<types::CodeableConcept>,

    /// Why the action should be performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Supporting documentation for the intended performer of the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documentation: Vec<types::RelatedArtifact>,

    /// What goals this action supports
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal_id: Vec<types::Id>,
    /// Primitive extension sibling for [`goal_id`](Self::goal_id) (FHIR `_goalId`).
    #[serde(rename = "_goalId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal_id_ext: Vec<Option<types::Element>>,

    /// The `PlanDefinition.action.subject[x]` choice element (0..1); see [`PlanDefinitionActionSubject`].
    #[serde(flatten)]
    pub subject: Option<PlanDefinitionActionSubject>,

    /// When the action should be triggered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trigger: Vec<types::TriggerDefinition>,

    /// Whether or not the action is applicable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<PlanDefinitionActionCondition>,

    /// Input data requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<PlanDefinitionActionInput>,

    /// Output data definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<PlanDefinitionActionOutput>,

    /// Relationship to another action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_action: Vec<PlanDefinitionActionRelatedAction>,

    /// The `PlanDefinition.action.timing[x]` choice element (0..1); see [`PlanDefinitionActionTiming`].
    #[serde(flatten)]
    pub timing: Option<PlanDefinitionActionTiming>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

    /// Who should participate in the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<PlanDefinitionActionParticipant>,

    /// create | update | remove | fire-event
    pub r#type: Option<types::CodeableConcept>,

    /// visual-group | logical-group | sentence-group
    pub grouping_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionGroupingBehavior>>,
    /// Primitive extension sibling for [`grouping_behavior`](Self::grouping_behavior) (FHIR `_groupingBehavior`).
    #[serde(rename = "_groupingBehavior")]
    pub grouping_behavior_ext: Option<types::Element>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionSelectionBehavior>>,
    /// Primitive extension sibling for [`selection_behavior`](Self::selection_behavior) (FHIR `_selectionBehavior`).
    #[serde(rename = "_selectionBehavior")]
    pub selection_behavior_ext: Option<types::Element>,

    /// must | could | must-unless-documented
    pub required_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionRequiredBehavior>>,
    /// Primitive extension sibling for [`required_behavior`](Self::required_behavior) (FHIR `_requiredBehavior`).
    #[serde(rename = "_requiredBehavior")]
    pub required_behavior_ext: Option<types::Element>,

    /// yes | no
    pub precheck_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionPrecheckBehavior>>,
    /// Primitive extension sibling for [`precheck_behavior`](Self::precheck_behavior) (FHIR `_precheckBehavior`).
    #[serde(rename = "_precheckBehavior")]
    pub precheck_behavior_ext: Option<types::Element>,

    /// single | multiple
    pub cardinality_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionCardinalityBehavior>>,
    /// Primitive extension sibling for [`cardinality_behavior`](Self::cardinality_behavior) (FHIR `_cardinalityBehavior`).
    #[serde(rename = "_cardinalityBehavior")]
    pub cardinality_behavior_ext: Option<types::Element>,

    /// The `PlanDefinition.action.definition[x]` choice element (0..1); see [`PlanDefinitionActionDefinition`].
    #[serde(flatten)]
    pub definition: Option<PlanDefinitionActionDefinition>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`).
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

    /// Dynamic aspects of the definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic_value: Vec<PlanDefinitionActionDynamicValue>,

    /// A sub-action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<PlanDefinitionAction>,
}

/// Whether or not the action is applicable.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionCondition;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionCondition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// applicability | start | stop
    pub kind: crate::r5::coded::Coded<crate::r5::codes::ActionConditionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Boolean-valued expression
    pub expression: Option<types::Expression>,
}

/// Input data requirements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionInput;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionInput {
///     related_data: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `relatedData` is the name this serializes to on the wire.
/// assert_eq!(json["relatedData"], ::serde_json::json!("pat-1"));
///
/// let back: PlanDefinitionActionInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::Id>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`).
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// Output data definition.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionOutput;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionOutput {
///     related_data: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `relatedData` is the name this serializes to on the wire.
/// assert_eq!(json["relatedData"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionOutput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::String>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`).
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// Relationship to another action.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionRelatedAction;
/// use fhir::r5::types;
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
pub struct PlanDefinitionActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What action is this related to
    pub target_id: types::Id,
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`).
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub relationship: crate::r5::coded::Coded<crate::r5::codes::ActionRelationshipType>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`).
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub end_relationship: Option<crate::r5::coded::Coded<crate::r5::codes::ActionRelationshipType>>,
    /// Primitive extension sibling for [`end_relationship`](Self::end_relationship) (FHIR `_endRelationship`).
    #[serde(rename = "_endRelationship")]
    pub end_relationship_ext: Option<types::Element>,

    /// The `PlanDefinition.action.relatedAction.offset[x]` choice element (0..1); see [`PlanDefinitionActionRelatedActionOffset`].
    #[serde(flatten)]
    pub offset: Option<PlanDefinitionActionRelatedActionOffset>,
}

/// Who should participate in the action.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionParticipant;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionParticipant {
///     actor_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `actorId` is the name this serializes to on the wire.
/// assert_eq!(json["actorId"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What actor
    pub actor_id: Option<types::String>,
    /// Primitive extension sibling for [`actor_id`](Self::actor_id) (FHIR `_actorId`).
    #[serde(rename = "_actorId")]
    pub actor_id_ext: Option<types::Element>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::ActionParticipantType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`).
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,
}

/// Dynamic aspects of the definition.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionDynamicValue;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionDynamicValue {
///     path: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `path` is the name this serializes to on the wire.
/// assert_eq!(json["path"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionDynamicValue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionDynamicValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The path to the element to be set dynamically
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// An expression that provides the dynamic value for the customization
    pub expression: Option<types::Expression>,
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
/// The `PlanDefinition.action.definition[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionDefinition {
    /// `definitionCanonical` variant.
    #[fhir("definitionCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
    /// `definitionUri` variant.
    #[fhir("definitionUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
}

/// The `PlanDefinition.action.relatedAction.offset[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionRelatedActionOffset {
    /// `offsetDuration` variant.
    #[fhir("offsetDuration")]
    Duration(Box<types::Duration>),
    /// `offsetRange` variant.
    #[fhir("offsetRange")]
    Range(Box<types::Range>),
}

/// The `PlanDefinition.action.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
    /// `subjectCanonical` variant.
    #[fhir("subjectCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `PlanDefinition.action.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionTiming {
    /// `timingAge` variant.
    #[fhir("timingAge")]
    Age(Box<types::Age>),
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

/// The `PlanDefinition.asNeeded[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `PlanDefinition.goal.target.detail[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    /// `detailString` variant.
    #[fhir("detailString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `detailBoolean` variant.
    #[fhir("detailBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `detailInteger` variant.
    #[fhir("detailInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `detailRatio` variant.
    #[fhir("detailRatio")]
    Ratio(Box<types::Ratio>),
}

/// The `PlanDefinition.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
    /// `subjectCanonical` variant.
    #[fhir("subjectCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `PlanDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
