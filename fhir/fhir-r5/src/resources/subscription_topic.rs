//! SubscriptionTopic
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubscriptionTopic
//!
//! Version: 5.0.0
//!
//! SubscriptionTopic Resource: Describes a stream of resource state changes identified by trigger criteria and annotated with labels useful to filter projections from this topic.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubscriptionTopic
///
/// Describes a stream of resource state changes identified by trigger criteria
/// and annotated with labels useful to filter projections from this topic. A
/// SubscriptionTopic defines the interesting events that a server can notify
/// subscribers about, along with the filters and notification shapes those
/// subscribers may request. It is a canonical, shareable definition that
/// Subscription resources reference to describe the changes they wish to receive.
///
/// Administratively, a SubscriptionTopic acts as the contract between a FHIR
/// server and its clients for event-driven notifications: it specifies which
/// resource types can trigger a notification (via resource-based or
/// event-based triggers), which element-level or state-transition conditions
/// must hold for a trigger to fire, which parameters a client may use to
/// filter the notifications it receives, and the shape (included and
/// reverse-included content) of the payload that will be sent. Publishers
/// author SubscriptionTopics to describe the interesting changes they are
/// willing to notify about (for example, new orders, completed observations,
/// or changes to a patient's status), and clients then create individual
/// Subscription resources that reference a topic by canonical URL, apply
/// their own filters, and choose a delivery channel.
///
/// # Related resources
///
/// A SubscriptionTopic is referenced by `Subscription` resources, which
/// instantiate the topic for a specific client and delivery channel. Trigger
/// and filter definitions frequently reference clinical resources such as
/// [`Patient`](crate::r5::resources::patient::Patient) or use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values to describe
/// triggering events.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_topic::SubscriptionTopic;
/// use fhir::r5::types;
///
/// let value = SubscriptionTopic {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: SubscriptionTopic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SubscriptionTopicDe")]
pub struct SubscriptionTopic {
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

    /// Canonical identifier for this subscription topic, represented as an absolute URI (globally unique); this is the value a `Subscription` uses to reference the topic
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier for subscription topic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the subscription topic
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `SubscriptionTopic.versionAlgorithm[x]` choice element (0..1); see [`SubscriptionTopicVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<SubscriptionTopicVersionAlgorithm>,

    /// Name for this subscription topic (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this subscription topic (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Based on FHIR protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub derived_from: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`).
    #[serde(rename = "_derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_ext: Vec<Option<types::Element>>,

    /// The publication lifecycle status of this topic definition: draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date status first applied
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The name of the individual or organization that published the SubscriptionTopic
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the SubscriptionTopic
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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

    /// When SubscriptionTopic is/was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// Date the Subscription Topic was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// The effective date range for the SubscriptionTopic
    pub effective_period: Option<types::Period>,

    /// Definition of a resource state change (create, update, delete) that can trigger a notification for this topic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_trigger: Vec<SubscriptionTopicResourceTrigger>,

    /// Event definitions the SubscriptionTopic, describing coded events (rather than plain resource state changes) that can trigger a notification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_trigger: Vec<SubscriptionTopicEventTrigger>,

    /// Properties by which a Subscription can filter notifications from the SubscriptionTopic, exposed to clients via `Subscription.filterBy`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub can_filter_by: Vec<SubscriptionTopicCanFilterBy>,

    /// Properties for describing the shape of notifications generated by this topic, including included and reverse-included content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notification_shape: Vec<SubscriptionTopicNotificationShape>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubscriptionTopicDe {
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
    url: types::Uri,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r5::choice::Slot<SubscriptionTopicVersionAlgorithm>,
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
    resource_trigger: Vec<SubscriptionTopicResourceTrigger>,
    #[serde(default)]
    event_trigger: Vec<SubscriptionTopicEventTrigger>,
    #[serde(default)]
    can_filter_by: Vec<SubscriptionTopicCanFilterBy>,
    #[serde(default)]
    notification_shape: Vec<SubscriptionTopicNotificationShape>,
}

impl ::core::convert::From<SubscriptionTopicDe> for SubscriptionTopic {
    fn from(v: SubscriptionTopicDe) -> Self {
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
            resource_trigger: v.resource_trigger,
            event_trigger: v.event_trigger,
            can_filter_by: v.can_filter_by,
            notification_shape: v.notification_shape,
        }
    }
}

/// Definition of a resource-based trigger for the subscription topic.
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_topic::SubscriptionTopicResourceTrigger;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Data Type or Resource (reference to definition) for this trigger definition
    pub resource: types::Uri,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// create | update | delete
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub supported_interaction: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`supported_interaction`](Self::supported_interaction) (FHIR `_supportedInteraction`).
    #[serde(rename = "_supportedInteraction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_interaction_ext: Vec<Option<types::Element>>,

    /// Query based trigger rule
    pub query_criteria: Option<SubscriptionTopicResourceTriggerQueryCriteria>,

    /// FHIRPath based trigger rule
    pub fhir_path_criteria: Option<types::String>,
    /// Primitive extension sibling for [`fhir_path_criteria`](Self::fhir_path_criteria) (FHIR `_fhirPathCriteria`).
    #[serde(rename = "_fhirPathCriteria")]
    pub fhir_path_criteria_ext: Option<types::Element>,
}

/// Query based trigger rule for a resource-based trigger.
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_topic::SubscriptionTopicResourceTriggerQueryCriteria;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`previous`](Self::previous) (FHIR `_previous`).
    #[serde(rename = "_previous")]
    pub previous_ext: Option<types::Element>,

    /// test-passes | test-fails
    pub result_for_create:
        Option<crate::r5::coded::Coded<crate::r5::codes::SubscriptiontopicCrBehavior>>,
    /// Primitive extension sibling for [`result_for_create`](Self::result_for_create) (FHIR `_resultForCreate`).
    #[serde(rename = "_resultForCreate")]
    pub result_for_create_ext: Option<types::Element>,

    /// Rule applied to current resource state
    pub current: Option<types::String>,
    /// Primitive extension sibling for [`current`](Self::current) (FHIR `_current`).
    #[serde(rename = "_current")]
    pub current_ext: Option<types::Element>,

    /// test-passes | test-fails
    pub result_for_delete:
        Option<crate::r5::coded::Coded<crate::r5::codes::SubscriptiontopicCrBehavior>>,
    /// Primitive extension sibling for [`result_for_delete`](Self::result_for_delete) (FHIR `_resultForDelete`).
    #[serde(rename = "_resultForDelete")]
    pub result_for_delete_ext: Option<types::Element>,

    /// Both must be true flag
    pub require_both: Option<types::Boolean>,
    /// Primitive extension sibling for [`require_both`](Self::require_both) (FHIR `_requireBoth`).
    #[serde(rename = "_requireBoth")]
    pub require_both_ext: Option<types::Element>,
}

/// Event definitions the SubscriptionTopic.
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_topic::SubscriptionTopicEventTrigger;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Event which can trigger a notification from the SubscriptionTopic
    pub event: types::CodeableConcept,

    /// Data Type or Resource (reference to definition) for this trigger definition
    pub resource: types::Uri,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,
}

/// Properties by which a Subscription can filter notifications from the SubscriptionTopic.
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_topic::SubscriptionTopicCanFilterBy;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// URL of the triggering Resource that this filter applies to
    pub resource: Option<types::Uri>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// Human-readable and computation-friendly name for a filter parameter usable by subscriptions on this topic, via Subscription.filterBy.filterParameter
    pub filter_parameter: types::String,
    /// Primitive extension sibling for [`filter_parameter`](Self::filter_parameter) (FHIR `_filterParameter`).
    #[serde(rename = "_filterParameter")]
    pub filter_parameter_ext: Option<types::Element>,

    /// Canonical URL for a filterParameter definition
    pub filter_definition: Option<types::Uri>,
    /// Primitive extension sibling for [`filter_definition`](Self::filter_definition) (FHIR `_filterDefinition`).
    #[serde(rename = "_filterDefinition")]
    pub filter_definition_ext: Option<types::Element>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub comparator:
        ::fhir_core::PrimVec<crate::r5::coded::Coded<crate::r5::codes::SearchComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`).
    #[serde(rename = "_comparator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparator_ext: Vec<Option<types::Element>>,

    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub modifier:
        ::fhir_core::PrimVec<crate::r5::coded::Coded<crate::r5::codes::SearchModifierCode>>,
    /// Primitive extension sibling for [`modifier`](Self::modifier) (FHIR `_modifier`).
    #[serde(rename = "_modifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_ext: Vec<Option<types::Element>>,
}

/// Properties for describing the shape of notifications generated by this topic.
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_topic::SubscriptionTopicNotificationShape;
/// use fhir::r5::types;
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
pub struct SubscriptionTopicNotificationShape {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// URL of the Resource that is the focus (main) resource in a notification shape
    pub resource: types::Uri,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// Include directives, rooted in the resource for this shape
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub include: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`include`](Self::include) (FHIR `_include`).
    #[serde(rename = "_include")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include_ext: Vec<Option<types::Element>>,

    /// Reverse include directives, rooted in the resource for this shape
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub rev_include: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`rev_include`](Self::rev_include) (FHIR `_revInclude`).
    #[serde(rename = "_revInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rev_include_ext: Vec<Option<types::Element>>,
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
/// The `SubscriptionTopic.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SubscriptionTopicVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
