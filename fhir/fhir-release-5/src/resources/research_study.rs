//! ResearchStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
//!
//! Version: 5.0.0
//!
//! ResearchStudy Resource: A scientific study of nature that sometimes includes processes involved in health and disease.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A scientific study of nature that sometimes includes processes involved in
/// health and disease.
///
/// For example, clinical trials are research studies that involve people. These
/// studies may be related to new ways to screen, prevent, diagnose, and treat
/// disease. They may also study certain outcomes and certain groups of people by
/// looking at data collected in the past or future. In FHIR R5 a ResearchStudy
/// captures the design, sponsors, objectives, outcome measures, comparison
/// groups, recruitment, and status of such an investigation.
///
/// A `ResearchStudy` acts as the administrative and organizational record for a
/// study protocol: it tracks the study's lifecycle status, phase, and type of
/// investigation; who sponsors and conducts it; where and when it takes place;
/// what conditions or focuses it addresses; how participants are recruited and
/// grouped for comparison; and what objectives and outcome measures define
/// success. It is commonly referenced by other resources that represent
/// individual participation or data collected under the study, such as
/// `ResearchSubject`, and it may reference [`Patient`](crate::r5::resources::patient::Patient)
/// or `Group` resources for enrolled or observed populations, as well as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values for
/// classifying status, phase, condition, and role information throughout.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudy;
/// use fhir::r5::types;
///
/// let value = ResearchStudy {
///     description_summary: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `descriptionSummary` is the name this serializes to on the wire.
/// assert_eq!(json["descriptionSummary"], ::serde_json::json!("# Heading"));
///
/// let back: ResearchStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudy {
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

    /// Canonical identifier for this study resource
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business Identifier for study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The business version for the study record
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this study (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human readable name of the study, typically shown in listings and reports
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Additional names for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label: Vec<ResearchStudyLabel>,

    /// Steps followed in executing study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<types::Reference>,

    /// Part of larger study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// References, URLs, and attachments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Date the resource last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The current lifecycle status of the study record itself: draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// treatment | prevention | diagnostic | supportive-care | screening | health-services-research | basic-science | device-feasibility
    pub primary_purpose_type: Option<types::CodeableConcept>,

    /// n-a | early-phase-1 | phase-1 | phase-1-phase-2 | phase-2 | phase-2-phase-3 | phase-3 | phase-4
    pub phase: Option<types::CodeableConcept>,

    /// Classifications of the study design characteristics
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study_design: Vec<types::CodeableConcept>,

    /// Drugs, devices, etc. under study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::CodeableReference>,

    /// Condition, disease, or health concern being studied
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<types::CodeableConcept>,

    /// Used to search for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<types::CodeableConcept>,

    /// Geographic area for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub region: Vec<types::CodeableConcept>,

    /// Brief text explaining the study
    pub description_summary: Option<types::Markdown>,
    /// Primitive extension sibling for [`description_summary`](Self::description_summary) (FHIR `_descriptionSummary`).
    #[serde(rename = "_descriptionSummary")]
    pub description_summary_ext: Option<types::Element>,

    /// Detailed narrative of the study
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// When the study began and ended
    pub period: Option<types::Period>,

    /// Facility where study activities are conducted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub site: Vec<types::Reference>,

    /// Comments made about the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Classification for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// Sponsors, collaborators, and other parties
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub associated_party: Vec<ResearchStudyAssociatedParty>,

    /// Status of study with time for that status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub progress_status: Vec<ResearchStudyProgressStatus>,

    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    pub why_stopped: Option<types::CodeableConcept>,

    /// Target or actual group of participants enrolled in study
    pub recruitment: Option<ResearchStudyRecruitment>,

    /// Defined path through the study for a subject
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparison_group: Vec<ResearchStudyComparisonGroup>,

    /// A goal for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objective: Vec<ResearchStudyObjective>,

    /// A variable measured during the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_measure: Vec<ResearchStudyOutcomeMeasure>,

    /// Link to results generated during the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference>,
}

/// Additional names for the study.
///
/// Provides alternative labels for the study such as official titles, acronyms,
/// scientific titles, or plain-language names, each classified by a type.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudyLabel;
/// use fhir::r5::types;
///
/// let value = ResearchStudyLabel {
///     value: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `value` is the name this serializes to on the wire.
/// assert_eq!(json["value"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyLabel = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyLabel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// primary | official | scientific | plain-language | subtitle | short-title | acronym | earlier-title | language | auto-translated | human-use | machine-use | duplicate-uid
    pub r#type: Option<types::CodeableConcept>,

    /// The name
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Sponsors, collaborators, and other parties.
///
/// Identifies organizations and individuals associated with the study along with
/// their role, the period during which they held that role, and any classifiers
/// describing the nature of the party.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudyAssociatedParty;
/// use fhir::r5::types;
///
/// let value = ResearchStudyAssociatedParty {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyAssociatedParty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyAssociatedParty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of associated party
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// sponsor | lead-sponsor | sponsor-investigator | primary-investigator | collaborator | funding-source | general-contact | recruitment-contact | sub-investigator | study-director | study-chair
    pub role: types::CodeableConcept,

    /// When active in the role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub period: Vec<types::Period>,

    /// nih | fda | government | nonprofit | academic | industry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// Individual or organization associated with study (use practitionerRole to specify their organisation)
    pub party: Option<types::Reference>,
}

/// Status of study with time for that status.
///
/// Records a labeled state of the study (for example recruitment status), whether
/// it is actual or anticipated, and the date range over which it applied.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudyProgressStatus;
/// use fhir::r5::types;
///
/// let value = ResearchStudyProgressStatus {
///     actual: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `actual` is the name this serializes to on the wire.
/// assert_eq!(json["actual"], ::serde_json::json!(true));
///
/// let back: ResearchStudyProgressStatus = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyProgressStatus {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for status or state (e.g. recruitment status)
    pub state: types::CodeableConcept,

    /// Actual if true else anticipated
    pub actual: Option<types::Boolean>,
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`).
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// Date range
    pub period: Option<types::Period>,
}

/// Target or actual group of participants enrolled in study.
///
/// Describes the intended and enrolled participant counts along with references
/// to the eligibility criteria and the actual group of enrolled participants.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudyRecruitment;
/// use fhir::r5::types;
///
/// let value = ResearchStudyRecruitment {
///     target_number: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `targetNumber` is the name this serializes to on the wire.
/// assert_eq!(json["targetNumber"], ::serde_json::json!(0));
///
/// let back: ResearchStudyRecruitment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyRecruitment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Estimated total number of participants to be enrolled
    pub target_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`target_number`](Self::target_number) (FHIR `_targetNumber`).
    #[serde(rename = "_targetNumber")]
    pub target_number_ext: Option<types::Element>,

    /// Actual total number of participants enrolled in study
    pub actual_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`actual_number`](Self::actual_number) (FHIR `_actualNumber`).
    #[serde(rename = "_actualNumber")]
    pub actual_number_ext: Option<types::Element>,

    /// Inclusion and exclusion criteria
    pub eligibility: Option<types::Reference>,

    /// Group of participants who were enrolled in study
    pub actual_group: Option<types::Reference>,
}

/// Defined path through the study for a subject.
///
/// Describes a comparison group or study arm, including its name, type, intended
/// exposures or interventions, and the group of participants observed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudyComparisonGroup;
/// use fhir::r5::types;
///
/// let value = ResearchStudyComparisonGroup {
///     link_id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("pat-1"));
///
/// let back: ResearchStudyComparisonGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyComparisonGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Allows the comparisonGroup for the study and the comparisonGroup for the subject to be linked easily
    pub link_id: Option<types::Id>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Label for study comparisonGroup
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Categorization of study comparisonGroup
    pub r#type: Option<types::CodeableConcept>,

    /// Short explanation of study path
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Interventions or exposures in this comparisonGroup or cohort
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intended_exposure: Vec<types::Reference>,

    /// Group of participants who were enrolled in study comparisonGroup
    pub observed_group: Option<types::Reference>,
}

/// A goal for the study.
///
/// States an objective of the study, categorized as primary, secondary, or
/// exploratory, with a descriptive narrative.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudyObjective;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// primary | secondary | exploratory
    pub r#type: Option<types::CodeableConcept>,

    /// Description of the objective
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// A variable measured during the study.
///
/// Defines an outcome measure of the study, including its name, type, narrative
/// description, and an optional reference to a structured outcome definition.
/// # Examples
///
/// ```
/// use fhir::r5::resources::research_study::ResearchStudyOutcomeMeasure;
/// use fhir::r5::types;
///
/// let value = ResearchStudyOutcomeMeasure {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyOutcomeMeasure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyOutcomeMeasure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for the outcome
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// primary | secondary | exploratory
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Description of the outcome
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Structured outcome definition
    pub reference: Option<types::Reference>,
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
