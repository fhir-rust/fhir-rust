//! NutritionIntake
//!
//! URL: http://hl7.org/fhir/StructureDefinition/NutritionIntake
//!
//! Version: 6.0.0-ballot3
//!
//! Record of intake by a patient
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of intake by a patient. A NutritionIntake may indicate that the
/// patient may be consuming the food (i.e., solid and/or liquid), breastmilk,
/// infant formula, supplements, enteral formula now or has consumed it in the
/// past. The source of this information can be the patient, significant other
/// (such as a family member or spouse), or a clinician. A common scenario
/// where this information is captured is during the history taking process
/// during a patient visit or stay or through an app that tracks food (i.e.,
/// solid and/or liquid), breastmilk, infant formula, supplements, enteral
/// formula consumed. The consumption information may come from sources such as
/// the patient's memory, from a nutrition label, or from a clinician
/// documenting observed intake.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_intake::NutritionIntake;
/// use fhir::r6::types;
///
/// let value = NutritionIntake {
///     recorded: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `recorded` is the name this serializes to on the wire.
/// assert_eq!(json["recorded"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: NutritionIntake = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "NutritionIntakeDe")]
#[fhir_version("r6")]
pub struct NutritionIntake {
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

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Fulfils plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed |
    /// entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// Code representing an overall type of nutrition intake
    pub code: Option<types::CodeableConcept>,

    /// Who is/was consuming the food (i.e. solid and/or liquid)
    pub subject: types::Reference,

    /// Encounter associated with NutritionIntake
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// The date/time or interval when the food (i.e. solid and/or liquid)
    /// is/was consumed
    /// The `NutritionIntake.occurrence[x]` choice element (0..1); see [`NutritionIntakeOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<NutritionIntakeOccurrence>,

    /// When the intake was recorded
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Indicates if this is a reported rather than a primary record. Can also
    /// indicate the source that provided the information about the consumption
    /// The `NutritionIntake.reported[x]` choice element (0..1); see [`NutritionIntakeReported`].
    #[serde(flatten)]
    pub reported: Option<NutritionIntakeReported>,

    /// The nutrition product intended for consumption and/or administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nutrition_item: Vec<NutritionIntakeNutritionItem>,

    /// Who or what performed the intake and how they were involved
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<NutritionIntakePerformer>,

    /// Where the intake occurred
    pub location: Option<types::Reference<crate::r6::resources::Location>>,

    /// Additional supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// Reason for why the food (i.e. solid and/or liquid) is /was consumed
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
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    #[serde(default)]
    part_of: Vec<types::Reference>,
    status: crate::coded::Coded<crate::r6::codes::EventStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    status_reason: Vec<types::CodeableConcept>,
    code: Option<types::CodeableConcept>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r6::choice::Slot<NutritionIntakeOccurrence>,
    recorded: Option<types::DateTime>,
    #[serde(rename = "_recorded")]
    recorded_ext: Option<types::Element>,
    #[serde(flatten)]
    reported: crate::r6::choice::Slot<NutritionIntakeReported>,
    #[serde(default)]
    nutrition_item: Vec<NutritionIntakeNutritionItem>,
    #[serde(default)]
    performer: Vec<NutritionIntakePerformer>,
    location: Option<types::Reference<crate::r6::resources::Location>>,
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
            nutrition_item: v.nutrition_item,
            performer: v.performer,
            location: v.location,
            derived_from: v.derived_from,
            reason: v.reason,
            note: v.note,
        }
    }
}

/// The nutrition product intended for consumption and/or administration.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_intake::NutritionIntakeNutritionItem;
/// use fhir::r6::types;
///
/// let value = NutritionIntakeNutritionItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionIntakeNutritionItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionIntakeNutritionItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of food (i.e. solid or liquid) product
    pub r#type: Option<types::CodeableConcept>,

    /// A product used for nutritional purposes (e.g. food or supplement)
    pub nutrition_product: Option<types::CodeableReference>,

    /// What nutrition item was consumed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumed_item: Vec<NutritionIntakeNutritionItemConsumedItem>,

    /// What nutrition item was not consumed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_consumed_item: Vec<NutritionIntakeNutritionItemNotConsumedItem>,
}

/// What nutrition item was consumed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_intake::NutritionIntakeNutritionItemConsumedItem;
/// use fhir::r6::types;
///
/// let value = NutritionIntakeNutritionItemConsumedItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionIntakeNutritionItemConsumedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "NutritionIntakeNutritionItemConsumedItemDe")]
#[fhir_version("r6")]
pub struct NutritionIntakeNutritionItemConsumedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Scheduled frequency of consumption
    pub schedule: Option<types::Timing>,

    /// Quantity of the specified food (i.e. solid and/or liquid)
    pub amount: Option<types::Quantity>,

    /// Rate of enteral feeding administration
    /// The `NutritionIntake.nutritionItem.consumedItem.rate[x]` choice element (0..1); see [`NutritionIntakeNutritionItemConsumedItemRate`].
    #[serde(flatten)]
    pub rate: Option<NutritionIntakeNutritionItemConsumedItemRate>,

    /// Nutrients and/or energy contained in the intake
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub total_intake: Vec<NutritionIntakeNutritionItemConsumedItemTotalIntake>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NutritionIntakeNutritionItemConsumedItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    schedule: Option<types::Timing>,
    amount: Option<types::Quantity>,
    #[serde(flatten)]
    rate: crate::r6::choice::Slot<NutritionIntakeNutritionItemConsumedItemRate>,
    #[serde(default)]
    total_intake: Vec<NutritionIntakeNutritionItemConsumedItemTotalIntake>,
}

impl ::core::convert::From<NutritionIntakeNutritionItemConsumedItemDe>
    for NutritionIntakeNutritionItemConsumedItem
{
    fn from(v: NutritionIntakeNutritionItemConsumedItemDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            schedule: v.schedule,
            amount: v.amount,
            rate: v.rate.0,
            total_intake: v.total_intake,
        }
    }
}

/// Nutrients and/or energy contained in the intake.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_intake::NutritionIntakeNutritionItemConsumedItemTotalIntake;
/// use fhir::r6::types;
///
/// let value = NutritionIntakeNutritionItemConsumedItemTotalIntake {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionIntakeNutritionItemConsumedItemTotalIntake = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionIntakeNutritionItemConsumedItemTotalIntake {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of nutrient consumed in the intake
    pub nutrient: types::CodeableReference,

    /// Total amount of nutrient consumed
    pub amount: types::Quantity,

    /// Total energy consumed in kilocalories or kilojoules
    pub energy: Option<types::Quantity>,
}

/// What nutrition item was not consumed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_intake::NutritionIntakeNutritionItemNotConsumedItem;
/// use fhir::r6::types;
///
/// let value = NutritionIntakeNutritionItemNotConsumedItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: NutritionIntakeNutritionItemNotConsumedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct NutritionIntakeNutritionItemNotConsumedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reason the nutrition item was not consumed
    pub reason: Option<types::CodeableConcept>,

    /// The intended frequency of consumption that was not followed
    pub schedule: Option<types::Timing>,

    /// Quantity of the specified food (i.e. solid and/or liquid) that was not
    /// consumed
    pub amount: Option<types::Quantity>,
}

/// Who or what performed the intake and how they were involved.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::nutrition_intake::NutritionIntakePerformer;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Who or what performed the intake
    pub actor: types::Reference,
}

/// The `NutritionIntake.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum NutritionIntakeOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

/// The `NutritionIntake.reported[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum NutritionIntakeReported {
    /// `reportedBoolean` variant.
    #[fhir("reportedBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `reportedReference` variant.
    #[fhir("reportedReference")]
    Reference(Box<types::Reference>),
}

/// The `NutritionIntake.nutritionItem.consumedItem.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum NutritionIntakeNutritionItemConsumedItemRate {
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
    type T = NutritionIntake;

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
