//! Dosage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Dosage
//!
//!
//!
//! How the medication is/was taken or should be taken
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Dosage Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::dosage::Dosage;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct Dosage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

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

    /// Supplemental instruction - e.g. "with meals"
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

    /// Amount of medication per dose
    /// The `Dosage.dose[x]` choice element (0..1); see [`DosageDose`].
    #[serde(flatten)]
    pub dose: Option<DosageDose>,

    /// Upper limit on medication per unit of time
    pub max_dose_per_period: Option<types::Ratio>,

    /// Upper limit on medication per administration
    pub max_dose_per_administration: Option<types::Quantity>,

    /// Upper limit on medication per lifetime of the patient
    pub max_dose_per_lifetime: Option<types::Quantity>,

    /// Amount of medication per unit of time
    /// The `Dosage.rate[x]` choice element (0..1); see [`DosageRate`].
    #[serde(flatten)]
    pub rate: Option<DosageRate>,
}

/// The `Dosage.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DosageAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `Dosage.dose[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DosageDose {
    /// `doseRange` variant.
    #[fhir("doseRange")]
    Range(Box<types::Range>),
    /// `doseQuantity` variant.
    #[fhir("doseQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `Dosage.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DosageRate {
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
