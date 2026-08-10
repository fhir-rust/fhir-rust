//! ResearchStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
//!
//! Version: 4.3.0
//!
//! Investigation to increase healthcare-related patient-independent knowledge
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A process where a researcher or organization plans and then executes a
/// series of steps intended to increase the field of healthcare-related
/// knowledge. This includes studies of safety, efficacy, comparative
/// effectiveness and other information about medications, devices, therapies
/// and other interventional and investigative techniques. A ResearchStudy
/// involves the gathering of information about human or animal subjects.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::research_study::ResearchStudy;
/// use fhir::r4b::types;
///
/// let value = ResearchStudy {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ResearchStudy {
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Name for this study
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Steps followed in executing study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<types::Reference<crate::r4b::resources::PlanDefinition>>,

    /// Part of larger study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference<crate::r4b::resources::ResearchStudy>>,

    /// active | administratively-completed | approved | closed-to-accrual |
    /// closed-to-accrual-and-intervention | completed | disapproved |
    /// in-review | temporarily-closed-to-accrual |
    /// temporarily-closed-to-accrual-and-intervention | withdrawn
    pub status: crate::coded::Coded<crate::r4b::codes::ResearchStudyStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// treatment | prevention | diagnostic | supportive-care | screening |
    /// health-services-research | basic-science | device-feasibility
    pub primary_purpose_type: Option<types::CodeableConcept>,

    /// n-a | early-phase-1 | phase-1 | phase-1-phase-2 | phase-2 |
    /// phase-2-phase-3 | phase-3 | phase-4
    pub phase: Option<types::CodeableConcept>,

    /// Classifications for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Drugs, devices, etc. under study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::CodeableConcept>,

    /// Condition being studied
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<types::CodeableConcept>,

    /// Contact details for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// References and dependencies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Used to search for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<types::CodeableConcept>,

    /// Geographic region(s) for study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::CodeableConcept>,

    /// What this is study doing
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Inclusion & exclusion criteria
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enrollment: Vec<types::Reference<crate::r4b::resources::Group>>,

    /// When the study began and ended
    pub period: Option<types::Period>,

    /// Organization that initiates and is legally responsible for the study
    pub sponsor: Option<types::Reference<crate::r4b::resources::Organization>>,

    /// Researcher who oversees multiple aspects of the study
    pub principal_investigator: Option<types::Reference>,

    /// Facility where study activities are conducted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site: Vec<types::Reference<crate::r4b::resources::Location>>,

    /// accrual-goal-met | closed-due-to-toxicity |
    /// closed-due-to-lack-of-study-progress |
    /// temporarily-closed-per-study-design
    pub reason_stopped: Option<types::CodeableConcept>,

    /// Comments made about the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Defined path through the study for a subject
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arm: Vec<ResearchStudyArm>,

    /// A goal for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objective: Vec<ResearchStudyObjective>,
}

/// Describes an expected sequence of events for one of the participants of a
/// study. E.g. Exposure to drug A, wash-out, exposure to drug B, wash-out,
/// follow-up.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::research_study::ResearchStudyArm;
/// use fhir::r4b::types;
///
/// let value = ResearchStudyArm {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyArm = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ResearchStudyArm {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for study arm
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Categorization of study arm
    pub r#type: Option<types::CodeableConcept>,

    /// Short explanation of study path
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// A goal that the study is aiming to achieve in terms of a scientific
/// question to be answered by the analysis of data collected during the study.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::research_study::ResearchStudyObjective;
/// use fhir::r4b::types;
///
/// let value = ResearchStudyObjective {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyObjective = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ResearchStudyObjective {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for the objective
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// primary | secondary | exploratory
    pub r#type: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ResearchStudy;

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
