//! MedicationAdministration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
//!
//!
//!
//! Administration of medication to a patient
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MedicationAdministration Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_administration::MedicationAdministration;
/// use fhir::r2::types;
///
/// let value = MedicationAdministration {
///     was_not_given: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `wasNotGiven` is the name this serializes to on the wire.
/// assert_eq!(json["wasNotGiven"], ::serde_json::json!(true));
///
/// let back: MedicationAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationAdministration {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// in-progress | on-hold | completed | entered-in-error | stopped
    pub status: crate::coded::Coded<crate::r2::codes::MedicationAdminStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Who received medication
    pub patient: types::Reference,

    /// Who administered substance
    pub practitioner: Option<types::Reference>,

    /// Encounter administered as part of
    pub encounter: Option<types::Reference>,

    /// Order administration performed against
    pub prescription: Option<types::Reference>,

    /// True if medication not administered
    pub was_not_given: Option<types::Boolean>,
    /// Primitive extension sibling for [`was_not_given`](Self::was_not_given) (FHIR `_wasNotGiven`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_wasNotGiven")]
    pub was_not_given_ext: Option<types::Element>,

    /// Reason administration not performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_not_given: Vec<types::CodeableConcept>,

    /// Reason administration performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_given: Vec<types::CodeableConcept>,

    /// Start and end time of administration
    /// The `MedicationAdministration.effectiveTime[x]` choice element (1..1); see [`MedicationAdministrationEffectiveTime`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub effective_time: Option<MedicationAdministrationEffectiveTime>,

    /// What was administered
    /// The `MedicationAdministration.medication[x]` choice element (1..1); see [`MedicationAdministrationMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationAdministrationMedication>,

    /// Device used to administer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference>,

    /// Information about the administration
    pub note: Option<types::String>,
    /// Primitive extension sibling for [`note`](Self::note) (FHIR `_note`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_note")]
    pub note_ext: Option<types::Element>,

    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,
}

/// Describes the medication dosage information details e.g. dose, rate, site,
/// route, etc.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_administration::MedicationAdministrationDosage;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct MedicationAdministrationDosage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Dosage Instructions
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Body site administered to
    /// The `MedicationAdministration.dosage.site[x]` choice element (0..1); see [`MedicationAdministrationDosageSite`].
    #[serde(flatten)]
    pub site: Option<MedicationAdministrationDosageSite>,

    /// Path of substance into body
    pub route: Option<types::CodeableConcept>,

    /// How drug was administered
    pub method: Option<types::CodeableConcept>,

    /// Amount administered in one dose
    pub quantity: Option<types::Quantity>,

    /// Dose quantity per unit of time
    /// The `MedicationAdministration.dosage.rate[x]` choice element (0..1); see [`MedicationAdministrationDosageRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationAdministrationDosageRate>,
}

/// The `MedicationAdministration.effectiveTime[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationEffectiveTime {
    /// `effectiveTimeDateTime` variant.
    #[fhir("effectiveTimeDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `effectiveTimePeriod` variant.
    #[fhir("effectiveTimePeriod")]
    Period(Box<types::Period>),
}

/// The `MedicationAdministration.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationAdministration.dosage.site[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationDosageSite {
    /// `siteCodeableConcept` variant.
    #[fhir("siteCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `siteReference` variant.
    #[fhir("siteReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationAdministration.dosage.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationDosageRate {
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
    /// `rateRange` variant.
    #[fhir("rateRange")]
    Range(Box<types::Range>),
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
