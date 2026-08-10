//! SubscriptionTopic
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubscriptionTopic
//!
//! Version: 4.3.0
//!
//! The definition of a specific topic for triggering events within the
//! Subscriptions framework
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes a stream of resource state changes identified by trigger criteria
/// and annotated with labels useful to filter projections from this topic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_topic::SubscriptionTopic;
/// use fhir::r4b::types;
///
/// let value = SubscriptionTopic {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: SubscriptionTopic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SubscriptionTopic {
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

    /// Canonical identifier for this subscription topic definition,
    /// represented as a URI (globally unique)
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business Identifier for this subscription topic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the subscription topic
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this subscription topic (Human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Based on FHIR protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Canonical>,
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

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date status first applied
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The name of the individual or organization that published the
    /// SubscriptionTopic
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the SubscriptionTopic
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction of the SubscriptionTopic (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this SubscriptionTopic is defined
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

    /// When SubscriptionTopic is/was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// Date the Subscription Topic was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// The effective date range for the SubscriptionTopic
    pub effective_period: Option<types::Period>,

    /// Definition of a resource-based trigger for the subscription topic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_trigger: Vec<SubscriptionTopicResourceTrigger>,

    /// Event definitions the SubscriptionTopic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_trigger: Vec<SubscriptionTopicEventTrigger>,

    /// Properties by which a Subscription can filter notifications from the
    /// SubscriptionTopic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub can_filter_by: Vec<SubscriptionTopicCanFilterBy>,

    /// Properties for describing the shape of notifications generated by this
    /// topic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notification_shape: Vec<SubscriptionTopicNotificationShape>,
}

/// List of properties by which Subscriptions on the SubscriptionTopic can be
/// filtered. May be defined Search Parameters (e.g., Encounter.patient) or
/// parameters defined within this SubscriptionTopic context (e.g., hub.event).
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_topic::SubscriptionTopicCanFilterBy;
/// use fhir::r4b::types;
///
/// let value = SubscriptionTopicCanFilterBy {
///     filter_definition: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `filterDefinition` is the name this serializes to on the wire.
/// assert_eq!(json["filterDefinition"], ::serde_json::json!("http://example.org"));
///
/// let back: SubscriptionTopicCanFilterBy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SubscriptionTopicCanFilterBy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of this filter parameter
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// URL of the triggering Resource that this filter applies to
    pub resource: Option<types::Uri>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// Human-readable and computation-friendly name for a filter parameter
    /// usable by subscriptions on this topic, via
    /// Subscription.filterBy.filterParameter
    pub filter_parameter: types::String,
    /// Primitive extension sibling for [`filter_parameter`](Self::filter_parameter) (FHIR `_filterParameter`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_filterParameter")]
    pub filter_parameter_ext: Option<types::Element>,

    /// Canonical URL for a filterParameter definition
    pub filter_definition: Option<types::Uri>,
    /// Primitive extension sibling for [`filter_definition`](Self::filter_definition) (FHIR `_filterDefinition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_filterDefinition")]
    pub filter_definition_ext: Option<types::Element>,

    /// \= | eq | ne | gt | lt | ge | le | sa | eb | ap | above | below | in |
    /// not-in | of-type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<crate::coded::Coded<crate::r4b::codes::SubscriptionSearchModifier>>,
    /// Primitive extension sibling for [`modifier`](Self::modifier) (FHIR `_modifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_ext: Vec<Option<types::Element>>,
}

/// Event definition which can be used to trigger the SubscriptionTopic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_topic::SubscriptionTopicEventTrigger;
/// use fhir::r4b::types;
///
/// let value = SubscriptionTopicEventTrigger {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: SubscriptionTopicEventTrigger = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SubscriptionTopicEventTrigger {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Text representation of the event trigger
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Event which can trigger a notification from the SubscriptionTopic
    pub event: types::CodeableConcept,

    /// Data Type or Resource (reference to definition) for this trigger
    /// definition
    pub resource: types::Uri,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,
}

/// List of properties to describe the shape (e.g., resources) included in
/// notifications from this Subscription Topic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_topic::SubscriptionTopicNotificationShape;
/// use fhir::r4b::types;
///
/// let value = SubscriptionTopicNotificationShape {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubscriptionTopicNotificationShape = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SubscriptionTopicNotificationShape {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// URL of the Resource that is the focus (main) resource in a notification
    /// shape
    pub resource: types::Uri,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// Include directives, rooted in the resource for this shape
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<types::String>,
    /// Primitive extension sibling for [`include`](Self::include) (FHIR `_include`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_include")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include_ext: Vec<Option<types::Element>>,

    /// Reverse include directives, rooted in the resource for this shape
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rev_include: Vec<types::String>,
    /// Primitive extension sibling for [`rev_include`](Self::rev_include) (FHIR `_revInclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_revInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rev_include_ext: Vec<Option<types::Element>>,
}

/// A definition of a resource-based event that triggers a notification based
/// on the SubscriptionTopic. The criteria may be just a human readable
/// description and/or a full FHIR search string or FHIRPath expression.
/// Multiple triggers are considered OR joined (e.g., a resource update
/// matching ANY of the definitions will trigger a notification).
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_topic::SubscriptionTopicResourceTrigger;
/// use fhir::r4b::types;
///
/// let value = SubscriptionTopicResourceTrigger {
///     fhir_path_criteria: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `fhirPathCriteria` is the name this serializes to on the wire.
/// assert_eq!(json["fhirPathCriteria"], ::serde_json::json!("abc"));
///
/// let back: SubscriptionTopicResourceTrigger = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SubscriptionTopicResourceTrigger {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Text representation of the resource trigger
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Data Type or Resource (reference to definition) for this trigger
    /// definition
    pub resource: types::Uri,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// create | update | delete
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_interaction: Vec<types::Code>,
    /// Primitive extension sibling for [`supported_interaction`](Self::supported_interaction) (FHIR `_supportedInteraction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_supportedInteraction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_interaction_ext: Vec<Option<types::Element>>,

    /// Query based trigger rule
    pub query_criteria: Option<SubscriptionTopicResourceTriggerQueryCriteria>,

    /// FHIRPath based trigger rule
    pub fhir_path_criteria: Option<types::String>,
    /// Primitive extension sibling for [`fhir_path_criteria`](Self::fhir_path_criteria) (FHIR `_fhirPathCriteria`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fhirPathCriteria")]
    pub fhir_path_criteria_ext: Option<types::Element>,
}

/// The FHIR query based rules that the server should use to determine when to
/// trigger a notification for this subscription topic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_topic::SubscriptionTopicResourceTriggerQueryCriteria;
/// use fhir::r4b::types;
///
/// let value = SubscriptionTopicResourceTriggerQueryCriteria {
///     require_both: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `requireBoth` is the name this serializes to on the wire.
/// assert_eq!(json["requireBoth"], ::serde_json::json!(true));
///
/// let back: SubscriptionTopicResourceTriggerQueryCriteria = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SubscriptionTopicResourceTriggerQueryCriteria {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Rule applied to previous resource state
    pub previous: Option<types::String>,
    /// Primitive extension sibling for [`previous`](Self::previous) (FHIR `_previous`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_previous")]
    pub previous_ext: Option<types::Element>,

    /// test-passes | test-fails
    pub result_for_create:
        Option<crate::coded::Coded<crate::r4b::codes::SubscriptiontopicCrBehavior>>,
    /// Primitive extension sibling for [`result_for_create`](Self::result_for_create) (FHIR `_resultForCreate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resultForCreate")]
    pub result_for_create_ext: Option<types::Element>,

    /// Rule applied to current resource state
    pub current: Option<types::String>,
    /// Primitive extension sibling for [`current`](Self::current) (FHIR `_current`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_current")]
    pub current_ext: Option<types::Element>,

    /// test-passes | test-fails
    pub result_for_delete:
        Option<crate::coded::Coded<crate::r4b::codes::SubscriptiontopicCrBehavior>>,
    /// Primitive extension sibling for [`result_for_delete`](Self::result_for_delete) (FHIR `_resultForDelete`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resultForDelete")]
    pub result_for_delete_ext: Option<types::Element>,

    /// Both must be true flag
    pub require_both: Option<types::Boolean>,
    /// Primitive extension sibling for [`require_both`](Self::require_both) (FHIR `_requireBoth`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requireBoth")]
    pub require_both_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubscriptionTopic;

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
