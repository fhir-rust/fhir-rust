//! AllergyIntolerance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AllergyIntolerance
//!
//!
//!
//! Allergy or Intolerance (generally: Risk Of Adverse reaction to a substance)
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for AllergyIntolerance Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::allergy_intolerance::AllergyIntolerance;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Date(/time) when manifestations showed
    pub onset: Option<types::DateTime>,
    /// Primitive extension sibling for [`onset`](Self::onset) (FHIR `_onset`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_onset")]
    pub onset_ext: Option<types::Element>,

    /// When recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Who recorded the sensitivity
    pub recorder: Option<types::Reference>,

    /// Who the sensitivity is for
    pub patient: types::Reference,

    /// Source of the information about the allergy
    pub reporter: Option<types::Reference>,

    /// Substance, (or class) considered to be responsible for risk
    pub substance: types::CodeableConcept,

    /// active | unconfirmed | confirmed | inactive | resolved | refuted |
    /// entered-in-error
    pub status: Option<crate::coded::Coded<crate::r2::codes::AllergyIntoleranceStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// CRITL | CRITH | CRITU
    pub criticality: Option<crate::coded::Coded<crate::r2::codes::AllergyIntoleranceCriticality>>,
    /// Primitive extension sibling for [`criticality`](Self::criticality) (FHIR `_criticality`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criticality")]
    pub criticality_ext: Option<types::Element>,

    /// allergy | intolerance - Underlying mechanism (if known)
    pub r#type: Option<crate::coded::Coded<crate::r2::codes::AllergyIntoleranceType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// food | medication | environment | other - Category of Substance
    pub category: Option<crate::coded::Coded<crate::r2::codes::AllergyIntoleranceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// Date(/time) of last known occurrence of a reaction
    pub last_occurence: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_occurence`](Self::last_occurence) (FHIR `_lastOccurence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastOccurence")]
    pub last_occurence_ext: Option<types::Element>,

    /// Additional text not captured in other fields
    pub note: Option<types::Annotation>,

    /// Adverse Reaction Events linked to exposure to substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<AllergyIntoleranceReaction>,
}

/// Details about each adverse reaction event linked to exposure to the
/// identified Substance.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::allergy_intolerance::AllergyIntoleranceReaction;
///
/// let value = AllergyIntoleranceReaction::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AllergyIntoleranceReaction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct AllergyIntoleranceReaction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific substance considered to be responsible for event
    pub substance: Option<types::CodeableConcept>,

    /// unlikely | likely | confirmed - clinical certainty about the specific
    /// substance
    pub certainty: Option<crate::coded::Coded<crate::r2::codes::ReactionEventCertainty>>,
    /// Primitive extension sibling for [`certainty`](Self::certainty) (FHIR `_certainty`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_certainty")]
    pub certainty_ext: Option<types::Element>,

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
    pub severity: Option<crate::coded::Coded<crate::r2::codes::ReactionEventSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// How the subject was exposed to the substance
    pub exposure_route: Option<types::CodeableConcept>,

    /// Text about event not captured in other fields
    pub note: Option<types::Annotation>,
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
