//! Ingredient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Ingredient
//!
//! Version: 5.0.0
//!
//! Ingredient Resource: An ingredient of a manufactured item or pharmaceutical product.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// An ingredient of a manufactured item or pharmaceutical product.
///
/// The Ingredient resource describes a single substance that is a constituent
/// part of a manufactured item or pharmaceutical product. It captures the
/// substance itself, the role it plays within the product (for example active
/// versus inactive, or excipient), any more precise function it performs (such
/// as antioxidant or alkalizing agent), and its strength expressed either as a
/// presentation amount per dosage unit or as a concentration per volume or mass.
/// Strength may additionally be stated in terms of a reference substance, and
/// may vary by country to accommodate differing regulatory expressions.
///
/// In FHIR R5 the Ingredient resource is part of the medication definition
/// domain and is used chiefly for regulated product information and medicinal
/// product authorization. It is typically referenced by definitional resources
/// such as `MedicinalProductDefinition`, `ManufacturedItemDefinition`, and
/// `AdministrableProductDefinition` through its `for` element, rather than
/// describing patient-specific medication use. The substance identity is carried
/// by a [`CodeableReference`](crate::r5::types::CodeableReference), and each
/// quantitative value is modeled with types such as
/// [`Ratio`](crate::r5::types::Ratio), [`Quantity`](crate::r5::types::Quantity),
/// and [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # See also
///
/// The `Substance` and `MedicinalProductDefinition` resources describe the
/// substance and product that an Ingredient relates to, while
/// [`CodeableReference`](crate::r5::types::CodeableReference) and
/// [`Reference`](crate::r5::types::Reference) provide the linking mechanisms.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::ingredient::Ingredient;
/// use fhir::r5::types;
///
/// let value = Ingredient {
///     allergenic_indicator: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `allergenicIndicator` is the name this serializes to on the wire.
/// assert_eq!(json["allergenicIndicator"], ::serde_json::json!(true));
///
/// let back: Ingredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
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

    /// An identifier or code by which the ingredient can be referenced
    pub identifier: Option<types::Identifier>,

    /// Publication lifecycle status of this Ingredient record, one of draft, active, retired, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// References to the product or products that this ingredient is a constituent part of.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#for: Vec<types::Reference>,

    /// The purpose the ingredient serves within the product, for example active or inactive.
    pub role: types::CodeableConcept,

    /// Precise action within the drug product, e.g. antioxidant, alkalizing agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<types::CodeableConcept>,

    /// A classification of the ingredient according to where in the physical item it tends to be used
    pub group: Option<types::CodeableConcept>,

    /// If the ingredient is a known or suspected allergen
    pub allergenic_indicator: Option<types::Boolean>,
    /// Primitive extension sibling for [`allergenic_indicator`](Self::allergenic_indicator) (FHIR `_allergenicIndicator`).
    #[serde(rename = "_allergenicIndicator")]
    pub allergenic_indicator_ext: Option<types::Element>,

    /// A place for providing any notes that are relevant to the component
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// An organization that manufactures this ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<IngredientManufacturer>,

    /// The substance that comprises this ingredient, including its identity and strength.
    pub substance: IngredientSubstance,
}

/// An organization that manufactures this ingredient.
/// # Examples
///
/// ```
/// use fhir::r5::resources::ingredient::IngredientManufacturer;
/// use fhir::r5::types;
///
/// let value = IngredientManufacturer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: IngredientManufacturer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientManufacturer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// allowed | possible | actual
    pub role: Option<crate::r5::coded::Coded<crate::r5::codes::IngredientManufacturerRole>>,
    /// Primitive extension sibling for [`role`](Self::role) (FHIR `_role`).
    #[serde(rename = "_role")]
    pub role_ext: Option<types::Element>,

    /// An organization that manufactures this ingredient
    pub manufacturer: types::Reference,
}

/// The substance that comprises this ingredient.
/// # Examples
///
/// ```
/// use fhir::r5::resources::ingredient::IngredientSubstance;
/// use fhir::r5::types;
///
/// let value = IngredientSubstance {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: IngredientSubstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A code or full resource that represents the ingredient substance
    pub code: types::CodeableReference,

    /// The quantity of substance, per presentation, or per volume or mass, and type of quantity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub strength: Vec<IngredientSubstanceStrength>,
}

/// The quantity of substance, per presentation, or per volume or mass, and type of quantity.
/// # Examples
///
/// ```
/// use fhir::r5::resources::ingredient::IngredientSubstanceStrength;
/// use fhir::r5::types;
///
/// let value = IngredientSubstanceStrength {
///     text_presentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `textPresentation` is the name this serializes to on the wire.
/// assert_eq!(json["textPresentation"], ::serde_json::json!("abc"));
///
/// let back: IngredientSubstanceStrength = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstanceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `Ingredient.substance.strength.presentation[x]` choice element (0..1); see [`IngredientSubstanceStrengthPresentation`].
    #[serde(flatten)]
    pub presentation: Option<IngredientSubstanceStrengthPresentation>,

    /// Text of either the whole presentation strength or a part of it
    pub text_presentation: Option<types::String>,
    /// Primitive extension sibling for [`text_presentation`](Self::text_presentation) (FHIR `_textPresentation`).
    #[serde(rename = "_textPresentation")]
    pub text_presentation_ext: Option<types::Element>,

    /// The `Ingredient.substance.strength.concentration[x]` choice element (0..1); see [`IngredientSubstanceStrengthConcentration`].
    #[serde(flatten)]
    pub concentration: Option<IngredientSubstanceStrengthConcentration>,

    /// Text of either the whole concentration strength or a part of it
    pub text_concentration: Option<types::String>,
    /// Primitive extension sibling for [`text_concentration`](Self::text_concentration) (FHIR `_textConcentration`).
    #[serde(rename = "_textConcentration")]
    pub text_concentration_ext: Option<types::Element>,

    /// A code that indicates if the strength is based on the ingredient substance as stated or on the substance base
    pub basis: Option<types::CodeableConcept>,

    /// When strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`).
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// Where the strength range applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country: Vec<types::CodeableConcept>,

    /// Strength expressed in terms of a reference substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_strength: Vec<IngredientSubstanceStrengthReferenceStrength>,
}

/// Strength expressed in terms of a reference substance.
/// # Examples
///
/// ```
/// use fhir::r5::resources::ingredient::IngredientSubstanceStrengthReferenceStrength;
/// use fhir::r5::types;
///
/// let value = IngredientSubstanceStrengthReferenceStrength {
///     measurement_point: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `measurementPoint` is the name this serializes to on the wire.
/// assert_eq!(json["measurementPoint"], ::serde_json::json!("abc"));
///
/// let back: IngredientSubstanceStrengthReferenceStrength = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstanceStrengthReferenceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Relevant reference substance
    pub substance: types::CodeableReference,

    /// The `Ingredient.substance.strength.referenceStrength.strength[x]` choice element (0..1); see [`IngredientSubstanceStrengthReferenceStrengthStrength`].
    #[serde(flatten)]
    pub strength: Option<IngredientSubstanceStrengthReferenceStrengthStrength>,

    /// When strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`).
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// Where the strength range applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country: Vec<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Ingredient;

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
/// The `Ingredient.substance.strength.concentration[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum IngredientSubstanceStrengthConcentration {
    /// `concentrationRatio` variant.
    #[fhir("concentrationRatio")]
    Ratio(Box<types::Ratio>),
    /// `concentrationRatioRange` variant.
    #[fhir("concentrationRatioRange")]
    RatioRange(Box<types::RatioRange>),
    /// `concentrationCodeableConcept` variant.
    #[fhir("concentrationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `concentrationQuantity` variant.
    #[fhir("concentrationQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `Ingredient.substance.strength.presentation[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum IngredientSubstanceStrengthPresentation {
    /// `presentationRatio` variant.
    #[fhir("presentationRatio")]
    Ratio(Box<types::Ratio>),
    /// `presentationRatioRange` variant.
    #[fhir("presentationRatioRange")]
    RatioRange(Box<types::RatioRange>),
    /// `presentationCodeableConcept` variant.
    #[fhir("presentationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `presentationQuantity` variant.
    #[fhir("presentationQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `Ingredient.substance.strength.referenceStrength.strength[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum IngredientSubstanceStrengthReferenceStrengthStrength {
    /// `strengthRatio` variant.
    #[fhir("strengthRatio")]
    Ratio(Box<types::Ratio>),
    /// `strengthRatioRange` variant.
    #[fhir("strengthRatioRange")]
    RatioRange(Box<types::RatioRange>),
    /// `strengthQuantity` variant.
    #[fhir("strengthQuantity")]
    Quantity(Box<types::Quantity>),
}
