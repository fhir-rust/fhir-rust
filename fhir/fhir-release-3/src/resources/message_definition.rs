//! MessageDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MessageDefinition
//!
//!
//!
//! A resource that defines a type of message that can be exchanged between
//! systems
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MessageDefinition Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::message_definition::MessageDefinition;
/// use fhir::r3::types;
///
/// let value = MessageDefinition {
///     response_required: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `responseRequired` is the name this serializes to on the wire.
/// assert_eq!(json["responseRequired"], ::serde_json::json!(true));
///
/// let back: MessageDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MessageDefinition {
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

    /// Logical URI to reference this message definition (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the message definition
    pub identifier: Option<types::Identifier>,

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

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r3::codes::PublicationStatus>,
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

    /// Date this was last changed
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

    /// Context the content is intended to support
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
    pub base: Option<types::Reference>,

    /// Protocol/workflow this is part of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<types::Reference>,

    /// Takes the place of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Event type
    pub event: types::Coding,

    /// Consequence | Currency | Notification
    pub category: Option<crate::coded::Coded<crate::r3::codes::MessageSignificanceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// Resource(s) that are the subject of the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<MessageDefinitionFocus>,

    /// Is a response required?
    pub response_required: Option<types::Boolean>,
    /// Primitive extension sibling for [`response_required`](Self::response_required) (FHIR `_responseRequired`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_responseRequired")]
    pub response_required_ext: Option<types::Element>,

    /// Responses to this message
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_response: Vec<MessageDefinitionAllowedResponse>,
}

/// Indicates what types of messages may be sent as an application-level
/// response to this message.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::message_definition::MessageDefinitionAllowedResponse;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MessageDefinitionAllowedResponse {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to allowed message definition response
    pub message: types::Reference,

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
/// use fhir::r3::resources::message_definition::MessageDefinitionFocus;
/// use fhir::r3::types;
///
/// let value = MessageDefinitionFocus {
///     min: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `min` is the name this serializes to on the wire.
/// assert_eq!(json["min"], ::serde_json::json!(0));
///
/// let back: MessageDefinitionFocus = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MessageDefinitionFocus {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of resource
    pub code: crate::coded::Coded<crate::r3::codes::ResourceTypes>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Profile that must be adhered to by focus
    pub profile: Option<types::Reference>,

    /// Minimum number of focuses of this type
    pub min: Option<types::UnsignedInt>,
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
