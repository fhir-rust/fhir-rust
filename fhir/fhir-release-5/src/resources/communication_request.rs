//! CommunicationRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CommunicationRequest
//!
//! Version: 5.0.0
//!
//! CommunicationRequest Resource: A request to convey information; e.g. the CDS system proposes that an alert be sent to a responsible provider, the CDS system proposes that the public health agency be notified about a reportable condition.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// CommunicationRequest
///
/// A request to convey information; e.g. the CDS system proposes that an alert be
/// sent to a responsible provider, the CDS system proposes that the public health
/// agency be notified about a reportable condition. It records the intent to
/// communicate, who should be involved, and what payload should be conveyed, and is
/// commonly used to schedule or prompt future communications within a workflow.
///
/// Clinically and administratively, `CommunicationRequest` supports order-driven or
/// decision-support-driven messaging: it captures the requester, the intended
/// recipient(s) or information provider(s), the subject the communication concerns,
/// the reason it is needed, and the timing (either a fixed date/time or a period)
/// during which the communication should occur. Its status and intent codes track
/// the request through a typical workflow lifecycle, from `draft` or `proposal`
/// through to `active`, `completed`, or `revoked`. Once the communication has
/// actually taken place, the corresponding event is recorded using the related
/// `Communication` resource, which references the fulfilled request via `based_on`.
///
/// # Related resources
///
/// - `Communication` — records the actual communication event that fulfills this request.
/// - [`Patient`](crate::r5::resources::patient::Patient) — commonly referenced as the `subject` or `about` this request concerns.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for `category`, `medium`, and `status_reason`.
/// - [`Reference`](crate::r5::types::Reference) — used for `requester`, `recipient`, and `information_provider`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::communication_request::CommunicationRequest;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationRequest {
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

    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Fulfills plan or proposal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Request(s) replaced by this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// The current lifecycle status of the request: draft | active | on-hold | revoked | completed | entered-in-error | unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Indicates the level of authority the request carries in the workflow, e.g. proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option.
    pub intent: crate::r5::coded::Coded<crate::r5::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Message category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if request is prohibiting action
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`).
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// A channel of communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medium: Vec<types::CodeableConcept>,

    /// The patient or group that is the focus of this communication request.
    pub subject: Option<types::Reference>,

    /// Resources that pertain to this communication request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub about: Vec<types::Reference>,

    /// The Encounter during which this CommunicationRequest was created
    pub encounter: Option<types::Reference>,

    /// Message payload
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<CommunicationRequestPayload>,

    /// The `CommunicationRequest.occurrence[x]` choice element (0..1); see [`CommunicationRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<CommunicationRequestOccurrence>,

    /// When request transitioned to being actionable
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
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

/// CommunicationRequestPayload
///
/// Text, attachment(s), or resource(s) to be communicated to the recipient.
/// # Examples
///
/// ```
/// use fhir::r5::resources::communication_request::CommunicationRequestPayload;
/// use fhir::r5::types;
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
pub struct CommunicationRequestPayload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `CommunicationRequest.payload.content[x]` choice element (0..1); see [`CommunicationRequestPayloadContent`].
    #[serde(flatten)]
    pub content: Option<CommunicationRequestPayloadContent>,
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
/// The `CommunicationRequest.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CommunicationRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

/// The `CommunicationRequest.payload.content[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
