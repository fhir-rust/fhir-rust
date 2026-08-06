//! SubscriptionStatus
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubscriptionStatus
//!
//! Version: 5.0.0
//!
//! SubscriptionStatus Resource: The SubscriptionStatus resource describes the state of a Subscription during notifications. It is not persisted.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubscriptionStatus is a status resource that conveys the current state of a
/// Subscription and is delivered as part of notification bundles. It is not
/// persisted on the server; instead it is generated for each notification to
/// report the subscription's status, the type of notification, and details
/// about the events that triggered it. It also carries any errors relevant to
/// the subscription.
///
/// In practice a server sends a SubscriptionStatus as the first entry of every
/// notification bundle it delivers for a subscription, whether that
/// notification is a handshake confirming the channel is ready, a periodic
/// heartbeat, an event notification carrying matching resources, or the
/// response to a query-status request. Clients use the `status`, `type`, and
/// `events_since_subscription_start` fields to track delivery health and
/// detect missed events, and use `notification_event` to correlate the
/// notification with the resources that triggered it.
///
/// # Related resources
///
/// - `Subscription` — the criteria and channel configuration this status reports on.
/// - `SubscriptionTopic` — the topic definition referenced by `topic`.
/// - [`Reference`](crate::r5::types::Reference) — used by `subscription` and event `focus`/`additional_context`.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used to represent entries in `error`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_status::SubscriptionStatus;
/// use fhir::r5::types;
///
/// let value = SubscriptionStatus {
///     events_since_subscription_start: Some(types::Integer64(9_000_000_000)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `eventsSinceSubscriptionStart` is the name this serializes to on the wire.
/// assert_eq!(json["eventsSinceSubscriptionStart"], ::serde_json::json!("9000000000"));
///
/// let back: SubscriptionStatus = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionStatus {
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

    /// Current state of the underlying Subscription: requested | active | error | off | entered-in-error.
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::SubscriptionStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of notification this resource represents: handshake | heartbeat | event-notification | query-status | query-event.
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::SubscriptionNotificationType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Running count of events delivered since the Subscription was created, useful for detecting gaps.
    pub events_since_subscription_start: Option<types::Integer64>,
    /// Primitive extension sibling for [`events_since_subscription_start`](Self::events_since_subscription_start) (FHIR `_eventsSinceSubscriptionStart`).
    #[serde(rename = "_eventsSinceSubscriptionStart")]
    pub events_since_subscription_start_ext: Option<types::Element>,

    /// Detailed information about any events relevant to this notification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notification_event: Vec<SubscriptionStatusNotificationEvent>,

    /// Reference to the Subscription responsible for this notification
    pub subscription: types::Reference,

    /// Canonical reference to the SubscriptionTopic this notification relates to
    pub topic: Option<types::Canonical>,
    /// Primitive extension sibling for [`topic`](Self::topic) (FHIR `_topic`).
    #[serde(rename = "_topic")]
    pub topic_ext: Option<types::Element>,

    /// List of errors on the subscription, if any occurred while generating the notification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<types::CodeableConcept>,
}

/// Detailed information about any events relevant to this notification.
/// # Examples
///
/// ```
/// use fhir::r5::resources::subscription_status::SubscriptionStatusNotificationEvent;
/// use fhir::r5::types;
///
/// let value = SubscriptionStatusNotificationEvent {
///     timestamp: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `timestamp` is the name this serializes to on the wire.
/// assert_eq!(json["timestamp"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: SubscriptionStatusNotificationEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionStatusNotificationEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Sequencing index of this event
    pub event_number: types::Integer64,
    /// Primitive extension sibling for [`event_number`](Self::event_number) (FHIR `_eventNumber`).
    #[serde(rename = "_eventNumber")]
    pub event_number_ext: Option<types::Element>,

    /// The instant this event occurred
    pub timestamp: Option<types::Instant>,
    /// Primitive extension sibling for [`timestamp`](Self::timestamp) (FHIR `_timestamp`).
    #[serde(rename = "_timestamp")]
    pub timestamp_ext: Option<types::Element>,

    /// Reference to the primary resource or information of this event
    pub focus: Option<types::Reference>,

    /// References related to the focus resource and/or context of this event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_context: Vec<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubscriptionStatus;

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
