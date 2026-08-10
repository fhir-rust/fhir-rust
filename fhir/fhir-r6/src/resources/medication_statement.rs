//! MedicationStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
//!
//! Version: 6.0.0-ballot3
//!
//! Record of medication being taken by a patient
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a medication that is being consumed by a patient. A
/// MedicationStatement may indicate that the patient may be taking the
/// medication now or has taken the medication in the past or will be taking
/// the medication in the future. The source of this information can be the
/// patient, significant other (such as a family member or spouse), or a
/// clinician. A common scenario where this information is captured is during
/// the history taking process during a patient visit or stay. The medication
/// information may come from sources such as the patient's memory, from a
/// prescription bottle, or from a list of medications the patient, clinician
/// or other party maintains. The primary difference between a
/// medicationstatement and a medicationadministration is that the medication
/// administration has complete administration information and is based on
/// actual administration information from the person who administered the
/// medication. A medicationstatement is often, if not always, less specific.
/// There is no required date/time when the medication was administered, in
/// fact we only know that a source has reported the patient is taking this
/// medication, where details such as time, quantity, or rate or even
/// medication product may be incomplete or missing or less precise. As stated
/// earlier, the Medication Statement information may come from the patient's
/// memory, from a prescription bottle or from a list of medications the
/// patient, clinician or other party maintains. Medication administration is
/// more formal and is not missing detailed information.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_statement::MedicationStatement;
/// use fhir::r6::types;
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
#[serde(from = "MedicationStatementDe")]
#[fhir_version("r6")]
pub struct MedicationStatement {
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

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// recorded | entered-in-error | draft
    pub status: crate::coded::Coded<crate::r6::codes::MedicationStatementStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type of medication statement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// What medication was taken
    pub medication: types::CodeableReference,

    /// Who is/was taking the medication
    pub subject: types::Reference,

    /// Encounter associated with MedicationStatement
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// The date/time or interval when the medication is/was/will be taken
    /// The `MedicationStatement.effective[x]` choice element (0..1); see [`MedicationStatementEffective`].
    #[serde(flatten)]
    pub effective: Option<MedicationStatementEffective>,

    /// When the usage was asserted?
    pub date_asserted: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_asserted`](Self::date_asserted) (FHIR `_dateAsserted`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateAsserted")]
    pub date_asserted_ext: Option<types::Element>,

    /// Who/What authored the statement
    pub author: Option<types::Reference>,

    /// Person or organization that provided the information about the taking
    /// of this medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_source: Vec<types::Reference>,

    /// Link to information used to derive the MedicationStatement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// Reason for why the medication is being/was taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Further information about the usage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Link to information relevant to the usage of a medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_clinical_information: Vec<types::Reference>,

    /// Full representation of the dosage instructions
    pub rendered_dosage_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`rendered_dosage_instruction`](Self::rendered_dosage_instruction) (FHIR `_renderedDosageInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_renderedDosageInstruction")]
    pub rendered_dosage_instruction_ext: Option<types::Element>,

    /// Details of how medication is/was taken or should be taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage: Vec<types::Dosage>,

    /// Indicates whether the medication is or is not being consumed or
    /// administered
    pub adherence: Option<MedicationStatementAdherence>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationStatementDe {
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
    part_of: Vec<types::Reference>,
    status: crate::coded::Coded<crate::r6::codes::MedicationStatementStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    medication: types::CodeableReference,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(flatten)]
    effective: crate::r6::choice::Slot<MedicationStatementEffective>,
    date_asserted: Option<types::DateTime>,
    #[serde(rename = "_dateAsserted")]
    date_asserted_ext: Option<types::Element>,
    author: Option<types::Reference>,
    #[serde(default)]
    information_source: Vec<types::Reference>,
    #[serde(default)]
    derived_from: Vec<types::Reference>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    related_clinical_information: Vec<types::Reference>,
    rendered_dosage_instruction: Option<types::Markdown>,
    #[serde(rename = "_renderedDosageInstruction")]
    rendered_dosage_instruction_ext: Option<types::Element>,
    #[serde(default)]
    dosage: Vec<types::Dosage>,
    adherence: Option<MedicationStatementAdherence>,
}

impl ::core::convert::From<MedicationStatementDe> for MedicationStatement {
    fn from(v: MedicationStatementDe) -> Self {
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
            part_of: v.part_of,
            status: v.status,
            status_ext: v.status_ext,
            category: v.category,
            medication: v.medication,
            subject: v.subject,
            encounter: v.encounter,
            effective: v.effective.0,
            date_asserted: v.date_asserted,
            date_asserted_ext: v.date_asserted_ext,
            author: v.author,
            information_source: v.information_source,
            derived_from: v.derived_from,
            reason: v.reason,
            note: v.note,
            related_clinical_information: v.related_clinical_information,
            rendered_dosage_instruction: v.rendered_dosage_instruction,
            rendered_dosage_instruction_ext: v.rendered_dosage_instruction_ext,
            dosage: v.dosage,
            adherence: v.adherence,
        }
    }
}

/// Indicates whether the medication is or is not being consumed or
/// administered.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::medication_statement::MedicationStatementAdherence;
/// use fhir::r6::types;
///
/// let value = MedicationStatementAdherence {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationStatementAdherence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MedicationStatementAdherence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of adherence
    pub code: types::CodeableConcept,

    /// Details of the reason for the current use of the medication
    pub reason: Option<types::CodeableConcept>,
}

/// The `MedicationStatement.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
    /// `effectiveTiming` variant.
    #[fhir("effectiveTiming")]
    Timing(Box<types::Timing>),
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
