//! RiskAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
//!
//! Version: 5.0.0
//!
//! RiskAssessment Resource: An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// RiskAssessment
///
/// An assessment of the likely outcome(s) for a patient or other subject as
/// well as the likelihood of each outcome. It captures the method used to make
/// the assessment, the condition or context that prompted it, and one or more
/// predictions describing the probability, qualitative risk, and timeframe of
/// each possible outcome. RiskAssessment resources are commonly used to record
/// clinical decision support results and probabilistic prognoses, such as the
/// likelihood of a disease occurring, a treatment succeeding, or a future
/// clinical event taking place. Assessments may be generated manually by a
/// clinician, derived from a decision-support tool or predictive algorithm, or
/// produced by a risk-scoring calculation engine, and can be linked back to the
/// observations, conditions, or other evidence (`basis`) that informed the
/// prediction.
///
/// Related resources: the subject of a RiskAssessment is typically a
/// [`Patient`](crate::r5::resources::patient::Patient), the evaluation method and
/// predicted outcomes are represented using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and a RiskAssessment
/// may reference a `Condition` being assessed or an `Encounter` during which the
/// assessment was performed.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::risk_assessment::RiskAssessment;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessment {
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

    /// Unique identifier for the assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Request fulfilled by this assessment
    pub based_on: Option<types::Reference>,

    /// Part of this occurrence
    pub parent: Option<types::Reference>,

    /// The status of the RiskAssessment, using the codes registered | preliminary | final | amended +
    pub status: crate::r5::coded::Coded<crate::r5::codes::ObservationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The algorithm, risk-scoring tool, or evaluation mechanism used to generate the assessment
    pub method: Option<types::CodeableConcept>,

    /// The type of the risk assessment being performed, for example a general clinical risk assessment or a specific screening tool
    pub code: Option<types::CodeableConcept>,

    /// The patient or group the risk assessment applies to
    pub subject: types::Reference,

    /// Where was assessment performed?
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `RiskAssessment.occurrence[x]` choice element (0..1); see [`RiskAssessmentOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<RiskAssessmentOccurrence>,

    /// Condition assessed
    pub condition: Option<types::Reference<crate::r5::resources::Condition>>,

    /// Who did assessment?
    pub performer: Option<types::Reference>,

    /// Why the assessment was necessary?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Information used in assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basis: Vec<types::Reference>,

    /// One or more predicted outcomes for the subject, each with its own probability, qualitative risk, and timeframe
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prediction: Vec<RiskAssessmentPrediction>,

    /// Recommended steps to reduce the predicted risk, or an indication that no mitigation is available
    pub mitigation: Option<types::String>,
    /// Primitive extension sibling for [`mitigation`](Self::mitigation) (FHIR `_mitigation`).
    #[serde(rename = "_mitigation")]
    pub mitigation_ext: Option<types::Element>,

    /// Comments on the risk assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// RiskAssessmentPrediction
///
/// Describes the expected outcome for the subject, including the likelihood of
/// that outcome and the timeframe over which the risk applies.
/// # Examples
///
/// ```
/// use fhir::r5::resources::risk_assessment::RiskAssessmentPrediction;
/// use fhir::r5::types;
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
pub struct RiskAssessmentPrediction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Possible outcome for the subject
    pub outcome: Option<types::CodeableConcept>,

    /// The `RiskAssessment.prediction.probability[x]` choice element (0..1); see [`RiskAssessmentPredictionProbability`].
    #[serde(flatten)]
    pub probability: Option<RiskAssessmentPredictionProbability>,

    /// Likelihood of specified outcome as a qualitative value
    pub qualitative_risk: Option<types::CodeableConcept>,

    /// Relative likelihood
    pub relative_risk: Option<types::Decimal>,
    /// Primitive extension sibling for [`relative_risk`](Self::relative_risk) (FHIR `_relativeRisk`).
    #[serde(rename = "_relativeRisk")]
    pub relative_risk_ext: Option<types::Element>,

    /// The `RiskAssessment.prediction.when[x]` choice element (0..1); see [`RiskAssessmentPredictionWhen`].
    #[serde(flatten)]
    pub when: Option<RiskAssessmentPredictionWhen>,

    /// Explanation of prediction
    pub rationale: Option<types::String>,
    /// Primitive extension sibling for [`rationale`](Self::rationale) (FHIR `_rationale`).
    #[serde(rename = "_rationale")]
    pub rationale_ext: Option<types::Element>,
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
/// The `RiskAssessment.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

/// The `RiskAssessment.prediction.probability[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentPredictionProbability {
    /// `probabilityDecimal` variant.
    #[fhir("probabilityDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `probabilityRange` variant.
    #[fhir("probabilityRange")]
    Range(Box<types::Range>),
}

/// The `RiskAssessment.prediction.when[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentPredictionWhen {
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
    /// `whenRange` variant.
    #[fhir("whenRange")]
    Range(Box<types::Range>),
}
