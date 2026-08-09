//! Measure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Measure
//!
//! Version: 5.0.0
//!
//! Measure Resource: The Measure resource provides the definition of a quality measure.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Measure resource provides the definition of a quality measure.
///
/// A quality measure is a quantitative tool used to assess the performance of an
/// individual clinician, an organization, or a health system with respect to a
/// specified process or outcome, by measuring actions, processes, states, or
/// outcomes of clinical care. In FHIR R5 the Measure resource is a canonical,
/// publishable knowledge artifact that captures the complete, computable
/// definition of such a measure: its metadata and versioning, the clinical logic
/// libraries it relies on, its population criteria (for example initial
/// population, numerator, and denominator), its scoring and improvement notation,
/// and any stratifiers or supplemental data to report alongside the score.
///
/// Measures are typically authored and distributed by publishers or measure
/// stewards and then evaluated against patient data to produce results. The
/// clinical logic is generally expressed in a referenced Library (commonly using
/// Clinical Quality Language, CQL), and the calculated results of applying a
/// Measure to a subject or population are conveyed in a separate MeasureReport
/// resource. This separation lets the same Measure definition be shared, versioned,
/// and reused across many evaluations and reporting contexts.
///
/// # See also
///
/// The `Library` resource typically holds the measure's executable logic, and the
/// `MeasureReport` resource conveys the results of evaluating a measure. Measure
/// subjects are frequently instances of [`Patient`](crate::r5::resources::patient::Patient).
/// Many descriptive fields are typed as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::Measure;
/// use fhir::r5::types;
///
/// let value = Measure {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: Measure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
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

    /// Canonical identifier for this measure, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the measure
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `Measure.versionAlgorithm[x]` choice element (0..1); see [`MeasureVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<MeasureVersionAlgorithm>,

    /// Name for this measure (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this measure (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the measure
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`).
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// Publication lifecycle state of this measure: draft, active, retired, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// The `Measure.subject[x]` choice element (0..1); see [`MeasureSubject`].
    #[serde(flatten)]
    pub subject: Option<MeasureSubject>,

    /// Population basis
    pub basis: Option<crate::r5::coded::Coded<crate::r5::codes::FhirTypes>>,
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`).
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the measure
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for measure (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this measure is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the measure
    pub usage: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`).
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the measure was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the measure was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the measure is expected to be used
    pub effective_period: Option<types::Period>,

    /// The category of the measure, such as Education, Treatment, Assessment, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Additional documentation, citations, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Canonical references to the Library resources that hold the measure's computable logic, such as CQL.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`).
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// Disclaimer for use of the measure or its referenced content
    pub disclaimer: Option<types::Markdown>,
    /// Primitive extension sibling for [`disclaimer`](Self::disclaimer) (FHIR `_disclaimer`).
    #[serde(rename = "_disclaimer")]
    pub disclaimer_ext: Option<types::Element>,

    /// How the measure is scored: proportion, ratio, continuous-variable, or cohort.
    pub scoring: Option<types::CodeableConcept>,

    /// What units?
    pub scoring_unit: Option<types::CodeableConcept>,

    /// opportunity | all-or-nothing | linear | weighted
    pub composite_scoring: Option<types::CodeableConcept>,

    /// process | outcome | structure | patient-reported-outcome | composite
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// How risk adjustment is applied for this measure
    pub risk_adjustment: Option<types::Markdown>,
    /// Primitive extension sibling for [`risk_adjustment`](Self::risk_adjustment) (FHIR `_riskAdjustment`).
    #[serde(rename = "_riskAdjustment")]
    pub risk_adjustment_ext: Option<types::Element>,

    /// How is rate aggregation performed for this measure
    pub rate_aggregation: Option<types::Markdown>,
    /// Primitive extension sibling for [`rate_aggregation`](Self::rate_aggregation) (FHIR `_rateAggregation`).
    #[serde(rename = "_rateAggregation")]
    pub rate_aggregation_ext: Option<types::Element>,

    /// Detailed description of why the measure exists
    pub rationale: Option<types::Markdown>,
    /// Primitive extension sibling for [`rationale`](Self::rationale) (FHIR `_rationale`).
    #[serde(rename = "_rationale")]
    pub rationale_ext: Option<types::Element>,

    /// Summary of clinical guidelines
    pub clinical_recommendation_statement: Option<types::Markdown>,
    /// Primitive extension sibling for [`clinical_recommendation_statement`](Self::clinical_recommendation_statement) (FHIR `_clinicalRecommendationStatement`).
    #[serde(rename = "_clinicalRecommendationStatement")]
    pub clinical_recommendation_statement_ext: Option<types::Element>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Defined terms used in the measure documentation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term: Vec<MeasureTerm>,

    /// Additional guidance for implementers (deprecated)
    pub guidance: Option<types::Markdown>,
    /// Primitive extension sibling for [`guidance`](Self::guidance) (FHIR `_guidance`).
    #[serde(rename = "_guidance")]
    pub guidance_ext: Option<types::Element>,

    /// The population criteria groups that define how the measure is scored and evaluated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<MeasureGroup>,

    /// What other data should be reported with the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_data: Vec<MeasureSupplementalData>,
}

/// Defined terms used in the measure documentation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::MeasureTerm;
/// use fhir::r5::types;
///
/// let value = MeasureTerm {
///     definition: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `definition` is the name this serializes to on the wire.
/// assert_eq!(json["definition"], ::serde_json::json!("# Heading"));
///
/// let back: MeasureTerm = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureTerm {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What term?
    pub code: Option<types::CodeableConcept>,

    /// Meaning of the term
    pub definition: Option<types::Markdown>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,
}

/// Population criteria group.
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::MeasureGroup;
/// use fhir::r5::types;
///
/// let value = MeasureGroup {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for group in measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the group
    pub code: Option<types::CodeableConcept>,

    /// Summary description
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// process | outcome | structure | patient-reported-outcome | composite
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The `Measure.group.subject[x]` choice element (0..1); see [`MeasureGroupSubject`].
    #[serde(flatten)]
    pub subject: Option<MeasureGroupSubject>,

    /// Population basis
    pub basis: Option<crate::r5::coded::Coded<crate::r5::codes::FhirTypes>>,
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`).
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,

    /// proportion | ratio | continuous-variable | cohort
    pub scoring: Option<types::CodeableConcept>,

    /// What units?
    pub scoring_unit: Option<types::CodeableConcept>,

    /// How is rate aggregation performed for this measure
    pub rate_aggregation: Option<types::Markdown>,
    /// Primitive extension sibling for [`rate_aggregation`](Self::rate_aggregation) (FHIR `_rateAggregation`).
    #[serde(rename = "_rateAggregation")]
    pub rate_aggregation_ext: Option<types::Element>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Logic used by the measure group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`).
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// Population criteria
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureGroupPopulation>,

    /// Stratifier criteria for the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stratifier: Vec<MeasureGroupStratifier>,
}

/// Population criteria.
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::MeasureGroupPopulation;
/// use fhir::r5::types;
///
/// let value = MeasureGroupPopulation {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroupPopulation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for population in measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// initial-population | numerator | numerator-exclusion | denominator | ...
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this population criteria
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The criteria that defines this population
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference<crate::r5::resources::Group>>,

    /// Which population
    pub input_population_id: Option<types::String>,
    /// Primitive extension sibling for [`input_population_id`](Self::input_population_id) (FHIR `_inputPopulationId`).
    #[serde(rename = "_inputPopulationId")]
    pub input_population_id_ext: Option<types::Element>,

    /// Aggregation method for a measure score (e.g. sum, average, median, ...)
    pub aggregate_method: Option<types::CodeableConcept>,
}

/// Stratifier criteria for the measure.
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::MeasureGroupStratifier;
/// use fhir::r5::types;
///
/// let value = MeasureGroupStratifier {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroupStratifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupStratifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for stratifier in measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the stratifier
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this stratifier
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// How the measure should be stratified
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference<crate::r5::resources::Group>>,

    /// Stratifier criteria component for the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<MeasureGroupStratifierComponent>,
}

/// Stratifier criteria component for the measure.
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::MeasureGroupStratifierComponent;
/// use fhir::r5::types;
///
/// let value = MeasureGroupStratifierComponent {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroupStratifierComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupStratifierComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for stratifier component in measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the stratifier component
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this stratifier component
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Component of how the measure should be stratified
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference<crate::r5::resources::Group>>,
}

/// What other data should be reported with the measure.
/// # Examples
///
/// ```
/// use fhir::r5::resources::measure::MeasureSupplementalData;
/// use fhir::r5::types;
///
/// let value = MeasureSupplementalData {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureSupplementalData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeasureSupplementalData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for supplementalData in measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the supplemental data
    pub code: Option<types::CodeableConcept>,

    /// supplemental-data | risk-adjustment-factor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<types::CodeableConcept>,

    /// The human readable description of this supplemental data
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Expression describing additional data to be reported
    pub criteria: types::Expression,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Measure;

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
/// The `Measure.group.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MeasureGroupSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `Measure.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MeasureSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `Measure.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MeasureVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
