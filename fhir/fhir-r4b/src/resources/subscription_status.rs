//! SubscriptionStatus
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubscriptionStatus
//!
//! Version: 4.3.0
//!
//! Status information about a Subscription provided during event notification
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The SubscriptionStatus resource describes the state of a Subscription
/// during notifications.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_status::SubscriptionStatus;
/// use fhir::r4b::types;
///
/// let value = SubscriptionStatus {
///     events_since_subscription_start: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `eventsSinceSubscriptionStart` is the name this serializes to on the wire.
/// assert_eq!(json["eventsSinceSubscriptionStart"], ::serde_json::json!("abc"));
///
/// let back: SubscriptionStatus = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SubscriptionStatus {
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

    /// requested | active | error | off | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r4b::codes::SubscriptionStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// handshake | heartbeat | event-notification | query-status | query-event
    pub r#type: crate::coded::Coded<crate::r4b::codes::SubscriptionNotificationType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Events since the Subscription was created
    pub events_since_subscription_start: Option<types::String>,
    /// Primitive extension sibling for [`events_since_subscription_start`](Self::events_since_subscription_start) (FHIR `_eventsSinceSubscriptionStart`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_eventsSinceSubscriptionStart")]
    pub events_since_subscription_start_ext: Option<types::Element>,

    /// Detailed information about any events relevant to this notification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notification_event: Vec<SubscriptionStatusNotificationEvent>,

    /// Reference to the Subscription responsible for this notification
    pub subscription: types::Reference<crate::r4b::resources::Subscription>,

    /// Reference to the SubscriptionTopic this notification relates to
    pub topic: Option<types::Canonical>,
    /// Primitive extension sibling for [`topic`](Self::topic) (FHIR `_topic`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_topic")]
    pub topic_ext: Option<types::Element>,

    /// List of errors on the subscription
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<types::CodeableConcept>,
}

/// Detailed information about events relevant to this subscription
/// notification.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::subscription_status::SubscriptionStatusNotificationEvent;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct SubscriptionStatusNotificationEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Event number
    pub event_number: types::String,
    /// Primitive extension sibling for [`event_number`](Self::event_number) (FHIR `_eventNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_eventNumber")]
    pub event_number_ext: Option<types::Element>,

    /// The instant this event occurred
    pub timestamp: Option<types::Instant>,
    /// Primitive extension sibling for [`timestamp`](Self::timestamp) (FHIR `_timestamp`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_timestamp")]
    pub timestamp_ext: Option<types::Element>,

    /// The focus of this event
    pub focus: Option<types::Reference>,

    /// Additional context for this event
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
