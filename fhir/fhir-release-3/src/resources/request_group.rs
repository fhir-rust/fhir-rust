//! RequestGroup
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RequestGroup
//!
//!
//!
//! A group of related requests
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for RequestGroup Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::request_group::RequestGroup;
/// use fhir::r3::types;
///
/// let value = RequestGroup {
///     authored_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authoredOn` is the name this serializes to on the wire.
/// assert_eq!(json["authoredOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: RequestGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct RequestGroup {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// Fulfills plan, proposal, or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Request(s) replaced by this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | suspended | cancelled | completed | entered-in-error |
    /// unknown
    pub status: crate::coded::Coded<crate::r3::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | order
    pub intent: crate::coded::Coded<crate::r3::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r3::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Who the request group is about
    pub subject: Option<types::Reference>,

    /// Encounter or Episode for the request group
    pub context: Option<types::Reference>,

    /// When the request group was authored
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Device or practitioner that authored the request group
    pub author: Option<types::Reference>,

    /// Reason for the request group
    /// The `RequestGroup.reason[x]` choice element (0..1); see [`RequestGroupReason`].
    #[serde(flatten)]
    pub reason: Option<RequestGroupReason>,

    /// Additional notes about the response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Proposed actions, if any
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<RequestGroupAction>,
}

/// The actions, if any, produced by the evaluation of the artifact.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::request_group::RequestGroupAction;
/// use fhir::r3::types;
///
/// let value = RequestGroupAction {
///     text_equivalent: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `textEquivalent` is the name this serializes to on the wire.
/// assert_eq!(json["textEquivalent"], ::serde_json::json!("abc"));
///
/// let back: RequestGroupAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct RequestGroupAction {
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

    /// Supporting documentation for the intended performer of the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documentation: Vec<types::RelatedArtifact>,

    /// Whether or not the action is applicable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<RequestGroupActionCondition>,

    /// Relationship to another action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_action: Vec<RequestGroupActionRelatedAction>,

    /// When the action should take place
    /// The `RequestGroup.action.timing[x]` choice element (0..1); see [`RequestGroupActionTiming`].
    #[serde(flatten)]
    pub timing: Option<RequestGroupActionTiming>,

    /// Who should perform the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<types::Reference>,

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

    /// The target of the action
    pub resource: Option<types::Reference>,

    /// Sub action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<RequestGroupAction>,
}

/// An expression that describes applicability criteria, or start/stop
/// conditions for the action.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::request_group::RequestGroupActionCondition;
/// use fhir::r3::types;
///
/// let value = RequestGroupActionCondition {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: RequestGroupActionCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct RequestGroupActionCondition {
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

/// A relationship to another action such as "before" or "30-60 minutes after
/// start of".
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::request_group::RequestGroupActionRelatedAction;
/// use fhir::r3::types;
///
/// let value = RequestGroupActionRelatedAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RequestGroupActionRelatedAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct RequestGroupActionRelatedAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What action this is related to
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
    /// The `RequestGroup.action.relatedAction.offset[x]` choice element (0..1); see [`RequestGroupActionRelatedActionOffset`].
    #[serde(flatten)]
    pub offset: Option<RequestGroupActionRelatedActionOffset>,
}

/// The `RequestGroup.reason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum RequestGroupReason {
    /// `reasonCodeableConcept` variant.
    #[fhir("reasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonReference` variant.
    #[fhir("reasonReference")]
    Reference(Box<types::Reference>),
}

/// The `RequestGroup.action.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum RequestGroupActionTiming {
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

/// The `RequestGroup.action.relatedAction.offset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum RequestGroupActionRelatedActionOffset {
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
    type T = RequestGroup;

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
