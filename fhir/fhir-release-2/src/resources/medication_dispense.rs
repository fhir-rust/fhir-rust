//! MedicationDispense
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
//!
//!
//!
//! Dispensing a medication to a named patient
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MedicationDispense Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_dispense::MedicationDispense;
/// use fhir::r2::types;
///
/// let value = MedicationDispense {
///     when_prepared: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `whenPrepared` is the name this serializes to on the wire.
/// assert_eq!(json["whenPrepared"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicationDispense = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationDispense {
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
    pub identifier: Option<types::Identifier>,

    /// in-progress | on-hold | completed | entered-in-error | stopped
    pub status: Option<crate::coded::Coded<crate::r2::codes::MedicationDispenseStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Who the dispense is for
    pub patient: Option<types::Reference>,

    /// Practitioner responsible for dispensing medication
    pub dispenser: Option<types::Reference>,

    /// Medication order that authorizes the dispense
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizing_prescription: Vec<types::Reference>,

    /// Trial fill, partial fill, emergency fill, etc.
    pub r#type: Option<types::CodeableConcept>,

    /// Amount dispensed
    pub quantity: Option<types::Quantity>,

    /// Days Supply
    pub days_supply: Option<types::Quantity>,

    /// What medication was supplied
    /// The `MedicationDispense.medication[x]` choice element (1..1); see [`MedicationDispenseMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationDispenseMedication>,

    /// Dispense processing time
    pub when_prepared: Option<types::DateTime>,
    /// Primitive extension sibling for [`when_prepared`](Self::when_prepared) (FHIR `_whenPrepared`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_whenPrepared")]
    pub when_prepared_ext: Option<types::Element>,

    /// When product was given out
    pub when_handed_over: Option<types::DateTime>,
    /// Primitive extension sibling for [`when_handed_over`](Self::when_handed_over) (FHIR `_whenHandedOver`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_whenHandedOver")]
    pub when_handed_over_ext: Option<types::Element>,

    /// Where the medication was sent
    pub destination: Option<types::Reference>,

    /// Who collected the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receiver: Vec<types::Reference>,

    /// Information about the dispense
    pub note: Option<types::String>,
    /// Primitive extension sibling for [`note`](Self::note) (FHIR `_note`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_note")]
    pub note_ext: Option<types::Element>,

    /// Medicine administration instructions to the patient/caregiver
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage_instruction: Vec<MedicationDispenseDosageInstruction>,

    /// Deals with substitution of one medicine for another
    pub substitution: Option<MedicationDispenseSubstitution>,
}

/// Indicates how the medication is to be used by the patient.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_dispense::MedicationDispenseDosageInstruction;
/// use fhir::r2::types;
///
/// let value = MedicationDispenseDosageInstruction {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: MedicationDispenseDosageInstruction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationDispenseDosageInstruction {
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

    /// E.g. "Take with food"
    pub additional_instructions: Option<types::CodeableConcept>,

    /// When medication should be administered
    pub timing: Option<types::Timing>,

    /// Take "as needed" f(or x)
    /// The `MedicationDispense.dosageInstruction.asNeeded[x]` choice element (0..1); see [`MedicationDispenseDosageInstructionAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<MedicationDispenseDosageInstructionAsNeeded>,

    /// Body site to administer to
    /// The `MedicationDispense.dosageInstruction.site[x]` choice element (0..1); see [`MedicationDispenseDosageInstructionSite`].
    #[serde(flatten)]
    pub site: Option<MedicationDispenseDosageInstructionSite>,

    /// How drug should enter body
    pub route: Option<types::CodeableConcept>,

    /// Technique for administering medication
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    /// The `MedicationDispense.dosageInstruction.dose[x]` choice element (0..1); see [`MedicationDispenseDosageInstructionDose`].
    #[serde(flatten)]
    pub dose: Option<MedicationDispenseDosageInstructionDose>,

    /// Amount of medication per unit of time
    /// The `MedicationDispense.dosageInstruction.rate[x]` choice element (0..1); see [`MedicationDispenseDosageInstructionRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationDispenseDosageInstructionRate>,

    /// Upper limit on medication per unit of time
    pub max_dose_per_period: Option<types::Ratio>,
}

/// Indicates whether or not substitution was made as part of the dispense. In
/// some cases substitution will be expected but does not happen, in other
/// cases substitution is not expected but does happen. This block explains
/// what substitution did or did not happen and why.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_dispense::MedicationDispenseSubstitution;
/// use fhir::r2::types;
///
/// let value = MedicationDispenseSubstitution {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MedicationDispenseSubstitution = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationDispenseSubstitution {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of substitution
    pub r#type: types::CodeableConcept,

    /// Why was substitution made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Who is responsible for the substitution
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_party: Vec<types::Reference>,
}

/// The `MedicationDispense.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationDispenseMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationDispense.dosageInstruction.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationDispenseDosageInstructionAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `MedicationDispense.dosageInstruction.site[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationDispenseDosageInstructionSite {
    /// `siteCodeableConcept` variant.
    #[fhir("siteCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `siteReference` variant.
    #[fhir("siteReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationDispense.dosageInstruction.dose[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationDispenseDosageInstructionDose {
    /// `doseRange` variant.
    #[fhir("doseRange")]
    Range(Box<types::Range>),
    /// `doseQuantity` variant.
    #[fhir("doseQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `MedicationDispense.dosageInstruction.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationDispenseDosageInstructionRate {
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
    type T = MedicationDispense;

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
