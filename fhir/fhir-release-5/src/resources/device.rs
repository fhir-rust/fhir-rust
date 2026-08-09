//! Device
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Device
//!
//! Version: 5.0.0
//!
//! Device Resource: describes the properties, administration, and type of a physical unit.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity.
///
/// The Device resource describes the regulated, administrative, and type
/// properties of a physical unit such as a knee replacement, blood pressure
/// cuff, or MRI. These values typically do not change much within a given
/// physical unit, for example the serial number, manufacturer name, and model
/// number.
///
/// This resource is used to track individual devices, including implants,
/// durable medical equipment, and diagnostic or therapeutic instruments,
/// throughout their lifecycle: identification via Unique Device Identifier
/// (UDI) barcode, association with an owning organization or physical
/// location, and linkage into clinical workflows such as procedures,
/// observations, and device usage. A `Device` instance may also represent a
/// software-only device, and devices may be linked together via `parent` and
/// `gateway` relationships to model composite or networked equipment.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — a device may be
///   implanted in or used by a patient (typically referenced from other
///   resources such as `DeviceUsage` rather than from `Device` itself).
/// - `DeviceDefinition` — describes the kind/model of device, of which a
///   `Device` instance is a specific realization.
/// - `DeviceMetric` and `DeviceUsage` — reference a `Device` to record
///   measurements produced by, or usage of, the device.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used
///   throughout this resource, e.g. for `category`, `r#type`, and
///   `property`, to represent coded classifications of the device.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device::Device;
/// use fhir::r5::types;
///
/// let value = Device {
///     display_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `displayName` is the name this serializes to on the wire.
/// assert_eq!(json["displayName"], ::serde_json::json!("abc"));
///
/// let back: Device = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Device {
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

    /// Instance identifier, such as a serial number or asset tag assigned by
    /// the manufacturer or owning organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The name used to display by default when the device is referenced
    pub display_name: Option<types::String>,
    /// Primitive extension sibling for [`display_name`](Self::display_name) (FHIR `_displayName`).
    #[serde(rename = "_displayName")]
    pub display_name_ext: Option<types::Element>,

    /// The reference to the definition for the device
    pub definition: Option<types::CodeableReference>,

    /// Unique Device Identifier (UDI) Barcode string, as scanned or otherwise
    /// recorded from the device's UDI carrier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi_carrier: Vec<DeviceUdiCarrier>,

    /// The record status of this Device instance: active | inactive |
    /// entered-in-error
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::DeviceStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// lost | damaged | destroyed | available
    pub availability_status: Option<types::CodeableConcept>,

    /// An identifier that supports traceability to the biological source event
    pub biological_source_event: Option<types::Identifier>,

    /// Name of the organization or individual that manufactured the device
    pub manufacturer: Option<types::String>,
    /// Primitive extension sibling for [`manufacturer`](Self::manufacturer) (FHIR `_manufacturer`).
    #[serde(rename = "_manufacturer")]
    pub manufacturer_ext: Option<types::Element>,

    /// Date when the device was made
    pub manufacture_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`manufacture_date`](Self::manufacture_date) (FHIR `_manufactureDate`).
    #[serde(rename = "_manufactureDate")]
    pub manufacture_date_ext: Option<types::Element>,

    /// Date and time of expiry of this device (if applicable)
    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`).
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// Lot number of manufacture
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`).
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// Serial number assigned by the manufacturer
    pub serial_number: Option<types::String>,
    /// Primitive extension sibling for [`serial_number`](Self::serial_number) (FHIR `_serialNumber`).
    #[serde(rename = "_serialNumber")]
    pub serial_number_ext: Option<types::Element>,

    /// The name or names of the device as known to the manufacturer and/or patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<DeviceName>,

    /// The manufacturer's model number for the device
    pub model_number: Option<types::String>,
    /// Primitive extension sibling for [`model_number`](Self::model_number) (FHIR `_modelNumber`).
    #[serde(rename = "_modelNumber")]
    pub model_number_ext: Option<types::Element>,

    /// The part number or catalog number of the device
    pub part_number: Option<types::String>,
    /// Primitive extension sibling for [`part_number`](Self::part_number) (FHIR `_partNumber`).
    #[serde(rename = "_partNumber")]
    pub part_number_ext: Option<types::Element>,

    /// Indicates a high-level grouping of the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// The kind or type of device, e.g. from a device nomenclature system
    /// such as SNOMED CT or GMDN
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The actual design of the device or software version running on the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<DeviceVersion>,

    /// Identifies the standards, specifications, or formal guidances supported by the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conforms_to: Vec<DeviceConformsTo>,

    /// Inherent, essentially fixed, characteristics of the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<DeviceProperty>,

    /// The designated condition for performing a task
    pub mode: Option<types::CodeableConcept>,

    /// The series of occurrences that repeats during the operation of the device
    pub cycle: Option<types::Count>,

    /// A measurement of time during the device's operation
    pub duration: Option<types::Duration>,

    /// Organization responsible for the device, such as the entity that
    /// owns, procures, or maintains it
    pub owner: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Details for human/organization for support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// Where the device is found
    pub location: Option<types::Reference<crate::r5::resources::Location>>,

    /// Network address to contact device
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Technical endpoints providing access to electronic services provided by the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r5::resources::Endpoint>>,

    /// Linked device acting as a communication/data collector, translator or controller
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gateway: Vec<types::CodeableReference>,

    /// Device notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Safety Characteristics of Device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety: Vec<types::CodeableConcept>,

    /// The higher level or encompassing device that this device is a logical part of
    pub parent: Option<types::Reference<crate::r5::resources::Device>>,
}

/// Unique Device Identifier (UDI) Barcode string.
///
/// Carries the unique device identifier information as encoded on a barcode
/// or RFID tag, including the mandatory device identifier and issuing
/// organization.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device::DeviceUdiCarrier;
/// use fhir::r5::types;
///
/// let value = DeviceUdiCarrier {
///     carrier_aidc: Some(types::Base64Binary("YWJj".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `carrierAIDC` is the name this serializes to on the wire.
/// assert_eq!(json["carrierAIDC"], ::serde_json::json!("YWJj"));
///
/// let back: DeviceUdiCarrier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUdiCarrier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Mandatory fixed portion of UDI
    pub device_identifier: types::String,
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`).
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// UDI Issuing Organization
    pub issuer: types::Uri,
    /// Primitive extension sibling for [`issuer`](Self::issuer) (FHIR `_issuer`).
    #[serde(rename = "_issuer")]
    pub issuer_ext: Option<types::Element>,

    /// Regional UDI authority
    pub jurisdiction: Option<types::Uri>,
    /// Primitive extension sibling for [`jurisdiction`](Self::jurisdiction) (FHIR `_jurisdiction`).
    #[serde(rename = "_jurisdiction")]
    pub jurisdiction_ext: Option<types::Element>,

    /// UDI Machine Readable Barcode String
    #[serde(rename = "carrierAIDC")]
    pub carrier_aidc: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`carrier_aidc`](Self::carrier_aidc) (FHIR `_carrierAIDC`).
    #[serde(rename = "_carrierAIDC")]
    pub carrier_aidc_ext: Option<types::Element>,

    /// UDI Human Readable Barcode String
    #[serde(rename = "carrierHRF")]
    pub carrier_hrf: Option<types::String>,
    /// Primitive extension sibling for [`carrier_hrf`](Self::carrier_hrf) (FHIR `_carrierHRF`).
    #[serde(rename = "_carrierHRF")]
    pub carrier_hrf_ext: Option<types::Element>,

    /// barcode | rfid | manual | card | self-reported | electronic-transmission | unknown
    pub entry_type: Option<crate::r5::coded::Coded<crate::r5::codes::UdiEntryType>>,
    /// Primitive extension sibling for [`entry_type`](Self::entry_type) (FHIR `_entryType`).
    #[serde(rename = "_entryType")]
    pub entry_type_ext: Option<types::Element>,
}

/// The name or names of the device as known to the manufacturer and/or patient.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device::DeviceName;
/// use fhir::r5::types;
///
/// let value = DeviceName {
///     display: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `display` is the name this serializes to on the wire.
/// assert_eq!(json["display"], ::serde_json::json!(true));
///
/// let back: DeviceName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The term that names the device
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// registered-name | user-friendly-name | patient-reported-name
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::DeviceNametype>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The preferred device name
    pub display: Option<types::Boolean>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,
}

/// The actual design of the device or software version running on the device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device::DeviceVersion;
/// use fhir::r5::types;
///
/// let value = DeviceVersion {
///     install_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `installDate` is the name this serializes to on the wire.
/// assert_eq!(json["installDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of the device version, e.g. manufacturer, approved, internal
    pub r#type: Option<types::CodeableConcept>,

    /// The hardware or software module of the device to which the version applies
    pub component: Option<types::Identifier>,

    /// The date the version was installed on the device
    pub install_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`install_date`](Self::install_date) (FHIR `_installDate`).
    #[serde(rename = "_installDate")]
    pub install_date_ext: Option<types::Element>,

    /// The version text
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Identifies the standards, specifications, or formal guidances for the
/// capabilities supported by the device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device::DeviceConformsTo;
/// use fhir::r5::types;
///
/// let value = DeviceConformsTo {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: DeviceConformsTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConformsTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Describes the common type of the standard, specification, or formal guidance
    pub category: Option<types::CodeableConcept>,

    /// Identifies the standard, specification, or formal guidance that the device adheres to
    pub specification: types::CodeableConcept,

    /// Specific form or variant of the standard
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,
}

/// Inherent, essentially fixed, characteristics of the device, e.g. time
/// properties, size, material.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device::DeviceProperty;
/// use fhir::r5::types;
///
/// let value = DeviceProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that specifies the property being represented
    pub r#type: types::CodeableConcept,

    /// The `Device.property.value[x]` choice element (0..1); see [`DevicePropertyValue`].
    #[serde(flatten)]
    pub value: Option<DevicePropertyValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Device;

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
/// The `Device.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DevicePropertyValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}
