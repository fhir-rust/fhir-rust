//! DeviceDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
//!
//! Version: 5.0.0
//!
//! DeviceDefinition Resource: This is a specialized resource that defines the characteristics and capabilities of a device.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.
///
/// The DeviceDefinition resource describes the "kind" of device, in contrast to
/// the Device resource which represents a specific instance. It captures
/// manufacturer information, identifiers, classifications, versions, properties,
/// materials, packaging, and regulatory details that are common to all instances
/// of that device model. It is used to catalog and reference device models across
/// clinical, administrative, and supply workflows.
///
/// Administratively, DeviceDefinition acts as a master catalog or product entry
/// that manufacturers, distributors, and healthcare organizations can publish and
/// reference so that individual device instances, orders, and regulatory
/// submissions do not need to repeat shared model-level data. Clinically, it
/// helps ensure that the correct kind of device (with its intended use,
/// contraindications, warnings, and conformance to standards) is selected and
/// tracked wherever it is used, for example in medication administration,
/// implant tracking, or supply chain management.
///
/// # See also
///
/// - [`types::Identifier`] for the business identifiers used across device
///   definitions and their UDI entries.
/// - [`types::CodeableConcept`] for coded classifications, properties, and
///   safety characteristics.
/// - [`types::Reference`] for links to related resources such as the owning
///   organization or manufacturer.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinition;
/// use fhir::r5::types;
///
/// let value = DeviceDefinition {
///     part_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `partNumber` is the name this serializes to on the wire.
/// assert_eq!(json["partNumber"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition {
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

    /// Additional narrative information to describe the device model, its intended purpose, and general use
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Business identifier(s) for this device definition, assigned by the manufacturer or a regulatory body
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,

    /// Regulatory identifier(s) associated with this device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_identifier: Vec<DeviceDefinitionRegulatoryIdentifier>,

    /// The part number or catalog number of the device
    pub part_number: Option<types::String>,
    /// Primitive extension sibling for [`part_number`](Self::part_number) (FHIR `_partNumber`).
    #[serde(rename = "_partNumber")]
    pub part_number_ext: Option<types::Element>,

    /// Name of device manufacturer, as a reference to an Organization resource
    pub manufacturer: Option<types::Reference>,

    /// The name or names of the device as given by the manufacturer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device_name: Vec<DeviceDefinitionDeviceName>,

    /// The catalog or model number for the device for example as defined by the manufacturer
    pub model_number: Option<types::String>,
    /// Primitive extension sibling for [`model_number`](Self::model_number) (FHIR `_modelNumber`).
    #[serde(rename = "_modelNumber")]
    pub model_number_ext: Option<types::Element>,

    /// What kind of device or device system this is, e.g. its risk class or device category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<DeviceDefinitionClassification>,

    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conforms_to: Vec<DeviceDefinitionConformsTo>,

    /// A device, part of the current one
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_part: Vec<DeviceDefinitionHasPart>,

    /// Information about the packaging of the device, i.e. how the device is packaged
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaging: Vec<DeviceDefinitionPackaging>,

    /// The version of the device or software
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<DeviceDefinitionVersion>,

    /// Safety characteristics of the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety: Vec<types::CodeableConcept>,

    /// Shelf Life and storage information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shelf_life_storage: Vec<types::ProductShelfLife>,

    /// Language code for the human-readable text strings produced by the device (all supported)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_code: Vec<types::CodeableConcept>,

    /// Inherent, essentially fixed, characteristics of this kind of device, e.g., time properties, size, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<DeviceDefinitionProperty>,

    /// The organization responsible for device, typically the manufacturer or its authorized representative
    pub owner: Option<types::Reference>,

    /// Details for human/organization for support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<DeviceDefinitionLink>,

    /// Device notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// A substance used to create the material(s) of which the device is made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<DeviceDefinitionMaterial>,

    /// lot-number | manufactured-date | serial-number | expiration-date | biological-source | software-version
    #[serde(rename = "productionIdentifierInUDI")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub production_identifier_in_udi:
        Vec<crate::r5::coded::Coded<crate::r5::codes::DeviceProductidentifierinudi>>,
    /// Primitive extension sibling for
    /// [`production_identifier_in_udi`](Self::production_identifier_in_udi)
    /// (FHIR `_productionIdentifierInUDI`).
    #[serde(rename = "_productionIdentifierInUDI")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub production_identifier_in_udi_ext: Vec<Option<types::Element>>,

    /// Information aimed at providing directions for the usage of this model of device
    pub guideline: Option<DeviceDefinitionGuideline>,

    /// Tracking of latest field safety corrective action
    pub corrective_action: Option<DeviceDefinitionCorrectiveAction>,

    /// Billing code or reference associated with the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub charge_item: Vec<DeviceDefinitionChargeItem>,
}

/// Unique Device Identifier (UDI) Barcode string.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionUdiDeviceIdentifier;
/// use fhir::r5::types;
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
pub struct DeviceDefinitionUdiDeviceIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdiction provided in the DeviceDefinition.udiDeviceIdentifier
    pub device_identifier: types::String,
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`).
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// The organization that assigns the identifier algorithm
    pub issuer: types::Uri,
    /// Primitive extension sibling for [`issuer`](Self::issuer) (FHIR `_issuer`).
    #[serde(rename = "_issuer")]
    pub issuer_ext: Option<types::Element>,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: types::Uri,
    /// Primitive extension sibling for [`jurisdiction`](Self::jurisdiction) (FHIR `_jurisdiction`).
    #[serde(rename = "_jurisdiction")]
    pub jurisdiction_ext: Option<types::Element>,

    /// Indicates whether and when the device is available on the market
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub market_distribution: Vec<DeviceDefinitionUdiDeviceIdentifierMarketDistribution>,
}

/// Indicates whether and when the device is available on the market.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionUdiDeviceIdentifierMarketDistribution;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionUdiDeviceIdentifierMarketDistribution {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionUdiDeviceIdentifierMarketDistribution = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionUdiDeviceIdentifierMarketDistribution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Begin and end dates for the commercial distribution of the device
    pub market_period: types::Period,

    /// National state or territory where the device is commercialized
    pub sub_jurisdiction: types::Uri,
    /// Primitive extension sibling for [`sub_jurisdiction`](Self::sub_jurisdiction) (FHIR `_subJurisdiction`).
    #[serde(rename = "_subJurisdiction")]
    pub sub_jurisdiction_ext: Option<types::Element>,
}

/// Regulatory identifier(s) associated with this device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionRegulatoryIdentifier;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionRegulatoryIdentifier {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionRegulatoryIdentifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionRegulatoryIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// basic | master | license
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::DevicedefinitionRegulatoryIdentifierType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The identifier itself
    pub device_identifier: types::String,
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`).
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// The organization that issued this identifier
    pub issuer: types::Uri,
    /// Primitive extension sibling for [`issuer`](Self::issuer) (FHIR `_issuer`).
    #[serde(rename = "_issuer")]
    pub issuer_ext: Option<types::Element>,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: types::Uri,
    /// Primitive extension sibling for [`jurisdiction`](Self::jurisdiction) (FHIR `_jurisdiction`).
    #[serde(rename = "_jurisdiction")]
    pub jurisdiction_ext: Option<types::Element>,
}

/// The name or names of the device as given by the manufacturer.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionDeviceName;
/// use fhir::r5::types;
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
pub struct DeviceDefinitionDeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A name that is used to refer to the device
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// registered-name | user-friendly-name | patient-reported-name
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::DeviceNametype>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// What kind of device or device system this is.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionClassification;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionClassification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionClassification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionClassification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A classification or risk class of the device model
    pub r#type: types::CodeableConcept,

    /// Further information qualifying this classification of the device model
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub justification: Vec<types::RelatedArtifact>,
}

/// Identifies the standards, specifications, or formal guidances for the
/// capabilities supported by the device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionConformsTo;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionConformsTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionConformsTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionConformsTo {
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

    /// Identifies the standard, specification, or formal guidance that the device adheres to the Device Specification type
    pub specification: types::CodeableConcept,

    /// The specific form or variant of the standard, specification or formal guidance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version_ext: Vec<Option<types::Element>>,

    /// Standard, regulation, certification, or guidance website, document, or other publication, or similar, supporting the conformance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::RelatedArtifact>,
}

/// A device, part of the current one.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionHasPart;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionHasPart {
///     count: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `count` is the name this serializes to on the wire.
/// assert_eq!(json["count"], ::serde_json::json!(42));
///
/// let back: DeviceDefinitionHasPart = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionHasPart {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to the part
    pub reference: types::Reference,

    /// Number of occurrences of the part
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`).
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,
}

/// Information about the packaging of the device, i.e. how the device is packaged.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionPackaging;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionPackaging {
///     count: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `count` is the name this serializes to on the wire.
/// assert_eq!(json["count"], ::serde_json::json!(42));
///
/// let back: DeviceDefinitionPackaging = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionPackaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier of the packaged medication
    pub identifier: Option<types::Identifier>,

    /// A code that defines the specific type of packaging
    pub r#type: Option<types::CodeableConcept>,

    /// The number of items contained in the package (devices or sub-packages)
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`).
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// An organization that distributes the packaged device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub distributor: Vec<DeviceDefinitionPackagingDistributor>,

    /// Unique Device Identifier (UDI) Barcode string on the packaging
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,

    /// Allows packages within packages
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaging: Vec<DeviceDefinitionPackaging>,
}

/// An organization that distributes the packaged device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionPackagingDistributor;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionPackagingDistributor {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionPackagingDistributor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionPackagingDistributor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Distributor's human-readable name
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Distributor as an Organization resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organization_reference: Vec<types::Reference>,
}

/// The version of the device or software.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionVersion;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionVersion {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionVersion {
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

    /// The version text
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Inherent, essentially fixed, characteristics of this kind of device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionProperty;
/// use fhir::r5::types;
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
pub struct DeviceDefinitionProperty {
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

    /// The `DeviceDefinition.property.value[x]` choice element (0..1); see [`DeviceDefinitionPropertyValue`].
    #[serde(flatten)]
    pub value: Option<DeviceDefinitionPropertyValue>,
}

/// An associated device, linking a previous or new device model to the focal
/// device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionLink;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionLink {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionLink = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type indicates the relationship of the related device to the device instance
    pub relation: types::Coding,

    /// A reference to the linked device
    pub related_device: types::CodeableReference,
}

/// A substance used to create the material(s) of which the device is made.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionMaterial;
/// use fhir::r5::types;
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
pub struct DeviceDefinitionMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A relevant substance that the device contains, may contain, or is made of
    pub substance: types::CodeableConcept,

    /// Indicates an alternative material of the device
    pub alternate: Option<types::Boolean>,
    /// Primitive extension sibling for [`alternate`](Self::alternate) (FHIR `_alternate`).
    #[serde(rename = "_alternate")]
    pub alternate_ext: Option<types::Element>,

    /// Whether the substance is a known or suspected allergen
    pub allergenic_indicator: Option<types::Boolean>,
    /// Primitive extension sibling for [`allergenic_indicator`](Self::allergenic_indicator) (FHIR `_allergenicIndicator`).
    #[serde(rename = "_allergenicIndicator")]
    pub allergenic_indicator_ext: Option<types::Element>,
}

/// Information aimed at providing directions for the usage of this model of
/// device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionGuideline;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionGuideline {
///     usage_instruction: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `usageInstruction` is the name this serializes to on the wire.
/// assert_eq!(json["usageInstruction"], ::serde_json::json!("# Heading"));
///
/// let back: DeviceDefinitionGuideline = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The circumstances that form the setting for using the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Detailed written and visual directions for the user on how to use the device
    pub usage_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage_instruction`](Self::usage_instruction) (FHIR `_usageInstruction`).
    #[serde(rename = "_usageInstruction")]
    pub usage_instruction_ext: Option<types::Element>,

    /// A source of information or reference for this guideline
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// A clinical condition for which the device was designed to be used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication: Vec<types::CodeableConcept>,

    /// A specific situation when a device should not be used because it may cause harm
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contraindication: Vec<types::CodeableConcept>,

    /// Specific hazard alert information that a user needs to know before using the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warning: Vec<types::CodeableConcept>,

    /// A description of the general purpose or medical use of the device or its function
    pub intended_use: Option<types::String>,
    /// Primitive extension sibling for [`intended_use`](Self::intended_use) (FHIR `_intendedUse`).
    #[serde(rename = "_intendedUse")]
    pub intended_use_ext: Option<types::Element>,
}

/// Tracking of latest field safety corrective action.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionCorrectiveAction;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionCorrectiveAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionCorrectiveAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionCorrectiveAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether the corrective action was a recall
    pub recall: types::Boolean,
    /// Primitive extension sibling for [`recall`](Self::recall) (FHIR `_recall`).
    #[serde(rename = "_recall")]
    pub recall_ext: Option<types::Element>,

    /// model | lot-numbers | serial-numbers
    pub scope: Option<crate::r5::coded::Coded<crate::r5::codes::DeviceCorrectiveactionscope>>,
    /// Primitive extension sibling for [`scope`](Self::scope) (FHIR `_scope`).
    #[serde(rename = "_scope")]
    pub scope_ext: Option<types::Element>,

    /// Start and end dates of the corrective action
    pub period: types::Period,
}

/// Billing code or reference associated with the device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_definition::DeviceDefinitionChargeItem;
/// use fhir::r5::types;
///
/// let value = DeviceDefinitionChargeItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionChargeItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionChargeItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The code or reference for the charge item
    pub charge_item_code: types::CodeableReference,

    /// Coefficient applicable to the billing code
    pub count: types::Quantity,

    /// A specific time period in which this charge item applies
    pub effective_period: Option<types::Period>,

    /// The context to which this charge item applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,
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
/// The `DeviceDefinition.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DeviceDefinitionPropertyValue {
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
