//! MedicationStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
//!
//!
//!
//! Record of medication being taken by a patient
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MedicationStatement Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_statement::MedicationStatement;
/// use fhir::r2::types;
///
/// let value = MedicationStatement {
///     date_asserted: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateAsserted` is the name this serializes to on the wire.
/// assert_eq!(json["dateAsserted"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicationStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationStatement {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who is/was taking the medication
    pub patient: types::Reference<crate::r2::resources::Patient>,

    pub information_source: Option<types::Reference>,

    /// When the statement was asserted?
    pub date_asserted: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_asserted`](Self::date_asserted) (FHIR `_dateAsserted`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateAsserted")]
    pub date_asserted_ext: Option<types::Element>,

    /// active | completed | entered-in-error | intended
    pub status: crate::coded::Coded<crate::r2::codes::MedicationStatementStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// True if medication is/was not being taken
    pub was_not_taken: Option<types::Boolean>,
    /// Primitive extension sibling for [`was_not_taken`](Self::was_not_taken) (FHIR `_wasNotTaken`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_wasNotTaken")]
    pub was_not_taken_ext: Option<types::Element>,

    /// True if asserting medication was not given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_not_taken: Vec<types::CodeableConcept>,

    /// The `MedicationStatement.reasonForUse[x]` choice element (0..1); see [`MedicationStatementReasonForUse`].
    #[serde(flatten)]
    pub reason_for_use: Option<MedicationStatementReasonForUse>,

    /// Over what period was medication consumed?
    /// The `MedicationStatement.effective[x]` choice element (0..1); see [`MedicationStatementEffective`].
    #[serde(flatten)]
    pub effective: Option<MedicationStatementEffective>,

    /// Further information about the statement
    pub note: Option<types::String>,
    /// Primitive extension sibling for [`note`](Self::note) (FHIR `_note`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_note")]
    pub note_ext: Option<types::Element>,

    /// Additional supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// What medication was taken
    /// The `MedicationStatement.medication[x]` choice element (1..1); see [`MedicationStatementMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationStatementMedication>,

    /// Details of how medication was taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage: Vec<MedicationStatementDosage>,
}

/// Indicates how the medication is/was used by the patient.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_statement::MedicationStatementDosage;
/// use fhir::r2::types;
///
/// let value = MedicationStatementDosage {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: MedicationStatementDosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationStatementDosage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reported dosage information
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// When/how often was medication taken
    pub timing: Option<types::Timing>,

    /// Take "as needed" (for x)
    /// The `MedicationStatement.dosage.asNeeded[x]` choice element (0..1); see [`MedicationStatementDosageAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<MedicationStatementDosageAsNeeded>,

    /// Where (on body) medication is/was administered
    /// The `MedicationStatement.dosage.site[x]` choice element (0..1); see [`MedicationStatementDosageSite`].
    #[serde(flatten)]
    pub site: Option<MedicationStatementDosageSite>,

    /// How the medication entered the body
    pub route: Option<types::CodeableConcept>,

    /// Technique used to administer medication
    pub method: Option<types::CodeableConcept>,

    /// Amount administered in one dose
    /// The `MedicationStatement.dosage.quantity[x]` choice element (0..1); see [`MedicationStatementDosageQuantity`].
    #[serde(flatten)]
    pub quantity: Option<MedicationStatementDosageQuantity>,

    /// Dose quantity per unit of time
    /// The `MedicationStatement.dosage.rate[x]` choice element (0..1); see [`MedicationStatementDosageRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationStatementDosageRate>,

    /// Maximum dose that was consumed per unit of time
    pub max_dose_per_period: Option<types::Ratio>,
}

/// The `MedicationStatement.reasonForUse[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementReasonForUse {
    /// `reasonForUseCodeableConcept` variant.
    #[fhir("reasonForUseCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonForUseReference` variant.
    #[fhir("reasonForUseReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationStatement.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

/// The `MedicationStatement.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationStatement.dosage.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementDosageAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `MedicationStatement.dosage.site[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementDosageSite {
    /// `siteCodeableConcept` variant.
    #[fhir("siteCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `siteReference` variant.
    #[fhir("siteReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationStatement.dosage.quantity[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementDosageQuantity {
    /// `quantityQuantity` variant.
    #[fhir("quantityQuantity")]
    Quantity(Box<types::Quantity>),
    /// `quantityRange` variant.
    #[fhir("quantityRange")]
    Range(Box<types::Range>),
}

/// The `MedicationStatement.dosage.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementDosageRate {
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
    type T = MedicationStatement;

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
