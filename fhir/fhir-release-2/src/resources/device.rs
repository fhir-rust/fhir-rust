//! Device
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Device
//!
//!
//!
//! An instance of a manufactured te that is used in the provision of
//! healthcare
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Device Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::device::Device;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Instance id from manufacturer, owner, and others
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// What kind of device this is
    pub r#type: types::CodeableConcept,

    /// Device notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// available | not-available | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r2::codes::Devicestatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Name of device manufacturer
    pub manufacturer: Option<types::String>,
    /// Primitive extension sibling for [`manufacturer`](Self::manufacturer) (FHIR `_manufacturer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_manufacturer")]
    pub manufacturer_ext: Option<types::Element>,

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

    /// Manufacture date
    pub manufacture_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`manufacture_date`](Self::manufacture_date) (FHIR `_manufactureDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_manufactureDate")]
    pub manufacture_date_ext: Option<types::Element>,

    /// Date and time of expiry of this device (if applicable)
    pub expiry: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiry`](Self::expiry) (FHIR `_expiry`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expiry")]
    pub expiry_ext: Option<types::Element>,

    /// FDA mandated Unique Device Identifier
    pub udi: Option<types::String>,
    /// Primitive extension sibling for [`udi`](Self::udi) (FHIR `_udi`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_udi")]
    pub udi_ext: Option<types::Element>,

    /// Lot number of manufacture
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// Organization responsible for device
    pub owner: Option<types::Reference>,

    /// Where the resource is found
    pub location: Option<types::Reference>,

    /// If the resource is affixed to a person
    pub patient: Option<types::Reference>,

    /// Details for human/organization for support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// Network address to contact device
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
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
