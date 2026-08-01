//! Subscription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Subscription
//!
//! Version: 5.0.0
//!
//! Subscription Resource: The subscription resource describes a particular client's request to be notified about a SubscriptionTopic.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Subscription resource describes a particular client's request to be
/// notified about a SubscriptionTopic. A Subscription is a persistent record
/// that identifies a topic of interest, optional filters to narrow the stream
/// of events, and a notification channel (such as a REST hook, WebSocket, email,
/// or messaging endpoint) through which the server delivers notifications when
/// matching events occur.
///
/// Administratively, a Subscription represents an agreement between a client
/// and a server: the client asks to be told when resources matching the
/// referenced topic change, and the server manages the subscription's
/// lifecycle (`requested`, `active`, `error`, `off`, or `entered-in-error`),
/// delivering periodic heartbeat and handshake notifications as needed to
/// confirm the channel is healthy. This is the R5 successor to the R4
/// subscription mechanism, decoupling the general subscription request from
/// the topic-specific trigger criteria defined by a `SubscriptionTopic`.
/// Because the events delivered may reference clinical resources such as
/// [`Patient`](crate::r5::resources::patient::Patient) records, servers
/// typically apply the same security and consent constraints to
/// notifications as they would to a direct read of those resources.
///
/// # See also
///
/// - `SubscriptionTopic` — defines the event trigger criteria a Subscription refers to via `topic`.
/// - `SubscriptionStatus` — the resource used to report handshake, heartbeat, and event notifications.
/// - [`Reference`](crate::r5::types::Reference) — used for `managing_entity`.
/// - [`Coding`](crate::r5::types::Coding) — used for `channel_type`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription::Subscription;
/// use fhir::r5::types;
///
/// let value = Subscription {
///     heartbeat_period: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `heartbeatPeriod` is the name this serializes to on the wire.
/// assert_eq!(json["heartbeatPeriod"], ::serde_json::json!(0));
///
/// let back: Subscription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
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

    /// Additional identifiers (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Human readable name for this subscription
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Current lifecycle state of the subscription: requested | active | error | off | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::SubscriptionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Canonical URL of the `SubscriptionTopic` that defines which events this subscription reacts to
    pub topic: types::Canonical,
    /// Primitive extension sibling for [`topic`](Self::topic) (FHIR `_topic`).
    #[serde(rename = "_topic")]
    pub topic_ext: Option<types::Element>,

    /// Contact details for source (e.g. troubleshooting)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// When to automatically delete the subscription
    pub end: Option<types::Instant>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`).
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Reference to the party (e.g. an Organization or Practitioner) responsible for Subscription changes
    pub managing_entity: Option<types::Reference>,

    /// Description of why this subscription was created
    pub reason: Option<types::String>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`).
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,

    /// Criteria for narrowing the subscription topic stream
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter_by: Vec<SubscriptionFilterBy>,

    /// Coded type of notification channel, such as a REST hook, WebSocket, email, or messaging endpoint
    pub channel_type: types::Coding,

    /// URL or address that the notification channel points to (meaning depends on `channel_type`)
    pub endpoint: Option<types::Url>,
    /// Primitive extension sibling for [`endpoint`](Self::endpoint) (FHIR `_endpoint`).
    #[serde(rename = "_endpoint")]
    pub endpoint_ext: Option<types::Element>,

    /// Channel type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<SubscriptionParameter>,

    /// Interval in seconds to send 'heartbeat' notification
    pub heartbeat_period: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`heartbeat_period`](Self::heartbeat_period) (FHIR `_heartbeatPeriod`).
    #[serde(rename = "_heartbeatPeriod")]
    pub heartbeat_period_ext: Option<types::Element>,

    /// Timeout in seconds to attempt notification delivery
    pub timeout: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`timeout`](Self::timeout) (FHIR `_timeout`).
    #[serde(rename = "_timeout")]
    pub timeout_ext: Option<types::Element>,

    /// MIME type to send, or omit for no payload
    pub content_type: Option<types::Code>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`).
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// empty | id-only | full-resource
    pub content: Option<crate::r5::coded::Coded<crate::r5::codes::SubscriptionPayloadContent>>,
    /// Primitive extension sibling for [`content`](Self::content) (FHIR `_content`).
    #[serde(rename = "_content")]
    pub content_ext: Option<types::Element>,

    /// Maximum number of events that can be combined in a single notification
    pub max_count: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`max_count`](Self::max_count) (FHIR `_maxCount`).
    #[serde(rename = "_maxCount")]
    pub max_count_ext: Option<types::Element>,
}

/// Criteria for narrowing the subscription topic stream. Each filter references
/// a filter parameter defined in the referenced SubscriptionTopic and applies a
/// comparator, modifier, and value to restrict the events that trigger a
/// notification.
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription::SubscriptionFilterBy;
/// use fhir::r5::types;
///
/// let value = SubscriptionFilterBy {
///     resource_type: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `resourceType` is the name this serializes to on the wire.
/// assert_eq!(json["resourceType"], ::serde_json::json!("http://example.org"));
///
/// let back: SubscriptionFilterBy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionFilterBy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Allowed Resource (reference to definition) for this Subscription filter
    pub resource_type: Option<types::Uri>,
    /// Primitive extension sibling for [`resource_type`](Self::resource_type) (FHIR `_resourceType`).
    #[serde(rename = "_resourceType")]
    pub resource_type_ext: Option<types::Element>,

    /// Filter label defined in SubscriptionTopic
    pub filter_parameter: types::String,
    /// Primitive extension sibling for [`filter_parameter`](Self::filter_parameter) (FHIR `_filterParameter`).
    #[serde(rename = "_filterParameter")]
    pub filter_parameter_ext: Option<types::Element>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<crate::r5::coded::Coded<crate::r5::codes::SearchComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`).
    #[serde(rename = "_comparator")]
    pub comparator_ext: Option<types::Element>,

    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<crate::r5::coded::Coded<crate::r5::codes::SearchModifierCode>>,
    /// Primitive extension sibling for [`modifier`](Self::modifier) (FHIR `_modifier`).
    #[serde(rename = "_modifier")]
    pub modifier_ext: Option<types::Element>,

    /// Literal value or resource path
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Channel-type-specific parameter used to configure the notification channel.
/// Each parameter is a name/value pair whose meaning is defined by the channel
/// type (for example, an HTTP header for a REST hook channel).
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription::SubscriptionParameter;
/// use fhir::r5::types;
///
/// let value = SubscriptionParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubscriptionParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name (key) of the parameter
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Value of the parameter to use or pass through
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Subscription;

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
