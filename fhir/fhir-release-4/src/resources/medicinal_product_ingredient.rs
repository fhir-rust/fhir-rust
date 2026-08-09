//! MedicinalProductIngredient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductIngredient
//!
//! Version: 4.0.1
//!
//! An ingredient of a manufactured item or pharmaceutical product
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An ingredient of a manufactured item or pharmaceutical product.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_ingredient::MedicinalProductIngredient;
/// use fhir::r4::types;
///
/// let value = MedicinalProductIngredient {
///     allergenic_indicator: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `allergenicIndicator` is the name this serializes to on the wire.
/// assert_eq!(json["allergenicIndicator"], ::serde_json::json!(true));
///
/// let back: MedicinalProductIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductIngredient {
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

    /// Identifier for the ingredient
    pub identifier: Option<types::Identifier>,

    /// Ingredient role e.g. Active ingredient, excipient
    pub role: types::CodeableConcept,

    /// If the ingredient is a known or suspected allergen
    pub allergenic_indicator: Option<types::Boolean>,
    /// Primitive extension sibling for [`allergenic_indicator`](Self::allergenic_indicator) (FHIR `_allergenicIndicator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allergenicIndicator")]
    pub allergenic_indicator_ext: Option<types::Element>,

    /// Manufacturer of this Ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference<crate::r4::resources::Organization>>,

    /// A specified substance that comprises this ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specified_substance: Vec<MedicinalProductIngredientSpecifiedSubstance>,

    /// The ingredient substance
    pub substance: Option<MedicinalProductIngredientSubstance>,
}

/// A specified substance that comprises this ingredient.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_ingredient::MedicinalProductIngredientSpecifiedSubstance;
/// use fhir::r4::types;
///
/// let value = MedicinalProductIngredientSpecifiedSubstance {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductIngredientSpecifiedSubstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductIngredientSpecifiedSubstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specified substance
    pub code: types::CodeableConcept,

    /// The group of specified substance, e.g. group 1 to 4
    pub group: types::CodeableConcept,

    /// Confidentiality level of the specified substance as the ingredient
    pub confidentiality: Option<types::CodeableConcept>,

    /// Quantity of the substance or specified substance present in the
    /// manufactured item or pharmaceutical product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
}

/// Quantity of the substance or specified substance present in the
/// manufactured item or pharmaceutical product.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_ingredient::MedicinalProductIngredientSpecifiedSubstanceStrength;
/// use fhir::r4::types;
///
/// let value = MedicinalProductIngredientSpecifiedSubstanceStrength {
///     measurement_point: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `measurementPoint` is the name this serializes to on the wire.
/// assert_eq!(json["measurementPoint"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductIngredientSpecifiedSubstanceStrength = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The quantity of substance in the unit of presentation, or in the volume
    /// (or mass) of the single pharmaceutical product or manufactured item
    pub presentation: types::Ratio,

    /// A lower limit for the quantity of substance in the unit of
    /// presentation. For use when there is a range of strengths, this is the
    /// lower limit, with the presentation attribute becoming the upper limit
    pub presentation_low_limit: Option<types::Ratio>,

    /// The strength per unitary volume (or mass)
    pub concentration: Option<types::Ratio>,

    /// A lower limit for the strength per unitary volume (or mass), for when
    /// there is a range. The concentration attribute then becomes the upper
    /// limit
    pub concentration_low_limit: Option<types::Ratio>,

    /// For when strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// The country or countries for which the strength range applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country: Vec<types::CodeableConcept>,

    /// Strength expressed in terms of a reference substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_strength:
        Vec<MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength>,
}

/// Strength expressed in terms of a reference substance.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_ingredient::MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength;
/// use fhir::r4::types;
///
/// let value = MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength {
///     measurement_point: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `measurementPoint` is the name this serializes to on the wire.
/// assert_eq!(json["measurementPoint"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Relevant reference substance
    pub substance: Option<types::CodeableConcept>,

    /// Strength expressed in terms of a reference substance
    pub strength: types::Ratio,

    /// Strength expressed in terms of a reference substance
    pub strength_low_limit: Option<types::Ratio>,

    /// For when strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// The country or countries for which the strength range applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country: Vec<types::CodeableConcept>,
}

/// The ingredient substance.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_ingredient::MedicinalProductIngredientSubstance;
/// use fhir::r4::types;
///
/// let value = MedicinalProductIngredientSubstance {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductIngredientSubstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductIngredientSubstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The ingredient substance
    pub code: types::CodeableConcept,

    /// Quantity of the substance or specified substance present in the
    /// manufactured item or pharmaceutical product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicinalProductIngredient;

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
