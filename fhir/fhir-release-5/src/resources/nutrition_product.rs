//! NutritionProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionProduct
//!
//! Version: 5.0.0
//!
//! NutritionProduct Resource: A food or supplement that is consumed by patients.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A food or supplement that is consumed by patients.
///
/// NutritionProduct is a definitional resource in FHIR R5 that describes a
/// food, enteral or oral nutritional supplement, infant formula, thickener, or
/// similar consumable product. It captures the product's classification, its
/// nutritional composition expressed as nutrients, the ingredients it contains,
/// any known or suspected allergens, descriptive characteristics such as color,
/// texture, or preparation, and one or more physical instances identified by
/// lot number, expiry date, or serial identifier. The resource lets systems
/// represent products consistently across nutrition ordering, dispensing,
/// inventory, and product-catalog scenarios so that clinical, dietary, and
/// supply workflows can reference a single well-defined product definition.
///
/// In typical use, a NutritionProduct is referenced by ordering and
/// administration resources rather than describing a specific patient event on
/// its own. The product's status indicates whether the definition is active,
/// inactive, or entered in error, and its manufacturer and category support
/// catalog lookup and reporting.
///
/// # See also
///
/// Related resources and data types include the `NutritionOrder` and
/// `NutritionIntake` resources that reference nutrition products,
/// [`Patient`](crate::r5::resources::patient::Patient) as the consumer of the
/// product, [`CodeableConcept`](crate::r5::types::CodeableConcept) used for
/// coded classification, and
/// [`CodeableReference`](crate::r5::types::CodeableReference) used to point to
/// nutrients, ingredients, and allergens.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_product::NutritionProduct;
/// use fhir::r5::types;
///
/// let value = NutritionProduct {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProduct {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coded identifier for the product that conveys its detailed nutrients and ingredients, drawn from food or supplement terminologies
    pub code: Option<types::CodeableConcept>,

    /// Lifecycle state of the product definition: active, inactive, or entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::NutritionproductStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Broad product groups or categories used to classify the product, such as Legume and Legume Products, Beverages, or Beef Products
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Manufacturer, representative or officially responsible for the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference>,

    /// The product's nutritional composition, expressed as nutrient amounts such as per pack, per serving, or per dose
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nutrient: Vec<NutritionProductNutrient>,

    /// Ingredients contained in this product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<NutritionProductIngredient>,

    /// Known or suspected allergens present in the product, supporting allergy screening and safe consumption
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub known_allergen: Vec<types::CodeableReference>,

    /// Specifies descriptive properties of the nutrition product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<NutritionProductCharacteristic>,

    /// One or several physical instances or occurrences of the nutrition product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<NutritionProductInstance>,

    /// Comments made about the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// The product's nutritional information expressed by the nutrients.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_product::NutritionProductNutrient;
/// use fhir::r5::types;
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
    pub item: Option<types::CodeableReference>,

    /// The amount of nutrient expressed in one or more units: X per pack / per serving / per dose
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub amount: Vec<types::Ratio>,
}

/// Ingredients contained in this product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_product::NutritionProductIngredient;
/// use fhir::r5::types;
///
/// let value = NutritionProductIngredient {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionProductIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub amount: Vec<types::Ratio>,
}

/// Specifies descriptive properties of the nutrition product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_product::NutritionProductCharacteristic;
/// use fhir::r5::types;
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

    /// The `NutritionProduct.characteristic.value[x]` choice element (0..1); see [`NutritionProductCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<NutritionProductCharacteristicValue>,
}

/// One or several physical instances or occurrences of the nutrition product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_product::NutritionProductInstance;
/// use fhir::r5::types;
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

    /// The identifier for the physical instance, typically a serial number or manufacturer number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The name for the specific product
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The identification of the batch or lot of the product
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`).
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// The expiry date or date and time for the product
    pub expiry: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiry`](Self::expiry) (FHIR `_expiry`).
    #[serde(rename = "_expiry")]
    pub expiry_ext: Option<types::Element>,

    /// The date until which the product is expected to be good for consumption
    pub use_by: Option<types::DateTime>,
    /// Primitive extension sibling for [`use_by`](Self::use_by) (FHIR `_useBy`).
    #[serde(rename = "_useBy")]
    pub use_by_ext: Option<types::Element>,

    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    pub biological_source_event: Option<types::Identifier>,
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
/// The `NutritionProduct.characteristic.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum NutritionProductCharacteristicValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r5::choice::Primitive<types::Base64Binary>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
}
