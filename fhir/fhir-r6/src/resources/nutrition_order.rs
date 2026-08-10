//! NutritionOrder
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionOrder
//!
//! Version: 6.0.0-ballot3
//!
//! Diet, formula or nutritional supplement request
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to an individual or group.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrder;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

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

    /// What this order fulfills
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Composite Request ID
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | on-hold | entered-in-error | ended | completed |
    /// revoked | unknown
    pub status: crate::coded::Coded<crate::r6::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r6::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Who requires the diet, formula or nutritional supplement
    pub subject: types::Reference,

    /// The encounter associated with this nutrition order
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Information to support fulfilling of the nutrition order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Date and time the nutrition order was requested
    pub date_time: types::DateTime,
    /// Primitive extension sibling for [`date_time`](Self::date_time) (FHIR `_dateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateTime")]
    pub date_time_ext: Option<types::Element>,

    /// Who ordered the diet, formula or nutritional supplement
    pub requester: Option<types::Reference>,

    /// Who is intended to perform the administration of the nutrition order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::CodeableReference>,

    /// List of the patient's food and nutrition-related allergies and
    /// intolerances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allergy_intolerance: Vec<types::Reference<crate::r6::resources::AllergyIntolerance>>,

    /// Order-specific modifier about the type of food that should be given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub food_preference_modifier: Vec<types::CodeableConcept>,

    /// Food that should not be given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_food_modifier: Vec<types::CodeableConcept>,

    /// Capture if patient is permitted to consume food from outside of current
    /// setting brought by the patient, family, and/or caregiver
    pub outside_food_allowed: Option<types::Boolean>,
    /// Primitive extension sibling for [`outside_food_allowed`](Self::outside_food_allowed) (FHIR `_outsideFoodAllowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outsideFoodAllowed")]
    pub outside_food_allowed_ext: Option<types::Element>,

    /// Oral diet components
    pub oral_diet: Option<NutritionOrderOralDiet>,

    /// Supplement components
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplement: Vec<NutritionOrderSupplement>,

    /// Enteral formula product
    pub enteral_formula: Option<NutritionOrderEnteralFormula>,

    /// Modular additive to add to the oral diet, supplement, and/or enteral
    /// feeding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<NutritionOrderAdditive>,

    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Indicates modular components to be provided in addition or mixed with the
/// oral diet, supplement, and/or enteral feeding.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderAdditive;
/// use fhir::r6::types;
///
/// let value = NutritionOrderAdditive {
///     product_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `productName` is the name this serializes to on the wire.
/// assert_eq!(json["productName"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderAdditive = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionOrderAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of modular component to add to the oral diet, supplement, and/or
    /// enteral feeding
    pub modular_type: Option<types::CodeableReference>,

    /// Product or brand name of the modular additive
    pub product_name: Option<types::String>,
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Amount of additive to be given or mixed in with the oral diet,
    /// supplement, and/or enteral feeding
    pub quantity: Option<types::Quantity>,

    /// How the additive should enter the patient's gastrointestinal tract
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub route_of_administration: Vec<types::CodeableConcept>,
}

/// Feeding provided through the gastrointestinal tract via a tube, catheter,
/// or stoma that delivers nutrition distal to the oral cavity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderEnteralFormula;
/// use fhir::r6::types;
///
/// let value = NutritionOrderEnteralFormula {
///     product_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `productName` is the name this serializes to on the wire.
/// assert_eq!(json["productName"], ::serde_json::json!("abc"));
///
/// let back: NutritionOrderEnteralFormula = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionOrderEnteralFormula {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of patient enteral feeding
    pub r#type: Option<types::CodeableReference>,

    /// Product or brand name of the enteral feeding
    pub product_name: Option<types::String>,
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Intended type of device for the enteral feeding administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_device: Vec<types::CodeableReference>,

    /// Amount of energy per specified volume of feeding that is required
    pub caloric_density: Option<types::Quantity>,

    /// How the enteral feeding should enter the patient's gastrointestinal
    /// tract
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub route_of_administration: Vec<types::CodeableConcept>,

    /// Formula feeding instruction as structured data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub administration: Vec<NutritionOrderEnteralFormulaAdministration>,

    /// Upper limit on formula feeding volume per unit of time
    pub max_volume_to_administer: Option<types::Quantity>,

    /// Formula feeding instructions expressed as text
    pub administration_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`administration_instruction`](Self::administration_instruction) (FHIR `_administrationInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_administrationInstruction")]
    pub administration_instruction_ext: Option<types::Element>,
}

/// Formula feeding administration instructions as structured data. This
/// repeating structure allows for changing the administration rate or volume
/// over time for both bolus and continuous feeding. An example of this would
/// be an instruction to increase the rate of continuous feeding every 2 hours.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderEnteralFormulaAdministration;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct NutritionOrderEnteralFormulaAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduling information for enteral feeding products
    pub schedule: Option<NutritionOrderEnteralFormulaAdministrationSchedule>,

    /// The volume of formula feeding to provide
    pub quantity: Option<types::Quantity>,

    /// Speed with which the formula feeding is provided per period of time
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
    rate: crate::r6::choice::Slot<NutritionOrderEnteralFormulaAdministrationRate>,
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

/// Schedule information for an enteral feeding.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderEnteralFormulaAdministrationSchedule;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct NutritionOrderEnteralFormulaAdministrationSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduled frequency of enteral feeding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<types::Timing>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

/// Diet given orally that may include texture modification, such as
/// International Dysphagia Diet Standardisation Initiative Framework -
/// Slightly Thick Level 1 drinks and Minced and International Dysphagia Diet
/// Standardisation Initiative Framework - Minced and Moist Level 5 food as
/// well as, for example, Decreased potassium diet (ie, nutrient modification),
/// Halal diet (ie, cultural modification), and/or Low microbial diet (eg,
/// other modification).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderOralDiet;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct NutritionOrderOralDiet {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of oral diet or diet restrictions that can be consumed orally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Scheduling information for oral diets
    pub schedule: Option<NutritionOrderOralDietSchedule>,

    /// The nutrient that is modified and the quantity in the diet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nutrient: Vec<NutritionOrderOralDietNutrient>,

    /// Texture modifications in addition to the oral diet type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub texture: Vec<NutritionOrderOralDietTexture>,

    /// Instructions or additional information about the oral diet
    pub instruction: Option<types::String>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,

    /// Amount of energy per specified volume of oral diet
    pub caloric_density: Option<types::Quantity>,
}

/// Defines the quantity and the nutrient modified (for example carbohydrate,
/// fiber or sodium) in the oral diet.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderOralDietNutrient;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct NutritionOrderOralDietNutrient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Nutrient modified in the oral diet type
    pub modifier: Option<types::CodeableConcept>,

    /// Quantity of the specified nutrient
    pub amount: Option<types::Quantity>,
}

/// Schedule information for an oral diet.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderOralDietSchedule;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

/// Class that describes any texture modifications in addition to the oral diet
/// type required for the patient to safely consume various types of foods
/// (i.e. solid and/or liquid).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderOralDietTexture;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct NutritionOrderOralDietTexture {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Food (i.e. solid and/or liquid) texture modifications in addition to
    /// those in the oral diet type
    pub modifier: Option<types::CodeableConcept>,

    /// Food (i.e. solid and/or liquid) types that undergo texture alteration
    pub r#type: Option<types::CodeableConcept>,
}

/// Oral nutritional products given in order to add further nutritional value
/// to the patient's diet.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderSupplement;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`product_name`](Self::product_name) (FHIR `_productName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productName")]
    pub product_name_ext: Option<types::Element>,

    /// Scheduling information for supplements
    pub schedule: Option<NutritionOrderSupplementSchedule>,

    /// Amount of the nutritional supplement
    pub quantity: Option<types::Quantity>,

    /// Instructions or additional information about the oral supplement
    pub instruction: Option<types::String>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,

    /// Amount of energy per specified volume of supplement that is required
    pub caloric_density: Option<types::Quantity>,
}

/// Schedule information for a supplement.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_order::NutritionOrderSupplementSchedule;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct NutritionOrderSupplementSchedule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduled frequency of supplement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing: Vec<types::Timing>,

    /// Take 'as needed'
    pub as_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Take 'as needed' for x
    pub as_needed_for: Option<types::CodeableConcept>,
}

/// The `NutritionOrder.enteralFormula.administration.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
