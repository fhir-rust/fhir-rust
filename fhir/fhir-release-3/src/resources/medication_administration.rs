//! MedicationAdministration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
//!
//!
//!
//! Administration of medication to a patient
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MedicationAdministration Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication_administration::MedicationAdministration;
/// use fhir::r3::types;
///
/// let value = MedicationAdministration {
///     not_given: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `notGiven` is the name this serializes to on the wire.
/// assert_eq!(json["notGiven"], ::serde_json::json!(true));
///
/// let back: MedicationAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
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

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// in-progress | on-hold | completed | entered-in-error | stopped |
    /// unknown
    pub status: crate::coded::Coded<crate::r3::codes::MedicationAdminStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type of medication usage
    pub category: Option<types::CodeableConcept>,

    /// What was administered
    /// The `MedicationAdministration.medication[x]` choice element (1..1); see [`MedicationAdministrationMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationAdministrationMedication>,

    /// Who received medication
    pub subject: types::Reference,

    /// Encounter or Episode of Care administered as part of
    pub context: Option<types::Reference>,

    /// Additional information to support administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Start and end time of administration
    /// The `MedicationAdministration.effective[x]` choice element (1..1); see [`MedicationAdministrationEffective`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub effective: Option<MedicationAdministrationEffective>,

    /// Who administered substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<MedicationAdministrationPerformer>,

    /// True if medication not administered
    pub not_given: Option<types::Boolean>,
    /// Primitive extension sibling for [`not_given`](Self::not_given) (FHIR `_notGiven`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_notGiven")]
    pub not_given_ext: Option<types::Element>,

    /// Reason administration not performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_not_given: Vec<types::CodeableConcept>,

    /// Reason administration performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition or Observation that supports why the medication was
    /// administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Request administration performed against
    pub prescription: Option<types::Reference>,

    /// Device used to administer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference>,

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
/// use fhir::r3::resources::medication_administration::MedicationAdministrationDosage;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MedicationAdministrationDosage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

/// The individual who was responsible for giving the medication to the
/// patient.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication_administration::MedicationAdministrationPerformer;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MedicationAdministrationPerformer {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Individual who was performing
    pub actor: types::Reference,

    /// Organization organization was acting for
    pub on_behalf_of: Option<types::Reference>,
}

/// The `MedicationAdministration.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationAdministration.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

/// The `MedicationAdministration.dosage.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
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
