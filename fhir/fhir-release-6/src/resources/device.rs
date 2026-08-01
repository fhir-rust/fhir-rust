//! Device
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Device
//!
//! Version: 6.0.0-ballot3
//!
//! Item used in healthcare
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource describes the properties (regulated, has real time clock,
/// etc.), administrative (manufacturer name, model number, serial number,
/// firmware, etc.), and type (knee replacement, blood pressure cuff, MRI,
/// etc.) of a physical unit (these values do not change much within a given
/// module, for example the serial number, manufacturer name, and model
/// number). An actual unit may consist of several modules in a distinct
/// hierarchy and these are represented by multiple Device resources and bound
/// through the 'parent' element.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device::Device;
/// use fhir::r6::types;
///
/// let value = Device {
///     manufacture_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `manufactureDate` is the name this serializes to on the wire.
/// assert_eq!(json["manufactureDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Device = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<::serde_json::Value>,

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

    /// Unique Device Identifier (UDI) value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi_carrier: Vec<DeviceUdiCarrier>,

    /// active | inactive | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r6::codes::DeviceStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// lost | damaged | destroyed | available
    pub availability_status: Option<types::CodeableConcept>,

    /// A production identifier of the donation, collection, or pooling event
    /// from which biological material in this device was derived
    pub biological_source_event: Option<types::Identifier>,

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

    /// The name or names of the device as known to the manufacturer and/or
    /// patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<DeviceName>,

    /// The manufacturer's model number for the device
    pub model_number: Option<types::String>,
    /// Primitive extension sibling for [`model_number`](Self::model_number) (FHIR `_modelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modelNumber")]
    pub model_number_ext: Option<types::Element>,

    /// The part number or catalog number of the device
    pub part_number: Option<types::String>,
    /// Primitive extension sibling for [`part_number`](Self::part_number) (FHIR `_partNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_partNumber")]
    pub part_number_ext: Option<types::Element>,

    /// Indicates a high-level grouping of the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// The kind or type of device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The actual design of the device or software version running on the
    /// device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device_version: Vec<DeviceDeviceVersion>,

    /// Identifies the standards, specifications, or formal guidances for the
    /// capabilities supported by the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conforms_to: Vec<DeviceConformsTo>,

    /// Inherent, essentially fixed, characteristics of the device. e.g., time
    /// properties, size, material, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<DeviceProperty>,

    /// Material added to a container device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<DeviceAdditive>,

    /// Details for human/organization for support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// Where the device is found
    pub location: Option<types::Reference>,

    /// Device notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Safety Characteristics of Device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety: Vec<types::CodeableConcept>,

    /// The higher level or encompassing device that this device is a logical
    /// part of
    pub parent: Option<types::Reference>,
}

/// Material added to a container device (typically used in specimen collection
/// or initial processing). The material may be added by the device
/// manufacturer or by a different party subsequent to manufacturing.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device::DeviceAdditive;
/// use fhir::r6::types;
///
/// let value = DeviceAdditive {
///     performed: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `performed` is the name this serializes to on the wire.
/// assert_eq!(json["performed"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceAdditive = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The additive substance
    pub r#type: types::CodeableReference,

    /// Quantity of additive substance within container
    pub quantity: Option<types::Quantity>,

    /// Entity adding substance to the container
    pub performer: Option<types::Reference>,

    /// When the additive substance was added to the container
    pub performed: Option<types::DateTime>,
    /// Primitive extension sibling for [`performed`](Self::performed) (FHIR `_performed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_performed")]
    pub performed_ext: Option<types::Element>,
}

/// Identifies the standards, specifications, or formal guidances for the
/// capabilities supported by the device. The device may be certified as
/// conformant to these specifications e.g., communication, performance,
/// process, measurement, or specialization standards.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device::DeviceConformsTo;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct DeviceConformsTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Describes the common type of the standard, specification, or formal
    /// guidance. communication | performance | measurement
    pub category: Option<types::CodeableConcept>,

    /// Identifies the standard, specification, or formal guidance that the
    /// device adheres to
    pub specification: types::CodeableConcept,

    /// Specific form or variant of the standard
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,
}

/// The actual design of the device or software version running on the device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device::DeviceDeviceVersion;
/// use fhir::r6::types;
///
/// let value = DeviceDeviceVersion {
///     install_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `installDate` is the name this serializes to on the wire.
/// assert_eq!(json["installDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceDeviceVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceDeviceVersion {
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

    /// The hardware or software module of the device to which the version
    /// applies
    pub component: Option<types::Identifier>,

    /// The date the version was installed on the device
    pub install_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`install_date`](Self::install_date) (FHIR `_installDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_installDate")]
    pub install_date_ext: Option<types::Element>,

    /// The version text
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// This represents the manufacturer's name of the device as provided by the
/// device, from a UDI label, or by a person describing the Device. This
/// typically would be used when a person provides the name(s) or when the
/// device represents one of the names available from DeviceDefinition.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device::DeviceName;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// registered-name | user-friendly-name | patient-reported-name
    pub r#type: types::CodeableConcept,

    /// The preferred device name
    pub display: Option<types::Boolean>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,
}

/// Static or essentially fixed characteristics or features of the device
/// (e.g., time or timing attributes, resolution, accuracy, intended use or
/// instructions for use, and physical attributes) that are not otherwise
/// captured in more specific attributes.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device::DeviceProperty;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Value of the property
    /// The `Device.property.value[x]` choice element (1..1); see [`DevicePropertyValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<DevicePropertyValue>,
}

/// Unique Device Identifier (UDI) placed on a device label or package. Note
/// that the Device may include multiple UDIs if it is sold in multiple
/// jurisdictions.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device::DeviceUdiCarrier;
/// use fhir::r6::types;
///
/// let value = DeviceUdiCarrier {
///     device_identifier_system: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `deviceIdentifierSystem` is the name this serializes to on the wire.
/// assert_eq!(json["deviceIdentifierSystem"], ::serde_json::json!("http://example.org"));
///
/// let back: DeviceUdiCarrier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// The namespace for the device identifier value
    pub device_identifier_system: Option<types::Uri>,
    /// Primitive extension sibling for [`device_identifier_system`](Self::device_identifier_system) (FHIR `_deviceIdentifierSystem`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deviceIdentifierSystem")]
    pub device_identifier_system_ext: Option<types::Element>,

    /// UDI Issuing Organization
    pub issuer: types::Uri,
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

    /// UDI Machine Readable value
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

    /// barcode | rfid | manual | card | self-reported |
    /// electronic-transmission | unknown
    pub entry_type: Option<crate::coded::Coded<crate::r6::codes::UdiEntryType>>,
    /// Primitive extension sibling for [`entry_type`](Self::entry_type) (FHIR `_entryType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_entryType")]
    pub entry_type_ext: Option<types::Element>,
}

/// The `Device.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
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
