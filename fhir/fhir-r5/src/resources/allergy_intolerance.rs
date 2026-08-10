//! AllergyIntolerance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
//!
//! Version: 5.0.0
//!
//! AllergyIntolerance Resource: Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Risk of harmful or undesirable, physiological response which is unique to an
/// individual and associated with exposure to a substance.
///
/// AllergyIntolerance records a clinical assessment of a propensity, or a
/// potential risk to an individual, of an adverse reaction upon future exposure
/// to the specified substance, or class of substance. It is used in FHIR R5 to
/// capture the allergy or intolerance itself along with any reaction events that
/// have been observed and the participants who asserted or recorded the record.
///
/// Clinically, this resource supports decision support and safety checking (for
/// example, prescribing or order-entry alerts), and it distinguishes between an
/// allergy (an immune-mediated response) and an intolerance (a non-immune
/// adverse reaction), while also allowing the underlying mechanism to be left
/// unspecified when it is not known. A single resource instance describes one
/// propensity for a subject; each observed occurrence of a reaction is recorded
/// as a repeating `reaction` component with details such as manifestation,
/// severity, and onset, and the individuals who reported or verified the
/// assessment can be captured via the `participant` component.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject for whom
///   the allergy or intolerance is recorded, referenced via the `patient` field.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for coded
///   values such as `clinical_status`, `verification_status`, `type`, and `code`.
/// - `Encounter` — optionally referenced via `encounter` to indicate the
///   clinical context in which the allergy or intolerance was asserted.
/// - `Condition` and `Observation` — related clinical resources that may also
///   reference or corroborate an allergy or intolerance assessment.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::allergy_intolerance::AllergyIntolerance;
/// use fhir::r5::types;
///
/// let value = AllergyIntolerance {
///     recorded_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `recordedDate` is the name this serializes to on the wire.
/// assert_eq!(json["recordedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AllergyIntolerance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "AllergyIntoleranceDe")]
pub struct AllergyIntolerance {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | inactive | resolved - current clinical status of this allergy or intolerance
    pub clinical_status: Option<types::CodeableConcept>,

    /// unconfirmed | presumed | confirmed | refuted | entered-in-error - assertion confidence
    pub verification_status: Option<types::CodeableConcept>,

    /// allergy | intolerance - Underlying mechanism (if known)
    pub r#type: Option<types::CodeableConcept>,

    /// food | medication | environment | biologic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<crate::r5::coded::Coded<crate::r5::codes::AllergyIntoleranceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`).
    #[serde(rename = "_category")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category_ext: Vec<Option<types::Element>>,

    /// low | high | unable-to-assess - estimated risk of harm from future exposure
    pub criticality:
        Option<crate::r5::coded::Coded<crate::r5::codes::AllergyIntoleranceCriticality>>,
    /// Primitive extension sibling for [`criticality`](Self::criticality) (FHIR `_criticality`).
    #[serde(rename = "_criticality")]
    pub criticality_ext: Option<types::Element>,

    /// Code that identifies the allergy or intolerance, e.g. a substance or product code
    pub code: Option<types::CodeableConcept>,

    /// Who the allergy or intolerance is for; typically a reference to a [`Patient`](crate::r5::resources::patient::Patient)
    pub patient: types::Reference<crate::r5::resources::Patient>,

    /// Encounter when the allergy or intolerance was asserted
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `AllergyIntolerance.onset[x]` choice element (0..1); see [`AllergyIntoleranceOnset`].
    #[serde(flatten)]
    pub onset: Option<AllergyIntoleranceOnset>,

    /// Date allergy or intolerance was first recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`).
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Who or what participated in the activities related to the allergy or
    /// intolerance and how they were involved
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<AllergyIntoleranceParticipant>,

    /// Date(/time) of last known occurrence of a reaction
    pub last_occurrence: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_occurrence`](Self::last_occurrence) (FHIR `_lastOccurrence`).
    #[serde(rename = "_lastOccurrence")]
    pub last_occurrence_ext: Option<types::Element>,

    /// Additional text not captured in other fields
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Adverse Reaction Events linked to exposure to substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<AllergyIntoleranceReaction>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllergyIntoleranceDe {
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
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    clinical_status: Option<types::CodeableConcept>,
    verification_status: Option<types::CodeableConcept>,
    r#type: Option<types::CodeableConcept>,
    #[serde(default)]
    category: Vec<crate::r5::coded::Coded<crate::r5::codes::AllergyIntoleranceCategory>>,
    #[serde(rename = "_category")]
    #[serde(default)]
    category_ext: Vec<Option<types::Element>>,
    criticality: Option<crate::r5::coded::Coded<crate::r5::codes::AllergyIntoleranceCriticality>>,
    #[serde(rename = "_criticality")]
    criticality_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    patient: types::Reference<crate::r5::resources::Patient>,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(flatten)]
    onset: crate::r5::choice::Slot<AllergyIntoleranceOnset>,
    recorded_date: Option<types::DateTime>,
    #[serde(rename = "_recordedDate")]
    recorded_date_ext: Option<types::Element>,
    #[serde(default)]
    participant: Vec<AllergyIntoleranceParticipant>,
    last_occurrence: Option<types::DateTime>,
    #[serde(rename = "_lastOccurrence")]
    last_occurrence_ext: Option<types::Element>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    reaction: Vec<AllergyIntoleranceReaction>,
}

impl ::core::convert::From<AllergyIntoleranceDe> for AllergyIntolerance {
    fn from(v: AllergyIntoleranceDe) -> Self {
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
            clinical_status: v.clinical_status,
            verification_status: v.verification_status,
            r#type: v.r#type,
            category: v.category,
            category_ext: v.category_ext,
            criticality: v.criticality,
            criticality_ext: v.criticality_ext,
            code: v.code,
            patient: v.patient,
            encounter: v.encounter,
            onset: v.onset.0,
            recorded_date: v.recorded_date,
            recorded_date_ext: v.recorded_date_ext,
            participant: v.participant,
            last_occurrence: v.last_occurrence,
            last_occurrence_ext: v.last_occurrence_ext,
            note: v.note,
            reaction: v.reaction,
        }
    }
}

/// Who or what participated in the activities related to the allergy or
/// intolerance and how they were involved.
/// # Examples
///
/// ```
/// use fhir::r5::resources::allergy_intolerance::AllergyIntoleranceParticipant;
/// use fhir::r5::types;
///
/// let value = AllergyIntoleranceParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AllergyIntoleranceParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntoleranceParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who or what participated in the activities related to the allergy or
    /// intolerance
    pub actor: types::Reference,
}

/// Adverse Reaction Events linked to exposure to substance.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::allergy_intolerance::AllergyIntoleranceReaction;
///
/// let value = AllergyIntoleranceReaction::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AllergyIntoleranceReaction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntoleranceReaction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific substance or pharmaceutical product considered to be responsible
    /// for event
    pub substance: Option<types::CodeableConcept>,

    /// Clinical symptoms/signs associated with the Event
    pub manifestation: vec1::Vec1<types::CodeableReference>,

    /// Description of the event as a whole
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Date(/time) when manifestations showed
    pub onset: Option<types::DateTime>,
    /// Primitive extension sibling for [`onset`](Self::onset) (FHIR `_onset`).
    #[serde(rename = "_onset")]
    pub onset_ext: Option<types::Element>,

    /// mild | moderate | severe (of event as a whole)
    pub severity: Option<crate::r5::coded::Coded<crate::r5::codes::ReactionEventSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`).
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// How the subject was exposed to the substance
    pub exposure_route: Option<types::CodeableConcept>,

    /// Text about event not captured in other fields
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AllergyIntolerance;

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
/// The `AllergyIntolerance.onset[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AllergyIntoleranceOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `onsetAge` variant.
    #[fhir("onsetAge")]
    Age(Box<types::Age>),
    /// `onsetPeriod` variant.
    #[fhir("onsetPeriod")]
    Period(Box<types::Period>),
    /// `onsetRange` variant.
    #[fhir("onsetRange")]
    Range(Box<types::Range>),
    /// `onsetString` variant.
    #[fhir("onsetString")]
    String(crate::r5::choice::Primitive<types::String>),
}
