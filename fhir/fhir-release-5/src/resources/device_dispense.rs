//! DeviceDispense
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceDispense
//!
//! Version: 5.0.0
//!
//! DeviceDispense Resource: Indicates that a device is to be or has been dispensed for a named person/patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Indicates that a device is to be or has been dispensed for a named
/// person/patient. This includes a description of the product (supply)
/// provided and the instructions for using the device. It records the
/// event of dispensing devices such as durable medical equipment or
/// point-of-care supplies to a patient or their caregiver.
///
/// `DeviceDispense` is used in supply chain and patient care workflows to
/// capture the actual handoff of a device, distinct from the `DeviceRequest`
/// that authorizes it and the `DeviceUsage` that records subsequent use.
/// It tracks who dispensed and received the device, the quantity supplied,
/// the preparation and hand-over dates, and any usage instructions, and it
/// can reference the fulfilled order via `based_on` and the encounter during
/// which dispensing occurred.
///
/// See also: [`Patient`](crate::r5::resources::patient::Patient) as the
/// typical `subject` of a dispensation, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) which is used for
/// coded fields such as `category` and `type`. Related resources include
/// `DeviceRequest`, `DeviceUsage`, and `Device` itself (referenced via
/// [`CodeableReference`](crate::r5::types::CodeableReference) in the
/// `device` field).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_dispense::DeviceDispense;
/// use fhir::r5::types;
///
/// let value = DeviceDispense {
///     prepared_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preparedDate` is the name this serializes to on the wire.
/// assert_eq!(json["preparedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceDispense = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDispense {
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

    /// Business identifier for this dispensation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The order or request that this dispense is fulfilling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The bigger event that this dispense is a part of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// The current lifecycle status of the dispense event: preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::DevicedispenseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Why a dispense was or was not performed
    pub status_reason: Option<types::CodeableReference>,

    /// Type of device dispense
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// The specific device (or type of device) that was supplied to the subject.
    pub device: types::CodeableReference,

    /// The person, typically a [`Patient`](crate::r5::resources::patient::Patient), for whom the device is intended.
    pub subject: types::Reference,

    /// Who collected the device or where the medication was delivered
    pub receiver: Option<types::Reference>,

    /// Encounter associated with event
    pub encounter: Option<types::Reference>,

    /// Information that supports the dispensing of the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Who performed event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<DeviceDispensePerformer>,

    /// Where the dispense occurred
    pub location: Option<types::Reference>,

    /// Trial fill, partial fill, emergency fill, etc
    pub r#type: Option<types::CodeableConcept>,

    /// Amount dispensed
    pub quantity: Option<types::Quantity>,

    /// When product was packaged and reviewed
    pub prepared_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`prepared_date`](Self::prepared_date) (FHIR `_preparedDate`).
    #[serde(rename = "_preparedDate")]
    pub prepared_date_ext: Option<types::Element>,

    /// When product was given out
    pub when_handed_over: Option<types::DateTime>,
    /// Primitive extension sibling for [`when_handed_over`](Self::when_handed_over) (FHIR `_whenHandedOver`).
    #[serde(rename = "_whenHandedOver")]
    pub when_handed_over_ext: Option<types::Element>,

    /// Where the device was sent or should be sent
    pub destination: Option<types::Reference>,

    /// Information about the dispense
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Full representation of the usage instructions
    pub usage_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage_instruction`](Self::usage_instruction) (FHIR `_usageInstruction`).
    #[serde(rename = "_usageInstruction")]
    pub usage_instruction_ext: Option<types::Element>,

    /// A list of relevant lifecycle events
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference>,
}

/// Indicates who or what performed the event, and how they were involved
/// in the device dispensing.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_dispense::DeviceDispensePerformer;
/// use fhir::r5::types;
///
/// let value = DeviceDispensePerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDispensePerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who performed the dispense and what they did
    pub function: Option<types::CodeableConcept>,

    /// Individual who was performing
    pub actor: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceDispense;

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
