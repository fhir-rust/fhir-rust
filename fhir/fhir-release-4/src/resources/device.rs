//! Device
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Device
//!
//! Version: 4.0.1
//!
//! Item used in healthcare
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may
/// be a medical or non-medical device.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device::Device;
/// use fhir::r4::types;
///
/// let value = Device {
///     distinct_identifier: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `distinctIdentifier` is the name this serializes to on the wire.
/// assert_eq!(json["distinctIdentifier"], ::serde_json::json!("abc"));
///
/// let back: Device = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Device {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The reference to the definition for the device
    pub definition: Option<types::Reference>,

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi_carrier: Vec<DeviceUdiCarrier>,

    /// active | inactive | entered-in-error | unknown
    pub status: Option<crate::coded::Coded<crate::r4::codes::DeviceStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// online | paused | standby | offline | not-ready | transduc-discon |
    /// hw-discon | off
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// The distinct identification string
    pub distinct_identifier: Option<types::String>,
    /// Primitive extension sibling for [`distinct_identifier`](Self::distinct_identifier) (FHIR `_distinctIdentifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_distinctIdentifier")]
    pub distinct_identifier_ext: Option<types::Element>,

    /// Name of device manufacturer
    pub manufacturer: Option<types::String>,
    /// Primitive extension sibling for [`manufacturer`](Self::manufacturer) (FHIR `_manufacturer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_manufacturer")]
    pub manufacturer_ext: Option<types::Element>,

    /// Date when the device was made
    pub manufacture_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`manufacture_date`](Self::manufacture_date) (FHIR `_manufactureDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_manufactureDate")]
    pub manufacture_date_ext: Option<types::Element>,

    /// Date and time of expiry of this device (if applicable)
    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// Lot number of manufacture
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// Serial number assigned by the manufacturer
    pub serial_number: Option<types::String>,
    /// Primitive extension sibling for [`serial_number`](Self::serial_number) (FHIR `_serialNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_serialNumber")]
    pub serial_number_ext: Option<types::Element>,

    /// The name of the device as given by the manufacturer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device_name: Vec<DeviceDeviceName>,

    /// The model number for the device
    pub model_number: Option<types::String>,
    /// Primitive extension sibling for [`model_number`](Self::model_number) (FHIR `_modelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modelNumber")]
    pub model_number_ext: Option<types::Element>,

    /// The part number of the device
    pub part_number: Option<types::String>,
    /// Primitive extension sibling for [`part_number`](Self::part_number) (FHIR `_partNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_partNumber")]
    pub part_number_ext: Option<types::Element>,

    /// The kind or type of device
    pub r#type: Option<types::CodeableConcept>,

    /// The capabilities supported on a device, the standards to which the
    /// device conforms for a particular purpose, and used for the
    /// communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialization: Vec<DeviceSpecialization>,

    /// The actual design of the device or software version running on the
    /// device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<DeviceVersion>,

    /// The actual configuration settings of a device as it actually operates,
    /// e.g., regulation status, time properties
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<DeviceProperty>,

    /// Patient to whom Device is affixed
    pub patient: Option<types::Reference>,

    /// Organization responsible for device
    pub owner: Option<types::Reference>,

    /// Details for human/organization for support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// Where the device is found
    pub location: Option<types::Reference>,

    /// Network address to contact device
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Device notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Safety Characteristics of Device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety: Vec<types::CodeableConcept>,

    /// The parent device
    pub parent: Option<types::Reference>,
}

/// This represents the manufacturer's name of the device as provided by the
/// device, from a UDI label, or by a person describing the Device. This
/// typically would be used when a person provides the name(s) or when the
/// device represents one of the names available from DeviceDefinition.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device::DeviceDeviceName;
/// use fhir::r4::types;
///
/// let value = DeviceDeviceName {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDeviceName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The name of the device
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// udi-label-name | user-friendly-name | patient-reported-name |
    /// manufacturer-name | model-name | other
    pub r#type: crate::coded::Coded<crate::r4::codes::DeviceNametype>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// The actual configuration settings of a device as it actually operates,
/// e.g., regulation status, time properties.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device::DeviceProperty;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct DeviceProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that specifies the property DeviceDefinitionPropetyCode
    /// (Extensible)
    pub r#type: types::CodeableConcept,

    /// Property value as a quantity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_quantity: Vec<types::Quantity>,

    /// Property value as a code, e.g., NTP4 (synced to NTP)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_code: Vec<types::CodeableConcept>,
}

/// The capabilities supported on a device, the standards to which the device
/// conforms for a particular purpose, and used for the communication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device::DeviceSpecialization;
/// use fhir::r4::types;
///
/// let value = DeviceSpecialization {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: DeviceSpecialization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceSpecialization {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The standard that is used to operate and communicate
    pub system_type: types::CodeableConcept,

    /// The version of the standard that is used to operate and communicate
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,
}

/// Unique device identifier (UDI) assigned to device label or package. Note
/// that the Device may include multiple udiCarriers as it either may include
/// just the udiCarrier for the jurisdiction it is sold, or for multiple
/// jurisdictions it could have been sold.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device::DeviceUdiCarrier;
/// use fhir::r4::types;
///
/// let value = DeviceUdiCarrier {
///     device_identifier: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `deviceIdentifier` is the name this serializes to on the wire.
/// assert_eq!(json["deviceIdentifier"], ::serde_json::json!("abc"));
///
/// let back: DeviceUdiCarrier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub device_identifier: Option<types::String>,
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// UDI Issuing Organization
    pub issuer: Option<types::Uri>,
    /// Primitive extension sibling for [`issuer`](Self::issuer) (FHIR `_issuer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issuer")]
    pub issuer_ext: Option<types::Element>,

    /// Regional UDI authority
    pub jurisdiction: Option<types::Uri>,
    /// Primitive extension sibling for [`jurisdiction`](Self::jurisdiction) (FHIR `_jurisdiction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_jurisdiction")]
    pub jurisdiction_ext: Option<types::Element>,

    /// UDI Machine Readable Barcode String
    #[serde(rename = "carrierAIDC")]
    pub carrier_aidc: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`carrier_aidc`](Self::carrier_aidc) (FHIR `_carrierAIDC`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_carrierAIDC")]
    pub carrier_aidc_ext: Option<types::Element>,

    /// UDI Human Readable Barcode String
    #[serde(rename = "carrierHRF")]
    pub carrier_hrf: Option<types::String>,
    /// Primitive extension sibling for [`carrier_hrf`](Self::carrier_hrf) (FHIR `_carrierHRF`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_carrierHRF")]
    pub carrier_hrf_ext: Option<types::Element>,

    /// barcode | rfid | manual +
    pub entry_type: Option<crate::coded::Coded<crate::r4::codes::UdiEntryType>>,
    /// Primitive extension sibling for [`entry_type`](Self::entry_type) (FHIR `_entryType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_entryType")]
    pub entry_type_ext: Option<types::Element>,
}

/// The actual design of the device or software version running on the device.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device::DeviceVersion;
/// use fhir::r4::types;
///
/// let value = DeviceVersion {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of the device version
    pub r#type: Option<types::CodeableConcept>,

    /// A single component of the device version
    pub component: Option<types::Identifier>,

    /// The version text
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
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
