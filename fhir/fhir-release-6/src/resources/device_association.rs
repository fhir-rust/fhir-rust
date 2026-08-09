//! DeviceAssociation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceAssociation
//!
//! Version: 6.0.0-ballot3
//!
//! A record of association or dissociation of a device with a patient
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of association of a device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_association::DeviceAssociation;
/// use fhir::r6::types;
///
/// let value = DeviceAssociation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceAssociation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceAssociation {
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

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Reference to the devices associated with the patient or group
    pub device: types::Reference<crate::r6::resources::Device>,

    /// Describes the relationship between the device and subject
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<types::CodeableConcept>,

    /// implanted | explanted | attached | entered-in-error | unknown
    pub status: types::CodeableConcept,

    /// The reasons given for the current association status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// The individual, group of individuals or device that the device is on or
    /// associated with
    pub subject: Option<types::Reference>,

    /// Current anatomical location of the device in/on subject
    pub body_structure: Option<types::Reference<crate::r6::resources::BodyStructure>>,

    /// Begin and end dates and times for the device association
    pub period: Option<types::Period>,

    /// The details about the device when it is in use to describe its
    /// operation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation: Vec<DeviceAssociationOperation>,
}

/// The details about the device when it is in use to describe its operation.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_association::DeviceAssociationOperation;
/// use fhir::r6::types;
///
/// let value = DeviceAssociationOperation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceAssociationOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceAssociationOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Device operational condition
    pub status: types::CodeableConcept,

    /// The individual performing the action enabled by the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operator: Vec<types::Reference>,

    /// Begin and end dates and times for the device's operation
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceAssociation;

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
