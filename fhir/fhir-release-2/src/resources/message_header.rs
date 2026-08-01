//! MessageHeader
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MessageHeader
//!
//!
//!
//! A resource that describes a message that is exchanged between systems
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MessageHeader Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::message_header::MessageHeader;
/// use fhir::r2::types;
///
/// let value = MessageHeader {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MessageHeader = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MessageHeader {
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

    /// Time that the message was sent
    pub timestamp: types::Instant,
    /// Primitive extension sibling for [`timestamp`](Self::timestamp) (FHIR `_timestamp`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_timestamp")]
    pub timestamp_ext: Option<types::Element>,

    /// Code for the event this message represents
    pub event: types::Coding,

    /// If this is a reply to prior message
    pub response: Option<MessageHeaderResponse>,

    /// Message Source Application
    pub source: MessageHeaderSource,

    /// Message Destination Application(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<MessageHeaderDestination>,

    /// The source of the data entry
    pub enterer: Option<types::Reference>,

    /// The source of the decision
    pub author: Option<types::Reference>,

    /// Intended "real-world" recipient for the data
    pub receiver: Option<types::Reference>,

    /// Final responsibility for event
    pub responsible: Option<types::Reference>,

    /// Cause of event
    pub reason: Option<types::CodeableConcept>,

    /// The actual content of the message
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<types::Reference>,
}

/// The destination application which the message is intended for.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::message_header::MessageHeaderDestination;
/// use fhir::r2::types;
///
/// let value = MessageHeaderDestination {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: MessageHeaderDestination = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MessageHeaderDestination {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of system
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Particular delivery destination within the destination
    pub target: Option<types::Reference>,

    /// Actual destination address or id
    pub endpoint: types::Uri,
    /// Primitive extension sibling for [`endpoint`](Self::endpoint) (FHIR `_endpoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endpoint")]
    pub endpoint_ext: Option<types::Element>,
}

/// Information about the message that this message is a response to. Only
/// present if this message is a response.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::message_header::MessageHeaderResponse;
/// use fhir::r2::types;
///
/// let value = MessageHeaderResponse {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MessageHeaderResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MessageHeaderResponse {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id of original message
    pub identifier: types::Id,
    /// Primitive extension sibling for [`identifier`](Self::identifier) (FHIR `_identifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_identifier")]
    pub identifier_ext: Option<types::Element>,

    /// ok | transient-error | fatal-error
    pub code: crate::coded::Coded<crate::r2::codes::ResponseCode>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Specific list of hints/warnings/errors
    pub details: Option<types::Reference>,
}

/// The source application from which this message originated.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::message_header::MessageHeaderSource;
/// use fhir::r2::types;
///
/// let value = MessageHeaderSource {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: MessageHeaderSource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MessageHeaderSource {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of system
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name of software running the system
    pub software: Option<types::String>,
    /// Primitive extension sibling for [`software`](Self::software) (FHIR `_software`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_software")]
    pub software_ext: Option<types::Element>,

    /// Version of software running
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Human contact for problems
    pub contact: Option<types::ContactPoint>,

    /// Actual message source address or id
    pub endpoint: types::Uri,
    /// Primitive extension sibling for [`endpoint`](Self::endpoint) (FHIR `_endpoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endpoint")]
    pub endpoint_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MessageHeader;

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
