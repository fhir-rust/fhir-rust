//! NutritionProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionProduct
//!
//! Version: 6.0.0-ballot3
//!
//! A product used for nutritional purposes (i.e. food or supplement)
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A food or supplement that is consumed by patients.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_product::NutritionProduct;
/// use fhir::r6::types;
///
/// let value = NutritionProduct {
///     ingredient_summary: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `ingredientSummary` is the name this serializes to on the wire.
/// assert_eq!(json["ingredientSummary"], ::serde_json::json!("# Heading"));
///
/// let back: NutritionProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionProduct {
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

    /// A code that can identify the product
    pub code: Option<types::CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::NutritionproductStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Broad product groups, like Fruit, Grain, Beverages, or Vegetables
    /// Products
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Manufacturer, representative or person officially responsible for the
    /// product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference>,

    /// The product's nutritional information expressed by the nutrients
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nutrient: Vec<NutritionProductNutrient>,

    /// Textual description of product ingredients
    pub ingredient_summary: Option<types::Markdown>,
    /// Primitive extension sibling for [`ingredient_summary`](Self::ingredient_summary) (FHIR `_ingredientSummary`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ingredientSummary")]
    pub ingredient_summary_ext: Option<types::Element>,

    /// Ingredients contained in this product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<NutritionProductIngredient>,

    /// The amount of energy present in the product expressed in kilocalories
    /// or kilojoules
    pub energy: Option<types::Quantity>,

    /// Specifies descriptive properties of the nutrition product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<NutritionProductCharacteristic>,

    /// One or several physical instances or occurrences of the nutrition
    /// product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<NutritionProductInstance>,

    /// Comments made about the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Specifies descriptive properties of the nutrition product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_product::NutritionProductCharacteristic;
/// use fhir::r6::types;
///
/// let value = NutritionProductCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionProductCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionProductCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code specifying the type of characteristic
    pub r#type: types::CodeableConcept,

    /// The value of the characteristic
    /// The `NutritionProduct.characteristic.value[x]` choice element (1..1); see [`NutritionProductCharacteristicValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<NutritionProductCharacteristicValue>,
}

/// Ingredients contained in this product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_product::NutritionProductIngredient;
/// use fhir::r6::types;
///
/// let value = NutritionProductIngredient {
///     allergen: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `allergen` is the name this serializes to on the wire.
/// assert_eq!(json["allergen"], ::serde_json::json!(true));
///
/// let back: NutritionProductIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionProductIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The ingredient contained in the product
    pub item: types::CodeableReference,

    /// The amount of ingredient that is in the product
    /// The `NutritionProduct.ingredient.amount[x]` choice element (0..1); see [`NutritionProductIngredientAmount`].
    #[serde(flatten)]
    pub amount: Option<NutritionProductIngredientAmount>,

    /// A known or suspected allergenic and/or substance that is associated
    /// with an intolerance
    pub allergen: Option<types::Boolean>,
    /// Primitive extension sibling for [`allergen`](Self::allergen) (FHIR `_allergen`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allergen")]
    pub allergen_ext: Option<types::Element>,
}

/// Conveys instance-level information about this product item. One or several
/// physical, countable instances or occurrences of the product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_product::NutritionProductInstance;
/// use fhir::r6::types;
///
/// let value = NutritionProductInstance {
///     lot_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lotNumber` is the name this serializes to on the wire.
/// assert_eq!(json["lotNumber"], ::serde_json::json!("abc"));
///
/// let back: NutritionProductInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionProductInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The amount of items or instances
    pub quantity: Option<types::Quantity>,

    /// The identifier for the physical instance, typically a serial number or
    /// manufacturer number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The name or brand for the specific product
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The identification of the batch or lot of the product
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// The expiry date or date and time for the product
    pub expiry: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiry`](Self::expiry) (FHIR `_expiry`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expiry")]
    pub expiry_ext: Option<types::Element>,

    /// The date until which the product is expected to be good for consumption
    pub use_by: Option<types::DateTime>,
    /// Primitive extension sibling for [`use_by`](Self::use_by) (FHIR `_useBy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_useBy")]
    pub use_by_ext: Option<types::Element>,

    /// An identifier of the donation, collection, or pooling event from which
    /// biological material in this nutrition product was derived
    pub biological_source_event: Option<types::Identifier>,
}

/// The product's nutritional information expressed by the nutrients.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_product::NutritionProductNutrient;
/// use fhir::r6::types;
///
/// let value = NutritionProductNutrient {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionProductNutrient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionProductNutrient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The (relevant) nutrients in the product
    pub item: types::CodeableReference,

    /// The amount of nutrient present in the product
    /// The `NutritionProduct.nutrient.amount[x]` choice element (0..1); see [`NutritionProductNutrientAmount`].
    #[serde(flatten)]
    pub amount: Option<NutritionProductNutrientAmount>,
}

/// The `NutritionProduct.characteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum NutritionProductCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
}

/// The `NutritionProduct.ingredient.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum NutritionProductIngredientAmount {
    /// `amountRatio` variant.
    #[fhir("amountRatio")]
    Ratio(Box<types::Ratio>),
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `NutritionProduct.nutrient.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum NutritionProductNutrientAmount {
    /// `amountRatio` variant.
    #[fhir("amountRatio")]
    Ratio(Box<types::Ratio>),
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = NutritionProduct;

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
