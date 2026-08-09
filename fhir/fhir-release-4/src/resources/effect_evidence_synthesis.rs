//! EffectEvidenceSynthesis
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EffectEvidenceSynthesis
//!
//! Version: 4.0.1
//!
//! A quantified estimate of effect based on a body of evidence
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is
/// derived from a combination of research studies.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::effect_evidence_synthesis::EffectEvidenceSynthesis;
/// use fhir::r4::types;
///
/// let value = EffectEvidenceSynthesis {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: EffectEvidenceSynthesis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EffectEvidenceSynthesis {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this effect evidence synthesis, represented as
    /// a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the effect evidence synthesis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the effect evidence synthesis
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this effect evidence synthesis (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this effect evidence synthesis (human friendly)
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

    /// Natural language description of the effect evidence synthesis
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

    /// Intended jurisdiction for effect evidence synthesis (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// When the effect evidence synthesis was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the effect evidence synthesis was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the effect evidence synthesis is expected to be used
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
    pub population: types::Reference<crate::r4::resources::EvidenceVariable>,

    /// What exposure?
    pub exposure: types::Reference<crate::r4::resources::EvidenceVariable>,

    /// What comparison exposure?
    pub exposure_alternative: types::Reference<crate::r4::resources::EvidenceVariable>,

    /// What outcome?
    pub outcome: types::Reference<crate::r4::resources::EvidenceVariable>,

    /// What sample size was involved?
    pub sample_size: Option<EffectEvidenceSynthesisSampleSize>,

    /// What was the result per exposure?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results_by_exposure: Vec<EffectEvidenceSynthesisResultsByExposure>,

    /// What was the estimated effect
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effect_estimate: Vec<EffectEvidenceSynthesisEffectEstimate>,

    /// How certain is the effect
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certainty: Vec<EffectEvidenceSynthesisCertainty>,
}

/// A description of the certainty of the effect estimate.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::effect_evidence_synthesis::EffectEvidenceSynthesisCertainty;
/// use fhir::r4::types;
///
/// let value = EffectEvidenceSynthesisCertainty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EffectEvidenceSynthesisCertainty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EffectEvidenceSynthesisCertainty {
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
    pub certainty_subcomponent: Vec<EffectEvidenceSynthesisCertaintyCertaintySubcomponent>,
}

/// A description of a component of the overall certainty.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::effect_evidence_synthesis::EffectEvidenceSynthesisCertaintyCertaintySubcomponent;
/// use fhir::r4::types;
///
/// let value = EffectEvidenceSynthesisCertaintyCertaintySubcomponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EffectEvidenceSynthesisCertaintyCertaintySubcomponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EffectEvidenceSynthesisCertaintyCertaintySubcomponent {
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

/// The estimated effect of the exposure variant.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::effect_evidence_synthesis::EffectEvidenceSynthesisEffectEstimate;
/// use fhir::r4::types;
///
/// let value = EffectEvidenceSynthesisEffectEstimate {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: EffectEvidenceSynthesisEffectEstimate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EffectEvidenceSynthesisEffectEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of effect estimate
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Type of efffect estimate
    pub r#type: Option<types::CodeableConcept>,

    /// Variant exposure states
    pub variant_state: Option<types::CodeableConcept>,

    /// Point estimate
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// What unit is the outcome described in?
    pub unit_of_measure: Option<types::CodeableConcept>,

    /// How precise the estimate is
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub precision_estimate: Vec<EffectEvidenceSynthesisEffectEstimatePrecisionEstimate>,
}

/// A description of the precision of the estimate for the effect.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::effect_evidence_synthesis::EffectEvidenceSynthesisEffectEstimatePrecisionEstimate;
/// use fhir::r4::types;
///
/// let value = EffectEvidenceSynthesisEffectEstimatePrecisionEstimate {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EffectEvidenceSynthesisEffectEstimatePrecisionEstimate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EffectEvidenceSynthesisEffectEstimatePrecisionEstimate {
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

/// A description of the results for each exposure considered in the effect
/// estimate.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::effect_evidence_synthesis::EffectEvidenceSynthesisResultsByExposure;
/// use fhir::r4::types;
///
/// let value = EffectEvidenceSynthesisResultsByExposure {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: EffectEvidenceSynthesisResultsByExposure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EffectEvidenceSynthesisResultsByExposure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of results by exposure
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// exposure | exposure-alternative
    pub exposure_state: Option<crate::coded::Coded<crate::r4::codes::ExposureState>>,
    /// Primitive extension sibling for [`exposure_state`](Self::exposure_state) (FHIR `_exposureState`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exposureState")]
    pub exposure_state_ext: Option<types::Element>,

    /// Variant exposure states
    pub variant_state: Option<types::CodeableConcept>,

    /// Risk evidence synthesis
    pub risk_evidence_synthesis: types::Reference<crate::r4::resources::RiskEvidenceSynthesis>,
}

/// A description of the size of the sample involved in the synthesis.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::effect_evidence_synthesis::EffectEvidenceSynthesisSampleSize;
/// use fhir::r4::types;
///
/// let value = EffectEvidenceSynthesisSampleSize {
///     number_of_studies: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfStudies` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfStudies"], ::serde_json::json!(42));
///
/// let back: EffectEvidenceSynthesisSampleSize = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EffectEvidenceSynthesisSampleSize {
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
    type T = EffectEvidenceSynthesis;

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
