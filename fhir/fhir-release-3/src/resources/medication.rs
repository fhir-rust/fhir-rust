//! Medication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Medication
//!
//!
//!
//! Definition of a Medication
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Medication Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication::Medication;
/// use fhir::r3::types;
///
/// let value = Medication {
///     is_brand: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isBrand` is the name this serializes to on the wire.
/// assert_eq!(json["isBrand"], ::serde_json::json!(true));
///
/// let back: Medication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Medication {
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

    /// Codes that identify this medication
    pub code: Option<types::CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r3::codes::MedicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// True if a brand
    pub is_brand: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_brand`](Self::is_brand) (FHIR `_isBrand`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isBrand")]
    pub is_brand_ext: Option<types::Element>,

    /// True if medication does not require a prescription
    pub is_over_the_counter: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_over_the_counter`](Self::is_over_the_counter) (FHIR `_isOverTheCounter`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isOverTheCounter")]
    pub is_over_the_counter_ext: Option<types::Element>,

    /// Manufacturer of the item
    pub manufacturer: Option<types::Reference>,

    /// powder | tablets | capsule +
    pub form: Option<types::CodeableConcept>,

    /// Active or inactive ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<MedicationIngredient>,

    /// Details about packaged medications
    pub package: Option<MedicationPackage>,

    /// Picture of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<types::Attachment>,
}

/// Identifies a particular constituent of interest in the product.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication::MedicationIngredient;
/// use fhir::r3::types;
///
/// let value = MedicationIngredient {
///     is_active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isActive` is the name this serializes to on the wire.
/// assert_eq!(json["isActive"], ::serde_json::json!(true));
///
/// let back: MedicationIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MedicationIngredient {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The product contained
    /// The `Medication.ingredient.item[x]` choice element (1..1); see [`MedicationIngredientItem`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub item: Option<MedicationIngredientItem>,

    /// Active ingredient indicator
    pub is_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_active`](Self::is_active) (FHIR `_isActive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isActive")]
    pub is_active_ext: Option<types::Element>,

    /// Quantity of ingredient present
    pub amount: Option<types::Ratio>,
}

/// Information that only applies to packages (not products).
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication::MedicationPackage;
/// use fhir::r3::types;
///
/// let value = MedicationPackage {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationPackage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MedicationPackage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// E.g. box, vial, blister-pack
    pub container: Option<types::CodeableConcept>,

    /// What is in the package
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content: Vec<MedicationPackageContent>,

    /// Identifies a single production run
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub batch: Vec<MedicationPackageBatch>,
}

/// Information about a group of medication produced or packaged from one
/// production run.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication::MedicationPackageBatch;
/// use fhir::r3::types;
///
/// let value = MedicationPackageBatch {
///     lot_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lotNumber` is the name this serializes to on the wire.
/// assert_eq!(json["lotNumber"], ::serde_json::json!("abc"));
///
/// let back: MedicationPackageBatch = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MedicationPackageBatch {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier assigned to batch
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// When batch will expire
    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,
}

/// A set of components that go to make up the described item.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication::MedicationPackageContent;
/// use fhir::r3::types;
///
/// let value = MedicationPackageContent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationPackageContent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MedicationPackageContent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The item in the package
    /// The `Medication.package.content.item[x]` choice element (1..1); see [`MedicationPackageContentItem`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub item: Option<MedicationPackageContentItem>,

    /// Quantity present in the package
    pub amount: Option<types::Quantity>,
}

/// The `Medication.ingredient.item[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationIngredientItem {
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
}

/// The `Medication.package.content.item[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationPackageContentItem {
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Medication;

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
