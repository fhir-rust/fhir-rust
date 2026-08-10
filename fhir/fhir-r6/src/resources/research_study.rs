//! ResearchStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchStudy
//!
//! Version: 6.0.0-ballot3
//!
//! Investigation to increase healthcare-related patient-independent knowledge
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A scientific study intended to increase health-related knowledge. For
/// example, clinical trials are research studies that involve people. These
/// studies may be related to new ways to screen, prevent, diagnose, and treat
/// disease. They may also study certain outcomes and certain groups of people
/// by looking at data collected in the past or future.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudy;
/// use fhir::r6::types;
///
/// let value = ResearchStudy {
///     cite_as: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `citeAs` is the name this serializes to on the wire.
/// assert_eq!(json["citeAs"], ::serde_json::json!("# Heading"));
///
/// let back: ResearchStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this study resource
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business Identifier for study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The business version for the study record
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this study (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human readable name of the study
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Additional names for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label: Vec<ResearchStudyLabel>,

    /// Steps followed in executing study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<types::Reference<crate::r6::resources::PlanDefinition>>,

    /// Part of larger study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference<crate::r6::resources::ResearchStudy>>,

    /// How to cite this ResearchStudy
    pub cite_as: Option<types::Markdown>,
    /// Primitive extension sibling for [`cite_as`](Self::cite_as) (FHIR `_citeAs`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_citeAs")]
    pub cite_as_ext: Option<types::Element>,

    /// Relationships to other Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<ResearchStudyRelatesTo>,

    /// Date the resource last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// treatment | prevention | diagnostic | supportive-care | screening |
    /// health-services-research | basic-science | device-feasibility
    pub primary_purpose_type: Option<types::CodeableConcept>,

    /// Classifier used for clinical trials
    pub phase: Option<types::CodeableConcept>,

    /// Classifications of the study design characteristics
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study_design: Vec<types::CodeableConcept>,

    /// Drugs, devices, etc. under study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::CodeableReference>,

    /// Condition being studied
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
    /// Primitive extension sibling for [`description_summary`](Self::description_summary) (FHIR `_descriptionSummary`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_descriptionSummary")]
    pub description_summary_ext: Option<types::Element>,

    /// Detailed narrative of the study
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// accrual-goal-met | closed-due-to-toxicity |
    /// closed-due-to-lack-of-study-progress |
    /// temporarily-closed-per-study-design
    pub why_stopped: Option<types::CodeableConcept>,

    /// Target or actual group of participants enrolled in study
    pub recruitment: Option<ResearchStudyRecruitment>,

    /// Defined path through the study for a subject
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparison_group: Vec<ResearchStudyComparisonGroup>,

    /// A goal for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub objective: Vec<ResearchStudyObjective>,

    /// Link to results generated during the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference>,
}

/// Sponsors, collaborators, and other parties.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyAssociatedParty;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// sponsor | lead-sponsor | sponsor-investigator | primary-investigator |
    /// collaborator | funding-source | general-contact | recruitment-contact |
    /// sub-investigator | study-chair | irb
    pub role: types::CodeableConcept,

    /// When active in the role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub period: Vec<types::Period>,

    /// nih | fda | government | nonprofit | academic | industry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// Individual or organization associated with study (use practitionerRole
    /// to specify their organisation)
    pub party: Option<types::Reference>,
}

/// Describes an expected event or sequence of events for one of the subjects
/// of a study. E.g. for a living subject: exposure to drug A, wash-out,
/// exposure to drug B, wash-out, follow-up. E.g. for a stability study: {store
/// sample from lot A at 25 degrees for 1 month}, {store sample from lot A at
/// 40 degrees for 1 month}.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyComparisonGroup;
/// use fhir::r6::types;
///
/// let value = ResearchStudyComparisonGroup {
///     target_number: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `targetNumber` is the name this serializes to on the wire.
/// assert_eq!(json["targetNumber"], ::serde_json::json!(0));
///
/// let back: ResearchStudyComparisonGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ResearchStudyComparisonGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Estimated total number of participants to be enrolled in the comparison
    /// group
    pub target_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`target_number`](Self::target_number) (FHIR `_targetNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetNumber")]
    pub target_number_ext: Option<types::Element>,

    /// Actual total number of participants enrolled in the comparison group
    pub actual_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`actual_number`](Self::actual_number) (FHIR `_actualNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actualNumber")]
    pub actual_number_ext: Option<types::Element>,

    /// Inclusion and exclusion criteria for the comparison group
    pub eligibility: Option<types::Reference<crate::r6::resources::Group>>,

    /// Group of participants who were enrolled in the comparison group
    pub observed_group: Option<types::Reference<crate::r6::resources::Group>>,
}

/// Additional names for the study.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyLabel;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ResearchStudyLabel {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// primary | official | scientific | plain-language | subtitle |
    /// short-title | acronym | earlier-title | language | auto-translated |
    /// human-use | machine-use | duplicate-uid
    pub r#type: Option<types::CodeableConcept>,

    /// The name
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A goal that the study is aiming to achieve in terms of a scientific
/// question to be answered by the analysis of data collected during the study.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyObjective;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Description of the objective
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// A variable measured during the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_measure: Vec<ResearchStudyObjectiveOutcomeMeasure>,
}

/// An "outcome measure", "endpoint", "effect measure" or "measure of effect"
/// is a specific measurement or observation used to quantify the effect of
/// experimental variables on the participants in a study, or for observational
/// studies, to describe patterns of diseases or traits or associations with
/// exposures, risk factors or treatment.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyObjectiveOutcomeMeasure;
/// use fhir::r6::types;
///
/// let value = ResearchStudyObjectiveOutcomeMeasure {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyObjectiveOutcomeMeasure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ResearchStudyObjectiveOutcomeMeasure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for the outcome measure
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// primary | secondary | exploratory
    pub r#type: Option<types::CodeableConcept>,

    /// Description of the outcome measure
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Definition of the outcome measure
    pub endpoint: types::Reference<crate::r6::resources::EvidenceVariable>,

    /// Population for this estimand
    pub population: Option<types::Reference<crate::r6::resources::Group>>,

    /// Comparison group of interest
    pub intervention: Option<types::Reference<crate::r6::resources::Group>>,

    /// Comparison group for comparison
    pub comparator: Option<types::Reference<crate::r6::resources::Group>>,

    /// Statistical measure for treatment effect estimate
    pub summary_measure: Option<types::CodeableConcept>,

    /// Handling of intercurrent event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_handling: Vec<ResearchStudyObjectiveOutcomeMeasureEventHandling>,
}

/// Handling of intercurrent event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyObjectiveOutcomeMeasureEventHandling;
/// use fhir::r6::types;
///
/// let value = ResearchStudyObjectiveOutcomeMeasureEventHandling {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: ResearchStudyObjectiveOutcomeMeasureEventHandling = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ResearchStudyObjectiveOutcomeMeasureEventHandling {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The event
    pub event: Option<types::CodeableConcept>,

    /// The group that is affected by this event handling
    pub group: Option<types::CodeableConcept>,

    /// How the data is handled
    pub handling: Option<types::CodeableConcept>,

    /// Text summary of event handling
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Status of study with time for that status.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyProgressStatus;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// Date range
    pub period: Option<types::Period>,
}

/// Target or actual group of participants enrolled in study.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyRecruitment;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`target_number`](Self::target_number) (FHIR `_targetNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetNumber")]
    pub target_number_ext: Option<types::Element>,

    /// Actual total number of participants enrolled in study
    pub actual_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`actual_number`](Self::actual_number) (FHIR `_actualNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actualNumber")]
    pub actual_number_ext: Option<types::Element>,

    /// Inclusion and exclusion criteria
    pub eligibility: Option<types::Reference<crate::r6::resources::Group>>,

    /// Group of participants who were enrolled in study
    pub actual_group: Option<types::Reference<crate::r6::resources::Group>>,
}

/// Relationships that this ResearchStudy has with other FHIR or non-FHIR
/// resources that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::research_study::ResearchStudyRelatesTo;
/// use fhir::r6::types;
///
/// let value = ResearchStudyRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ResearchStudyRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ResearchStudyRelatesToDe")]
#[fhir_version("r6")]
pub struct ResearchStudyRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// documentation | justification | predecessor | successor | derived-from
    /// | depends-on | composed-of | part-of | amends | amended-with | appends
    /// | appended-with | cites | cited-by | comments-on | comment-in |
    /// contains | contained-in | corrects | correction-in | replaces |
    /// replaced-with | retracts | retracted-by | signs | similar-to | supports
    /// | supported-with | transforms | transformed-into | transformed-with |
    /// specification-of | created-with | cite-as | summarizes
    pub r#type: crate::coded::Coded<crate::r6::codes::ArtifactRelationshipType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The artifact that is related to this ResearchStudy
    /// The `ResearchStudy.relatesTo.target[x]` choice element (1..1); see [`ResearchStudyRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<ResearchStudyRelatesToTarget>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResearchStudyRelatesToDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: crate::coded::Coded<crate::r6::codes::ArtifactRelationshipType>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    #[serde(flatten)]
    target: crate::r6::choice::Slot<ResearchStudyRelatesToTarget>,
}

impl ::core::convert::From<ResearchStudyRelatesToDe> for ResearchStudyRelatesTo {
    fn from(v: ResearchStudyRelatesToDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            target: v.target.0,
        }
    }
}

/// The `ResearchStudy.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ResearchStudyRelatesToTarget {
    /// `targetUri` variant.
    #[fhir("targetUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `targetAttachment` variant.
    #[fhir("targetAttachment")]
    Attachment(Box<types::Attachment>),
    /// `targetCanonical` variant.
    #[fhir("targetCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `targetReference` variant.
    #[fhir("targetReference")]
    Reference(Box<types::Reference>),
    /// `targetMarkdown` variant.
    #[fhir("targetMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
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
