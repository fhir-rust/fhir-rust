//! DeviceAssociation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceAssociation
//!
//! Version: 5.0.0
//!
//! DeviceAssociation Resource: A record of association of a device.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The DeviceAssociation resource records the association of a device with a
/// patient, group, or other subject over a period of time. It captures the
/// relationship between a physical device and the individual it is implanted in,
/// attached to, or otherwise associated with, along with the current status of
/// that association and its anatomical location. It is commonly used to track
/// implantable, wearable, and attached devices throughout their lifecycle.
///
/// Clinically and administratively, DeviceAssociation supersedes the narrower
/// `Device.patient` and `Device.location` linkages used in earlier FHIR
/// versions by providing a dedicated, time-bounded record that can express
/// multiple concurrent or historical associations for the same device. This
/// supports use cases such as tracking when an implantable device was
/// implanted and later explanted, recording who is operating or wearing a
/// device, and noting the body site where a device is currently located. The
/// `status` element conveys the associations lifecycle (for example,
/// implanted, explanted, or attached), while `operation` can describe periods
/// during which the device was actively in use and by whom.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) or `Group` — typical
///   subjects that a device is associated with.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for the
///   association `status`, `status_reason`, and `category`.
/// - [`Reference`](crate::r5::types::Reference) — used to point to the
///   associated `device`, `subject`, and `body_structure`.
/// - `Device` and `DeviceUsage` — related resources describing the device
///   itself and its usage over time.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_association::DeviceAssociation;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAssociation {
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

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Reference to the device that is the subject of this association
    pub device: types::Reference<crate::r5::resources::Device>,

    /// Describes the relationship between the device and subject, such as parent/child or usage relationships
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// The current lifecycle status of the association: implanted | explanted | attached | entered-in-error | unknown
    pub status: types::CodeableConcept,

    /// The reasons given for the current association status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// The patient, group, or other individual that the device is on or associated with
    pub subject: Option<types::Reference>,

    /// Current anatomical location of the device in/on subject, when applicable
    pub body_structure: Option<types::Reference<crate::r5::resources::BodyStructure>>,

    /// Begin and end dates and times for the device association
    pub period: Option<types::Period>,

    /// The details about the device when it is in use to describe its operation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation: Vec<DeviceAssociationOperation>,
}

/// The details about the device when it is in use to describe its operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_association::DeviceAssociationOperation;
/// use fhir::r5::types;
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
