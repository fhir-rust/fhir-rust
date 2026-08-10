//! RequestGroup
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RequestGroup
//!
//! Version: 4.0.1
//!
//! A group of related requests
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A group of related requests that can be used to capture intended activities
/// that have inter-dependencies such as "give this medication after that one".
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::request_group::RequestGroup;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct RequestGroup {
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
    pub contained: Vec<crate::r4::resources::Resource>,

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

    /// draft | active | on-hold | revoked | completed | entered-in-error |
    /// unknown
    pub status: crate::coded::Coded<crate::r4::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r4::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r4::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// What's being requested/ordered
    pub code: Option<types::CodeableConcept>,

    /// Who the request group is about
    pub subject: Option<types::Reference>,

    /// Created as part of
    pub encounter: Option<types::Reference<crate::r4::resources::Encounter>>,

    /// When the request group was authored
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Device or practitioner that authored the request group
    pub author: Option<types::Reference>,

    /// Why the request group is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Why the request group is needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

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
/// use fhir::r4::resources::request_group::RequestGroupAction;
/// use fhir::r4::types;
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
#[serde(from = "RequestGroupActionDe")]
#[fhir_version("r4")]
pub struct RequestGroupAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

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

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r4::codes::RequestPriority>>,
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
    pub r#type: Option<types::CodeableConcept>,

    /// visual-group | logical-group | sentence-group
    pub grouping_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionGroupingBehavior>>,
    /// Primitive extension sibling for [`grouping_behavior`](Self::grouping_behavior) (FHIR `_groupingBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupingBehavior")]
    pub grouping_behavior_ext: Option<types::Element>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionSelectionBehavior>>,
    /// Primitive extension sibling for [`selection_behavior`](Self::selection_behavior) (FHIR `_selectionBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_selectionBehavior")]
    pub selection_behavior_ext: Option<types::Element>,

    /// must | could | must-unless-documented
    pub required_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionRequiredBehavior>>,
    /// Primitive extension sibling for [`required_behavior`](Self::required_behavior) (FHIR `_requiredBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requiredBehavior")]
    pub required_behavior_ext: Option<types::Element>,

    /// yes | no
    pub precheck_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionPrecheckBehavior>>,
    /// Primitive extension sibling for [`precheck_behavior`](Self::precheck_behavior) (FHIR `_precheckBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precheckBehavior")]
    pub precheck_behavior_ext: Option<types::Element>,

    /// single | multiple
    pub cardinality_behavior:
        Option<crate::coded::Coded<crate::r4::codes::ActionCardinalityBehavior>>,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestGroupActionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    prefix: Option<types::String>,
    #[serde(rename = "_prefix")]
    prefix_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    text_equivalent: Option<types::String>,
    #[serde(rename = "_textEquivalent")]
    text_equivalent_ext: Option<types::Element>,
    priority: Option<crate::coded::Coded<crate::r4::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    #[serde(default)]
    code: Vec<types::CodeableConcept>,
    #[serde(default)]
    documentation: Vec<types::RelatedArtifact>,
    #[serde(default)]
    condition: Vec<RequestGroupActionCondition>,
    #[serde(default)]
    related_action: Vec<RequestGroupActionRelatedAction>,
    #[serde(flatten)]
    timing: crate::r4::choice::Slot<RequestGroupActionTiming>,
    #[serde(default)]
    participant: Vec<types::Reference>,
    r#type: Option<types::CodeableConcept>,
    grouping_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionGroupingBehavior>>,
    #[serde(rename = "_groupingBehavior")]
    grouping_behavior_ext: Option<types::Element>,
    selection_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionSelectionBehavior>>,
    #[serde(rename = "_selectionBehavior")]
    selection_behavior_ext: Option<types::Element>,
    required_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionRequiredBehavior>>,
    #[serde(rename = "_requiredBehavior")]
    required_behavior_ext: Option<types::Element>,
    precheck_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionPrecheckBehavior>>,
    #[serde(rename = "_precheckBehavior")]
    precheck_behavior_ext: Option<types::Element>,
    cardinality_behavior: Option<crate::coded::Coded<crate::r4::codes::ActionCardinalityBehavior>>,
    #[serde(rename = "_cardinalityBehavior")]
    cardinality_behavior_ext: Option<types::Element>,
    resource: Option<types::Reference>,
    #[serde(default)]
    action: Vec<RequestGroupAction>,
}

impl ::core::convert::From<RequestGroupActionDe> for RequestGroupAction {
    fn from(v: RequestGroupActionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
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
            condition: v.condition,
            related_action: v.related_action,
            timing: v.timing.0,
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
/// use fhir::r4::resources::request_group::RequestGroupActionCondition;
/// use fhir::r4::types;
///
/// let value = RequestGroupActionCondition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RequestGroupActionCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct RequestGroupActionCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// applicability | start | stop
    pub kind: crate::coded::Coded<crate::r4::codes::ActionConditionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Boolean-valued expression
    pub expression: Option<types::Expression>,
}

/// A relationship to another action such as "before" or "30-60 minutes after
/// start of".
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::request_group::RequestGroupActionRelatedAction;
/// use fhir::r4::types;
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
#[serde(from = "RequestGroupActionRelatedActionDe")]
#[fhir_version("r4")]
pub struct RequestGroupActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    pub relationship: crate::coded::Coded<crate::r4::codes::ActionRelationshipType>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// Time offset for the relationship
    /// The `RequestGroup.action.relatedAction.offset[x]` choice element (0..1); see [`RequestGroupActionRelatedActionOffset`].
    #[serde(flatten)]
    pub offset: Option<RequestGroupActionRelatedActionOffset>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RequestGroupActionRelatedActionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    action_id: types::Id,
    #[serde(rename = "_actionId")]
    action_id_ext: Option<types::Element>,
    relationship: crate::coded::Coded<crate::r4::codes::ActionRelationshipType>,
    #[serde(rename = "_relationship")]
    relationship_ext: Option<types::Element>,
    #[serde(flatten)]
    offset: crate::r4::choice::Slot<RequestGroupActionRelatedActionOffset>,
}

impl ::core::convert::From<RequestGroupActionRelatedActionDe> for RequestGroupActionRelatedAction {
    fn from(v: RequestGroupActionRelatedActionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            action_id: v.action_id,
            action_id_ext: v.action_id_ext,
            relationship: v.relationship,
            relationship_ext: v.relationship_ext,
            offset: v.offset.0,
        }
    }
}

/// The `RequestGroup.action.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum RequestGroupActionTiming {
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
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

/// The `RequestGroup.action.relatedAction.offset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
