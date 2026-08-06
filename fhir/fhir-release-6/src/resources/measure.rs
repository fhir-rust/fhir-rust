//! Measure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Measure
//!
//! Version: 6.0.0-ballot3
//!
//! A quality measure definition
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The Measure resource provides the definition of a quality measure.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::Measure;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Measure {
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

    /// Canonical identifier for this measure, represented as a URI (globally
    /// unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the measure
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `Measure.versionAlgorithm[x]` choice element (0..1); see [`MeasureVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<MeasureVersionAlgorithm>,

    /// Name for this measure (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this measure (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the measure
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location,
    /// Device
    /// The `Measure.subject[x]` choice element (0..1); see [`MeasureSubject`].
    #[serde(flatten)]
    pub subject: Option<MeasureSubject>,

    /// Population basis (deprecated, use group.basis)
    pub basis: Option<crate::coded::Coded<crate::r6::codes::FhirTypes>>,
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the measure
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the measure
    pub usage: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When the measure was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the measure was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the measure is expected to be used
    pub effective_period: Option<types::Period>,

    /// The frequency in which this measure should be reported (e.g. 1 '/a' -
    /// yearly, 4 '/a' - quarterly)
    pub reporting_frequency: Option<types::Quantity>,

    /// The category of the measure, such as Education, Treatment, Assessment,
    /// etc
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

    /// Logic used by the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// Disclaimer for use of the measure or its referenced content
    pub disclaimer: Option<types::Markdown>,
    /// Primitive extension sibling for [`disclaimer`](Self::disclaimer) (FHIR `_disclaimer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disclaimer")]
    pub disclaimer_ext: Option<types::Element>,

    /// proportion | ratio | continuous-variable | cohort | composite
    /// (deprecated, use group.scoring)
    pub scoring: Option<types::CodeableConcept>,

    /// What units? (deprecated, use group.scoringUnit)
    pub scoring_unit: Option<types::CodeableConcept>,

    /// How many decimals (The number of decimal places to include in the score
    /// when the score is a decimal-valued result) (deprecated, use
    /// group.scoringPrecision)
    pub scoring_precision: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`scoring_precision`](Self::scoring_precision) (FHIR `_scoringPrecision`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_scoringPrecision")]
    pub scoring_precision_ext: Option<types::Element>,

    /// opportunity | all-or-nothing | linear | weighted (deprecated, use
    /// group.compositeScoring)
    pub composite_scoring: Option<types::CodeableConcept>,

    /// process | outcome | structure | patient-reported-outcome | composite
    /// (deprecated, use group.type)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// How risk adjustment is applied for this measure
    pub risk_adjustment: Option<types::Markdown>,
    /// Primitive extension sibling for [`risk_adjustment`](Self::risk_adjustment) (FHIR `_riskAdjustment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_riskAdjustment")]
    pub risk_adjustment_ext: Option<types::Element>,

    /// How is rate aggregation performed for this measure
    pub rate_aggregation: Option<types::Markdown>,
    /// Primitive extension sibling for [`rate_aggregation`](Self::rate_aggregation) (FHIR `_rateAggregation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rateAggregation")]
    pub rate_aggregation_ext: Option<types::Element>,

    /// Justification for the measure in terms of impact, gap in care, and
    /// evidence
    pub rationale: Option<types::Markdown>,
    /// Primitive extension sibling for [`rationale`](Self::rationale) (FHIR `_rationale`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rationale")]
    pub rationale_ext: Option<types::Element>,

    /// Summary of clinical guidelines
    pub clinical_recommendation_statement: Option<types::Markdown>,
    /// Primitive extension sibling for [`clinical_recommendation_statement`](Self::clinical_recommendation_statement) (FHIR `_clinicalRecommendationStatement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_clinicalRecommendationStatement")]
    pub clinical_recommendation_statement_ext: Option<types::Element>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Explanation of improvement notation (deprecated, use
    /// group.improvementNotationGuidance)
    pub improvement_notation_guidance: Option<types::Markdown>,
    /// Primitive extension sibling for [`improvement_notation_guidance`](Self::improvement_notation_guidance) (FHIR `_improvementNotationGuidance`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_improvementNotationGuidance")]
    pub improvement_notation_guidance_ext: Option<types::Element>,

    /// Defined terms used in the measure documentation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term: Vec<MeasureTerm>,

    /// Additional guidance for implementers (deprecated)
    pub guidance: Option<types::Markdown>,
    /// Primitive extension sibling for [`guidance`](Self::guidance) (FHIR `_guidance`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_guidance")]
    pub guidance_ext: Option<types::Element>,

    /// Population criteria group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<MeasureGroup>,

    /// What other data should be reported with the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_data: Vec<MeasureSupplementalData>,
}

/// A group of population criteria for the measure.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::MeasureGroup;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the group
    pub code: Option<types::CodeableConcept>,

    /// Summary description
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// process | outcome | structure | patient-reported-outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location,
    /// Device
    /// The `Measure.group.subject[x]` choice element (0..1); see [`MeasureGroupSubject`].
    #[serde(flatten)]
    pub subject: Option<MeasureGroupSubject>,

    /// Population basis
    pub basis: Option<crate::coded::Coded<crate::r6::codes::FhirTypes>>,
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,

    /// proportion | ratio | continuous-variable | cohort | composite
    pub scoring: Option<types::CodeableConcept>,

    /// What units?
    pub scoring_unit: Option<types::CodeableConcept>,

    /// How many decimals (The number of decimal places to include in the score
    /// when the score is a decimal-valued result)
    pub scoring_precision: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`scoring_precision`](Self::scoring_precision) (FHIR `_scoringPrecision`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_scoringPrecision")]
    pub scoring_precision_ext: Option<types::Element>,

    /// opportunity | all-or-nothing | linear | weighted
    pub composite_scoring: Option<types::CodeableConcept>,

    /// A component of a composite measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<MeasureGroupComponent>,

    /// How is rate aggregation performed for this measure
    pub rate_aggregation: Option<types::Markdown>,
    /// Primitive extension sibling for [`rate_aggregation`](Self::rate_aggregation) (FHIR `_rateAggregation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rateAggregation")]
    pub rate_aggregation_ext: Option<types::Element>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Explanation of improvement notation
    pub improvement_notation_guidance: Option<types::Markdown>,
    /// Primitive extension sibling for [`improvement_notation_guidance`](Self::improvement_notation_guidance) (FHIR `_improvementNotationGuidance`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_improvementNotationGuidance")]
    pub improvement_notation_guidance_ext: Option<types::Element>,

    /// Logic used by the measure group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`):
    /// carries `id` and/or `extension` for the primitive value.
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

/// If this is a composite measure, a component of the composite.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::MeasureGroupComponent;
/// use fhir::r6::types;
///
/// let value = MeasureGroupComponent {
///     group_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `groupId` is the name this serializes to on the wire.
/// assert_eq!(json["groupId"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroupComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MeasureGroupComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What measure?
    pub measure: Option<types::Canonical>,
    /// Primitive extension sibling for [`measure`](Self::measure) (FHIR `_measure`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measure")]
    pub measure_ext: Option<types::Element>,

    /// What group?
    pub group_id: Option<types::String>,
    /// Primitive extension sibling for [`group_id`](Self::group_id) (FHIR `_groupId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupId")]
    pub group_id_ext: Option<types::Element>,

    /// What weight?
    pub weight: Option<types::Decimal>,
    /// Primitive extension sibling for [`weight`](Self::weight) (FHIR `_weight`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_weight")]
    pub weight_ext: Option<types::Element>,
}

/// A population criteria for the measure.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::MeasureGroupPopulation;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// initial-population | numerator | numerator-exclusion | denominator |
    /// denominator-exclusion | denominator-exception | measure-population |
    /// measure-population-exclusion | measure-observation
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this population criteria
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The criteria that defines this population
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference>,

    /// Which population
    pub input_population_id: Option<types::String>,
    /// Primitive extension sibling for [`input_population_id`](Self::input_population_id) (FHIR `_inputPopulationId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_inputPopulationId")]
    pub input_population_id_ext: Option<types::Element>,

    /// Aggregation method for a measure score (e.g. sum, average, median,
    /// minimum, maximum, count)
    pub aggregate_method: Option<types::CodeableConcept>,
}

/// The stratifier criteria for the measure report, specified as either the
/// name of a valid CQL expression defined within a referenced library or a
/// valid FHIR Resource Path.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::MeasureGroupStratifier;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the stratifier
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this stratifier
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// How the measure should be stratified
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference>,

    /// What stratum values?
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// What units?
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,

    /// Stratifier criteria component for the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<MeasureGroupStratifierComponent>,
}

/// A component of the stratifier criteria for the measure report, specified as
/// either the name of a valid CQL expression defined within a referenced
/// library or a valid FHIR Resource Path.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::MeasureGroupStratifierComponent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the stratifier component
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this stratifier component
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Component of how the measure should be stratified
    pub criteria: Option<types::Expression>,

    /// A group resource that defines this population
    pub group_definition: Option<types::Reference>,

    /// What stratum values?
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// What units?
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,
}

/// The supplemental data criteria for the measure report, specified as either
/// the name of a valid CQL expression within a referenced library, or a valid
/// FHIR Resource Path.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::MeasureSupplementalData;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Meaning of the supplemental data
    pub code: Option<types::CodeableConcept>,

    /// supplemental-data | risk-adjustment-factor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<types::CodeableConcept>,

    /// The human readable description of this supplemental data
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Expression describing additional data to be reported
    pub criteria: types::Expression,

    /// What supplemental data values?
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// What units?
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,
}

/// Provides a description of an individual term used within the measure.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure::MeasureTerm;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,
}

/// The `Measure.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MeasureVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `Measure.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MeasureSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `Measure.group.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MeasureGroupSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
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
