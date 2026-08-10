//! MedicationStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationStatement
//!
//!
//!
//! Record of medication being taken by a patient
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MedicationStatement Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication_statement::MedicationStatement;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
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

    /// Encounter / Episode associated with MedicationStatement
    pub context: Option<types::Reference>,

    /// active | completed | entered-in-error | intended | stopped | on-hold
    pub status: crate::coded::Coded<crate::r3::codes::MedicationStatementStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type of medication usage
    pub category: Option<types::CodeableConcept>,

    /// What medication was taken
    /// The `MedicationStatement.medication[x]` choice element (1..1); see [`MedicationStatementMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationStatementMedication>,

    /// The date/time or interval when the medication was taken
    /// The `MedicationStatement.effective[x]` choice element (0..1); see [`MedicationStatementEffective`].
    #[serde(flatten)]
    pub effective: Option<MedicationStatementEffective>,

    /// When the statement was asserted?
    pub date_asserted: Option<types::DateTime>,
    /// Primitive extension sibling for [`date_asserted`](Self::date_asserted) (FHIR `_dateAsserted`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateAsserted")]
    pub date_asserted_ext: Option<types::Element>,

    /// Person or organization that provided the information about the taking
    /// of this medication
    pub information_source: Option<types::Reference>,

    /// Who is/was taking the medication
    pub subject: types::Reference,

    /// Additional supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// y | n | unk | na
    pub taken: crate::coded::Coded<crate::r3::codes::MedicationStatementTaken>,
    /// Primitive extension sibling for [`taken`](Self::taken) (FHIR `_taken`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_taken")]
    pub taken_ext: Option<types::Element>,

    /// True if asserting medication was not given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_not_taken: Vec<types::CodeableConcept>,

    /// Reason for why the medication is being/was taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition or observation that supports why the medication is being/was
    /// taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Further information about the statement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Details of how medication is/was taken or should be taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage: Vec<types::Dosage>,
}

/// The `MedicationStatement.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationStatement.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationStatementEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
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
