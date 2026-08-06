//! Communication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Communication
//!
//! Version: 6.0.0-ballot3
//!
//! A clinical or business level record of information being transmitted or
//! shared
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A clinical or business level record of information being transmitted or
/// shared; e.g. an alert that was sent to a responsible provider, a public
/// health agency communication to a provider/reporter in response to a case
/// report for a reportable condition.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::communication::Communication;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct Communication {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Request fulfilled by this communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event (e.g. Communication, Procedure)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Reply to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub in_response_to: Vec<types::Reference>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed |
    /// entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Message category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// A channel of communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medium: Vec<types::CodeableConcept>,

    /// Focus of message
    pub subject: Option<types::Reference>,

    /// Description of the purpose/content
    pub topic: Option<types::CodeableConcept>,

    /// Resources that pertain to this communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<types::Reference>,

    /// The Encounter during which this Communication was created
    pub encounter: Option<types::Reference>,

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

    /// Who the information is shared with
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,

    /// Who shares the information
    pub sender: Option<types::Reference>,

    /// Indication for message
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Message payload
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<CommunicationPayload>,

    /// Comments made about the communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Text, attachment(s), or resource(s) that was communicated to the recipient.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::communication::CommunicationPayload;
/// use fhir::r6::types;
///
/// let value = CommunicationPayload {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CommunicationPayload = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CommunicationPayload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Message part content
    /// The `Communication.payload.content[x]` choice element (1..1); see [`CommunicationPayloadContent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub content: Option<CommunicationPayloadContent>,
}

/// The `Communication.payload.content[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CommunicationPayloadContent {
    /// `contentAttachment` variant.
    #[fhir("contentAttachment")]
    Attachment(Box<types::Attachment>),
    /// `contentReference` variant.
    #[fhir("contentReference")]
    Reference(Box<types::Reference>),
    /// `contentCodeableConcept` variant.
    #[fhir("contentCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
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
