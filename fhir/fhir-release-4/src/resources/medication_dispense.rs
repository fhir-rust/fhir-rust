//! MedicationDispense
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationDispense
//!
//! Version: 4.0.1
//!
//! Dispensing a medication to a named patient
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Indicates that a medication product is to be or has been dispensed for a
/// named person/patient. This includes a description of the medication product
/// (supply) provided and the instructions for administering the medication.
/// The medication dispense is the result of a pharmacy system responding to a
/// medication order.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_dispense::MedicationDispense;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct MedicationDispense {
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

    /// Event that dispense is part of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// preparation | in-progress | cancelled | on-hold | completed |
    /// entered-in-error | stopped | declined | unknown
    pub status: crate::coded::Coded<crate::r4::codes::MedicationdispenseStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Why a dispense was not performed
    /// The `MedicationDispense.statusReason[x]` choice element (0..1); see [`MedicationDispenseStatusReason`].
    #[serde(flatten)]
    pub status_reason: Option<MedicationDispenseStatusReason>,

    /// Type of medication dispense
    pub category: Option<types::CodeableConcept>,

    /// What medication was supplied
    /// The `MedicationDispense.medication[x]` choice element (1..1); see [`MedicationDispenseMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationDispenseMedication>,

    /// Who the dispense is for
    pub subject: Option<types::Reference>,

    /// Encounter / Episode associated with event
    pub context: Option<types::Reference>,

    /// Information that supports the dispensing of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// Who performed event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<MedicationDispensePerformer>,

    /// Where the dispense occurred
    pub location: Option<types::Reference>,

    /// Medication order that authorizes the dispense
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizing_prescription: Vec<types::Reference>,

    /// Trial fill, partial fill, emergency fill, etc.
    pub r#type: Option<types::CodeableConcept>,

    /// Amount dispensed
    pub quantity: Option<types::Quantity>,

    /// Amount of medication expressed as a timing amount
    pub days_supply: Option<types::Quantity>,

    /// When product was packaged and reviewed
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// How the medication is to be used by the patient or administered by the
    /// caregiver
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage_instruction: Vec<types::Dosage>,

    /// Whether a substitution was performed on the dispense
    pub substitution: Option<MedicationDispenseSubstitution>,

    /// Clinical issue with action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detected_issue: Vec<types::Reference>,

    /// A list of relevant lifecycle events
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference>,
}

/// Indicates who or what performed the event.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_dispense::MedicationDispensePerformer;
/// use fhir::r4::types;
///
/// let value = MedicationDispensePerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationDispensePerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who performed the dispense and what they did
    pub function: Option<types::CodeableConcept>,

    /// Individual who was performing
    pub actor: types::Reference,
}

/// Indicates whether or not substitution was made as part of the dispense. In
/// some cases, substitution will be expected but does not happen, in other
/// cases substitution is not expected but does happen. This block explains
/// what substitution did or did not happen and why. If nothing is specified,
/// substitution was not done.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medication_dispense::MedicationDispenseSubstitution;
/// use fhir::r4::types;
///
/// let value = MedicationDispenseSubstitution {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationDispenseSubstitution = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicationDispenseSubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether a substitution was or was not performed on the dispense
    pub was_substituted: types::Boolean,
    /// Primitive extension sibling for [`was_substituted`](Self::was_substituted) (FHIR `_wasSubstituted`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_wasSubstituted")]
    pub was_substituted_ext: Option<types::Element>,

    /// Code signifying whether a different drug was dispensed from what was
    /// prescribed
    pub r#type: Option<types::CodeableConcept>,

    /// Why was substitution made
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Who is responsible for the substitution
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_party: Vec<types::Reference>,
}

/// The `MedicationDispense.statusReason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationDispenseStatusReason {
    /// `statusReasonCodeableConcept` variant.
    #[fhir("statusReasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `statusReasonReference` variant.
    #[fhir("statusReasonReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationDispense.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationDispenseMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
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
