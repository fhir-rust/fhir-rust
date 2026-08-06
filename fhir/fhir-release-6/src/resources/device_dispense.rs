//! DeviceDispense
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceDispense
//!
//! Version: 6.0.0-ballot3
//!
//! A record of dispensation of a device
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Indicates that a device is to be or has been dispensed for a named
/// person/patient. This includes a description of the product (supply)
/// provided and the instructions for using the device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_dispense::DeviceDispense;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceDispense {
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

    /// Business identifier for this dispensation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The order or request that this dispense is fulfilling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The bigger event that this dispense is a part of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// preparation | in-progress | cancelled | on-hold | completed |
    /// entered-in-error | stopped | declined | unknown
    pub status: crate::coded::Coded<crate::r6::codes::DevicedispenseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Why a dispense was or was not performed
    pub status_reason: Option<types::CodeableReference>,

    /// Type of device dispense
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// What device was supplied
    pub device: types::CodeableReference,

    /// Who the dispense is for
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
    /// Primitive extension sibling for [`prepared_date`](Self::prepared_date) (FHIR `_preparedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preparedDate")]
    pub prepared_date_ext: Option<types::Element>,

    /// When product was given out
    pub when_handed_over: Option<types::DateTime>,
    /// Primitive extension sibling for [`when_handed_over`](Self::when_handed_over) (FHIR `_whenHandedOver`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_whenHandedOver")]
    pub when_handed_over_ext: Option<types::Element>,

    /// Where the device was sent or should be sent
    pub destination: Option<types::Reference>,

    /// Information about the dispense
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Full representation of the usage instructions
    pub usage_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage_instruction`](Self::usage_instruction) (FHIR `_usageInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usageInstruction")]
    pub usage_instruction_ext: Option<types::Element>,

    /// A list of relevant lifecycle events
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference>,
}

/// Indicates who or what performed the event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_dispense::DeviceDispensePerformer;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
