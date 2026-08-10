//! MessageDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MessageDefinition
//!
//! Version: 5.0.0
//!
//! MessageDefinition Resource: Defines the characteristics of a message that can be shared between systems, including the type of event that initiates the message, the content to be transmitted and what response(s), if any, are permitted.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// MessageDefinition defines the characteristics of a message that can be
/// shared between systems, including the type of event that initiates the
/// message, the content to be transmitted, and what response(s), if any, are
/// permitted. In FHIR R5 messaging, information is exchanged as a Bundle whose
/// first entry is a MessageHeader; MessageDefinition is the canonical,
/// conformance-style artifact that describes the expected shape of such an
/// exchange for a particular triggering event. It specifies the event code (or
/// EventDefinition) that initiates the message, the focal resources that form
/// the message payload together with their profiles and cardinality, whether a
/// response is required, and which response messages are allowed in return.
///
/// MessageDefinition is used primarily at design and conformance time rather
/// than at runtime: implementers and integration engines publish and consume
/// MessageDefinitions to advertise, discover, and negotiate the messages that
/// systems can send and receive, and to validate that an actual message
/// conforms to an agreed contract. Being a canonical resource, it carries
/// standard metadata such as url, version, status, and publisher so it can be
/// referenced stably across implementation guides and registries.
///
/// See also: the focal payload details are held in
/// [`MessageDefinitionFocus`], the permitted replies in
/// [`MessageDefinitionAllowedResponse`], event categorization uses
/// [`Coding`](crate::r5::types::Coding), and jurisdictional scope uses
/// [`CodeableConcept`](crate::r5::types::CodeableConcept). Related messaging
/// artifacts include the `MessageHeader`, `EventDefinition`, and
/// `GraphDefinition` resources.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::message_definition::MessageDefinition;
/// use fhir::r5::types;
///
/// let value = MessageDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: MessageDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageDefinition {
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

    /// The cannonical URL for a given MessageDefinition
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business Identifier for a given MessageDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the message definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `MessageDefinition.versionAlgorithm[x]` choice element (0..1); see [`MessageDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<MessageDefinitionVersionAlgorithm>,

    /// Name for this message definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this message definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Takes the place of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Canonical>,
    /// Primitive extension sibling for [`replaces`](Self::replaces) (FHIR `_replaces`).
    #[serde(rename = "_replaces")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces_ext: Vec<Option<types::Element>>,

    /// Publication lifecycle state of this definition: draft, active, retired, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the message definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for message definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this message definition is defined
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

    /// Definition this one is based on
    pub base: Option<types::Canonical>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`).
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,

    /// Protocol/workflow this is part of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<types::Canonical>,
    /// Primitive extension sibling for [`parent`](Self::parent) (FHIR `_parent`).
    #[serde(rename = "_parent")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_ext: Vec<Option<types::Element>>,

    /// The `MessageDefinition.event[x]` choice element (0..1); see [`MessageDefinitionEvent`].
    #[serde(flatten)]
    pub event: Option<MessageDefinitionEvent>,

    /// consequence | currency | notification
    pub category: Option<crate::r5::coded::Coded<crate::r5::codes::MessageSignificanceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`).
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// The focal resources that make up the message payload, with their profiles and cardinality.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<MessageDefinitionFocus>,

    /// always | on-error | never | on-success
    pub response_required:
        Option<crate::r5::coded::Coded<crate::r5::codes::MessageheaderResponseRequest>>,
    /// Primitive extension sibling for [`response_required`](Self::response_required) (FHIR `_responseRequired`).
    #[serde(rename = "_responseRequired")]
    pub response_required_ext: Option<types::Element>,

    /// The response messages that are permitted in reply to this message.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_response: Vec<MessageDefinitionAllowedResponse>,

    /// Canonical reference to a GraphDefinition
    pub graph: Option<types::Canonical>,
    /// Primitive extension sibling for [`graph`](Self::graph) (FHIR `_graph`).
    #[serde(rename = "_graph")]
    pub graph_ext: Option<types::Element>,
}

/// Resource(s) that are the subject of the event, identifying the type of
/// resource and any profile constraints along with cardinality.
/// # Examples
///
/// ```
/// use fhir::r5::resources::message_definition::MessageDefinitionFocus;
/// use fhir::r5::types;
///
/// let value = MessageDefinitionFocus {
///     profile: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `profile` is the name this serializes to on the wire.
/// assert_eq!(json["profile"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: MessageDefinitionFocus = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageDefinitionFocus {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of resource
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Profile that must be adhered to by focus
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,

    /// Minimum number of focuses of this type
    pub min: types::UnsignedInt,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`).
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Maximum number of focuses of this type
    pub max: Option<types::String>,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`).
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,
}

/// Responses to this message, referencing the allowed MessageDefinition
/// response and describing when it should be used.
/// # Examples
///
/// ```
/// use fhir::r5::resources::message_definition::MessageDefinitionAllowedResponse;
/// use fhir::r5::types;
///
/// let value = MessageDefinitionAllowedResponse {
///     situation: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `situation` is the name this serializes to on the wire.
/// assert_eq!(json["situation"], ::serde_json::json!("# Heading"));
///
/// let back: MessageDefinitionAllowedResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageDefinitionAllowedResponse {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to allowed message definition response
    pub message: types::Canonical,
    /// Primitive extension sibling for [`message`](Self::message) (FHIR `_message`).
    #[serde(rename = "_message")]
    pub message_ext: Option<types::Element>,

    /// When should this response be used
    pub situation: Option<types::Markdown>,
    /// Primitive extension sibling for [`situation`](Self::situation) (FHIR `_situation`).
    #[serde(rename = "_situation")]
    pub situation_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MessageDefinition;

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
/// The `MessageDefinition.event[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MessageDefinitionEvent {
    /// `eventCoding` variant.
    #[fhir("eventCoding")]
    Coding(Box<types::Coding>),
    /// `eventUri` variant.
    #[fhir("eventUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
}

/// The `MessageDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MessageDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
