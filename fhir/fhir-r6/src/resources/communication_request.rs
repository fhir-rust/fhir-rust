//! CommunicationRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CommunicationRequest
//!
//! Version: 6.0.0-ballot3
//!
//! A request to convey information from a sender to a recipient
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A request to convey information from a sender to a recipient.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::communication_request::CommunicationRequest;
/// use fhir::r6::types;
///
/// let value = CommunicationRequest {
///     do_not_perform: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doNotPerform` is the name this serializes to on the wire.
/// assert_eq!(json["doNotPerform"], ::serde_json::json!(true));
///
/// let back: CommunicationRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "CommunicationRequestDe")]
#[fhir_version("r6")]
pub struct CommunicationRequest {
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

    /// Fulfills plan or proposal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Request(s) replaced by this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference<crate::r6::resources::CommunicationRequest>>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | on-hold | entered-in-error | ended | completed |
    /// revoked | unknown
    pub status: crate::coded::Coded<crate::r6::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r6::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Message category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if request is prohibiting action
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// A channel of communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medium: Vec<types::CodeableConcept>,

    /// Focus of message
    pub subject: Option<types::Reference>,

    /// Resources that pertain to this communication request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<types::Reference>,

    /// The Encounter during which this CommunicationRequest was created
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Message payload
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<CommunicationRequestPayload>,

    /// When scheduled
    /// The `CommunicationRequest.occurrence[x]` choice element (0..1); see [`CommunicationRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<CommunicationRequestOccurrence>,

    /// When request transitioned to being actionable
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Who asks for the information to be shared
    pub requester: Option<types::Reference>,

    /// Who to share the information with
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,

    /// Who should share the information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_provider: Vec<types::Reference>,

    /// Why is communication needed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Comments made about communication request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommunicationRequestDe {
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
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    #[serde(default)]
    replaces: Vec<types::Reference<crate::r6::resources::CommunicationRequest>>,
    group_identifier: Option<types::Identifier>,
    status: crate::coded::Coded<crate::r6::codes::RequestStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    status_reason: Option<types::CodeableConcept>,
    intent: crate::coded::Coded<crate::r6::codes::RequestIntent>,
    #[serde(rename = "_intent")]
    intent_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    do_not_perform: Option<types::Boolean>,
    #[serde(rename = "_doNotPerform")]
    do_not_perform_ext: Option<types::Element>,
    #[serde(default)]
    medium: Vec<types::CodeableConcept>,
    subject: Option<types::Reference>,
    #[serde(default)]
    about: Vec<types::Reference>,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(default)]
    payload: Vec<CommunicationRequestPayload>,
    #[serde(flatten)]
    occurrence: crate::r6::choice::Slot<CommunicationRequestOccurrence>,
    authored_on: Option<types::DateTime>,
    #[serde(rename = "_authoredOn")]
    authored_on_ext: Option<types::Element>,
    requester: Option<types::Reference>,
    #[serde(default)]
    recipient: Vec<types::Reference>,
    #[serde(default)]
    information_provider: Vec<types::Reference>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<CommunicationRequestDe> for CommunicationRequest {
    fn from(v: CommunicationRequestDe) -> Self {
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
            identifier: v.identifier,
            based_on: v.based_on,
            replaces: v.replaces,
            group_identifier: v.group_identifier,
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            intent: v.intent,
            intent_ext: v.intent_ext,
            category: v.category,
            priority: v.priority,
            priority_ext: v.priority_ext,
            do_not_perform: v.do_not_perform,
            do_not_perform_ext: v.do_not_perform_ext,
            medium: v.medium,
            subject: v.subject,
            about: v.about,
            encounter: v.encounter,
            payload: v.payload,
            occurrence: v.occurrence.0,
            authored_on: v.authored_on,
            authored_on_ext: v.authored_on_ext,
            requester: v.requester,
            recipient: v.recipient,
            information_provider: v.information_provider,
            reason: v.reason,
            note: v.note,
        }
    }
}

/// Text, attachment(s), or resource(s) to be communicated to the recipient.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::communication_request::CommunicationRequestPayload;
/// use fhir::r6::types;
///
/// let value = CommunicationRequestPayload {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CommunicationRequestPayload = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "CommunicationRequestPayloadDe")]
#[fhir_version("r6")]
pub struct CommunicationRequestPayload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Message part content
    /// The `CommunicationRequest.payload.content[x]` choice element (1..1); see [`CommunicationRequestPayloadContent`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub content: Option<CommunicationRequestPayloadContent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommunicationRequestPayloadDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    content: crate::r6::choice::Slot<CommunicationRequestPayloadContent>,
}

impl ::core::convert::From<CommunicationRequestPayloadDe> for CommunicationRequestPayload {
    fn from(v: CommunicationRequestPayloadDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            content: v.content.0,
        }
    }
}

/// The `CommunicationRequest.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CommunicationRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

/// The `CommunicationRequest.payload.content[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CommunicationRequestPayloadContent {
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
    type T = CommunicationRequest;

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
