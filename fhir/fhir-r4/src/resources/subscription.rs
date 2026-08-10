//! Subscription
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Subscription
//!
//! Version: 4.0.1
//!
//! Server push subscription criteria
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The subscription resource is used to define a push-based subscription from
/// a server to another system. Once a subscription is registered with the
/// server, the server checks every resource that is created or updated, and if
/// the resource matches the given criteria, it sends a message on the defined
/// "channel" so that another system can take an appropriate action.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::subscription::Subscription;
/// use fhir::r4::types;
///
/// let value = Subscription {
///     end: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `end` is the name this serializes to on the wire.
/// assert_eq!(json["end"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Subscription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// requested | active | error | off
    pub status: crate::coded::Coded<crate::r4::codes::SubscriptionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Contact details for source (e.g. troubleshooting)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// When to automatically delete the subscription
    pub end: Option<types::Instant>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Description of why this subscription was created
    pub reason: types::String,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,

    /// Rule for server push
    pub criteria: types::String,
    /// Primitive extension sibling for [`criteria`](Self::criteria) (FHIR `_criteria`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criteria")]
    pub criteria_ext: Option<types::Element>,

    /// Latest error note
    pub error: Option<types::String>,
    /// Primitive extension sibling for [`error`](Self::error) (FHIR `_error`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_error")]
    pub error_ext: Option<types::Element>,

    /// The channel on which to report matches to the criteria
    pub channel: SubscriptionChannel,
}

/// Details where to send notifications when resources are received that meet
/// the criteria.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::subscription::SubscriptionChannel;
/// use fhir::r4::types;
///
/// let value = SubscriptionChannel {
///     endpoint: Some(types::Url("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `endpoint` is the name this serializes to on the wire.
/// assert_eq!(json["endpoint"], ::serde_json::json!("http://example.org"));
///
/// let back: SubscriptionChannel = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SubscriptionChannel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// rest-hook | websocket | email | sms | message
    pub r#type: crate::coded::Coded<crate::r4::codes::SubscriptionChannelType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Where the channel points to
    pub endpoint: Option<types::Url>,
    /// Primitive extension sibling for [`endpoint`](Self::endpoint) (FHIR `_endpoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endpoint")]
    pub endpoint_ext: Option<types::Element>,

    /// MIME type to send, or omit for no payload
    pub payload: Option<types::Code>,
    /// Primitive extension sibling for [`payload`](Self::payload) (FHIR `_payload`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_payload")]
    pub payload_ext: Option<types::Element>,

    /// Usage depends on the channel type
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub header: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`header`](Self::header) (FHIR `_header`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_header")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub header_ext: Vec<Option<types::Element>>,
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
