//! MedicinalProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProduct
//!
//! Version: 4.0.1
//!
//! Detailed definition of a medicinal product, typically for uses other than
//! direct patient care (e.g. regulatory use)
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Detailed definition of a medicinal product, typically for uses other than
/// direct patient care (e.g. regulatory use).
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::medicinal_product::MedicinalProduct;
///
/// let value = MedicinalProduct::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MedicinalProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProduct {
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

    /// Business identifier for this product. Could be an MPID
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Regulatory type, e.g. Investigational or Authorized
    pub r#type: Option<types::CodeableConcept>,

    /// If this medicine applies to human or veterinary uses
    pub domain: Option<types::Coding>,

    /// The dose form for a single part product, or combined form of a multiple
    /// part product
    pub combined_pharmaceutical_dose_form: Option<types::CodeableConcept>,

    /// The legal status of supply of the medicinal product as classified by
    /// the regulator
    pub legal_status_of_supply: Option<types::CodeableConcept>,

    /// Whether the Medicinal Product is subject to additional monitoring for
    /// regulatory reasons
    pub additional_monitoring_indicator: Option<types::CodeableConcept>,

    /// Whether the Medicinal Product is subject to special measures for
    /// regulatory reasons
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_measures: Vec<types::String>,
    /// Primitive extension sibling for [`special_measures`](Self::special_measures) (FHIR `_specialMeasures`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_specialMeasures")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_measures_ext: Vec<Option<types::Element>>,

    /// If authorised for use in children
    pub paediatric_use_indicator: Option<types::CodeableConcept>,

    /// Allows the product to be classified by various systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product_classification: Vec<types::CodeableConcept>,

    /// Marketing status of the medicinal product, in contrast to marketing
    /// authorizaton
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marketing_status: Vec<types::MarketingStatus>,

    /// Pharmaceutical aspects of product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pharmaceutical_product:
        Vec<types::Reference<crate::r4::resources::MedicinalProductPharmaceutical>>,

    /// Package representation for the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaged_medicinal_product:
        Vec<types::Reference<crate::r4::resources::MedicinalProductPackaged>>,

    /// Supporting documentation, typically for regulatory submission
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attached_document: Vec<types::Reference<crate::r4::resources::DocumentReference>>,

    /// A master file for to the medicinal product (e.g. Pharmacovigilance
    /// System Master File)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub master_file: Vec<types::Reference<crate::r4::resources::DocumentReference>>,

    /// A product specific contact, person (in a role), or an organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::Reference>,

    /// Clinical trials or studies that this product is involved in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clinical_trial: Vec<types::Reference<crate::r4::resources::ResearchStudy>>,

    /// The product's name, including full name and possibly coded parts
    pub name: ::vec1::Vec1<MedicinalProductName>,

    /// Reference to another product, e.g. for linking authorised to
    /// investigational product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cross_reference: Vec<types::Identifier>,

    /// An operation applied to the product, for manufacturing or
    /// adminsitrative purpose
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturing_business_operation: Vec<MedicinalProductManufacturingBusinessOperation>,

    /// Indicates if the medicinal product has an orphan designation for the
    /// treatment of a rare disease
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_designation: Vec<MedicinalProductSpecialDesignation>,
}

/// An operation applied to the product, for manufacturing or adminsitrative
/// purpose.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product::MedicinalProductManufacturingBusinessOperation;
/// use fhir::r4::types;
///
/// let value = MedicinalProductManufacturingBusinessOperation {
///     effective_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `effectiveDate` is the name this serializes to on the wire.
/// assert_eq!(json["effectiveDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicinalProductManufacturingBusinessOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductManufacturingBusinessOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of manufacturing operation
    pub operation_type: Option<types::CodeableConcept>,

    /// Regulatory authorization reference number
    pub authorisation_reference_number: Option<types::Identifier>,

    /// Regulatory authorization date
    pub effective_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`effective_date`](Self::effective_date) (FHIR `_effectiveDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_effectiveDate")]
    pub effective_date_ext: Option<types::Element>,

    /// To indicate if this proces is commercially confidential
    pub confidentiality_indicator: Option<types::CodeableConcept>,

    /// The manufacturer or establishment associated with the process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference<crate::r4::resources::Organization>>,

    /// A regulator which oversees the operation
    pub regulator: Option<types::Reference<crate::r4::resources::Organization>>,
}

/// The product's name, including full name and possibly coded parts.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product::MedicinalProductName;
/// use fhir::r4::types;
///
/// let value = MedicinalProductName {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductName {
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
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Coding words or phrases of the name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name_part: Vec<MedicinalProductNameNamePart>,

    /// Country where the name applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_language: Vec<MedicinalProductNameCountryLanguage>,
}

/// Country where the name applies.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product::MedicinalProductNameCountryLanguage;
/// use fhir::r4::types;
///
/// let value = MedicinalProductNameCountryLanguage {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductNameCountryLanguage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductNameCountryLanguage {
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

/// Coding words or phrases of the name.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product::MedicinalProductNameNamePart;
/// use fhir::r4::types;
///
/// let value = MedicinalProductNameNamePart {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductNameNamePart = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductNameNamePart {
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
    /// Primitive extension sibling for [`part`](Self::part) (FHIR `_part`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_part")]
    pub part_ext: Option<types::Element>,

    /// Idenifying type for this part of the name (e.g. strength part)
    pub r#type: types::Coding,
}

/// Indicates if the medicinal product has an orphan designation for the
/// treatment of a rare disease.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product::MedicinalProductSpecialDesignation;
/// use fhir::r4::types;
///
/// let value = MedicinalProductSpecialDesignation {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicinalProductSpecialDesignation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MedicinalProductSpecialDesignationDe")]
#[fhir_version("r4")]
pub struct MedicinalProductSpecialDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier for the designation, or procedure number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The type of special designation, e.g. orphan drug, minor use
    pub r#type: Option<types::CodeableConcept>,

    /// The intended use of the product, e.g. prevention, treatment
    pub intended_use: Option<types::CodeableConcept>,

    /// Condition for which the medicinal use applies
    /// The `MedicinalProduct.specialDesignation.indication[x]` choice element (0..1); see [`MedicinalProductSpecialDesignationIndication`].
    #[serde(flatten)]
    pub indication: Option<MedicinalProductSpecialDesignationIndication>,

    /// For example granted, pending, expired or withdrawn
    pub status: Option<types::CodeableConcept>,

    /// Date when the designation was granted
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Animal species for which this applies
    pub species: Option<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicinalProductSpecialDesignationDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    r#type: Option<types::CodeableConcept>,
    intended_use: Option<types::CodeableConcept>,
    #[serde(flatten)]
    indication: crate::r4::choice::Slot<MedicinalProductSpecialDesignationIndication>,
    status: Option<types::CodeableConcept>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    species: Option<types::CodeableConcept>,
}

impl ::core::convert::From<MedicinalProductSpecialDesignationDe>
    for MedicinalProductSpecialDesignation
{
    fn from(v: MedicinalProductSpecialDesignationDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            r#type: v.r#type,
            intended_use: v.intended_use,
            indication: v.indication.0,
            status: v.status,
            date: v.date,
            date_ext: v.date_ext,
            species: v.species,
        }
    }
}

/// The `MedicinalProduct.specialDesignation.indication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicinalProductSpecialDesignationIndication {
    /// `indicationCodeableConcept` variant.
    #[fhir("indicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `indicationReference` variant.
    #[fhir("indicationReference")]
    Reference(Box<types::Reference>),
}
