//! MessageHeader
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MessageHeader
//!
//! Version: 4.0.1
//!
//! A resource that describes a message that is exchanged between systems
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The header for a message exchange that is either requesting or responding
/// to an action. The reference(s) that are the subject of the action as well
/// as other information related to the action are typically transmitted in a
/// bundle in which the MessageHeader resource instance is the first resource
/// in the bundle.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::message_header::MessageHeader;
/// use fhir::r4::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MessageHeaderDe")]
#[fhir_version("r4")]
pub struct MessageHeader {
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

    /// Code for the event this message represents or link to event definition
    /// The `MessageHeader.event[x]` choice element (1..1); see [`MessageHeaderEvent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub event: Option<MessageHeaderEvent>,

    /// Message destination application(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<MessageHeaderDestination>,

    /// Real world sender of the message
    pub sender: Option<types::Reference>,

    /// The source of the data entry
    pub enterer: Option<types::Reference>,

    /// The source of the decision
    pub author: Option<types::Reference>,

    /// Message source application
    pub source: MessageHeaderSource,

    /// Final responsibility for event
    pub responsible: Option<types::Reference>,

    /// Cause of event
    pub reason: Option<types::CodeableConcept>,

    /// If this is a reply to prior message
    pub response: Option<MessageHeaderResponse>,

    /// The actual content of the message
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference>,

    /// Link to the definition for this message
    pub definition: Option<types::Canonical>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageHeaderDe {
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
    contained: Vec<crate::r4::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    event: crate::r4::choice::Slot<MessageHeaderEvent>,
    #[serde(default)]
    destination: Vec<MessageHeaderDestination>,
    sender: Option<types::Reference>,
    enterer: Option<types::Reference>,
    author: Option<types::Reference>,
    source: MessageHeaderSource,
    responsible: Option<types::Reference>,
    reason: Option<types::CodeableConcept>,
    response: Option<MessageHeaderResponse>,
    #[serde(default)]
    focus: Vec<types::Reference>,
    definition: Option<types::Canonical>,
    #[serde(rename = "_definition")]
    definition_ext: Option<types::Element>,
}

impl ::core::convert::From<MessageHeaderDe> for MessageHeader {
    fn from(v: MessageHeaderDe) -> Self {
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
            event: v.event.0,
            destination: v.destination,
            sender: v.sender,
            enterer: v.enterer,
            author: v.author,
            source: v.source,
            responsible: v.responsible,
            reason: v.reason,
            response: v.response,
            focus: v.focus,
            definition: v.definition,
            definition_ext: v.definition_ext,
        }
    }
}

/// The destination application which the message is intended for.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::message_header::MessageHeaderDestination;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct MessageHeaderDestination {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of system
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Particular delivery destination within the destination
    pub target: Option<types::Reference<crate::r4::resources::Device>>,

    /// Actual destination address or id
    pub endpoint: types::Url,
    /// Primitive extension sibling for [`endpoint`](Self::endpoint) (FHIR `_endpoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endpoint")]
    pub endpoint_ext: Option<types::Element>,

    /// Intended "real-world" recipient for the data
    pub receiver: Option<types::Reference>,
}

/// Information about the message that this message is a response to. Only
/// present if this message is a response.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::message_header::MessageHeaderResponse;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct MessageHeaderResponse {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id of original message
    pub identifier: types::Id,
    /// Primitive extension sibling for [`identifier`](Self::identifier) (FHIR `_identifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_identifier")]
    pub identifier_ext: Option<types::Element>,

    /// ok | transient-error | fatal-error
    pub code: crate::coded::Coded<crate::r4::codes::ResponseCode>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Specific list of hints/warnings/errors
    pub details: Option<types::Reference<crate::r4::resources::OperationOutcome>>,
}

/// The source application from which this message originated.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::message_header::MessageHeaderSource;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct MessageHeaderSource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    pub endpoint: types::Url,
    /// Primitive extension sibling for [`endpoint`](Self::endpoint) (FHIR `_endpoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endpoint")]
    pub endpoint_ext: Option<types::Element>,
}

/// The `MessageHeader.event[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MessageHeaderEvent {
    /// `eventCoding` variant.
    #[fhir("eventCoding")]
    Coding(Box<types::Coding>),
    /// `eventUri` variant.
    #[fhir("eventUri")]
    Uri(crate::r4::choice::Primitive<types::Uri>),
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
