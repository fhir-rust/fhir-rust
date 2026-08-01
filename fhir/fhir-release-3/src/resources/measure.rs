//! Measure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Measure
//!
//!
//!
//! A quality measure definition
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Measure Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure::Measure;
/// use fhir::r3::types;
///
/// let value = Measure {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: Measure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Measure {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Logical URI to reference this measure (globally unique)
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

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r3::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date this was last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Natural language description of the measure
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Why this measure is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the measure
    pub usage: Option<types::String>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

    /// When the measure was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the measure was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the measure is expected to be used
    pub effective_period: Option<types::Period>,

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for measure (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// E.g. Education, Treatment, Assessment, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// A content contributor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<types::Contributor>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Additional documentation, citations, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Logic used by the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Reference>,

    /// Disclaimer for use of the measure or its referenced content
    pub disclaimer: Option<types::Markdown>,
    /// Primitive extension sibling for [`disclaimer`](Self::disclaimer) (FHIR `_disclaimer`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disclaimer")]
    pub disclaimer_ext: Option<types::Element>,

    /// proportion | ratio | continuous-variable | cohort
    pub scoring: Option<types::CodeableConcept>,

    /// opportunity | all-or-nothing | linear | weighted
    pub composite_scoring: Option<types::CodeableConcept>,

    /// process | outcome | structure | patient-reported-outcome | composite
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// How is risk adjustment applied for this measure
    pub risk_adjustment: Option<types::String>,
    /// Primitive extension sibling for [`risk_adjustment`](Self::risk_adjustment) (FHIR `_riskAdjustment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_riskAdjustment")]
    pub risk_adjustment_ext: Option<types::Element>,

    /// How is rate aggregation performed for this measure
    pub rate_aggregation: Option<types::String>,
    /// Primitive extension sibling for [`rate_aggregation`](Self::rate_aggregation) (FHIR `_rateAggregation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rateAggregation")]
    pub rate_aggregation_ext: Option<types::Element>,

    /// Why does this measure exist
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

    /// Improvement notation for the measure, e.g. higher score indicates
    /// better quality
    pub improvement_notation: Option<types::String>,
    /// Primitive extension sibling for [`improvement_notation`](Self::improvement_notation) (FHIR `_improvementNotation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_improvementNotation")]
    pub improvement_notation_ext: Option<types::Element>,

    /// Defined terms used in the measure documentation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Markdown>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_ext: Vec<Option<types::Element>>,

    /// Additional guidance for implementers
    pub guidance: Option<types::Markdown>,
    /// Primitive extension sibling for [`guidance`](Self::guidance) (FHIR `_guidance`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_guidance")]
    pub guidance_ext: Option<types::Element>,

    /// The measure set, e.g. Preventive Care and Screening
    pub set: Option<types::String>,
    /// Primitive extension sibling for [`set`](Self::set) (FHIR `_set`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_set")]
    pub set_ext: Option<types::Element>,

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
/// use fhir::r3::resources::measure::MeasureGroup;
/// use fhir::r3::types;
///
/// let value = MeasureGroup {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MeasureGroup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier
    pub identifier: types::Identifier,

    /// Short name
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Summary description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Population criteria
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureGroupPopulation>,

    /// Stratifier criteria for the measure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stratifier: Vec<MeasureGroupStratifier>,
}

/// A population criteria for the measure.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure::MeasureGroupPopulation;
/// use fhir::r3::types;
///
/// let value = MeasureGroupPopulation {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroupPopulation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MeasureGroupPopulation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier
    pub identifier: Option<types::Identifier>,

    /// initial-population | numerator | numerator-exclusion | denominator |
    /// denominator-exclusion | denominator-exception | measure-population |
    /// measure-population-exclusion | measure-observation
    pub code: Option<types::CodeableConcept>,

    /// Short name
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The human readable description of this population criteria
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The name of a valid referenced CQL expression (may be namespaced) that
    /// defines this population criteria
    pub criteria: types::String,
    /// Primitive extension sibling for [`criteria`](Self::criteria) (FHIR `_criteria`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criteria")]
    pub criteria_ext: Option<types::Element>,
}

/// The stratifier criteria for the measure report, specified as either the
/// name of a valid CQL expression defined within a referenced library, or a
/// valid FHIR Resource Path.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure::MeasureGroupStratifier;
/// use fhir::r3::types;
///
/// let value = MeasureGroupStratifier {
///     criteria: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `criteria` is the name this serializes to on the wire.
/// assert_eq!(json["criteria"], ::serde_json::json!("abc"));
///
/// let back: MeasureGroupStratifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MeasureGroupStratifier {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identifier for the stratifier used to coordinate the reported data
    /// back to this stratifier
    pub identifier: Option<types::Identifier>,

    /// How the measure should be stratified
    pub criteria: Option<types::String>,
    /// Primitive extension sibling for [`criteria`](Self::criteria) (FHIR `_criteria`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criteria")]
    pub criteria_ext: Option<types::Element>,

    /// Path to the stratifier
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,
}

/// The supplemental data criteria for the measure report, specified as either
/// the name of a valid CQL expression within a referenced library, or a valid
/// FHIR Resource Path.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure::MeasureSupplementalData;
/// use fhir::r3::types;
///
/// let value = MeasureSupplementalData {
///     criteria: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `criteria` is the name this serializes to on the wire.
/// assert_eq!(json["criteria"], ::serde_json::json!("abc"));
///
/// let back: MeasureSupplementalData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MeasureSupplementalData {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier, unique within the measure
    pub identifier: Option<types::Identifier>,

    /// supplemental-data | risk-adjustment-factor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<types::CodeableConcept>,

    /// Expression describing additional data to be reported
    pub criteria: Option<types::String>,
    /// Primitive extension sibling for [`criteria`](Self::criteria) (FHIR `_criteria`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_criteria")]
    pub criteria_ext: Option<types::Element>,

    /// Path to the supplemental data element
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,
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
