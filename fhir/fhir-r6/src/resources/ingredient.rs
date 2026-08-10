//! Ingredient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Ingredient
//!
//! Version: 6.0.0-ballot3
//!
//! An ingredient of a manufactured item or pharmaceutical product
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An ingredient of a manufactured item or pharmaceutical product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::ingredient::Ingredient;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Ingredient {
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

    /// An identifier or code by which the ingredient can be referenced
    pub identifier: Option<types::Identifier>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The product which this ingredient is a constituent part of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#for: Vec<types::Reference>,

    /// Purpose of the ingredient within the product, e.g. active, inactive
    pub role: types::CodeableConcept,

    /// Precise action within the drug product, e.g. antioxidant, alkalizing
    /// agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<types::CodeableConcept>,

    /// A classification of the ingredient according to where in the physical
    /// item it tends to be used, such the outer shell of a tablet, inner body
    /// or ink
    pub group: Option<types::CodeableConcept>,

    /// If the ingredient is a known or suspected allergen
    pub allergenic_indicator: Option<types::Boolean>,
    /// Primitive extension sibling for [`allergenic_indicator`](Self::allergenic_indicator) (FHIR `_allergenicIndicator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allergenicIndicator")]
    pub allergenic_indicator_ext: Option<types::Element>,

    /// A place for providing any notes that are relevant to the component,
    /// e.g. removed during process, adjusted for loss on drying
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// An organization that manufactures this ingredient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<IngredientManufacturer>,

    /// The substance that comprises this ingredient
    pub substance: IngredientSubstance,
}

/// The organization(s) that manufacture this ingredient. Can be used to
/// indicate: 1) Organizations we are aware of that manufacture this ingredient
/// \2) Specific Manufacturer(s) currently being used 3) Set of organisations
/// allowed to manufacture this ingredient for this product Users must be clear
/// on the application of context relevant to their use case.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::ingredient::IngredientManufacturer;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub role: Option<crate::coded::Coded<crate::r6::codes::IngredientManufacturerRole>>,
    /// Primitive extension sibling for [`role`](Self::role) (FHIR `_role`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_role")]
    pub role_ext: Option<types::Element>,

    /// An organization that manufactures this ingredient
    pub manufacturer: types::Reference<crate::r6::resources::Organization>,
}

/// The substance that comprises this ingredient.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::ingredient::IngredientSubstance;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// The quantity of substance, per presentation, or per volume or mass, and
    /// type of quantity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub strength: Vec<IngredientSubstanceStrength>,
}

/// The quantity of substance in the unit of presentation, or in the volume (or
/// mass) of the single pharmaceutical product or manufactured item. The
/// allowed repetitions do not represent different strengths, but are different
/// representations - mathematically equivalent - of a single strength.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::ingredient::IngredientSubstanceStrength;
/// use fhir::r6::types;
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
#[serde(from = "IngredientSubstanceStrengthDe")]
#[fhir_version("r6")]
pub struct IngredientSubstanceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The quantity of substance in the unit of presentation
    /// The `Ingredient.substance.strength.presentation[x]` choice element (0..1); see [`IngredientSubstanceStrengthPresentation`].
    #[serde(flatten)]
    pub presentation: Option<IngredientSubstanceStrengthPresentation>,

    /// Text of either the whole presentation strength or a part of it (rest
    /// being in Strength.presentation as a ratio)
    pub text_presentation: Option<types::String>,
    /// Primitive extension sibling for [`text_presentation`](Self::text_presentation) (FHIR `_textPresentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_textPresentation")]
    pub text_presentation_ext: Option<types::Element>,

    /// The strength per unitary volume (or mass)
    /// The `Ingredient.substance.strength.concentration[x]` choice element (0..1); see [`IngredientSubstanceStrengthConcentration`].
    #[serde(flatten)]
    pub concentration: Option<IngredientSubstanceStrengthConcentration>,

    /// Text of either the whole concentration strength or a part of it (rest
    /// being in Strength.concentration as a ratio)
    pub text_concentration: Option<types::String>,
    /// Primitive extension sibling for [`text_concentration`](Self::text_concentration) (FHIR `_textConcentration`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_textConcentration")]
    pub text_concentration_ext: Option<types::Element>,

    /// A code that indicates if the strength is, for example, based on the
    /// ingredient substance as stated or on the substance base (when the
    /// ingredient is a salt)
    pub basis: Option<types::CodeableConcept>,

    /// When strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// Where the strength range applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country: Vec<types::CodeableConcept>,

    /// Strength expressed in terms of a reference substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_strength: Vec<IngredientSubstanceStrengthReferenceStrength>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IngredientSubstanceStrengthDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    presentation: crate::r6::choice::Slot<IngredientSubstanceStrengthPresentation>,
    text_presentation: Option<types::String>,
    #[serde(rename = "_textPresentation")]
    text_presentation_ext: Option<types::Element>,
    #[serde(flatten)]
    concentration: crate::r6::choice::Slot<IngredientSubstanceStrengthConcentration>,
    text_concentration: Option<types::String>,
    #[serde(rename = "_textConcentration")]
    text_concentration_ext: Option<types::Element>,
    basis: Option<types::CodeableConcept>,
    measurement_point: Option<types::String>,
    #[serde(rename = "_measurementPoint")]
    measurement_point_ext: Option<types::Element>,
    #[serde(default)]
    country: Vec<types::CodeableConcept>,
    #[serde(default)]
    reference_strength: Vec<IngredientSubstanceStrengthReferenceStrength>,
}

impl ::core::convert::From<IngredientSubstanceStrengthDe> for IngredientSubstanceStrength {
    fn from(v: IngredientSubstanceStrengthDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            presentation: v.presentation.0,
            text_presentation: v.text_presentation,
            text_presentation_ext: v.text_presentation_ext,
            concentration: v.concentration.0,
            text_concentration: v.text_concentration,
            text_concentration_ext: v.text_concentration_ext,
            basis: v.basis,
            measurement_point: v.measurement_point,
            measurement_point_ext: v.measurement_point_ext,
            country: v.country,
            reference_strength: v.reference_strength,
        }
    }
}

/// Strength expressed in terms of a reference substance. For when the
/// ingredient strength is additionally expressed as equivalent to the strength
/// of some other closely related substance (e.g. salt vs. base). Reference
/// strength represents the strength (quantitative composition) of the active
/// moiety of the active substance. There are situations when the active
/// substance and active moiety are different, therefore both a strength and a
/// reference strength are needed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::ingredient::IngredientSubstanceStrengthReferenceStrength;
/// use fhir::r6::types;
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
#[serde(from = "IngredientSubstanceStrengthReferenceStrengthDe")]
#[fhir_version("r6")]
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

    /// Strength expressed in terms of a reference substance
    /// The `Ingredient.substance.strength.referenceStrength.strength[x]` choice element (1..1); see [`IngredientSubstanceStrengthReferenceStrengthStrength`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub strength: Option<IngredientSubstanceStrengthReferenceStrengthStrength>,

    /// When strength is measured at a particular point or distance
    pub measurement_point: Option<types::String>,
    /// Primitive extension sibling for [`measurement_point`](Self::measurement_point) (FHIR `_measurementPoint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measurementPoint")]
    pub measurement_point_ext: Option<types::Element>,

    /// Where the strength range applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country: Vec<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IngredientSubstanceStrengthReferenceStrengthDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    substance: types::CodeableReference,
    #[serde(flatten)]
    strength: crate::r6::choice::Slot<IngredientSubstanceStrengthReferenceStrengthStrength>,
    measurement_point: Option<types::String>,
    #[serde(rename = "_measurementPoint")]
    measurement_point_ext: Option<types::Element>,
    #[serde(default)]
    country: Vec<types::CodeableConcept>,
}

impl ::core::convert::From<IngredientSubstanceStrengthReferenceStrengthDe>
    for IngredientSubstanceStrengthReferenceStrength
{
    fn from(v: IngredientSubstanceStrengthReferenceStrengthDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            substance: v.substance,
            strength: v.strength.0,
            measurement_point: v.measurement_point,
            measurement_point_ext: v.measurement_point_ext,
            country: v.country,
        }
    }
}

/// The `Ingredient.substance.strength.presentation[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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

/// The `Ingredient.substance.strength.concentration[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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

/// The `Ingredient.substance.strength.referenceStrength.strength[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
