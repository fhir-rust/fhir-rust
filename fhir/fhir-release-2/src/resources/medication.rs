//! Medication
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Medication
//!
//!
//!
//! Definition of a Medication
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Medication Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication::Medication;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Codes that identify this medication
    pub code: Option<types::CodeableConcept>,

    /// True if a brand
    pub is_brand: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_brand`](Self::is_brand) (FHIR `_isBrand`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isBrand")]
    pub is_brand_ext: Option<types::Element>,

    /// Manufacturer of the item
    pub manufacturer: Option<types::Reference<crate::r2::resources::Organization>>,

    /// Administrable medication details
    pub product: Option<MedicationProduct>,

    /// Details about packaged medications
    pub package: Option<MedicationPackage>,
}

/// Information that only applies to packages (not products).
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication::MedicationPackage;
/// use fhir::r2::types;
///
/// let value = MedicationPackage {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MedicationPackage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationPackage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
}

/// A set of components that go to make up the described item.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication::MedicationPackageContent;
/// use fhir::r2::types;
///
/// let value = MedicationPackageContent {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MedicationPackageContent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationPackageContent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A product in the package
    pub item: types::Reference<crate::r2::resources::Medication>,

    /// Quantity present in the package
    pub amount: Option<types::Quantity>,
}

/// Information that only applies to products (not packages).
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication::MedicationProduct;
/// use fhir::r2::types;
///
/// let value = MedicationProduct {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MedicationProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationProduct {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// powder | tablets | carton +
    pub form: Option<types::CodeableConcept>,

    /// Active or inactive ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<MedicationProductIngredient>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub batch: Vec<MedicationProductBatch>,
}

/// Information about a group of medication produced or packaged from one
/// production run.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication::MedicationProductBatch;
/// use fhir::r2::types;
///
/// let value = MedicationProductBatch {
///     lot_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lotNumber` is the name this serializes to on the wire.
/// assert_eq!(json["lotNumber"], ::serde_json::json!("abc"));
///
/// let back: MedicationProductBatch = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationProductBatch {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,
}

/// Identifies a particular constituent of interest in the product.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication::MedicationProductIngredient;
/// use fhir::r2::types;
///
/// let value = MedicationProductIngredient {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MedicationProductIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationProductIngredient {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The product contained
    pub item: types::Reference,

    /// Quantity of ingredient present
    pub amount: Option<types::Ratio>,
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
