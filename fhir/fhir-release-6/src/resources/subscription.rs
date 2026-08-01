//! Subscription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Subscription
//!
//! Version: 6.0.0-ballot3
//!
//! Notification about a SubscriptionTopic
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The subscription resource describes a particular client's request to be
/// notified about a SubscriptionTopic.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::subscription::Subscription;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Subscription {
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// requested | active | error | off | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::SubscriptionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reference to the subscription topic being subscribed to
    pub topic: types::Canonical,
    /// Primitive extension sibling for [`topic`](Self::topic) (FHIR `_topic`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_topic")]
    pub topic_ext: Option<types::Element>,

    /// Contact details for source (e.g. troubleshooting)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// When to automatically delete the subscription
    pub end: Option<types::Instant>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Entity responsible for Subscription changes
    pub managing_entity: Option<types::Reference>,

    /// Description of why this subscription was created
    pub reason: Option<types::String>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,

    /// Criteria for narrowing the subscription topic stream
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter_by: Vec<SubscriptionFilterBy>,

    /// Channel type for notifications
    pub channel_type: types::Coding,

    /// Where the channel points to
    pub endpoint: Option<types::Url>,
    /// Primitive extension sibling for [`endpoint`](Self::endpoint) (FHIR `_endpoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endpoint")]
    pub endpoint_ext: Option<types::Element>,

    /// Channel type dependent information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<SubscriptionParameter>,

    /// Interval in seconds to send 'heartbeat' notification
    pub heartbeat_period: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`heartbeat_period`](Self::heartbeat_period) (FHIR `_heartbeatPeriod`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_heartbeatPeriod")]
    pub heartbeat_period_ext: Option<types::Element>,

    /// Timeout in seconds to attempt notification delivery
    pub timeout: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`timeout`](Self::timeout) (FHIR `_timeout`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_timeout")]
    pub timeout_ext: Option<types::Element>,

    /// MIME type to send, or omit for no payload
    pub content_type: Option<types::Code>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// empty | id-only | full-resource
    pub content: Option<crate::coded::Coded<crate::r6::codes::SubscriptionPayloadContent>>,
    /// Primitive extension sibling for [`content`](Self::content) (FHIR `_content`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_content")]
    pub content_ext: Option<types::Element>,

    /// Maximum number of events that can be combined in a single notification
    pub max_count: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`max_count`](Self::max_count) (FHIR `_maxCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_maxCount")]
    pub max_count_ext: Option<types::Element>,
}

/// The filter properties to be applied to narrow the subscription topic
/// stream. When multiple filters are applied, evaluates to true if all the
/// conditions applicable to that resource are met; otherwise it returns false
/// (i.e., logical AND).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::subscription::SubscriptionFilterBy;
/// use fhir::r6::types;
///
/// let value = SubscriptionFilterBy {
///     resource: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `resource` is the name this serializes to on the wire.
/// assert_eq!(json["resource"], ::serde_json::json!("http://example.org"));
///
/// let back: SubscriptionFilterBy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub resource: Option<types::Uri>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// Filter label defined in SubscriptionTopic
    pub filter_parameter: types::String,
    /// Primitive extension sibling for [`filter_parameter`](Self::filter_parameter) (FHIR `_filterParameter`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_filterParameter")]
    pub filter_parameter_ext: Option<types::Element>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<crate::coded::Coded<crate::r6::codes::SearchComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comparator")]
    pub comparator_ext: Option<types::Element>,

    /// missing | exact | contains | not | text | in | not-in | below | above |
    /// type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<crate::coded::Coded<crate::r6::codes::SearchModifierCode>>,
    /// Primitive extension sibling for [`modifier`](Self::modifier) (FHIR `_modifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modifier")]
    pub modifier_ext: Option<types::Element>,

    /// Literal value or resource path
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Event to filter by
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<types::CodeableConcept>,
}

/// Channel-dependent information to send as part of the notification (e.g.,
/// HTTP Headers).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::subscription::SubscriptionParameter;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Value of the parameter to use or pass through
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
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
