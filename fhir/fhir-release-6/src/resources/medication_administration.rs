//! MedicationAdministration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
//!
//! Version: 6.0.0-ballot3
//!
//! Administration of medication to a patient
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes the event of a patient consuming or otherwise being administered
/// a medication. This may be as simple as swallowing a tablet or it may be a
/// long running infusion. Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner. This event can also be used to record waste using a status of
/// not-done and the appropriate statusReason.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_administration::MedicationAdministration;
/// use fhir::r6::types;
///
/// let value = MedicationAdministration {
///     is_sub_potent: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isSubPotent` is the name this serializes to on the wire.
/// assert_eq!(json["isSubPotent"], ::serde_json::json!(true));
///
/// let back: MedicationAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationAdministration {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Plan this is fulfilled by this administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// in-progress | not-done | on-hold | completed | entered-in-error |
    /// stopped | unknown
    pub status: crate::coded::Coded<crate::r6::codes::MedicationAdminStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason administration not performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// Type of medication administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// What was administered
    pub medication: types::CodeableReference,

    /// Who received medication
    pub subject: types::Reference,

    /// Encounter administered as part of
    pub encounter: Option<types::Reference>,

    /// Additional information to support administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Specific date/time or interval of time during which the administration
    /// took place (or did not take place)
    /// The `MedicationAdministration.occurrence[x]` choice element (1..1); see [`MedicationAdministrationOccurrence`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub occurrence: Option<MedicationAdministrationOccurrence>,

    /// When the MedicationAdministration was first captured in the subject's
    /// record
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Full dose was not administered
    pub is_sub_potent: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_sub_potent`](Self::is_sub_potent) (FHIR `_isSubPotent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isSubPotent")]
    pub is_sub_potent_ext: Option<types::Element>,

    /// Reason full dose was not administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_potent_reason: Vec<types::CodeableConcept>,

    /// Who or what performed the medication administration and what type of
    /// performance they did
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<MedicationAdministrationPerformer>,

    /// Reason that supports why the medication was administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Request administration performed against
    pub request: Option<types::Reference>,

    /// Device used to administer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::CodeableReference>,

    /// Information about the administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference>,
}

/// Describes the medication dosage information details e.g. dose, rate, site,
/// route, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_administration::MedicationAdministrationDosage;
/// use fhir::r6::types;
///
/// let value = MedicationAdministrationDosage {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministrationDosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationAdministrationDosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Body site administered to
    pub site: Option<types::CodeableConcept>,

    /// Path of substance into body
    pub route: Option<types::CodeableConcept>,

    /// How drug was administered
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    pub dose: Option<types::Quantity>,

    /// Dose quantity per unit of time
    /// The `MedicationAdministration.dosage.rate[x]` choice element (0..1); see [`MedicationAdministrationDosageRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationAdministrationDosageRate>,
}

/// The performer of the medication treatment. For devices this is the device
/// that performed the administration of the medication. An IV Pump would be an
/// example of a device that is performing the administration. Both the IV Pump
/// and the practitioner that set the rate or bolus on the pump can be listed
/// as performers.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_administration::MedicationAdministrationPerformer;
/// use fhir::r6::types;
///
/// let value = MedicationAdministrationPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministrationPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationAdministrationPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who or what performed the medication administration
    pub actor: types::CodeableReference,
}

/// The `MedicationAdministration.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `MedicationAdministration.dosage.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationDosageRate {
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
    /// `rateQuantity` variant.
    #[fhir("rateQuantity")]
    Quantity(Box<types::Quantity>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationAdministration;

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
