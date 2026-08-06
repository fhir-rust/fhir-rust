//! ResearchSubject
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
//!
//! Version: 6.0.0-ballot3
//!
//! Participant or object which is the recipient of investigative activities in
//! a study
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A ResearchSubject is a participant or object which is the recipient of
/// investigative activities in a research study.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_subject::ResearchSubject;
/// use fhir::r6::types;
///
/// let value = ResearchSubject {
///     assigned_comparison_group: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `assignedComparisonGroup` is the name this serializes to on the wire.
/// assert_eq!(json["assignedComparisonGroup"], ::serde_json::json!("pat-1"));
///
/// let back: ResearchSubject = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ResearchSubject {
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

    /// Business Identifier for research subject in a study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Start and end of participation
    pub period: Option<types::Period>,

    /// Study subject is part of
    pub study: types::Reference,

    /// Who or what is part of study
    pub subject: types::Reference,

    /// A duration in the lifecycle of the ResearchSubject within a
    /// ResearchStudy
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_state: Vec<ResearchSubjectSubjectState>,

    /// A significant event in the progress of a ResearchSubject
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_milestone: Vec<ResearchSubjectSubjectMilestone>,

    /// What path should be followed
    pub assigned_comparison_group: Option<types::Id>,
    /// Primitive extension sibling for [`assigned_comparison_group`](Self::assigned_comparison_group) (FHIR `_assignedComparisonGroup`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_assignedComparisonGroup")]
    pub assigned_comparison_group_ext: Option<types::Element>,

    /// What path was followed
    pub actual_comparison_group: Option<types::Id>,
    /// Primitive extension sibling for [`actual_comparison_group`](Self::actual_comparison_group) (FHIR `_actualComparisonGroup`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actualComparisonGroup")]
    pub actual_comparison_group_ext: Option<types::Element>,

    /// Agreement to participate in study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consent: Vec<types::Reference>,
}

/// A significant event in the progress of a ResearchSubject.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::research_subject::ResearchSubjectSubjectMilestone;
///
/// let value = ResearchSubjectSubjectMilestone::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ResearchSubjectSubjectMilestone = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ResearchSubjectSubjectMilestone {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// SignedUp | Screened | Randomized
    pub milestone: ::vec1::Vec1<types::CodeableConcept>,

    /// The date/time when this milestone event was completed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    pub reason: Option<types::CodeableConcept>,
}

/// A duration in the lifecycle of the ResearchSubject within a ResearchStudy.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_subject::ResearchSubjectSubjectState;
/// use fhir::r6::types;
///
/// let value = ResearchSubjectSubjectState {
///     end_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `endDate` is the name this serializes to on the wire.
/// assert_eq!(json["endDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ResearchSubjectSubjectState = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ResearchSubjectSubjectState {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// candidate | in-prescreening | in-screening | eligible | ineligible |
    /// on-study | on-study-intervention | in-follow-up | off-study
    pub code: types::CodeableConcept,

    /// The date a research subject entered the given state
    pub start_date: types::DateTime,
    /// Primitive extension sibling for [`start_date`](Self::start_date) (FHIR `_startDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_startDate")]
    pub start_date_ext: Option<types::Element>,

    /// The date a research subject exited or left the given state
    pub end_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`end_date`](Self::end_date) (FHIR `_endDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endDate")]
    pub end_date_ext: Option<types::Element>,

    /// State change reason
    pub reason: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ResearchSubject;

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
