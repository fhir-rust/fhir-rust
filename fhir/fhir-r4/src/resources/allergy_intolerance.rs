//! AllergyIntolerance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
//!
//! Version: 4.0.1
//!
//! Allergy or Intolerance (generally: Risk of adverse reaction to a substance)
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Risk of harmful or undesirable, physiological response which is unique to
/// an individual and associated with exposure to a substance.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::allergy_intolerance::AllergyIntolerance;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct AllergyIntolerance {
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

    /// External ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | inactive | resolved
    pub clinical_status: Option<types::CodeableConcept>,

    /// unconfirmed | confirmed | refuted | entered-in-error
    pub verification_status: Option<types::CodeableConcept>,

    /// allergy | intolerance - Underlying mechanism (if known)
    pub r#type: Option<crate::coded::Coded<crate::r4::codes::AllergyIntoleranceType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// food | medication | environment | biologic
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub category:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r4::codes::AllergyIntoleranceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category_ext: Vec<Option<types::Element>>,

    /// low | high | unable-to-assess
    pub criticality: Option<crate::coded::Coded<crate::r4::codes::AllergyIntoleranceCriticality>>,
    /// Primitive extension sibling for [`criticality`](Self::criticality) (FHIR `_criticality`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criticality")]
    pub criticality_ext: Option<types::Element>,

    /// Code that identifies the allergy or intolerance
    pub code: Option<types::CodeableConcept>,

    /// Who the sensitivity is for
    pub patient: types::Reference<crate::r4::resources::Patient>,

    /// Encounter when the allergy or intolerance was asserted
    pub encounter: Option<types::Reference<crate::r4::resources::Encounter>>,

    /// When allergy or intolerance was identified
    /// The `AllergyIntolerance.onset[x]` choice element (0..1); see [`AllergyIntoleranceOnset`].
    #[serde(flatten)]
    pub onset: Option<AllergyIntoleranceOnset>,

    /// Date first version of the resource instance was recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Who recorded the sensitivity
    pub recorder: Option<types::Reference>,

    /// Source of the information about the allergy
    pub asserter: Option<types::Reference>,

    /// Date(/time) of last known occurrence of a reaction
    pub last_occurrence: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_occurrence`](Self::last_occurrence) (FHIR `_lastOccurrence`):
    /// carries `id` and/or `extension` for the primitive value.
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
    contained: Vec<crate::r4::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    clinical_status: Option<types::CodeableConcept>,
    verification_status: Option<types::CodeableConcept>,
    r#type: Option<crate::coded::Coded<crate::r4::codes::AllergyIntoleranceType>>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    #[serde(default)]
    category:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r4::codes::AllergyIntoleranceCategory>>,
    #[serde(rename = "_category")]
    #[serde(default)]
    category_ext: Vec<Option<types::Element>>,
    criticality: Option<crate::coded::Coded<crate::r4::codes::AllergyIntoleranceCriticality>>,
    #[serde(rename = "_criticality")]
    criticality_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    patient: types::Reference<crate::r4::resources::Patient>,
    encounter: Option<types::Reference<crate::r4::resources::Encounter>>,
    #[serde(flatten)]
    onset: crate::r4::choice::Slot<AllergyIntoleranceOnset>,
    recorded_date: Option<types::DateTime>,
    #[serde(rename = "_recordedDate")]
    recorded_date_ext: Option<types::Element>,
    recorder: Option<types::Reference>,
    asserter: Option<types::Reference>,
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
            type_ext: v.type_ext,
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
            recorder: v.recorder,
            asserter: v.asserter,
            last_occurrence: v.last_occurrence,
            last_occurrence_ext: v.last_occurrence_ext,
            note: v.note,
            reaction: v.reaction,
        }
    }
}

/// Details about each adverse reaction event linked to exposure to the
/// identified substance.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::allergy_intolerance::AllergyIntoleranceReaction;
///
/// let value = AllergyIntoleranceReaction::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AllergyIntoleranceReaction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct AllergyIntoleranceReaction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific substance or pharmaceutical product considered to be
    /// responsible for event
    pub substance: Option<types::CodeableConcept>,

    /// Clinical symptoms/signs associated with the Event
    pub manifestation: ::vec1::Vec1<types::CodeableConcept>,

    /// Description of the event as a whole
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Date(/time) when manifestations showed
    pub onset: Option<types::DateTime>,
    /// Primitive extension sibling for [`onset`](Self::onset) (FHIR `_onset`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_onset")]
    pub onset_ext: Option<types::Element>,

    /// mild | moderate | severe (of event as a whole)
    pub severity: Option<crate::coded::Coded<crate::r4::codes::ReactionEventSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// How the subject was exposed to the substance
    pub exposure_route: Option<types::CodeableConcept>,

    /// Text about event not captured in other fields
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// The `AllergyIntolerance.onset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum AllergyIntoleranceOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
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
    String(crate::r4::choice::Primitive<types::String>),
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
