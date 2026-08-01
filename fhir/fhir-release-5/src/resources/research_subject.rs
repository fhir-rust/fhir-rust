//! ResearchSubject
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchSubject
//!
//! Version: 5.0.0
//!
//! ResearchSubject Resource: A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A ResearchSubject is a participant or object which is the recipient of
/// investigative activities in a research study.
///
/// A ResearchSubject links a subject (such as a patient, group, specimen, or
/// device) to a specific ResearchStudy and records the details of that subject's
/// participation. It tracks the subject's status over time through a series of
/// progress states and milestones, the period of participation, and the
/// comparison group (arm) the subject was assigned to and actually followed. In
/// FHIR R5 it is commonly used to manage enrollment and consent within clinical
/// trials and other research activities.
///
/// Administratively, ResearchSubject acts as the join point between a research
/// study and the real-world entity taking part in it, allowing researchers,
/// coordinators, and downstream systems to answer questions such as who is
/// currently enrolled, which arm of the study a subject followed, when
/// participation began and ended, and what consent was obtained. Because the
/// resource captures identity, status, and study linkage separately from
/// clinical observations, it can be used alongside other resources to drive
/// eligibility screening, protocol deviation tracking, and study reporting
/// without duplicating clinical data already recorded elsewhere.
///
/// # Related resources
///
/// - `ResearchStudy` — the study this subject is participating in (referenced by `study`).
/// - [`Patient`](crate::r5::resources::patient::Patient) — a common type of
///   entity referenced by `subject`, though `subject` may also reference a
///   group, specimen, or device.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for
///   `progress.type`, `progress.subject_state`, `progress.milestone`, and
///   `progress.reason`.
/// - [`Reference`](crate::r5::types::Reference) — used for `study`, `subject`,
///   and `consent`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_subject::ResearchSubject;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubject {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for research subject in a study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The overall lifecycle status of this record: draft | active | retired | unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// A chronological record of the subject's states and milestones (such as
    /// screening, enrollment, or withdrawal) during the study.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub progress: Vec<ResearchSubjectProgress>,

    /// The overall start and end date of the subject's participation in the study.
    pub period: Option<types::Period>,

    /// Reference to the `ResearchStudy` that this subject is participating in.
    pub study: types::Reference,

    /// Reference to the individual or entity (for example a
    /// [`Patient`](crate::r5::resources::patient::Patient), group, specimen,
    /// or device) that is the subject of the research.
    pub subject: types::Reference,

    /// What path should be followed
    pub assigned_comparison_group: Option<types::Id>,
    /// Primitive extension sibling for [`assigned_comparison_group`](Self::assigned_comparison_group) (FHIR `_assignedComparisonGroup`).
    #[serde(rename = "_assignedComparisonGroup")]
    pub assigned_comparison_group_ext: Option<types::Element>,

    /// What path was followed
    pub actual_comparison_group: Option<types::Id>,
    /// Primitive extension sibling for [`actual_comparison_group`](Self::actual_comparison_group) (FHIR `_actualComparisonGroup`).
    #[serde(rename = "_actualComparisonGroup")]
    pub actual_comparison_group_ext: Option<types::Element>,

    /// Agreement to participate in study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consent: Vec<types::Reference>,
}

/// Subject status.
///
/// Records the current and historical states and milestones a research subject
/// has passed through during participation in a study, along with the reason and
/// dates for each state change.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_subject::ResearchSubjectProgress;
/// use fhir::r5::types;
///
/// let value = ResearchSubjectProgress {
///     start_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `startDate` is the name this serializes to on the wire.
/// assert_eq!(json["startDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ResearchSubjectProgress = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubjectProgress {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// state | milestone
    pub r#type: Option<types::CodeableConcept>,

    /// candidate | eligible | follow-up | ineligible | not-registered |
    /// off-study | on-study | on-study-intervention | on-study-observation |
    /// pending-on-study | potential-candidate | screening | withdrawn
    pub subject_state: Option<types::CodeableConcept>,

    /// SignedUp | Screened | Randomized
    pub milestone: Option<types::CodeableConcept>,

    /// State change reason
    pub reason: Option<types::CodeableConcept>,

    /// State change date
    pub start_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`start_date`](Self::start_date) (FHIR `_startDate`).
    #[serde(rename = "_startDate")]
    pub start_date_ext: Option<types::Element>,

    /// State change date
    pub end_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`end_date`](Self::end_date) (FHIR `_endDate`).
    #[serde(rename = "_endDate")]
    pub end_date_ext: Option<types::Element>,
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
