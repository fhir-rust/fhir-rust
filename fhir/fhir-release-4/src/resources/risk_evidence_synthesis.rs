//! RiskEvidenceSynthesis
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RiskEvidenceSynthesis
//!
//! Version: 4.0.1
//!
//! A quantified estimate of risk based on a body of evidence
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome
/// in a population plus exposure state where the risk estimate is derived from
/// a combination of research studies.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::risk_evidence_synthesis::RiskEvidenceSynthesis;
/// use fhir::r4::types;
///
/// let value = RiskEvidenceSynthesis {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: RiskEvidenceSynthesis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct RiskEvidenceSynthesis {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this risk evidence synthesis, represented as a
    /// URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the risk evidence synthesis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the risk evidence synthesis
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this risk evidence synthesis (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this risk evidence synthesis (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
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

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the risk evidence synthesis
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for risk evidence synthesis (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// When the risk evidence synthesis was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the risk evidence synthesis was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the risk evidence synthesis is expected to be used
    pub effective_period: Option<types::Period>,

    /// The category of the EffectEvidenceSynthesis, such as Education,
    /// Treatment, Assessment, etc.
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

    /// Additional documentation, citations, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Type of synthesis
    pub synthesis_type: Option<types::CodeableConcept>,

    /// Type of study
    pub study_type: Option<types::CodeableConcept>,

    /// What population?
    pub population: types::Reference,

    /// What exposure?
    pub exposure: Option<types::Reference>,

    /// What outcome?
    pub outcome: types::Reference,

    /// What sample size was involved?
    pub sample_size: Option<RiskEvidenceSynthesisSampleSize>,

    /// What was the estimated risk
    pub risk_estimate: Option<RiskEvidenceSynthesisRiskEstimate>,

    /// How certain is the risk
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty: Vec<RiskEvidenceSynthesisCertainty>,
}

/// A description of the certainty of the risk estimate.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::risk_evidence_synthesis::RiskEvidenceSynthesisCertainty;
/// use fhir::r4::types;
///
/// let value = RiskEvidenceSynthesisCertainty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RiskEvidenceSynthesisCertainty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct RiskEvidenceSynthesisCertainty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Certainty rating
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rating: Vec<types::CodeableConcept>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// A component that contributes to the overall certainty
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty_subcomponent: Vec<RiskEvidenceSynthesisCertaintyCertaintySubcomponent>,
}

/// A description of a component of the overall certainty.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::risk_evidence_synthesis::RiskEvidenceSynthesisCertaintyCertaintySubcomponent;
/// use fhir::r4::types;
///
/// let value = RiskEvidenceSynthesisCertaintyCertaintySubcomponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RiskEvidenceSynthesisCertaintyCertaintySubcomponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct RiskEvidenceSynthesisCertaintyCertaintySubcomponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of subcomponent of certainty rating
    pub r#type: Option<types::CodeableConcept>,

    /// Subcomponent certainty rating
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rating: Vec<types::CodeableConcept>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// The estimated risk of the outcome.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::risk_evidence_synthesis::RiskEvidenceSynthesisRiskEstimate;
/// use fhir::r4::types;
///
/// let value = RiskEvidenceSynthesisRiskEstimate {
///     denominator_count: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `denominatorCount` is the name this serializes to on the wire.
/// assert_eq!(json["denominatorCount"], ::serde_json::json!(42));
///
/// let back: RiskEvidenceSynthesisRiskEstimate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct RiskEvidenceSynthesisRiskEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of risk estimate
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Type of risk estimate
    pub r#type: Option<types::CodeableConcept>,

    /// Point estimate
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// What unit is the outcome described in?
    pub unit_of_measure: Option<types::CodeableConcept>,

    /// Sample size for group measured
    pub denominator_count: Option<types::Integer>,
    /// Primitive extension sibling for [`denominator_count`](Self::denominator_count) (FHIR `_denominatorCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_denominatorCount")]
    pub denominator_count_ext: Option<types::Element>,

    /// Number with the outcome
    pub numerator_count: Option<types::Integer>,
    /// Primitive extension sibling for [`numerator_count`](Self::numerator_count) (FHIR `_numeratorCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numeratorCount")]
    pub numerator_count_ext: Option<types::Element>,

    /// How precise the estimate is
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub precision_estimate: Vec<RiskEvidenceSynthesisRiskEstimatePrecisionEstimate>,
}

/// A description of the precision of the estimate for the effect.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::risk_evidence_synthesis::RiskEvidenceSynthesisRiskEstimatePrecisionEstimate;
/// use fhir::r4::types;
///
/// let value = RiskEvidenceSynthesisRiskEstimatePrecisionEstimate {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RiskEvidenceSynthesisRiskEstimatePrecisionEstimate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct RiskEvidenceSynthesisRiskEstimatePrecisionEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of precision estimate
    pub r#type: Option<types::CodeableConcept>,

    /// Level of confidence interval
    pub level: Option<types::Decimal>,
    /// Primitive extension sibling for [`level`](Self::level) (FHIR `_level`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_level")]
    pub level_ext: Option<types::Element>,

    /// Lower bound
    pub from: Option<types::Decimal>,
    /// Primitive extension sibling for [`from`](Self::from) (FHIR `_from`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_from")]
    pub from_ext: Option<types::Element>,

    /// Upper bound
    pub to: Option<types::Decimal>,
    /// Primitive extension sibling for [`to`](Self::to) (FHIR `_to`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_to")]
    pub to_ext: Option<types::Element>,
}

/// A description of the size of the sample involved in the synthesis.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::risk_evidence_synthesis::RiskEvidenceSynthesisSampleSize;
/// use fhir::r4::types;
///
/// let value = RiskEvidenceSynthesisSampleSize {
///     number_of_studies: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfStudies` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfStudies"], ::serde_json::json!(42));
///
/// let back: RiskEvidenceSynthesisSampleSize = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct RiskEvidenceSynthesisSampleSize {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of sample size
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// How many studies?
    pub number_of_studies: Option<types::Integer>,
    /// Primitive extension sibling for [`number_of_studies`](Self::number_of_studies) (FHIR `_numberOfStudies`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfStudies")]
    pub number_of_studies_ext: Option<types::Element>,

    /// How many participants?
    pub number_of_participants: Option<types::Integer>,
    /// Primitive extension sibling for [`number_of_participants`](Self::number_of_participants) (FHIR `_numberOfParticipants`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfParticipants")]
    pub number_of_participants_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RiskEvidenceSynthesis;

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
