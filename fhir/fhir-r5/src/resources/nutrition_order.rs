//! NutritionOrder
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
//!
//! Version: 5.0.0
//!
//! NutritionOrder Resource: A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.
///
/// A NutritionOrder captures the details of what a patient should be fed and
/// is the primary FHIR R5 resource used to request nutrition services. It
/// spans three complementary areas of nutrition care: oral diets (including
/// diet types, texture modifications, fluid consistency, and nutrient limits),
/// oral nutritional supplements, and enteral (tube) feeding formulas with
/// their additives, caloric density, route of administration, and delivery
/// schedules. Each order records its ordering context through status and
/// intent codes, the subject who requires the nutrition, the associated
/// encounter, the requested date and time, and the orderer and performers.
/// To support safe fulfillment it can also reference the patient's food and
/// nutrition-related allergies and intolerances, along with food preference
/// and exclusion modifiers.
///
/// In clinical workflows a NutritionOrder is authored by a clinician or
/// dietitian and communicated to dietary or nursing services so that meals,
/// supplements, or tube feedings are prepared and delivered according to the
/// patient's needs. As a request-pattern resource it participates in
/// order-management flows and can be based on protocols or plans and grouped
/// with related orders.
///
/// # See also
///
/// The subject is commonly a [`Patient`](crate::r5::resources::patient::Patient),
/// referenced together with the visit [`Encounter`](crate::r5::resources::encounter::Encounter).
/// Dietary safety draws on the patient's
/// [`AllergyIntolerance`](crate::r5::resources::allergy_intolerance::AllergyIntolerance)
/// records. Coded values such as diet types and food modifiers use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), while resource
/// links use [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrder;
/// use fhir::r5::types;
///
/// let value = NutritionOrder {
///     outside_food_allowed: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `outsideFoodAllowed` is the name this serializes to on the wire.
/// assert_eq!(json["outsideFoodAllowed"], ::serde_json::json!(true));
///
/// let back: NutritionOrder = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrder {
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

    /// Identifiers assigned to this order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`).
    #[serde(rename = "_instantiates")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_ext: Vec<Option<types::Element>>,

    /// What this order fulfills
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Composite Request ID
    pub group_identifier: Option<types::Identifier>,

    /// Workflow status of the order: draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// How the order should be understood in the request workflow: proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: crate::r5::coded::Coded<crate::r5::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Who requires the diet, formula or nutritional supplement, typically a reference to a [`Patient`](crate::r5::resources::patient::Patient)
    pub subject: types::Reference,

    /// The encounter associated with this nutrition order
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// Information to support fulfilling of the nutrition order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Date and time the nutrition order was requested
    pub date_time: types::DateTime,
    /// Primitive extension sibling for [`date_time`](Self::date_time) (FHIR `_dateTime`).
    #[serde(rename = "_dateTime")]
    pub date_time_ext: Option<types::Element>,

    /// Who ordered the diet, formula or nutritional supplement
    pub orderer: Option<types::Reference>,

    /// Who is desired to perform the administration of what is being ordered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::CodeableReference>,

    /// References to the patient's food and nutrition-related allergies and intolerances that inform safe fulfillment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allergy_intolerance: Vec<types::Reference<crate::r5::resources::AllergyIntolerance>>,

    /// Order-specific modifier about the type of food that should be given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub food_preference_modifier: Vec<types::CodeableConcept>,

    /// Order-specific modifier about the type of food that should not be given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_food_modifier: Vec<types::CodeableConcept>,

    /// Capture when a food item is brought in by the patient and/or family
    pub outside_food_allowed: Option<types::Boolean>,
    /// Primitive extension sibling for [`outside_food_allowed`](Self::outside_food_allowed) (FHIR `_outsideFoodAllowed`).
    #[serde(rename = "_outsideFoodAllowed")]
    pub outside_food_allowed_ext: Option<types::Element>,

    /// Detailed instructions for an oral diet, including diet type, scheduling, nutrient limits, and texture or fluid consistency modifications
    pub oral_diet: Option<NutritionOrderOralDiet>,

    /// One or more oral nutritional supplements to be provided, such as a specific product, quantity, and administration schedule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplement: Vec<NutritionOrderSupplement>,

    /// Formula feeding instructions for enteral (tube) nutrition, including formula type, additives, caloric density, route, and delivery rate
    pub enteral_formula: Option<NutritionOrderEnteralFormula>,

    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Oral diet components.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderOralDiet;
/// use fhir::r5::types;
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
pub struct NutritionOrderOralDiet {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of oral diet or diet restrictions that describe what can be consumed orally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Scheduling information for oral diets
    pub schedule: Option<NutritionOrderOralDietSchedule>,

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
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`).
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,
}

/// Scheduling information for oral diets.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderOralDietSchedule;
/// use fhir::r5::types;
///
/// let value = NutritionOrderOralDietSchedule {
///     as_needed: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `asNeeded` is the name this serializes to on the wire.
/// assert_eq!(json["asNeeded"], ::serde_json::json!(true));
///
/// let back: NutritionOrderOralDietSchedule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDietSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduled frequency of diet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<types::Timing>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`).
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

/// Required nutrient modifications.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderOralDietNutrient;
/// use fhir::r5::types;
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

/// Required texture modifications.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderOralDietTexture;
/// use fhir::r5::types;
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

    /// Concepts that are used to identify an entity that is ingested for nutritional purposes
    pub food_type: Option<types::CodeableConcept>,
}

/// Supplement components.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderSupplement;
/// use fhir::r5::types;
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
    pub r#type: Option<types::CodeableReference>,

    /// Product or brand name of the nutritional supplement
    pub product_name: Option<types::String>,
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`).
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Scheduling information for supplements
    pub schedule: Option<NutritionOrderSupplementSchedule>,

    /// Amount of the nutritional supplement
    pub quantity: Option<types::Quantity>,

    /// Instructions or additional information about the oral supplement
    pub instruction: Option<types::String>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`).
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,
}

/// Scheduling information for supplements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderSupplementSchedule;
/// use fhir::r5::types;
///
/// let value = NutritionOrderSupplementSchedule {
///     as_needed: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `asNeeded` is the name this serializes to on the wire.
/// assert_eq!(json["asNeeded"], ::serde_json::json!(true));
///
/// let back: NutritionOrderSupplementSchedule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderSupplementSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduled frequency of diet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<types::Timing>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`).
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

/// Enteral formula components.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderEnteralFormula;
/// use fhir::r5::types;
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
    pub base_formula_type: Option<types::CodeableReference>,

    /// Product or brand name of the enteral or infant formula
    pub base_formula_product_name: Option<types::String>,
    /// Primitive extension sibling for [`base_formula_product_name`](Self::base_formula_product_name) (FHIR `_baseFormulaProductName`).
    #[serde(rename = "_baseFormulaProductName")]
    pub base_formula_product_name_ext: Option<types::Element>,

    /// Intended type of device for the administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_device: Vec<types::CodeableReference>,

    /// Components to add to the feeding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<NutritionOrderEnteralFormulaAdditive>,

    /// Amount of energy per specified volume that is required
    pub caloric_density: Option<types::Quantity>,

    /// How the formula should enter the patient's gastrointestinal tract
    pub route_of_administration: Option<types::CodeableConcept>,

    /// Formula feeding instruction as structured data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub administration: Vec<NutritionOrderEnteralFormulaAdministration>,

    /// Upper limit on formula volume per unit of time
    pub max_volume_to_deliver: Option<types::Quantity>,

    /// Formula feeding instructions expressed as text
    pub administration_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`administration_instruction`](Self::administration_instruction) (FHIR `_administrationInstruction`).
    #[serde(rename = "_administrationInstruction")]
    pub administration_instruction_ext: Option<types::Element>,
}

/// Components to add to the feeding.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderEnteralFormulaAdditive;
/// use fhir::r5::types;
///
/// let value = NutritionOrderEnteralFormulaAdditive {
///     product_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `productName` is the name this serializes to on the wire.
/// assert_eq!(json["productName"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderEnteralFormulaAdditive = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormulaAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of modular component to add to the feeding
    pub r#type: Option<types::CodeableReference>,

    /// Product or brand name of the modular additive
    pub product_name: Option<types::String>,
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`).
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Amount of additive to be given or mixed in
    pub quantity: Option<types::Quantity>,
}

/// Formula feeding instruction as structured data.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderEnteralFormulaAdministration;
/// use fhir::r5::types;
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
pub struct NutritionOrderEnteralFormulaAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduling information for enteral formula products
    pub schedule: Option<NutritionOrderEnteralFormulaAdministrationSchedule>,

    /// The volume of formula to provide
    pub quantity: Option<types::Quantity>,

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
    schedule: Option<NutritionOrderEnteralFormulaAdministrationSchedule>,
    quantity: Option<types::Quantity>,
    #[serde(flatten)]
    rate: crate::r5::choice::Slot<NutritionOrderEnteralFormulaAdministrationRate>,
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

/// Scheduling information for enteral formula products.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_order::NutritionOrderEnteralFormulaAdministrationSchedule;
/// use fhir::r5::types;
///
/// let value = NutritionOrderEnteralFormulaAdministrationSchedule {
///     as_needed: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `asNeeded` is the name this serializes to on the wire.
/// assert_eq!(json["asNeeded"], ::serde_json::json!(true));
///
/// let back: NutritionOrderEnteralFormulaAdministrationSchedule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormulaAdministrationSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduled frequency of enteral formula
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<types::Timing>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`).
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
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
/// The `NutritionOrder.enteralFormula.administration.rate[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum NutritionOrderEnteralFormulaAdministrationRate {
    /// `rateQuantity` variant.
    #[fhir("rateQuantity")]
    Quantity(Box<types::Quantity>),
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
}
