//! DeviceRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
//!
//! Version: 5.0.0
//!
//! DeviceRequest Resource: Represents a request a device to be provided to a specific patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Represents a request for a device to be provided to a specific patient.
///
/// The device may be an implantable device to be subsequently implanted, or an
/// external assistive device, such as a walker, to be delivered and subsequently
/// used. A DeviceRequest captures the intent, priority, requested device, subject,
/// timing, and supporting clinical context for the order. In FHIR R5 it is a
/// request workflow resource frequently used to drive device supply and fulfillment.
///
/// Clinically, a DeviceRequest is created by an ordering clinician or system to
/// communicate that a device is needed for a particular patient, and to track that
/// order through its lifecycle (from draft, through active fulfillment, to
/// completion or revocation). It participates in the FHIR request/event workflow
/// pattern alongside resources that report on the resulting event, and it may
/// reference prior requests it fulfills or replaces via `based_on` and `replaces`.
///
/// Related resources: the `subject` is typically a
/// [`Patient`](crate::r5::resources::patient::Patient); the requested item is
/// described via [`CodeableReference`](crate::r5::types::CodeableReference), which
/// may point to a `Device` or `DeviceDefinition`; and detailed dosing/usage-style
/// parameters use [`CodeableConcept`](crate::r5::types::CodeableConcept) and
/// [`Quantity`](crate::r5::types::Quantity) values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_request::DeviceRequest;
/// use fhir::r5::types;
///
/// let value = DeviceRequest {
///     do_not_perform: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doNotPerform` is the name this serializes to on the wire.
/// assert_eq!(json["doNotPerform"], ::serde_json::json!(true));
///
/// let back: DeviceRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Request identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// What request fulfills
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// What request replaces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Identifier of composite request
    pub group_identifier: Option<types::Identifier>,

    /// Status of the request in its lifecycle: draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::RequestStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Indicates the level of authority of the request, e.g. proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: crate::r5::coded::Coded<crate::r5::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if the request is to stop or not to start using the device
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`).
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// The device being requested, as a coded concept and/or reference to a Device or DeviceDefinition
    pub code: types::CodeableReference,

    /// Quantity of devices to supply
    pub quantity: Option<types::Integer>,
    /// Primitive extension sibling for [`quantity`](Self::quantity) (FHIR `_quantity`).
    #[serde(rename = "_quantity")]
    pub quantity_ext: Option<types::Element>,

    /// Device details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<DeviceRequestParameter>,

    /// The patient (or group/location/device) for whom the device is being requested
    pub subject: types::Reference,

    /// Encounter motivating request
    pub encounter: Option<types::Reference>,

    /// The `DeviceRequest.occurrence[x]` choice element (0..1); see [`DeviceRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<DeviceRequestOccurrence>,

    /// When recorded
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// The individual or entity who initiated the request and is responsible for its activation
    pub requester: Option<types::Reference>,

    /// The desired individual or entity to perform the fulfillment of the request
    pub performer: Option<types::CodeableReference>,

    /// Coded/Linked Reason for request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// PRN status of request
    pub as_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`).
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Device usage reason
    pub as_needed_for: Option<types::CodeableConcept>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Additional clinical information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Notes or comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Request provenance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference>,
}

/// Device details.
///
/// Specific parameters for the ordered item, expressed as a coded detail and an
/// associated value.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_request::DeviceRequestParameter;
/// use fhir::r5::types;
///
/// let value = DeviceRequestParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceRequestParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequestParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Device detail
    pub code: Option<types::CodeableConcept>,

    /// The `DeviceRequest.parameter.value[x]` choice element (0..1); see [`DeviceRequestParameterValue`].
    #[serde(flatten)]
    pub value: Option<DeviceRequestParameterValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceRequest;

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
/// The `DeviceRequest.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DeviceRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `DeviceRequest.parameter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DeviceRequestParameterValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
}
