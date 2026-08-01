//! MedicationOrder
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationOrder
//!
//!
//!
//! Prescription of medication to for patient
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MedicationOrder Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_order::MedicationOrder;
/// use fhir::r2::types;
///
/// let value = MedicationOrder {
///     date_written: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateWritten` is the name this serializes to on the wire.
/// assert_eq!(json["dateWritten"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicationOrder = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationOrder {
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

    /// When prescription was authorized
    pub date_written: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_written`](Self::date_written) (FHIR `_dateWritten`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateWritten")]
    pub date_written_ext: Option<types::Element>,

    /// active | on-hold | completed | entered-in-error | stopped | draft
    pub status: Option<crate::coded::Coded<crate::r2::codes::MedicationOrderStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// When prescription was stopped
    pub date_ended: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_ended`](Self::date_ended) (FHIR `_dateEnded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateEnded")]
    pub date_ended_ext: Option<types::Element>,

    /// Why prescription was stopped
    pub reason_ended: Option<types::CodeableConcept>,

    /// Who prescription is for
    pub patient: Option<types::Reference>,

    /// Who ordered the medication(s)
    pub prescriber: Option<types::Reference>,

    /// Created during encounter/admission/stay
    pub encounter: Option<types::Reference>,

    /// Reason or indication for writing the prescription
    /// The `MedicationOrder.reason[x]` choice element (0..1); see [`MedicationOrderReason`].
    #[serde(flatten)]
    pub reason: Option<MedicationOrderReason>,

    /// Information about the prescription
    pub note: Option<types::String>,
    /// Primitive extension sibling for [`note`](Self::note) (FHIR `_note`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_note")]
    pub note_ext: Option<types::Element>,

    /// Medication to be taken
    /// The `MedicationOrder.medication[x]` choice element (1..1); see [`MedicationOrderMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationOrderMedication>,

    /// How medication should be taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage_instruction: Vec<MedicationOrderDosageInstruction>,

    /// Medication supply authorization
    pub dispense_request: Option<MedicationOrderDispenseRequest>,

    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationOrderSubstitution>,

    /// An order/prescription that this supersedes
    pub prior_prescription: Option<types::Reference>,
}

/// Indicates the specific details for the dispense or medication supply part
/// of a medication order (also known as a Medication Prescription). Note that
/// this information is NOT always sent with the order. There may be in some
/// settings (e.g. hospitals) institutional or system support for completing
/// the dispense details in the pharmacy department.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_order::MedicationOrderDispenseRequest;
/// use fhir::r2::types;
///
/// let value = MedicationOrderDispenseRequest {
///     number_of_repeats_allowed: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfRepeatsAllowed` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfRepeatsAllowed"], ::serde_json::json!(1));
///
/// let back: MedicationOrderDispenseRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationOrderDispenseRequest {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Product to be supplied
    /// The `MedicationOrder.dispenseRequest.medication[x]` choice element (0..1); see [`MedicationOrderDispenseRequestMedication`].
    #[serde(flatten)]
    pub medication: Option<MedicationOrderDispenseRequestMedication>,

    /// Time period supply is authorized for
    pub validity_period: Option<types::Period>,

    /// Number of refills authorized
    pub number_of_repeats_allowed: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`number_of_repeats_allowed`](Self::number_of_repeats_allowed) (FHIR `_numberOfRepeatsAllowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfRepeatsAllowed")]
    pub number_of_repeats_allowed_ext: Option<types::Element>,

    /// Amount of medication to supply per dispense
    pub quantity: Option<types::Quantity>,

    /// Number of days supply per dispense
    pub expected_supply_duration: Option<types::Quantity>,
}

/// Indicates how the medication is to be used by the patient.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_order::MedicationOrderDosageInstruction;
/// use fhir::r2::types;
///
/// let value = MedicationOrderDosageInstruction {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: MedicationOrderDosageInstruction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationOrderDosageInstruction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Dosage instructions expressed as text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Supplemental instructions - e.g. "with meals"
    pub additional_instructions: Option<types::CodeableConcept>,

    /// When medication should be administered
    pub timing: Option<types::Timing>,

    /// Take "as needed" (for x)
    /// The `MedicationOrder.dosageInstruction.asNeeded[x]` choice element (0..1); see [`MedicationOrderDosageInstructionAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<MedicationOrderDosageInstructionAsNeeded>,

    /// Body site to administer to
    /// The `MedicationOrder.dosageInstruction.site[x]` choice element (0..1); see [`MedicationOrderDosageInstructionSite`].
    #[serde(flatten)]
    pub site: Option<MedicationOrderDosageInstructionSite>,

    /// How drug should enter body
    pub route: Option<types::CodeableConcept>,

    /// Technique for administering medication
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    /// The `MedicationOrder.dosageInstruction.dose[x]` choice element (0..1); see [`MedicationOrderDosageInstructionDose`].
    #[serde(flatten)]
    pub dose: Option<MedicationOrderDosageInstructionDose>,

    /// Amount of medication per unit of time
    /// The `MedicationOrder.dosageInstruction.rate[x]` choice element (0..1); see [`MedicationOrderDosageInstructionRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationOrderDosageInstructionRate>,

    /// Upper limit on medication per unit of time
    pub max_dose_per_period: Option<types::Ratio>,
}

/// Indicates whether or not substitution can or should be part of the
/// dispense. In some cases substitution must happen, in other cases
/// substitution must not happen, and in others it does not matter. This block
/// explains the prescriber's intent. If nothing is specified substitution may
/// be done.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::medication_order::MedicationOrderSubstitution;
/// use fhir::r2::types;
///
/// let value = MedicationOrderSubstitution {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: MedicationOrderSubstitution = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct MedicationOrderSubstitution {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// generic | formulary +
    pub r#type: types::CodeableConcept,

    /// Why should (not) substitution be made
    pub reason: Option<types::CodeableConcept>,
}

/// The `MedicationOrder.reason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationOrderReason {
    /// `reasonCodeableConcept` variant.
    #[fhir("reasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonReference` variant.
    #[fhir("reasonReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationOrder.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationOrderMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationOrder.dispenseRequest.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationOrderDispenseRequestMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationOrder.dosageInstruction.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationOrderDosageInstructionAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `MedicationOrder.dosageInstruction.site[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationOrderDosageInstructionSite {
    /// `siteCodeableConcept` variant.
    #[fhir("siteCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `siteReference` variant.
    #[fhir("siteReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationOrder.dosageInstruction.dose[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationOrderDosageInstructionDose {
    /// `doseRange` variant.
    #[fhir("doseRange")]
    Range(Box<types::Range>),
    /// `doseQuantity` variant.
    #[fhir("doseQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `MedicationOrder.dosageInstruction.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationOrderDosageInstructionRate {
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
    type T = MedicationOrder;

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
