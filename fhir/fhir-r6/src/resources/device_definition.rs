//! DeviceDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! The definition of a kind of device or device component
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This is a specialized resource that defines the characteristics and
/// capabilities of a device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinition;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DeviceDefinitionDe")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this DeviceDefinition, represented as an
    /// absolute URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the DeviceDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the DeviceDefinition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `DeviceDefinition.versionAlgorithm[x]` choice element (0..1); see [`DeviceDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<DeviceDefinitionVersionAlgorithm>,

    /// Name for this DeviceDefinition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this DeviceDefinition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// The part number or catalog number of the device
    pub part_number: Option<types::String>,
    /// Primitive extension sibling for [`part_number`](Self::part_number) (FHIR `_partNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_partNumber")]
    pub part_number_ext: Option<types::Element>,

    /// Name of device manufacturer
    pub manufacturer: Option<types::Reference<crate::r6::resources::Organization>>,

    /// The catalog or model number for the device for example as defined by
    /// the manufacturer
    pub model_number: Option<types::String>,
    /// Primitive extension sibling for [`model_number`](Self::model_number) (FHIR `_modelNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modelNumber")]
    pub model_number_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// The name of the organization responsible for publishing the definition
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for DeviceDefinition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this DeviceDefinition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,

    /// Regulatory identifier(s) associated with this device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regulatory_identifier: Vec<DeviceDefinitionRegulatoryIdentifier>,

    /// The name or names of the device as given by the manufacturer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device_name: Vec<DeviceDefinitionDeviceName>,

    /// What kind of device or device system this is
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<DeviceDefinitionClassification>,

    /// Identifies the standards, specifications, or formal guidances for the
    /// capabilities supported by the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conforms_to: Vec<DeviceDefinitionConformsTo>,

    /// A device, part of the current one
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_part: Vec<DeviceDefinitionHasPart>,

    /// Information about the packaging of the device, i.e. how the device is
    /// packaged
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaging: Vec<DeviceDefinitionPackaging>,

    /// The version of the device or software
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device_version: Vec<DeviceDefinitionDeviceVersion>,

    /// Safety characteristics of the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub safety: Vec<types::CodeableConcept>,

    /// Shelf Life and storage information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shelf_life_storage: Vec<types::ProductShelfLife>,

    /// Language code for the human-readable text strings produced by the
    /// device (all supported)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_language: Vec<types::Code>,
    /// Primitive extension sibling for [`output_language`](Self::output_language) (FHIR `_outputLanguage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outputLanguage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_language_ext: Vec<Option<types::Element>>,

    /// Inherent, essentially fixed, characteristics of this kind of device,
    /// e.g., time properties, size, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<DeviceDefinitionProperty>,

    /// An associated device, attached to, used with, communicating with or
    /// linking a previous or new device model to the focal device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<DeviceDefinitionLink>,

    /// Device notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// A substance used to create the material(s) of which the device is made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<DeviceDefinitionMaterial>,

    /// lot-number | manufactured-date | serial-number | expiration-date |
    /// biological-source | software-version
    #[serde(rename = "productionIdentifierInUDI")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub production_identifier_in_udi: Vec<types::CodeableConcept>,

    /// Information aimed at providing directions for the usage of this model
    /// of device
    pub guideline: Option<DeviceDefinitionGuideline>,

    /// Tracking of latest field safety corrective action
    pub corrective_action: Option<DeviceDefinitionCorrectiveAction>,

    /// Billing code or reference associated with the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub charge_item: Vec<DeviceDefinitionChargeItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceDefinitionDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r6::choice::Slot<DeviceDefinitionVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    part_number: Option<types::String>,
    #[serde(rename = "_partNumber")]
    part_number_ext: Option<types::Element>,
    manufacturer: Option<types::Reference<crate::r6::resources::Organization>>,
    model_number: Option<types::String>,
    #[serde(rename = "_modelNumber")]
    model_number_ext: Option<types::Element>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    #[serde(default)]
    udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,
    #[serde(default)]
    regulatory_identifier: Vec<DeviceDefinitionRegulatoryIdentifier>,
    #[serde(default)]
    device_name: Vec<DeviceDefinitionDeviceName>,
    #[serde(default)]
    classification: Vec<DeviceDefinitionClassification>,
    #[serde(default)]
    conforms_to: Vec<DeviceDefinitionConformsTo>,
    #[serde(default)]
    has_part: Vec<DeviceDefinitionHasPart>,
    #[serde(default)]
    packaging: Vec<DeviceDefinitionPackaging>,
    #[serde(default)]
    device_version: Vec<DeviceDefinitionDeviceVersion>,
    #[serde(default)]
    safety: Vec<types::CodeableConcept>,
    #[serde(default)]
    shelf_life_storage: Vec<types::ProductShelfLife>,
    #[serde(default)]
    output_language: Vec<types::Code>,
    #[serde(rename = "_outputLanguage")]
    #[serde(default)]
    output_language_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    property: Vec<DeviceDefinitionProperty>,
    #[serde(default)]
    link: Vec<DeviceDefinitionLink>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    material: Vec<DeviceDefinitionMaterial>,
    #[serde(rename = "productionIdentifierInUDI")]
    #[serde(default)]
    production_identifier_in_udi: Vec<types::CodeableConcept>,
    guideline: Option<DeviceDefinitionGuideline>,
    corrective_action: Option<DeviceDefinitionCorrectiveAction>,
    #[serde(default)]
    charge_item: Vec<DeviceDefinitionChargeItem>,
}

impl ::core::convert::From<DeviceDefinitionDe> for DeviceDefinition {
    fn from(v: DeviceDefinitionDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            part_number: v.part_number,
            part_number_ext: v.part_number_ext,
            manufacturer: v.manufacturer,
            model_number: v.model_number,
            model_number_ext: v.model_number_ext,
            date: v.date,
            date_ext: v.date_ext,
            contact: v.contact,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            udi_device_identifier: v.udi_device_identifier,
            regulatory_identifier: v.regulatory_identifier,
            device_name: v.device_name,
            classification: v.classification,
            conforms_to: v.conforms_to,
            has_part: v.has_part,
            packaging: v.packaging,
            device_version: v.device_version,
            safety: v.safety,
            shelf_life_storage: v.shelf_life_storage,
            output_language: v.output_language,
            output_language_ext: v.output_language_ext,
            property: v.property,
            link: v.link,
            note: v.note,
            material: v.material,
            production_identifier_in_udi: v.production_identifier_in_udi,
            guideline: v.guideline,
            corrective_action: v.corrective_action,
            charge_item: v.charge_item,
        }
    }
}

/// Billing code or reference associated with the device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionChargeItem;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// What kind of device or device system this is.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionClassification;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
/// capabilities supported by the device. The device may be certified as
/// conformant to these specifications e.g., communication, performance,
/// process, measurement, or specialization standards.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionConformsTo;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct DeviceDefinitionConformsTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Describes the common type of the standard, specification, or formal
    /// guidance
    pub category: Option<types::CodeableConcept>,

    /// Identifies the standard, specification, or formal guidance that the
    /// device adheres to the Device Specification type
    pub specification: types::CodeableConcept,

    /// The specific form or variant of the standard, specification or formal
    /// guidance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version_ext: Vec<Option<types::Element>>,

    /// Standard, regulation, certification, or guidance website, document, or
    /// other publication, or similar, supporting the conformance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::RelatedArtifact>,
}

/// Tracking of latest field safety corrective action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionCorrectiveAction;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`recall`](Self::recall) (FHIR `_recall`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recall")]
    pub recall_ext: Option<types::Element>,

    /// model | lot-numbers | serial-numbers
    pub scope: Option<crate::coded::Coded<crate::r6::codes::DeviceCorrectiveactionscope>>,
    /// Primitive extension sibling for [`scope`](Self::scope) (FHIR `_scope`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_scope")]
    pub scope_ext: Option<types::Element>,

    /// Start and end dates of the corrective action
    pub period: types::Period,
}

/// The name or names of the device as given by the manufacturer.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionDeviceName;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// registered-name | user-friendly-name | patient-reported-name
    pub r#type: types::CodeableConcept,
}

/// The version of the device or software.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionDeviceVersion;
/// use fhir::r6::types;
///
/// let value = DeviceDefinitionDeviceVersion {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceDefinitionDeviceVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceDefinitionDeviceVersion {
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

    /// The version text
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Information aimed at providing directions for the usage of this model of
/// device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionGuideline;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Detailed written and visual directions for the user on how to use the
    /// device
    pub usage_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage_instruction`](Self::usage_instruction) (FHIR `_usageInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usageInstruction")]
    pub usage_instruction_ext: Option<types::Element>,

    /// A source of information or reference for this guideline
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// A clinical condition for which the device was designed to be used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication: Vec<types::CodeableConcept>,

    /// A specific situation when a device should not be used because it may
    /// cause harm
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contraindication: Vec<types::CodeableConcept>,

    /// Specific hazard alert information that a user needs to know before
    /// using the device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warning: Vec<types::CodeableConcept>,

    /// A description of the general purpose or medical use of the device or
    /// its function
    pub intended_use: Option<types::String>,
    /// Primitive extension sibling for [`intended_use`](Self::intended_use) (FHIR `_intendedUse`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intendedUse")]
    pub intended_use_ext: Option<types::Element>,
}

/// A device that is part (for example a component) of the present device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionHasPart;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub reference: types::Reference<crate::r6::resources::DeviceDefinition>,

    /// Number of occurrences of the part
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,
}

/// An associated device, attached to, used with, communicating with or linking
/// a previous or new device model to the focal device.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionLink;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct DeviceDefinitionLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type indicates the relationship of the related device to the device
    /// instance
    pub relation: types::Coding,

    /// A reference to the linked device
    pub related_device: types::CodeableReference,
}

/// A substance used to create the material(s) of which the device is made.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionMaterial;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct DeviceDefinitionMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A relevant substance that the device contains, may contain, or is made
    /// of
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

/// Information about the packaging of the device, i.e. how the device is
/// packaged.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionPackaging;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
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
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionPackagingDistributor;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Distributor as an Organization resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organization_reference: Vec<types::Reference<crate::r6::resources::Organization>>,
}

/// Static or essentially fixed characteristics or features of this kind of
/// device that are otherwise not captured in more specific attributes, e.g.,
/// time or timing attributes, resolution, accuracy, and physical attributes.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionProperty;
/// use fhir::r6::types;
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
#[serde(from = "DeviceDefinitionPropertyDe")]
#[fhir_version("r6")]
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

    /// Value of the property
    /// The `DeviceDefinition.property.value[x]` choice element (1..1); see [`DeviceDefinitionPropertyValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<DeviceDefinitionPropertyValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceDefinitionPropertyDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<DeviceDefinitionPropertyValue>,
}

impl ::core::convert::From<DeviceDefinitionPropertyDe> for DeviceDefinitionProperty {
    fn from(v: DeviceDefinitionPropertyDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            value: v.value.0,
        }
    }
}

/// Identifier associated with the regulatory documentation (certificates,
/// technical documentation, post-market surveillance documentation and
/// reports) of a set of device models sharing the same intended purpose, risk
/// class and essential design and manufacturing characteristics. One example
/// is the Basic UDI-DI in Europe.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionRegulatoryIdentifier;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub r#type: crate::coded::Coded<crate::r6::codes::DevicedefinitionRegulatoryIdentifierType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The identifier itself
    pub device_identifier: types::String,
    /// Primitive extension sibling for [`device_identifier`](Self::device_identifier) (FHIR `_deviceIdentifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deviceIdentifier")]
    pub device_identifier_ext: Option<types::Element>,

    /// The organization that issued this identifier
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

/// Unique device identifier (UDI) assigned to device label or package. Note
/// that the Device may include multiple udiCarriers as it either may include
/// just the udiCarrier for the jurisdiction it is sold, or for multiple
/// jurisdictions it could have been sold.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionUdiDeviceIdentifier;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// references this DeviceDefintiion for the issuer and jurisdiction
    /// provided in the DeviceDefinition.udiDeviceIdentifier
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

    /// Indicates whether and when the device is available on the market
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub market_distribution: Vec<DeviceDefinitionUdiDeviceIdentifierMarketDistribution>,
}

/// Indicates where and when the device is available on the market.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_definition::DeviceDefinitionUdiDeviceIdentifierMarketDistribution;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`sub_jurisdiction`](Self::sub_jurisdiction) (FHIR `_subJurisdiction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subJurisdiction")]
    pub sub_jurisdiction_ext: Option<types::Element>,
}

/// The `DeviceDefinition.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `DeviceDefinition.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
