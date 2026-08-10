//! MessageHeader
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MessageHeader
//!
//! Version: 5.0.0
//!
//! MessageHeader Resource: The header for a message exchange that is either requesting or responding to an action.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The header for a message exchange that is either requesting or responding
/// to an action. MessageHeader is the anchor of the FHIR messaging paradigm,
/// in which discrete events are exchanged between systems as self-contained
/// units of work. The reference(s) that are the subject of the action, along
/// with any other information related to the action, are transmitted inside a
/// Bundle of type "message" whose first entry is always the MessageHeader
/// resource instance. This resource identifies the triggering event, and
/// conveys the routing, source, destination, sender, author, responsible
/// party, reason, and response context needed to reliably process the message.
///
/// A MessageHeader supports both request messages, which ask a receiving
/// system to perform an action, and response messages, which report the
/// outcome of a prior request via the response element. It is commonly used in
/// clinical and administrative integration scenarios such as notifying an
/// application of an admission, discharge, or transfer event, or acknowledging
/// receipt of an earlier message.
///
/// Related resources: the payload referenced by the focus element is carried
/// alongside this header in the enclosing `Bundle`, and routing frequently
/// points at `Endpoint`, `Organization`, `Device`, or
/// [`Practitioner`](crate::r5::resources::practitioner::Practitioner)
/// resources. The reason for an event is expressed as a
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::message_header::MessageHeader;
/// use fhir::r5::types;
///
/// let value = MessageHeader {
///     definition: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `definition` is the name this serializes to on the wire.
/// assert_eq!(json["definition"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: MessageHeader = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
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

    /// The `MessageHeader.event[x]` choice element (0..1); see [`MessageHeaderEvent`].
    #[serde(flatten)]
    pub event: Option<MessageHeaderEvent>,

    /// Message destination application(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<MessageHeaderDestination>,

    /// Real world sender of the message
    pub sender: Option<types::Reference>,

    /// The source of the decision
    pub author: Option<types::Reference>,

    /// Required description of the source application from which this message originated.
    pub source: MessageHeaderSource,

    /// Final responsibility for event
    pub responsible: Option<types::Reference>,

    /// Cause of event
    pub reason: Option<types::CodeableConcept>,

    /// If this is a reply to prior message
    pub response: Option<MessageHeaderResponse>,

    /// References to the actual subject data of the message, carried elsewhere in the enclosing Bundle.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference>,

    /// Link to the definition for this message
    pub definition: Option<types::Canonical>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,
}

/// Message destination application(s).
///
/// The destination application(s) which the message is intended for, including
/// the actual address or Endpoint resource and the intended real-world
/// recipient for the data.
/// # Examples
///
/// ```
/// use fhir::r5::resources::message_header::MessageHeaderDestination;
/// use fhir::r5::types;
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
pub struct MessageHeaderDestination {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `MessageHeader.destination.endpoint[x]` choice element (0..1); see [`MessageHeaderDestinationEndpoint`].
    #[serde(flatten)]
    pub endpoint: Option<MessageHeaderDestinationEndpoint>,

    /// Name of system
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Particular delivery destination within the destination
    pub target: Option<types::Reference<crate::r5::resources::Device>>,

    /// Intended "real-world" recipient for the data
    pub receiver: Option<types::Reference>,
}

/// Message source application.
///
/// The source application from which this message originated, including the
/// actual address or Endpoint resource, the name and version of the software,
/// and a human contact for problems.
/// # Examples
///
/// ```
/// use fhir::r5::resources::message_header::MessageHeaderSource;
/// use fhir::r5::types;
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
pub struct MessageHeaderSource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `MessageHeader.source.endpoint[x]` choice element (0..1); see [`MessageHeaderSourceEndpoint`].
    #[serde(flatten)]
    pub endpoint: Option<MessageHeaderSourceEndpoint>,

    /// Name of system
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name of software running the system
    pub software: Option<types::String>,
    /// Primitive extension sibling for [`software`](Self::software) (FHIR `_software`).
    #[serde(rename = "_software")]
    pub software_ext: Option<types::Element>,

    /// Version of software running
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Human contact for problems
    pub contact: Option<types::ContactPoint>,
}

/// If this is a reply to prior message.
///
/// Information about the message that this message is a response to, present
/// only if this message is a response. Carries the identifier of the original
/// message, a status code, and an optional reference to details.
/// # Examples
///
/// ```
/// use fhir::r5::resources::message_header::MessageHeaderResponse;
/// use fhir::r5::types;
///
/// let value = MessageHeaderResponse {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MessageHeaderResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeaderResponse {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Bundle.identifier of original message
    pub identifier: types::Identifier,

    /// ok | transient-error | fatal-error
    pub code: crate::r5::coded::Coded<crate::r5::codes::ResponseCode>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Specific list of hints/warnings/errors
    pub details: Option<types::Reference<crate::r5::resources::OperationOutcome>>,
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
/// The `MessageHeader.destination.endpoint[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MessageHeaderDestinationEndpoint {
    /// `endpointUrl` variant.
    #[fhir("endpointUrl")]
    Url(crate::r5::choice::Primitive<types::Url>),
    /// `endpointReference` variant.
    #[fhir("endpointReference")]
    Reference(Box<types::Reference>),
}

/// The `MessageHeader.event[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MessageHeaderEvent {
    /// `eventCoding` variant.
    #[fhir("eventCoding")]
    Coding(Box<types::Coding>),
    /// `eventCanonical` variant.
    #[fhir("eventCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `MessageHeader.source.endpoint[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MessageHeaderSourceEndpoint {
    /// `endpointUrl` variant.
    #[fhir("endpointUrl")]
    Url(crate::r5::choice::Primitive<types::Url>),
    /// `endpointReference` variant.
    #[fhir("endpointReference")]
    Reference(Box<types::Reference>),
}
