//! Communication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Communication
//!
//!
//!
//! A record of information transmitted from a sender to a receiver
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Communication Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::communication::Communication;
/// use fhir::r2::types;
///
/// let value = Communication {
///     sent: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sent` is the name this serializes to on the wire.
/// assert_eq!(json["sent"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Communication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Communication {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Message category
    pub category: Option<types::CodeableConcept>,

    /// Message sender
    pub sender: Option<types::Reference>,

    /// Message recipient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,

    /// Message payload
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<CommunicationPayload>,

    /// A channel of communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medium: Vec<types::CodeableConcept>,

    /// in-progress | completed | suspended | rejected | failed
    pub status: Option<crate::coded::Coded<crate::r2::codes::CommunicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Encounter leading to message
    pub encounter: Option<types::Reference<crate::r2::resources::Encounter>>,

    /// When sent
    pub sent: Option<types::DateTime>,
    /// Primitive extension sibling for [`sent`](Self::sent) (FHIR `_sent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sent")]
    pub sent_ext: Option<types::Element>,

    /// When received
    pub received: Option<types::DateTime>,
    /// Primitive extension sibling for [`received`](Self::received) (FHIR `_received`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_received")]
    pub received_ext: Option<types::Element>,

    /// Indication for message
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Focus of message
    pub subject: Option<types::Reference<crate::r2::resources::Patient>>,

    /// CommunicationRequest producing this message
    pub request_detail: Option<types::Reference<crate::r2::resources::CommunicationRequest>>,
}

/// Text, attachment(s), or resource(s) that was communicated to the recipient.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::communication::CommunicationPayload;
/// use fhir::r2::types;
///
/// let value = CommunicationPayload {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: CommunicationPayload = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct CommunicationPayload {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Message part content
    /// The `Communication.payload.content[x]` choice element (1..1); see [`CommunicationPayloadContent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub content: Option<CommunicationPayloadContent>,
}

/// The `Communication.payload.content[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum CommunicationPayloadContent {
    /// `contentString` variant.
    #[fhir("contentString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Communication;

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
