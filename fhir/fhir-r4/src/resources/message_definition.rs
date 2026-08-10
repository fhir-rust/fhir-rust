//! MessageDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MessageDefinition
//!
//! Version: 4.0.1
//!
//! A resource that defines a type of message that can be exchanged between
//! systems
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Defines the characteristics of a message that can be shared between
/// systems, including the type of event that initiates the message, the
/// content to be transmitted and what response(s), if any, are permitted.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::message_definition::MessageDefinition;
/// use fhir::r4::types;
///
/// let value = MessageDefinition {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: MessageDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MessageDefinitionDe")]
#[fhir_version("r4")]
pub struct MessageDefinition {
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

    /// Business Identifier for a given MessageDefinition
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Primary key for the message definition on a given server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the message definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this message definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this message definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Takes the place of
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub replaces: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`replaces`](Self::replaces) (FHIR `_replaces`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_replaces")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces_ext: Vec<Option<types::Element>>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the message definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Definition this one is based on
    pub base: Option<types::Canonical>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,

    /// Protocol/workflow this is part of
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub parent: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`parent`](Self::parent) (FHIR `_parent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_parent")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_ext: Vec<Option<types::Element>>,

    /// Event code or link to the EventDefinition
    /// The `MessageDefinition.event[x]` choice element (1..1); see [`MessageDefinitionEvent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub event: Option<MessageDefinitionEvent>,

    /// consequence | currency | notification
    pub category: Option<crate::coded::Coded<crate::r4::codes::MessageSignificanceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// Resource(s) that are the subject of the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<MessageDefinitionFocus>,

    /// always | on-error | never | on-success
    pub response_required:
        Option<crate::coded::Coded<crate::r4::codes::MessageheaderResponseRequest>>,
    /// Primitive extension sibling for [`response_required`](Self::response_required) (FHIR `_responseRequired`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_responseRequired")]
    pub response_required_ext: Option<types::Element>,

    /// Responses to this message
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_response: Vec<MessageDefinitionAllowedResponse>,

    /// Canonical reference to a GraphDefinition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub graph: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`graph`](Self::graph) (FHIR `_graph`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_graph")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub graph_ext: Vec<Option<types::Element>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageDefinitionDe {
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
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    #[serde(default)]
    replaces: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_replaces")]
    #[serde(default)]
    replaces_ext: Vec<Option<types::Element>>,
    status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    date: types::DateTime,
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
    base: Option<types::Canonical>,
    #[serde(rename = "_base")]
    base_ext: Option<types::Element>,
    #[serde(default)]
    parent: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_parent")]
    #[serde(default)]
    parent_ext: Vec<Option<types::Element>>,
    #[serde(flatten)]
    event: crate::r4::choice::Slot<MessageDefinitionEvent>,
    category: Option<crate::coded::Coded<crate::r4::codes::MessageSignificanceCategory>>,
    #[serde(rename = "_category")]
    category_ext: Option<types::Element>,
    #[serde(default)]
    focus: Vec<MessageDefinitionFocus>,
    response_required: Option<crate::coded::Coded<crate::r4::codes::MessageheaderResponseRequest>>,
    #[serde(rename = "_responseRequired")]
    response_required_ext: Option<types::Element>,
    #[serde(default)]
    allowed_response: Vec<MessageDefinitionAllowedResponse>,
    #[serde(default)]
    graph: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_graph")]
    #[serde(default)]
    graph_ext: Vec<Option<types::Element>>,
}

impl ::core::convert::From<MessageDefinitionDe> for MessageDefinition {
    fn from(v: MessageDefinitionDe) -> Self {
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
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            replaces: v.replaces,
            replaces_ext: v.replaces_ext,
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
            base: v.base,
            base_ext: v.base_ext,
            parent: v.parent,
            parent_ext: v.parent_ext,
            event: v.event.0,
            category: v.category,
            category_ext: v.category_ext,
            focus: v.focus,
            response_required: v.response_required,
            response_required_ext: v.response_required_ext,
            allowed_response: v.allowed_response,
            graph: v.graph,
            graph_ext: v.graph_ext,
        }
    }
}

/// Indicates what types of messages may be sent as an application-level
/// response to this message.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::message_definition::MessageDefinitionAllowedResponse;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`message`](Self::message) (FHIR `_message`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_message")]
    pub message_ext: Option<types::Element>,

    /// When should this response be used
    pub situation: Option<types::Markdown>,
    /// Primitive extension sibling for [`situation`](Self::situation) (FHIR `_situation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_situation")]
    pub situation_ext: Option<types::Element>,
}

/// Identifies the resource (or resources) that are being addressed by the
/// event. For example, the Encounter for an admit message or two Account
/// records for a merge.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::message_definition::MessageDefinitionFocus;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub code: crate::coded::Coded<crate::r4::codes::ResourceTypes>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Profile that must be adhered to by focus
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,

    /// Minimum number of focuses of this type
    pub min: types::UnsignedInt,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Maximum number of focuses of this type
    pub max: Option<types::String>,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,
}

/// The `MessageDefinition.event[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MessageDefinitionEvent {
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
