//! MedicationAdministration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
//!
//! Version: 4.0.1
//!
//! Administration of medication to a patient
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes the event of a patient consuming or otherwise being administered
/// a medication. This may be as simple as swallowing a tablet or it may be a
/// long running infusion. Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_administration::MedicationAdministration;
/// use fhir::r4::types;
///
/// let value = MedicationAdministration {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
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
    pub instantiates: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiates")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_ext: Vec<Option<types::Element>>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// in-progress | not-done | on-hold | completed | entered-in-error |
    /// stopped | unknown
    pub status: crate::coded::Coded<crate::r4::codes::MedicationAdminStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason administration not performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

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

    /// Who performed the medication administration and what they did
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<MedicationAdministrationPerformer>,

    /// Reason administration performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition or observation that supports why the medication was
    /// administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Request administration performed against
    pub request: Option<types::Reference<crate::r4::resources::MedicationRequest>>,

    /// Device used to administer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::Reference<crate::r4::resources::Device>>,

    /// Information about the administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference<crate::r4::resources::Provenance>>,
}

/// Describes the medication dosage information details e.g. dose, rate, site,
/// route, etc.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_administration::MedicationAdministrationDosage;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// Indicates who or what performed the medication administration and how they
/// were involved.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_administration::MedicationAdministrationPerformer;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

    /// Who performed the medication administration
    pub actor: types::Reference,
}

/// The `MedicationAdministration.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

/// The `MedicationAdministration.dosage.rate[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
