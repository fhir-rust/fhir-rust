//! Device
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Device
//!
//!
//!
//! Item used in healthcare
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Device Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device::Device;
/// use fhir::r3::types;
///
/// let value = Device {
///     lot_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lotNumber` is the name this serializes to on the wire.
/// assert_eq!(json["lotNumber"], ::serde_json::json!("abc"));
///
/// let back: Device = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Device {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Unique Device Identifier (UDI) Barcode string
    pub udi: Option<DeviceUdi>,

    /// active | inactive | entered-in-error | unknown
    pub status: Option<crate::coded::Coded<crate::r3::codes::DeviceStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// What kind of device this is
    pub r#type: Option<types::CodeableConcept>,

    /// Lot number of manufacture
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

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

    /// Model id assigned by the manufacturer
    pub model: Option<types::String>,
    /// Primitive extension sibling for [`model`](Self::model) (FHIR `_model`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_model")]
    pub model_ext: Option<types::Element>,

    /// Version number (i.e. software)
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Patient to whom Device is affixed
    pub patient: Option<types::Reference<crate::r3::resources::Patient>>,

    /// Organization responsible for device
    pub owner: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Details for human/organization for support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// Where the resource is found
    pub location: Option<types::Reference<crate::r3::resources::Location>>,

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
}

/// [Unique device identifier (UDI)](device.html#5.11.3.2.2) assigned to device
/// label or package.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device::DeviceUdi;
/// use fhir::r3::types;
///
/// let value = DeviceUdi {
///     device_identifier: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `deviceIdentifier` is the name this serializes to on the wire.
/// assert_eq!(json["deviceIdentifier"], ::serde_json::json!("abc"));
///
/// let back: DeviceUdi = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DeviceUdi {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Mandatory fixed portion of UDI
    pub device_identifier: Option<types::String>,
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// Device Name as appears on UDI label
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Regional UDI authority
    pub jurisdiction: Option<types::Uri>,
    /// Primitive extension sibling for [`jurisdiction`](Self::jurisdiction) (FHIR `_jurisdiction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_jurisdiction")]
    pub jurisdiction_ext: Option<types::Element>,

    /// UDI Human Readable Barcode String
    #[serde(rename = "carrierHRF")]
    pub carrier_hrf: Option<types::String>,
    /// Primitive extension sibling for [`carrier_hrf`](Self::carrier_hrf) (FHIR `_carrierHRF`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_carrierHRF")]
    pub carrier_hrf_ext: Option<types::Element>,

    /// UDI Machine Readable Barcode String
    #[serde(rename = "carrierAIDC")]
    pub carrier_aidc: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`carrier_aidc`](Self::carrier_aidc) (FHIR `_carrierAIDC`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_carrierAIDC")]
    pub carrier_aidc_ext: Option<types::Element>,

    /// UDI Issuing Organization
    pub issuer: Option<types::Uri>,
    /// Primitive extension sibling for [`issuer`](Self::issuer) (FHIR `_issuer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issuer")]
    pub issuer_ext: Option<types::Element>,

    /// barcode | rfid | manual +
    pub entry_type: Option<crate::coded::Coded<crate::r3::codes::UdiEntryType>>,
    /// Primitive extension sibling for [`entry_type`](Self::entry_type) (FHIR `_entryType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_entryType")]
    pub entry_type_ext: Option<types::Element>,
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
