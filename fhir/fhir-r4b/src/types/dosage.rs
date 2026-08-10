//! Dosage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Dosage
//!
//! Version: 4.3.0
//!
//! How the medication is/was taken or should be taken
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Dosage Type: Indicates how the medication
/// is/was taken or should be taken by the patient.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::dosage::Dosage;
/// use fhir::r4b::types;
///
/// let value = Dosage {
///     patient_instruction: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `patientInstruction` is the name this serializes to on the wire.
/// assert_eq!(json["patientInstruction"], ::serde_json::json!("abc"));
///
/// let back: Dosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct Dosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The order of the dosage instructions
    pub sequence: Option<types::Integer>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Supplemental instruction or warnings to the patient - e.g. "with
    /// meals", "may cause drowsiness"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_instruction: Vec<types::CodeableConcept>,

    /// Patient or consumer oriented instructions
    pub patient_instruction: Option<types::String>,
    /// Primitive extension sibling for [`patient_instruction`](Self::patient_instruction) (FHIR `_patientInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_patientInstruction")]
    pub patient_instruction_ext: Option<types::Element>,

    /// When medication should be administered
    pub timing: Option<types::Timing>,

    /// Take "as needed" (for x)
    /// The `Dosage.asNeeded[x]` choice element (0..1); see [`DosageAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<DosageAsNeeded>,

    /// Body site to administer to
    pub site: Option<types::CodeableConcept>,

    /// How drug should enter body
    pub route: Option<types::CodeableConcept>,

    /// Technique for administering medication
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dose_and_rate: Vec<DosageDoseAndRate>,

    /// Upper limit on medication per unit of time
    pub max_dose_per_period: Option<types::Ratio>,

    /// Upper limit on medication per administration
    pub max_dose_per_administration: Option<types::Quantity>,

    /// Upper limit on medication per lifetime of the patient
    pub max_dose_per_lifetime: Option<types::Quantity>,
}

/// The amount of medication administered.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::dosage::DosageDoseAndRate;
/// use fhir::r4b::types;
///
/// let value = DosageDoseAndRate {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DosageDoseAndRate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct DosageDoseAndRate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The kind of dose or rate specified
    pub r#type: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    /// The `Dosage.doseAndRate.dose[x]` choice element (0..1); see [`DosageDoseAndRateDose`].
    #[serde(flatten)]
    pub dose: Option<DosageDoseAndRateDose>,

    /// Amount of medication per unit of time
    /// The `Dosage.doseAndRate.rate[x]` choice element (0..1); see [`DosageDoseAndRateRate`].
    #[serde(flatten)]
    pub rate: Option<DosageDoseAndRateRate>,
}

/// The `Dosage.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum DosageAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `Dosage.doseAndRate.dose[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum DosageDoseAndRateDose {
    /// `doseRange` variant.
    #[fhir("doseRange")]
    Range(Box<types::Range>),
    /// `doseQuantity` variant.
    #[fhir("doseQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `Dosage.doseAndRate.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum DosageDoseAndRateRate {
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
    /// `rateRange` variant.
    #[fhir("rateRange")]
    Range(Box<types::Range>),
    /// `rateQuantity` variant.
    #[fhir("rateQuantity")]
    Quantity(Box<types::Quantity>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Dosage;

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
