//! NutritionIntake
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionIntake
//!
//! Version: 5.0.0
//!
//! NutritionIntake Resource: A record of food or fluid that is being consumed by a patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of food or fluid that is being consumed by a patient. A
/// NutritionIntake may indicate that the patient may be consuming the food or
/// fluid now or has consumed the food or fluid in the past. The source of this
/// information can be the patient, a significant other (such as a family member
/// or spouse), or a clinician. A common scenario where this information is
/// captured is during the history taking process at an encounter or visit.
///
/// In FHIR R5 this resource supports nutrition and dietary workflows by
/// documenting what was actually taken in, distinct from what was ordered or
/// dispensed. It captures oral intake, enteral feeding, and fluid consumption,
/// including cases where an item was offered but refused or not consumed. Each
/// entry can record the specific food or fluid product, its scheduled timing,
/// the amount and administration rate, and an optional nutrient label breakdown,
/// which makes the resource useful both for clinical review during history
/// taking and for downstream dietary and nutritional analysis. A NutritionIntake
/// is always anchored to a subject and may be linked to the encounter during
/// which it was recorded, the performer who administered or observed the intake,
/// and the location where it occurred.
///
/// # Related resources
///
/// The [`subject`](NutritionIntake::subject) is commonly a
/// [`Patient`](crate::r5::resources::patient::Patient), and the
/// [`encounter`](NutritionIntake::encounter) references the
/// [`Encounter`](crate::r5::resources::encounter::Encounter) in which the intake
/// was captured. Coded and referenced values throughout use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`CodeableReference`](crate::r5::types::CodeableReference), and
/// [`Reference`](crate::r5::types::Reference). This resource is closely related
/// to `NutritionOrder`, which represents the request or prescription, and to
/// `NutritionProduct`, which describes the food or fluid product itself.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::nutrition_intake::NutritionIntake;
///
/// let value = NutritionIntake::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: NutritionIntake = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "NutritionIntakeDe")]
pub struct NutritionIntake {
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

    /// External identifier
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

    /// Fulfils plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Lifecycle state of the intake record: preparation, in-progress, not-done, on-hold, stopped, completed, entered-in-error, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// Code representing an overall type of nutrition intake
    pub code: Option<types::CodeableConcept>,

    /// Who is or was consuming the food or fluid; typically a reference to a Patient.
    pub subject: types::Reference,

    /// Encounter associated with NutritionIntake
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `NutritionIntake.occurrence[x]` choice element (0..1); see [`NutritionIntakeOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<NutritionIntakeOccurrence>,

    /// When the intake was recorded
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`).
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// The `NutritionIntake.reported[x]` choice element (0..1); see [`NutritionIntakeReported`].
    #[serde(flatten)]
    pub reported: Option<NutritionIntakeReported>,

    /// What food or fluid product or item was consumed, with its type, amount, timing, and rate.
    pub consumed_item: vec1::Vec1<NutritionIntakeConsumedItem>,

    /// Total nutrient for the whole meal, product, serving
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient_label: Vec<NutritionIntakeIngredientLabel>,

    /// Who was performed in the intake
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<NutritionIntakePerformer>,

    /// Where the intake occurred
    pub location: Option<types::Reference<crate::r5::resources::Location>>,

    /// Additional supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// Reason for why the food or fluid is /was consumed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Further information about the consumption
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NutritionIntakeDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    instantiates_canonical: Vec<types::Canonical>,
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default)]
    instantiates_canonical_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    instantiates_uri: Vec<types::Uri>,
    #[serde(rename = "_instantiatesUri")]
    #[serde(default)]
    instantiates_uri_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    #[serde(default)]
    part_of: Vec<types::Reference>,
    status: crate::r5::coded::Coded<crate::r5::codes::EventStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    status_reason: Vec<types::CodeableConcept>,
    code: Option<types::CodeableConcept>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r5::choice::Slot<NutritionIntakeOccurrence>,
    recorded: Option<types::DateTime>,
    #[serde(rename = "_recorded")]
    recorded_ext: Option<types::Element>,
    #[serde(flatten)]
    reported: crate::r5::choice::Slot<NutritionIntakeReported>,
    consumed_item: vec1::Vec1<NutritionIntakeConsumedItem>,
    #[serde(default)]
    ingredient_label: Vec<NutritionIntakeIngredientLabel>,
    #[serde(default)]
    performer: Vec<NutritionIntakePerformer>,
    location: Option<types::Reference<crate::r5::resources::Location>>,
    #[serde(default)]
    derived_from: Vec<types::Reference>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<NutritionIntakeDe> for NutritionIntake {
    fn from(v: NutritionIntakeDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            instantiates_canonical: v.instantiates_canonical,
            instantiates_canonical_ext: v.instantiates_canonical_ext,
            instantiates_uri: v.instantiates_uri,
            instantiates_uri_ext: v.instantiates_uri_ext,
            based_on: v.based_on,
            part_of: v.part_of,
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            code: v.code,
            subject: v.subject,
            encounter: v.encounter,
            occurrence: v.occurrence.0,
            recorded: v.recorded,
            recorded_ext: v.recorded_ext,
            reported: v.reported.0,
            consumed_item: v.consumed_item,
            ingredient_label: v.ingredient_label,
            performer: v.performer,
            location: v.location,
            derived_from: v.derived_from,
            reason: v.reason,
            note: v.note,
        }
    }
}

/// What food or fluid product or item was consumed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_intake::NutritionIntakeConsumedItem;
/// use fhir::r5::types;
///
/// let value = NutritionIntakeConsumedItem {
///     not_consumed: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `notConsumed` is the name this serializes to on the wire.
/// assert_eq!(json["notConsumed"], ::serde_json::json!(true));
///
/// let back: NutritionIntakeConsumedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakeConsumedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of food or fluid product
    pub r#type: types::CodeableConcept,

    /// Code that identifies the food or fluid product that was consumed
    pub nutrition_product: types::CodeableReference,

    /// Scheduled frequency of consumption
    pub schedule: Option<types::Timing>,

    /// Quantity of the specified food
    pub amount: Option<types::Quantity>,

    /// Rate at which enteral feeding was administered
    pub rate: Option<types::Quantity>,

    /// Flag to indicate if the food or fluid item was refused or otherwise not consumed
    pub not_consumed: Option<types::Boolean>,
    /// Primitive extension sibling for [`not_consumed`](Self::not_consumed) (FHIR `_notConsumed`).
    #[serde(rename = "_notConsumed")]
    pub not_consumed_ext: Option<types::Element>,

    /// Reason food or fluid was not consumed
    pub not_consumed_reason: Option<types::CodeableConcept>,
}

/// Total nutrient for the whole meal, product, serving.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_intake::NutritionIntakeIngredientLabel;
/// use fhir::r5::types;
///
/// let value = NutritionIntakeIngredientLabel {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionIntakeIngredientLabel = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakeIngredientLabel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Total nutrient consumed
    pub nutrient: types::CodeableReference,

    /// Total amount of nutrient consumed
    pub amount: types::Quantity,
}

/// Who was performed in the intake.
/// # Examples
///
/// ```
/// use fhir::r5::resources::nutrition_intake::NutritionIntakePerformer;
/// use fhir::r5::types;
///
/// let value = NutritionIntakePerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionIntakePerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performer
    pub function: Option<types::CodeableConcept>,

    /// Who performed the intake
    pub actor: types::Reference,
}

/// The `NutritionIntake.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum NutritionIntakeOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

/// The `NutritionIntake.reported[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum NutritionIntakeReported {
    /// `reportedBoolean` variant.
    #[fhir("reportedBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `reportedReference` variant.
    #[fhir("reportedReference")]
    Reference(Box<types::Reference>),
}
