//! RiskAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
//!
//!
//!
//! Potential outcomes for a subject with likelihood
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for RiskAssessment Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::risk_assessment::RiskAssessment;
/// use fhir::r3::types;
///
/// let value = RiskAssessment {
///     mitigation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `mitigation` is the name this serializes to on the wire.
/// assert_eq!(json["mitigation"], ::serde_json::json!("abc"));
///
/// let back: RiskAssessment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct RiskAssessment {
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

    /// Unique identifier for the assessment
    pub identifier: Option<types::Identifier>,

    /// Request fulfilled by this assessment
    pub based_on: Option<types::Reference>,

    /// Part of this occurrence
    pub parent: Option<types::Reference>,

    /// registered | preliminary | final | amended +
    pub status: crate::coded::Coded<crate::r3::codes::ObservationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Evaluation mechanism
    pub method: Option<types::CodeableConcept>,

    /// Type of assessment
    pub code: Option<types::CodeableConcept>,

    /// Who/what does assessment apply to?
    pub subject: Option<types::Reference>,

    /// Where was assessment performed?
    pub context: Option<types::Reference>,

    /// When was assessment made?
    /// The `RiskAssessment.occurrence[x]` choice element (0..1); see [`RiskAssessmentOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<RiskAssessmentOccurrence>,

    /// Condition assessed
    pub condition: Option<types::Reference>,

    /// Who did assessment?
    pub performer: Option<types::Reference>,

    /// Why the assessment was necessary?
    /// The `RiskAssessment.reason[x]` choice element (0..1); see [`RiskAssessmentReason`].
    #[serde(flatten)]
    pub reason: Option<RiskAssessmentReason>,

    /// Information used in assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basis: Vec<types::Reference>,

    /// Outcome predicted
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prediction: Vec<RiskAssessmentPrediction>,

    /// How to reduce risk
    pub mitigation: Option<types::String>,
    /// Primitive extension sibling for [`mitigation`](Self::mitigation) (FHIR `_mitigation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mitigation")]
    pub mitigation_ext: Option<types::Element>,

    /// Comments on the risk assessment
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// Describes the expected outcome for the subject.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::risk_assessment::RiskAssessmentPrediction;
/// use fhir::r3::types;
///
/// let value = RiskAssessmentPrediction {
///     rationale: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `rationale` is the name this serializes to on the wire.
/// assert_eq!(json["rationale"], ::serde_json::json!("abc"));
///
/// let back: RiskAssessmentPrediction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct RiskAssessmentPrediction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Possible outcome for the subject
    pub outcome: types::CodeableConcept,

    /// Likelihood of specified outcome
    /// The `RiskAssessment.prediction.probability[x]` choice element (0..1); see [`RiskAssessmentPredictionProbability`].
    #[serde(flatten)]
    pub probability: Option<RiskAssessmentPredictionProbability>,

    /// Likelihood of specified outcome as a qualitative value
    pub qualitative_risk: Option<types::CodeableConcept>,

    /// Relative likelihood
    pub relative_risk: Option<types::Decimal>,
    /// Primitive extension sibling for [`relative_risk`](Self::relative_risk) (FHIR `_relativeRisk`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relativeRisk")]
    pub relative_risk_ext: Option<types::Element>,

    /// Timeframe or age range
    /// The `RiskAssessment.prediction.when[x]` choice element (0..1); see [`RiskAssessmentPredictionWhen`].
    #[serde(flatten)]
    pub when: Option<RiskAssessmentPredictionWhen>,

    /// Explanation of prediction
    pub rationale: Option<types::String>,
    /// Primitive extension sibling for [`rationale`](Self::rationale) (FHIR `_rationale`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rationale")]
    pub rationale_ext: Option<types::Element>,
}

/// The `RiskAssessment.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

/// The `RiskAssessment.reason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentReason {
    /// `reasonCodeableConcept` variant.
    #[fhir("reasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonReference` variant.
    #[fhir("reasonReference")]
    Reference(Box<types::Reference>),
}

/// The `RiskAssessment.prediction.probability[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentPredictionProbability {
    /// `probabilityDecimal` variant.
    #[fhir("probabilityDecimal")]
    Decimal(crate::r3::choice::Primitive<types::Decimal>),
    /// `probabilityRange` variant.
    #[fhir("probabilityRange")]
    Range(Box<types::Range>),
}

/// The `RiskAssessment.prediction.when[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentPredictionWhen {
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
    /// `whenRange` variant.
    #[fhir("whenRange")]
    Range(Box<types::Range>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RiskAssessment;

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
