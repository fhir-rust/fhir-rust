//! AllergyIntolerance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
//!
//!
//!
//! Allergy or Intolerance (generally: Risk of adverse reaction to a substance)
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for AllergyIntolerance Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::allergy_intolerance::AllergyIntolerance;
/// use fhir::r3::types;
///
/// let value = AllergyIntolerance {
///     asserted_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `assertedDate` is the name this serializes to on the wire.
/// assert_eq!(json["assertedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AllergyIntolerance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AllergyIntolerance {
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

    /// External ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | inactive | resolved
    pub clinical_status: Option<crate::coded::Coded<crate::r3::codes::AllergyClinicalStatus>>,
    /// Primitive extension sibling for [`clinical_status`](Self::clinical_status) (FHIR `_clinicalStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_clinicalStatus")]
    pub clinical_status_ext: Option<types::Element>,

    /// unconfirmed | confirmed | refuted | entered-in-error
    pub verification_status: crate::coded::Coded<crate::r3::codes::AllergyVerificationStatus>,
    /// Primitive extension sibling for [`verification_status`](Self::verification_status) (FHIR `_verificationStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_verificationStatus")]
    pub verification_status_ext: Option<types::Element>,

    /// allergy | intolerance - Underlying mechanism (if known)
    pub r#type: Option<crate::coded::Coded<crate::r3::codes::AllergyIntoleranceType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// food | medication | environment | biologic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<crate::coded::Coded<crate::r3::codes::AllergyIntoleranceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category_ext: Vec<Option<types::Element>>,

    /// low | high | unable-to-assess
    pub criticality: Option<crate::coded::Coded<crate::r3::codes::AllergyIntoleranceCriticality>>,
    /// Primitive extension sibling for [`criticality`](Self::criticality) (FHIR `_criticality`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criticality")]
    pub criticality_ext: Option<types::Element>,

    /// Code that identifies the allergy or intolerance
    pub code: Option<types::CodeableConcept>,

    /// Who the sensitivity is for
    pub patient: types::Reference,

    /// When allergy or intolerance was identified
    /// The `AllergyIntolerance.onset[x]` choice element (0..1); see [`AllergyIntoleranceOnset`].
    #[serde(flatten)]
    pub onset: Option<AllergyIntoleranceOnset>,

    /// Date record was believed accurate
    pub asserted_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`asserted_date`](Self::asserted_date) (FHIR `_assertedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_assertedDate")]
    pub asserted_date_ext: Option<types::Element>,

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

/// Details about each adverse reaction event linked to exposure to the
/// identified substance.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::allergy_intolerance::AllergyIntoleranceReaction;
///
/// let value = AllergyIntoleranceReaction::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AllergyIntoleranceReaction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AllergyIntoleranceReaction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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
    pub severity: Option<crate::coded::Coded<crate::r3::codes::ReactionEventSeverity>>,
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
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum AllergyIntoleranceOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
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
    String(crate::r3::choice::Primitive<types::String>),
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
