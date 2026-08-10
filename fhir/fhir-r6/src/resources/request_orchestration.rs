//! RequestOrchestration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RequestOrchestration
//!
//! Version: 6.0.0-ballot3
//!
//! A set of related requests
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A set of related requests that can be used to capture intended activities
/// that have inter-dependencies such as "give this medication after that one".
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::request_orchestration::RequestOrchestration;
/// use fhir::r6::types;
///
/// let value = RequestOrchestration {
///     authored_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authoredOn` is the name this serializes to on the wire.
/// assert_eq!(json["authoredOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: RequestOrchestration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RequestOrchestration {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates_canonical: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates_uri: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// Fulfills plan, proposal, or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Request(s) replaced by this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | on-hold | entered-in-error | ended | completed |
    /// revoked | unknown
    pub status: crate::coded::Coded<crate::r6::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r6::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// What's being requested/ordered
    pub code: Option<types::CodeableConcept>,

    /// Who the request orchestration is about
    pub subject: Option<types::Reference>,

    /// Created as part of
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// When the request orchestration was authored
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Device or practitioner that authored the request orchestration
    pub author: Option<types::Reference>,

    /// Why the request orchestration is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// What goals
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference<crate::r6::resources::Goal>>,

    /// Additional notes about the response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Proposed actions, if any
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<RequestOrchestrationAction>,
}

/// The actions, if any, produced by the evaluation of the artifact.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::request_orchestration::RequestOrchestrationAction;
/// use fhir::r6::types;
///
/// let value = RequestOrchestrationAction {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: RequestOrchestrationAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "RequestOrchestrationActionDe")]
#[fhir_version("r6")]
pub struct RequestOrchestrationAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pointer to specific item from the PlanDefinition
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prefix")]
    pub prefix_ext: Option<types::Element>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Short description of the action
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Static text equivalent of the action, used if the dynamic aspects
    /// cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::Markdown>,
    /// Primitive extension sibling for [`text_equivalent`](Self::text_equivalent) (FHIR `_textEquivalent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_textEquivalent")]
    pub text_equivalent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Code representing the meaning of the action or sub-actions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Supporting documentation for the intended performer of the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documentation: Vec<types::RelatedArtifact>,

    /// What goals
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference<crate::r6::resources::Goal>>,

    /// Whether or not the action is applicable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<RequestOrchestrationActionCondition>,

    /// Input data requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<RequestOrchestrationActionInput>,

    /// Output data definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<RequestOrchestrationActionOutput>,

    /// Relationship to another action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_action: Vec<RequestOrchestrationActionRelatedAction>,

    /// When the action should take place
    /// The `RequestOrchestration.action.timing[x]` choice element (0..1); see [`RequestOrchestrationActionTiming`].
    #[serde(flatten)]
    pub timing: Option<RequestOrchestrationActionTiming>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

    /// Who should perform the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<RequestOrchestrationActionParticipant>,

    /// create | update | remove | fire-event
    pub r#type: Option<types::CodeableConcept>,

    /// visual-group | logical-group | sentence-group
    pub grouping_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionGroupingBehavior>>,
    /// Primitive extension sibling for [`grouping_behavior`](Self::grouping_behavior) (FHIR `_groupingBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupingBehavior")]
    pub grouping_behavior_ext: Option<types::Element>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionSelectionBehavior>>,
    /// Primitive extension sibling for [`selection_behavior`](Self::selection_behavior) (FHIR `_selectionBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_selectionBehavior")]
    pub selection_behavior_ext: Option<types::Element>,

    /// must | could | must-unless-documented
    pub required_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionRequiredBehavior>>,
    /// Primitive extension sibling for [`required_behavior`](Self::required_behavior) (FHIR `_requiredBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requiredBehavior")]
    pub required_behavior_ext: Option<types::Element>,

    /// yes | no
    pub precheck_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionPrecheckBehavior>>,
    /// Primitive extension sibling for [`precheck_behavior`](Self::precheck_behavior) (FHIR `_precheckBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precheckBehavior")]
    pub precheck_behavior_ext: Option<types::Element>,

    /// single | multiple
    pub cardinality_behavior:
        Option<crate::coded::Coded<crate::r6::codes::ActionCardinalityBehavior>>,
    /// Primitive extension sibling for [`cardinality_behavior`](Self::cardinality_behavior) (FHIR `_cardinalityBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cardinalityBehavior")]
    pub cardinality_behavior_ext: Option<types::Element>,

    /// The target of the action
    pub resource: Option<types::Reference>,

    /// Description of the activity to be performed
    /// The `RequestOrchestration.action.definition[x]` choice element (0..1); see [`RequestOrchestrationActionDefinition`].
    #[serde(flatten)]
    pub definition: Option<RequestOrchestrationActionDefinition>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

    /// Dynamic aspects of the definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic_value: Vec<RequestOrchestrationActionDynamicValue>,

    /// Sub action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<RequestOrchestrationAction>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestOrchestrationActionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    link_id: Option<types::String>,
    #[serde(rename = "_linkId")]
    link_id_ext: Option<types::Element>,
    prefix: Option<types::String>,
    #[serde(rename = "_prefix")]
    prefix_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    text_equivalent: Option<types::Markdown>,
    #[serde(rename = "_textEquivalent")]
    text_equivalent_ext: Option<types::Element>,
    priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    #[serde(default)]
    code: Vec<types::CodeableConcept>,
    #[serde(default)]
    documentation: Vec<types::RelatedArtifact>,
    #[serde(default)]
    goal: Vec<types::Reference<crate::r6::resources::Goal>>,
    #[serde(default)]
    condition: Vec<RequestOrchestrationActionCondition>,
    #[serde(default)]
    input: Vec<RequestOrchestrationActionInput>,
    #[serde(default)]
    output: Vec<RequestOrchestrationActionOutput>,
    #[serde(default)]
    related_action: Vec<RequestOrchestrationActionRelatedAction>,
    #[serde(flatten)]
    timing: crate::r6::choice::Slot<RequestOrchestrationActionTiming>,
    location: Option<types::CodeableReference>,
    #[serde(default)]
    participant: Vec<RequestOrchestrationActionParticipant>,
    r#type: Option<types::CodeableConcept>,
    grouping_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionGroupingBehavior>>,
    #[serde(rename = "_groupingBehavior")]
    grouping_behavior_ext: Option<types::Element>,
    selection_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionSelectionBehavior>>,
    #[serde(rename = "_selectionBehavior")]
    selection_behavior_ext: Option<types::Element>,
    required_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionRequiredBehavior>>,
    #[serde(rename = "_requiredBehavior")]
    required_behavior_ext: Option<types::Element>,
    precheck_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionPrecheckBehavior>>,
    #[serde(rename = "_precheckBehavior")]
    precheck_behavior_ext: Option<types::Element>,
    cardinality_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionCardinalityBehavior>>,
    #[serde(rename = "_cardinalityBehavior")]
    cardinality_behavior_ext: Option<types::Element>,
    resource: Option<types::Reference>,
    #[serde(flatten)]
    definition: crate::r6::choice::Slot<RequestOrchestrationActionDefinition>,
    transform: Option<types::Canonical>,
    #[serde(rename = "_transform")]
    transform_ext: Option<types::Element>,
    #[serde(default)]
    dynamic_value: Vec<RequestOrchestrationActionDynamicValue>,
    #[serde(default)]
    action: Vec<RequestOrchestrationAction>,
}

impl ::core::convert::From<RequestOrchestrationActionDe> for RequestOrchestrationAction {
    fn from(v: RequestOrchestrationActionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            link_id: v.link_id,
            link_id_ext: v.link_id_ext,
            prefix: v.prefix,
            prefix_ext: v.prefix_ext,
            title: v.title,
            title_ext: v.title_ext,
            description: v.description,
            description_ext: v.description_ext,
            text_equivalent: v.text_equivalent,
            text_equivalent_ext: v.text_equivalent_ext,
            priority: v.priority,
            priority_ext: v.priority_ext,
            code: v.code,
            documentation: v.documentation,
            goal: v.goal,
            condition: v.condition,
            input: v.input,
            output: v.output,
            related_action: v.related_action,
            timing: v.timing.0,
            location: v.location,
            participant: v.participant,
            r#type: v.r#type,
            grouping_behavior: v.grouping_behavior,
            grouping_behavior_ext: v.grouping_behavior_ext,
            selection_behavior: v.selection_behavior,
            selection_behavior_ext: v.selection_behavior_ext,
            required_behavior: v.required_behavior,
            required_behavior_ext: v.required_behavior_ext,
            precheck_behavior: v.precheck_behavior,
            precheck_behavior_ext: v.precheck_behavior_ext,
            cardinality_behavior: v.cardinality_behavior,
            cardinality_behavior_ext: v.cardinality_behavior_ext,
            resource: v.resource,
            definition: v.definition.0,
            transform: v.transform,
            transform_ext: v.transform_ext,
            dynamic_value: v.dynamic_value,
            action: v.action,
        }
    }
}

/// An expression that describes applicability criteria, or start/stop
/// conditions for the action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::request_orchestration::RequestOrchestrationActionCondition;
/// use fhir::r6::types;
///
/// let value = RequestOrchestrationActionCondition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RequestOrchestrationActionCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RequestOrchestrationActionCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// applicability | start | stop
    pub kind: crate::coded::Coded<crate::r6::codes::ActionConditionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Boolean-valued expression
    pub expression: Option<types::Expression>,
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
/// use fhir::r6::resources::request_orchestration::RequestOrchestrationActionDynamicValue;
/// use fhir::r6::types;
///
/// let value = RequestOrchestrationActionDynamicValue {
///     path: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `path` is the name this serializes to on the wire.
/// assert_eq!(json["path"], ::serde_json::json!("abc"));
///
/// let back: RequestOrchestrationActionDynamicValue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RequestOrchestrationActionDynamicValue {
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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// An expression that provides the dynamic value for the customization
    pub expression: Option<types::Expression>,
}

/// Defines input data requirements for the action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::request_orchestration::RequestOrchestrationActionInput;
/// use fhir::r6::types;
///
/// let value = RequestOrchestrationActionInput {
///     related_data: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `relatedData` is the name this serializes to on the wire.
/// assert_eq!(json["relatedData"], ::serde_json::json!("pat-1"));
///
/// let back: RequestOrchestrationActionInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RequestOrchestrationActionInput {
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::Id>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// Defines the outputs of the action, if any.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::request_orchestration::RequestOrchestrationActionOutput;
/// use fhir::r6::types;
///
/// let value = RequestOrchestrationActionOutput {
///     related_data: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `relatedData` is the name this serializes to on the wire.
/// assert_eq!(json["relatedData"], ::serde_json::json!("abc"));
///
/// let back: RequestOrchestrationActionOutput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RequestOrchestrationActionOutput {
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::String>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// The participant that should perform or be responsible for this action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::request_orchestration::RequestOrchestrationActionParticipant;
/// use fhir::r6::types;
///
/// let value = RequestOrchestrationActionParticipant {
///     type_canonical: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `typeCanonical` is the name this serializes to on the wire.
/// assert_eq!(json["typeCanonical"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: RequestOrchestrationActionParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "RequestOrchestrationActionParticipantDe")]
#[fhir_version("r6")]
pub struct RequestOrchestrationActionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// careteam | device | group | healthcareservice | location | organization
    /// | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::ActionParticipantType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent, etc
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,

    /// Who/what is participating?
    /// The `RequestOrchestration.action.participant.actor[x]` choice element (0..1); see [`RequestOrchestrationActionParticipantActor`].
    #[serde(flatten)]
    pub actor: Option<RequestOrchestrationActionParticipantActor>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestOrchestrationActionParticipantDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: Option<crate::coded::Coded<crate::r6::codes::ActionParticipantType>>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    type_canonical: Option<types::Canonical>,
    #[serde(rename = "_typeCanonical")]
    type_canonical_ext: Option<types::Element>,
    type_reference: Option<types::Reference>,
    role: Option<types::CodeableConcept>,
    function: Option<types::CodeableConcept>,
    #[serde(flatten)]
    actor: crate::r6::choice::Slot<RequestOrchestrationActionParticipantActor>,
}

impl ::core::convert::From<RequestOrchestrationActionParticipantDe>
    for RequestOrchestrationActionParticipant
{
    fn from(v: RequestOrchestrationActionParticipantDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            type_canonical: v.type_canonical,
            type_canonical_ext: v.type_canonical_ext,
            type_reference: v.type_reference,
            role: v.role,
            function: v.function,
            actor: v.actor.0,
        }
    }
}

/// A relationship to another action such as "before" or "30-60 minutes after
/// start of".
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::request_orchestration::RequestOrchestrationActionRelatedAction;
/// use fhir::r6::types;
///
/// let value = RequestOrchestrationActionRelatedAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RequestOrchestrationActionRelatedAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "RequestOrchestrationActionRelatedActionDe")]
#[fhir_version("r6")]
pub struct RequestOrchestrationActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What action this is related to
    pub target_id: types::Id,
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start
    /// | concurrent-with-end | after | after-start | after-end
    pub relationship: crate::coded::Coded<crate::r6::codes::ActionRelationshipType>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start
    /// | concurrent-with-end | after | after-start | after-end
    pub end_relationship: Option<crate::coded::Coded<crate::r6::codes::ActionRelationshipType>>,
    /// Primitive extension sibling for [`end_relationship`](Self::end_relationship) (FHIR `_endRelationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endRelationship")]
    pub end_relationship_ext: Option<types::Element>,

    /// Time offset for the relationship
    /// The `RequestOrchestration.action.relatedAction.offset[x]` choice element (0..1); see [`RequestOrchestrationActionRelatedActionOffset`].
    #[serde(flatten)]
    pub offset: Option<RequestOrchestrationActionRelatedActionOffset>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestOrchestrationActionRelatedActionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    target_id: types::Id,
    #[serde(rename = "_targetId")]
    target_id_ext: Option<types::Element>,
    relationship: crate::coded::Coded<crate::r6::codes::ActionRelationshipType>,
    #[serde(rename = "_relationship")]
    relationship_ext: Option<types::Element>,
    end_relationship: Option<crate::coded::Coded<crate::r6::codes::ActionRelationshipType>>,
    #[serde(rename = "_endRelationship")]
    end_relationship_ext: Option<types::Element>,
    #[serde(flatten)]
    offset: crate::r6::choice::Slot<RequestOrchestrationActionRelatedActionOffset>,
}

impl ::core::convert::From<RequestOrchestrationActionRelatedActionDe>
    for RequestOrchestrationActionRelatedAction
{
    fn from(v: RequestOrchestrationActionRelatedActionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            target_id: v.target_id,
            target_id_ext: v.target_id_ext,
            relationship: v.relationship,
            relationship_ext: v.relationship_ext,
            end_relationship: v.end_relationship,
            end_relationship_ext: v.end_relationship_ext,
            offset: v.offset.0,
        }
    }
}

/// The `RequestOrchestration.action.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RequestOrchestrationActionTiming {
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `timingAge` variant.
    #[fhir("timingAge")]
    Age(Box<types::Age>),
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

/// The `RequestOrchestration.action.definition[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RequestOrchestrationActionDefinition {
    /// `definitionCanonical` variant.
    #[fhir("definitionCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `definitionUri` variant.
    #[fhir("definitionUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

/// The `RequestOrchestration.action.participant.actor[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RequestOrchestrationActionParticipantActor {
    /// `actorCanonical` variant.
    #[fhir("actorCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `actorReference` variant.
    #[fhir("actorReference")]
    Reference(Box<types::Reference>),
}

/// The `RequestOrchestration.action.relatedAction.offset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RequestOrchestrationActionRelatedActionOffset {
    /// `offsetDuration` variant.
    #[fhir("offsetDuration")]
    Duration(Box<types::Duration>),
    /// `offsetRange` variant.
    #[fhir("offsetRange")]
    Range(Box<types::Range>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RequestOrchestration;

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
