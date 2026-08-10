//! Evidence
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Evidence
//!
//! Version: 4.3.0
//!
//! Single evidence bit
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The Evidence Resource provides a machine-interpretable expression of an
/// evidence concept including the evidence variables (eg population,
/// exposures/interventions, comparators, outcomes, measured variables,
/// confounding variables), the statistics, and the certainty of this evidence.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4b::resources::evidence::Evidence;
///
/// let value = Evidence::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Evidence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct Evidence {
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

    /// Canonical identifier for this evidence, represented as a globally
    /// unique URI
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the summary
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of this summary
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this summary (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Citation for this evidence
    /// The `Evidence.citeAs[x]` choice element (0..1); see [`EvidenceCiteAs`].
    #[serde(flatten)]
    pub cite_as: Option<EvidenceCiteAs>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// When the summary was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the summary was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

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

    /// Link or citation to artifact associated with the summary
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Description of the particular summary
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Declarative description of the Evidence
    pub assertion: Option<types::Markdown>,
    /// Primitive extension sibling for [`assertion`](Self::assertion) (FHIR `_assertion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_assertion")]
    pub assertion_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Evidence variable such as population, exposure, or outcome
    pub variable_definition: ::vec1::Vec1<EvidenceVariableDefinition>,

    /// The method to combine studies
    pub synthesis_type: Option<types::CodeableConcept>,

    /// The type of study that produced this evidence
    pub study_type: Option<types::CodeableConcept>,

    /// Values and parameters for a single statistic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statistic: Vec<EvidenceStatistic>,

    /// Certainty or quality of the evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty: Vec<EvidenceCertainty>,
}

/// Assessment of certainty, confidence in the estimates, or quality of the
/// evidence.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence::EvidenceCertainty;
/// use fhir::r4b::types;
///
/// let value = EvidenceCertainty {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: EvidenceCertainty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceCertainty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of certainty
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Aspect of certainty being rated
    pub r#type: Option<types::CodeableConcept>,

    /// Assessment or judgement of the aspect
    pub rating: Option<types::CodeableConcept>,

    /// Individual or group who did the rating
    pub rater: Option<types::String>,
    /// Primitive extension sibling for [`rater`](Self::rater) (FHIR `_rater`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rater")]
    pub rater_ext: Option<types::Element>,

    /// A domain or subdomain of certainty
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subcomponent: Vec<EvidenceCertainty>,
}

/// Values and parameters for a single statistic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence::EvidenceStatistic;
/// use fhir::r4b::types;
///
/// let value = EvidenceStatistic {
///     number_of_events: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfEvents` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfEvents"], ::serde_json::json!(0));
///
/// let back: EvidenceStatistic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceStatistic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of content
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Type of statistic, eg relative risk
    pub statistic_type: Option<types::CodeableConcept>,

    /// Associated category for categorical variable
    pub category: Option<types::CodeableConcept>,

    /// Statistic value
    pub quantity: Option<types::Quantity>,

    /// The number of events associated with the statistic
    pub number_of_events: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_events`](Self::number_of_events) (FHIR `_numberOfEvents`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfEvents")]
    pub number_of_events_ext: Option<types::Element>,

    /// The number of participants affected
    pub number_affected: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_affected`](Self::number_affected) (FHIR `_numberAffected`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberAffected")]
    pub number_affected_ext: Option<types::Element>,

    /// Number of samples in the statistic
    pub sample_size: Option<EvidenceStatisticSampleSize>,

    /// An attribute of the Statistic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,

    /// An aspect of the statistical model
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub model_characteristic: Vec<EvidenceStatisticModelCharacteristic>,
}

/// A statistical attribute of the statistic such as a measure of
/// heterogeneity.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence::EvidenceStatisticAttributeEstimate;
/// use fhir::r4b::types;
///
/// let value = EvidenceStatisticAttributeEstimate {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: EvidenceStatisticAttributeEstimate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceStatisticAttributeEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of the attribute estimate
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnote or explanatory note about the estimate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The type of attribute estimate, eg confidence interval or p value
    pub r#type: Option<types::CodeableConcept>,

    /// The singular quantity of the attribute estimate, for attribute
    /// estimates represented as single values; also used to report unit of
    /// measure
    pub quantity: Option<types::Quantity>,

    /// Level of confidence interval, eg 0.95 for 95% confidence interval
    pub level: Option<types::Decimal>,
    /// Primitive extension sibling for [`level`](Self::level) (FHIR `_level`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_level")]
    pub level_ext: Option<types::Element>,

    /// Lower and upper bound values of the attribute estimate
    pub range: Option<types::Range>,

    /// A nested attribute estimate; which is the attribute estimate of an
    /// attribute estimate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
}

/// A component of the method to generate the statistic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence::EvidenceStatisticModelCharacteristic;
/// use fhir::r4b::types;
///
/// let value = EvidenceStatisticModelCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceStatisticModelCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceStatisticModelCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Model specification
    pub code: types::CodeableConcept,

    /// Numerical value to complete model specification
    pub value: Option<types::Quantity>,

    /// A variable adjusted for in the adjusted analysis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable: Vec<EvidenceStatisticModelCharacteristicVariable>,

    /// An attribute of the statistic used as a model characteristic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_estimate: Vec<EvidenceStatisticAttributeEstimate>,
}

/// A variable adjusted for in the adjusted analysis.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence::EvidenceStatisticModelCharacteristicVariable;
/// use fhir::r4b::types;
///
/// let value = EvidenceStatisticModelCharacteristicVariable {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceStatisticModelCharacteristicVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceStatisticModelCharacteristicVariable {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of the variable
    pub variable_definition: types::Reference,

    /// continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<crate::coded::Coded<crate::r4b::codes::VariableHandling>>,
    /// Primitive extension sibling for [`handling`](Self::handling) (FHIR `_handling`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_handling")]
    pub handling_ext: Option<types::Element>,

    /// Description for grouping of ordinal or polychotomous variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_category: Vec<types::CodeableConcept>,

    /// Discrete value for grouping of ordinal or polychotomous variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_quantity: Vec<types::Quantity>,

    /// Range of values for grouping of ordinal or polychotomous variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_range: Vec<types::Range>,
}

/// Number of samples in the statistic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence::EvidenceStatisticSampleSize;
/// use fhir::r4b::types;
///
/// let value = EvidenceStatisticSampleSize {
///     number_of_studies: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfStudies` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfStudies"], ::serde_json::json!(0));
///
/// let back: EvidenceStatisticSampleSize = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceStatisticSampleSize {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of sample size for statistic
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnote or explanatory note about the sample size
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Number of contributing studies
    pub number_of_studies: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_studies`](Self::number_of_studies) (FHIR `_numberOfStudies`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfStudies")]
    pub number_of_studies_ext: Option<types::Element>,

    /// Cumulative number of participants
    pub number_of_participants: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_participants`](Self::number_of_participants) (FHIR `_numberOfParticipants`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfParticipants")]
    pub number_of_participants_ext: Option<types::Element>,

    /// Number of participants with known results for measured variables
    pub known_data_count: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`known_data_count`](Self::known_data_count) (FHIR `_knownDataCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_knownDataCount")]
    pub known_data_count_ext: Option<types::Element>,
}

/// Evidence variable such as population, exposure, or outcome.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence::EvidenceVariableDefinition;
/// use fhir::r4b::types;
///
/// let value = EvidenceVariableDefinition {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: EvidenceVariableDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceVariableDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A text description or summary of the variable
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// population | subpopulation | exposure | referenceExposure |
    /// measuredVariable | confounder
    pub variable_role: types::CodeableConcept,

    /// Definition of the actual variable related to the statistic(s)
    pub observed: Option<types::Reference>,

    /// Definition of the intended variable related to the Evidence
    pub intended: Option<types::Reference>,

    /// low | moderate | high | exact
    pub directness_match: Option<types::CodeableConcept>,
}

/// The `Evidence.citeAs[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceCiteAs {
    /// `citeAsReference` variant.
    #[fhir("citeAsReference")]
    Reference(Box<types::Reference>),
    /// `citeAsMarkdown` variant.
    #[fhir("citeAsMarkdown")]
    Markdown(crate::r4b::choice::Primitive<types::Markdown>),
}
