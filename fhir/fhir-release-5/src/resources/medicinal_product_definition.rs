//! MedicinalProductDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductDefinition
//!
//! Version: 5.0.0
//!
//! MedicinalProductDefinition Resource: Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// MedicinalProductDefinition
///
/// A detailed definition of a medicinal product, typically for uses other than
/// direct patient care such as regulatory submissions, drug catalogs, prescribing
/// support, and adverse event (pharmacovigilance) management. In FHIR R5 it models
/// a medicine as an administrative and regulatory whole: its business identity
/// (including a Medicinal Product Identifier, or MPID), its regulatory type and
/// domain (human or veterinary), its lifecycle status, its names as registered in
/// different countries and jurisdictions, its classification and marketing status,
/// its ingredients and impurities, and the manufacturing or administrative
/// operations and cross-references that relate it to other products. It is the
/// central node of the FHIR medication-definition module and is usually assembled
/// from, and points to, more granular definitional resources rather than being
/// used to record a specific administration or dispense to a person.
///
/// # Related resources
///
/// This resource composes and references several supporting types and resources.
/// It commonly references `Ingredient`, `PackagedProductDefinition`,
/// `AdministrableProductDefinition`, `ManufacturedItemDefinition`, and
/// `RegulatedAuthorization`. Its contacts and manufacturing organizations are
/// typically references to `Organization`, and many of its coded fields use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`CodeableReference`](crate::r5::types::CodeableReference), and
/// [`Coding`](crate::r5::types::Coding).
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinition;
///
/// let value = MedicinalProductDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicinalProductDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinition {
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

    /// Business identifiers for this product, which may include a regulatory Medicinal Product Identifier (MPID)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Regulatory type, e.g. Investigational or Authorized
    pub r#type: Option<types::CodeableConcept>,

    /// If this medicine applies to human or veterinary uses
    pub domain: Option<types::CodeableConcept>,

    /// A business identifier relating to a specific version of the product
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The lifecycle status of this product record, such as active, retired, or pending
    pub status: Option<types::CodeableConcept>,

    /// The date at which the given status became applicable
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`).
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// General description of this product
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The dose form for a single part product, or combined form of a multiple part product
    pub combined_pharmaceutical_dose_form: Option<types::CodeableConcept>,

    /// The path by which the product is taken into or makes contact with the body
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub route: Vec<types::CodeableConcept>,

    /// Description of indication(s) for this product, used when structured indications are not required
    pub indication: Option<types::Markdown>,
    /// Primitive extension sibling for [`indication`](Self::indication) (FHIR `_indication`).
    #[serde(rename = "_indication")]
    pub indication_ext: Option<types::Element>,

    /// The legal status of supply of the medicinal product as classified by the regulator
    pub legal_status_of_supply: Option<types::CodeableConcept>,

    /// Whether the Medicinal Product is subject to additional monitoring for regulatory reasons
    pub additional_monitoring_indicator: Option<types::CodeableConcept>,

    /// Whether the Medicinal Product is subject to special measures for regulatory reasons
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_measures: Vec<types::CodeableConcept>,

    /// If authorised for use in children
    pub pediatric_use_indicator: Option<types::CodeableConcept>,

    /// Allows the product to be classified by various systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<types::CodeableConcept>,

    /// Marketing status of the medicinal product, in contrast to marketing authorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marketing_status: Vec<types::MarketingStatus>,

    /// Package type for the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaged_medicinal_product: Vec<types::CodeableConcept>,

    /// Types of medicinal manufactured items and/or devices that this product consists of, such as tablets, capsule, or syringes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comprised_of: Vec<types::Reference>,

    /// The ingredients of this medicinal product - when not detailed in other resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<types::CodeableConcept>,

    /// Any component of the drug product which is not the chemical entity defined as the drug substance, or an excipient in the drug product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub impurity: Vec<types::CodeableReference>,

    /// Additional documentation about the medicinal product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attached_document: Vec<types::Reference>,

    /// A master file for the medicinal product (e.g. Pharmacovigilance System Master File)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub master_file: Vec<types::Reference>,

    /// A product specific contact, person (in a role), or an organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<MedicinalProductDefinitionContact>,

    /// Clinical trials or studies that this product is involved in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clinical_trial: Vec<types::Reference>,

    /// A code that this product is known by, within some formal terminology
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// The product's registered name or names, including the full name and any coded parts, per country and jurisdiction
    pub name: vec1::Vec1<MedicinalProductDefinitionName>,

    /// Reference to another product, e.g. for linking authorised to investigational product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cross_reference: Vec<MedicinalProductDefinitionCrossReference>,

    /// A manufacturing or administrative process for the medicinal product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation: Vec<MedicinalProductDefinitionOperation>,

    /// Key product features such as "sugar free", "modified release"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<MedicinalProductDefinitionCharacteristic>,
}

/// MedicinalProductDefinition.contact
///
/// A product specific contact, person (in a role), or an organization.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinitionContact;
/// use fhir::r5::types;
///
/// let value = MedicinalProductDefinitionContact {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductDefinitionContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionContact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Allows the contact to be classified, for example QPPV, Pharmacovigilance Enquiry Information
    pub r#type: Option<types::CodeableConcept>,

    /// A product specific contact, person (in a role), or an organization
    pub contact: types::Reference,
}

/// MedicinalProductDefinition.name
///
/// The product's name, including full name and possibly coded parts.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinitionName;
/// use fhir::r5::types;
///
/// let value = MedicinalProductDefinitionName {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductDefinitionName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The full product name
    pub product_name: types::String,
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`).
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Type of product name, such as rINN, BAN, Proprietary, Non-Proprietary
    pub r#type: Option<types::CodeableConcept>,

    /// Coding words or phrases of the name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<MedicinalProductDefinitionNamePart>,

    /// Country and jurisdiction where the name applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<MedicinalProductDefinitionNameUsage>,
}

/// MedicinalProductDefinition.name.part
///
/// Coding words or phrases of the name.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinitionNamePart;
/// use fhir::r5::types;
///
/// let value = MedicinalProductDefinitionNamePart {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductDefinitionNamePart = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionNamePart {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A fragment of a product name
    pub part: types::String,
    /// Primitive extension sibling for [`part`](Self::part) (FHIR `_part`).
    #[serde(rename = "_part")]
    pub part_ext: Option<types::Element>,

    /// Identifying type for this part of the name (e.g. strength part)
    pub r#type: types::CodeableConcept,
}

/// MedicinalProductDefinition.name.usage
///
/// Country and jurisdiction where the name applies.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinitionNameUsage;
/// use fhir::r5::types;
///
/// let value = MedicinalProductDefinitionNameUsage {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductDefinitionNameUsage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionNameUsage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Country code for where this name applies
    pub country: types::CodeableConcept,

    /// Jurisdiction code for where this name applies
    pub jurisdiction: Option<types::CodeableConcept>,

    /// Language code for this name
    pub language: types::CodeableConcept,
}

/// MedicinalProductDefinition.crossReference
///
/// Reference to another product, e.g. for linking authorised to investigational product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinitionCrossReference;
/// use fhir::r5::types;
///
/// let value = MedicinalProductDefinitionCrossReference {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductDefinitionCrossReference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionCrossReference {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to another product, e.g. for linking authorised to investigational product
    pub product: types::CodeableReference,

    /// The type of relationship, for instance branded to generic or virtual to actual product
    pub r#type: Option<types::CodeableConcept>,
}

/// MedicinalProductDefinition.operation
///
/// A manufacturing or administrative process for the medicinal product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinitionOperation;
/// use fhir::r5::types;
///
/// let value = MedicinalProductDefinitionOperation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductDefinitionOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of manufacturing operation e.g. manufacturing itself, re-packaging
    pub r#type: Option<types::CodeableReference>,

    /// Date range of applicability
    pub effective_date: Option<types::Period>,

    /// The organization responsible for the particular process, e.g. the manufacturer or importer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub organization: Vec<types::Reference>,

    /// Specifies whether this process is considered proprietary or confidential
    pub confidentiality_indicator: Option<types::CodeableConcept>,
}

/// MedicinalProductDefinition.characteristic
///
/// Key product features such as "sugar free", "modified release".
/// # Examples
///
/// ```
/// use fhir::r5::resources::medicinal_product_definition::MedicinalProductDefinitionCharacteristic;
/// use fhir::r5::types;
///
/// let value = MedicinalProductDefinitionCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductDefinitionCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductDefinitionCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A code expressing the type of characteristic
    pub r#type: types::CodeableConcept,

    /// The `MedicinalProductDefinition.characteristic.value[x]` choice element (0..1); see [`MedicinalProductDefinitionCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<MedicinalProductDefinitionCharacteristicValue>,
}

/// The `MedicinalProductDefinition.characteristic.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicinalProductDefinitionCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}
