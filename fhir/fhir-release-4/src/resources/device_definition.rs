//! DeviceDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
//!
//! Version: 4.0.1
//!
//! An instance of a medical-related component of a medical device
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The characteristics, operational status and capabilities of a
/// medical-related component of a medical device.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device_definition::DeviceDefinition;
/// use fhir::r4::types;
///
/// let value = DeviceDefinition {
///     model_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `modelNumber` is the name this serializes to on the wire.
/// assert_eq!(json["modelNumber"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDefinition {
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

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,

    /// Name of device manufacturer
    /// The `DeviceDefinition.manufacturer[x]` choice element (0..1); see [`DeviceDefinitionManufacturer`].
    #[serde(flatten)]
    pub manufacturer: Option<DeviceDefinitionManufacturer>,

    /// A name given to the device to identify it
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device_name: Vec<DeviceDefinitionDeviceName>,

    /// The model number for the device
    pub model_number: Option<types::String>,
    /// Primitive extension sibling for [`model_number`](Self::model_number) (FHIR `_modelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modelNumber")]
    pub model_number_ext: Option<types::Element>,

    /// What kind of device or device system this is
    pub r#type: Option<types::CodeableConcept>,

    /// The capabilities supported on a device, the standards to which the
    /// device conforms for a particular purpose, and used for the
    /// communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialization: Vec<DeviceDefinitionSpecialization>,

    /// Available versions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version_ext: Vec<Option<types::Element>>,

    /// Safety characteristics of the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety: Vec<types::CodeableConcept>,

    /// Shelf Life and storage information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shelf_life_storage: Vec<types::ProductShelfLife>,

    /// Dimensions, color etc.
    pub physical_characteristics: Option<types::ProdCharacteristic>,

    /// Language code for the human-readable text strings produced by the
    /// device (all supported)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_code: Vec<types::CodeableConcept>,

    /// Device capabilities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capability: Vec<DeviceDefinitionCapability>,

    /// The actual configuration settings of a device as it actually operates,
    /// e.g., regulation status, time properties
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<DeviceDefinitionProperty>,

    /// Organization responsible for device
    pub owner: Option<types::Reference>,

    /// Details for human/organization for support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// Network address to contact device
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Access to on-line information
    pub online_information: Option<types::Uri>,
    /// Primitive extension sibling for [`online_information`](Self::online_information) (FHIR `_onlineInformation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_onlineInformation")]
    pub online_information_ext: Option<types::Element>,

    /// Device notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The quantity of the device present in the packaging (e.g. the number of
    /// devices present in a pack, or the number of devices in the same package
    /// of the medicinal product)
    pub quantity: Option<types::Quantity>,

    /// The parent device it can be part of
    pub parent_device: Option<types::Reference>,

    /// A substance used to create the material(s) of which the device is made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<DeviceDefinitionMaterial>,
}

/// Device capabilities.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device_definition::DeviceDefinitionCapability;
/// use fhir::r4::types;
///
/// let value = DeviceDefinitionCapability {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionCapability = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDefinitionCapability {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of capability
    pub r#type: types::CodeableConcept,

    /// Description of capability
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<types::CodeableConcept>,
}

/// A name given to the device to identify it.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device_definition::DeviceDefinitionDeviceName;
/// use fhir::r4::types;
///
/// let value = DeviceDefinitionDeviceName {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionDeviceName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDefinitionDeviceName {
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

/// A substance used to create the material(s) of which the device is made.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device_definition::DeviceDefinitionMaterial;
/// use fhir::r4::types;
///
/// let value = DeviceDefinitionMaterial {
///     allergenic_indicator: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `allergenicIndicator` is the name this serializes to on the wire.
/// assert_eq!(json["allergenicIndicator"], ::serde_json::json!(true));
///
/// let back: DeviceDefinitionMaterial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDefinitionMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The substance
    pub substance: types::CodeableConcept,

    /// Indicates an alternative material of the device
    pub alternate: Option<types::Boolean>,
    /// Primitive extension sibling for [`alternate`](Self::alternate) (FHIR `_alternate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alternate")]
    pub alternate_ext: Option<types::Element>,

    /// Whether the substance is a known or suspected allergen
    pub allergenic_indicator: Option<types::Boolean>,
    /// Primitive extension sibling for [`allergenic_indicator`](Self::allergenic_indicator) (FHIR `_allergenicIndicator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allergenicIndicator")]
    pub allergenic_indicator_ext: Option<types::Element>,
}

/// The actual configuration settings of a device as it actually operates,
/// e.g., regulation status, time properties.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::device_definition::DeviceDefinitionProperty;
/// use fhir::r4::types;
///
/// let value = DeviceDefinitionProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDefinitionProperty {
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
/// use fhir::r4::resources::device_definition::DeviceDefinitionSpecialization;
/// use fhir::r4::types;
///
/// let value = DeviceDefinitionSpecialization {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionSpecialization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDefinitionSpecialization {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The standard that is used to operate and communicate
    pub system_type: types::String,
    /// Primitive extension sibling for [`system_type`](Self::system_type) (FHIR `_systemType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_systemType")]
    pub system_type_ext: Option<types::Element>,

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
/// use fhir::r4::resources::device_definition::DeviceDefinitionUdiDeviceIdentifier;
/// use fhir::r4::types;
///
/// let value = DeviceDefinitionUdiDeviceIdentifier {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionUdiDeviceIdentifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identifier that is to be associated with every Device that
    /// references this DeviceDefintiion for the issuer and jurisdication
    /// porvided in the DeviceDefinition.udiDeviceIdentifier
    pub device_identifier: types::String,
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// The organization that assigns the identifier algorithm
    pub issuer: types::Uri,
    /// Primitive extension sibling for [`issuer`](Self::issuer) (FHIR `_issuer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issuer")]
    pub issuer_ext: Option<types::Element>,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: types::Uri,
    /// Primitive extension sibling for [`jurisdiction`](Self::jurisdiction) (FHIR `_jurisdiction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_jurisdiction")]
    pub jurisdiction_ext: Option<types::Element>,
}

/// The `DeviceDefinition.manufacturer[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceDefinitionManufacturer {
    /// `manufacturerString` variant.
    #[fhir("manufacturerString")]
    String(crate::r4::choice::Primitive<types::String>),
    /// `manufacturerReference` variant.
    #[fhir("manufacturerReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceDefinition;

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
