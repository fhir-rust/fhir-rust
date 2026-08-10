//! NutritionOrder
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
//!
//! Version: 4.0.1
//!
//! Diet, formula or nutritional supplement request
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::nutrition_order::NutritionOrder;
/// use fhir::r4::types;
///
/// let value = NutritionOrder {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrder = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct NutritionOrder {
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

    /// Identifiers assigned to this order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates_canonical: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates_uri: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiates")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_ext: Vec<Option<types::Element>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error |
    /// unknown
    pub status: crate::coded::Coded<crate::r4::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r4::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// The person who requires the diet, formula or nutritional supplement
    pub patient: types::Reference<crate::r4::resources::Patient>,

    /// The encounter associated with this nutrition order
    pub encounter: Option<types::Reference<crate::r4::resources::Encounter>>,

    /// Date and time the nutrition order was requested
    pub date_time: types::DateTime,
    /// Primitive extension sibling for [`date_time`](Self::date_time) (FHIR `_dateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateTime")]
    pub date_time_ext: Option<types::Element>,

    /// Who ordered the diet, formula or nutritional supplement
    pub orderer: Option<types::Reference>,

    /// List of the patient's food and nutrition-related allergies and
    /// intolerances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allergy_intolerance: Vec<types::Reference<crate::r4::resources::AllergyIntolerance>>,

    /// Order-specific modifier about the type of food that should be given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub food_preference_modifier: Vec<types::CodeableConcept>,

    /// Order-specific modifier about the type of food that should not be given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_food_modifier: Vec<types::CodeableConcept>,

    /// Oral diet components
    pub oral_diet: Option<NutritionOrderOralDiet>,

    /// Supplement components
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplement: Vec<NutritionOrderSupplement>,

    /// Enteral formula components
    pub enteral_formula: Option<NutritionOrderEnteralFormula>,

    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Feeding provided through the gastrointestinal tract via a tube, catheter,
/// or stoma that delivers nutrition distal to the oral cavity.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::nutrition_order::NutritionOrderEnteralFormula;
/// use fhir::r4::types;
///
/// let value = NutritionOrderEnteralFormula {
///     base_formula_product_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `baseFormulaProductName` is the name this serializes to on the wire.
/// assert_eq!(json["baseFormulaProductName"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderEnteralFormula = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct NutritionOrderEnteralFormula {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of enteral or infant formula
    pub base_formula_type: Option<types::CodeableConcept>,

    /// Product or brand name of the enteral or infant formula
    pub base_formula_product_name: Option<types::String>,
    /// Primitive extension sibling for [`base_formula_product_name`](Self::base_formula_product_name) (FHIR `_baseFormulaProductName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_baseFormulaProductName")]
    pub base_formula_product_name_ext: Option<types::Element>,

    /// Type of modular component to add to the feeding
    pub additive_type: Option<types::CodeableConcept>,

    /// Product or brand name of the modular additive
    pub additive_product_name: Option<types::String>,
    /// Primitive extension sibling for [`additive_product_name`](Self::additive_product_name) (FHIR `_additiveProductName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_additiveProductName")]
    pub additive_product_name_ext: Option<types::Element>,

    /// Amount of energy per specified volume that is required
    pub caloric_density: Option<types::Quantity>,

    /// How the formula should enter the patient's gastrointestinal tract
    pub routeof_administration: Option<types::CodeableConcept>,

    /// Formula feeding instruction as structured data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub administration: Vec<NutritionOrderEnteralFormulaAdministration>,

    /// Upper limit on formula volume per unit of time
    pub max_volume_to_deliver: Option<types::Quantity>,

    /// Formula feeding instructions expressed as text
    pub administration_instruction: Option<types::String>,
    /// Primitive extension sibling for [`administration_instruction`](Self::administration_instruction) (FHIR `_administrationInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_administrationInstruction")]
    pub administration_instruction_ext: Option<types::Element>,
}

/// Formula administration instructions as structured data. This repeating
/// structure allows for changing the administration rate or volume over time
/// for both bolus and continuous feeding. An example of this would be an
/// instruction to increase the rate of continuous feeding every 2 hours.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::nutrition_order::NutritionOrderEnteralFormulaAdministration;
/// use fhir::r4::types;
///
/// let value = NutritionOrderEnteralFormulaAdministration {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderEnteralFormulaAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "NutritionOrderEnteralFormulaAdministrationDe")]
#[fhir_version("r4")]
pub struct NutritionOrderEnteralFormulaAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduled frequency of enteral feeding
    pub schedule: Option<types::Timing>,

    /// The volume of formula to provide
    pub quantity: Option<types::Quantity>,

    /// Speed with which the formula is provided per period of time
    /// The `NutritionOrder.enteralFormula.administration.rate[x]` choice element (0..1); see [`NutritionOrderEnteralFormulaAdministrationRate`].
    #[serde(flatten)]
    pub rate: Option<NutritionOrderEnteralFormulaAdministrationRate>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NutritionOrderEnteralFormulaAdministrationDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    schedule: Option<types::Timing>,
    quantity: Option<types::Quantity>,
    #[serde(flatten)]
    rate: crate::r4::choice::Slot<NutritionOrderEnteralFormulaAdministrationRate>,
}

impl ::core::convert::From<NutritionOrderEnteralFormulaAdministrationDe>
    for NutritionOrderEnteralFormulaAdministration
{
    fn from(v: NutritionOrderEnteralFormulaAdministrationDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            schedule: v.schedule,
            quantity: v.quantity,
            rate: v.rate.0,
        }
    }
}

/// Diet given orally in contrast to enteral (tube) feeding.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::nutrition_order::NutritionOrderOralDiet;
/// use fhir::r4::types;
///
/// let value = NutritionOrderOralDiet {
///     instruction: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instruction` is the name this serializes to on the wire.
/// assert_eq!(json["instruction"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderOralDiet = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct NutritionOrderOralDiet {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of oral diet or diet restrictions that describe what can be
    /// consumed orally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Scheduled frequency of diet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<types::Timing>,

    /// Required nutrient modifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nutrient: Vec<NutritionOrderOralDietNutrient>,

    /// Required texture modifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texture: Vec<NutritionOrderOralDietTexture>,

    /// The required consistency of fluids and liquids provided to the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fluid_consistency_type: Vec<types::CodeableConcept>,

    /// Instructions or additional information about the oral diet
    pub instruction: Option<types::String>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,
}

/// Class that defines the quantity and type of nutrient modifications (for
/// example carbohydrate, fiber or sodium) required for the oral diet.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::nutrition_order::NutritionOrderOralDietNutrient;
/// use fhir::r4::types;
///
/// let value = NutritionOrderOralDietNutrient {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderOralDietNutrient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct NutritionOrderOralDietNutrient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of nutrient that is being modified
    pub modifier: Option<types::CodeableConcept>,

    /// Quantity of the specified nutrient
    pub amount: Option<types::Quantity>,
}

/// Class that describes any texture modifications required for the patient to
/// safely consume various types of solid foods.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::nutrition_order::NutritionOrderOralDietTexture;
/// use fhir::r4::types;
///
/// let value = NutritionOrderOralDietTexture {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderOralDietTexture = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct NutritionOrderOralDietTexture {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code to indicate how to alter the texture of the foods, e.g. pureed
    pub modifier: Option<types::CodeableConcept>,

    /// Concepts that are used to identify an entity that is ingested for
    /// nutritional purposes
    pub food_type: Option<types::CodeableConcept>,
}

/// Oral nutritional products given in order to add further nutritional value
/// to the patient's diet.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::nutrition_order::NutritionOrderSupplement;
/// use fhir::r4::types;
///
/// let value = NutritionOrderSupplement {
///     product_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `productName` is the name this serializes to on the wire.
/// assert_eq!(json["productName"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderSupplement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct NutritionOrderSupplement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of supplement product requested
    pub r#type: Option<types::CodeableConcept>,

    /// Product or brand name of the nutritional supplement
    pub product_name: Option<types::String>,
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Scheduled frequency of supplement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<types::Timing>,

    /// Amount of the nutritional supplement
    pub quantity: Option<types::Quantity>,

    /// Instructions or additional information about the oral supplement
    pub instruction: Option<types::String>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,
}

/// The `NutritionOrder.enteralFormula.administration.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum NutritionOrderEnteralFormulaAdministrationRate {
    /// `rateQuantity` variant.
    #[fhir("rateQuantity")]
    Quantity(Box<types::Quantity>),
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = NutritionOrder;

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
