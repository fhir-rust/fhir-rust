//! MedicinalProductPackaged
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductPackaged
//!
//! Version: 4.0.1
//!
//! A medicinal product in a container or package
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A medicinal product in a container or package.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::medicinal_product_packaged::MedicinalProductPackaged;
///
/// let value = MedicinalProductPackaged::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicinalProductPackaged = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPackaged {
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

    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The product with this is a pack for
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Textual description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The legal status of supply of the medicinal product as classified by
    /// the regulator
    pub legal_status_of_supply: Option<types::CodeableConcept>,

    /// Marketing information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marketing_status: Vec<types::MarketingStatus>,

    /// Manufacturer of this Package Item
    pub marketing_authorization: Option<types::Reference>,

    /// Manufacturer of this Package Item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference>,

    /// Batch numbering
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub batch_identifier: Vec<MedicinalProductPackagedBatchIdentifier>,

    /// A packaging item, as a contained for medicine, possibly with other
    /// packaging items within
    pub package_item: ::vec1::Vec1<MedicinalProductPackagedPackageItem>,
}

/// Batch numbering.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_packaged::MedicinalProductPackagedBatchIdentifier;
/// use fhir::r4::types;
///
/// let value = MedicinalProductPackagedBatchIdentifier {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductPackagedBatchIdentifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPackagedBatchIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A number appearing on the outer packaging of a specific batch
    pub outer_packaging: types::Identifier,

    /// A number appearing on the immediate packaging (and not the outer
    /// packaging)
    pub immediate_packaging: Option<types::Identifier>,
}

/// A packaging item, as a contained for medicine, possibly with other
/// packaging items within.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_packaged::MedicinalProductPackagedPackageItem;
/// use fhir::r4::types;
///
/// let value = MedicinalProductPackagedPackageItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductPackagedPackageItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductPackagedPackageItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Including possibly Data Carrier Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The physical type of the container of the medicine
    pub r#type: types::CodeableConcept,

    /// The quantity of this package in the medicinal product, at the current
    /// level of packaging. The outermost is always 1
    pub quantity: types::Quantity,

    /// Material type of the package item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<types::CodeableConcept>,

    /// A possible alternate material for the packaging
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_material: Vec<types::CodeableConcept>,

    /// A device accompanying a medicinal product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference>,

    /// The manufactured item as contained in the packaged medicinal product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufactured_item: Vec<types::Reference>,

    /// Allows containers within containers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_item: Vec<MedicinalProductPackagedPackageItem>,

    /// Dimensions, color etc.
    pub physical_characteristics: Option<types::ProdCharacteristic>,

    /// Other codeable characteristics
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_characteristics: Vec<types::CodeableConcept>,

    /// Shelf Life and storage information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shelf_life_storage: Vec<types::ProductShelfLife>,

    /// Manufacturer of this Package Item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference>,
}
