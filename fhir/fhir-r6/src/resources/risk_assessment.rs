//! RiskAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RiskAssessment
//!
//! Version: 6.0.0-ballot3
//!
//! Potential outcomes for a subject with likelihood
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An assessment of the likely outcome(s) for a patient or other subject as
/// well as the likelihood of each outcome.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::risk_assessment::RiskAssessment;
/// use fhir::r6::types;
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
#[serde(from = "RiskAssessmentDe")]
#[fhir_version("r6")]
pub struct RiskAssessment {
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

    /// Unique identifier for the assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Request fulfilled by this assessment
    pub based_on: Option<types::Reference>,

    /// Part of this occurrence
    pub parent: Option<types::Reference>,

    /// registered | specimen-in-process | preliminary | final | amended |
    /// corrected | appended | cancelled | entered-in-error | unknown |
    /// cannot-be-obtained
    pub status: crate::coded::Coded<crate::r6::codes::ObservationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Evaluation mechanism
    pub method: Option<types::CodeableConcept>,

    /// Type of assessment
    pub code: Option<types::CodeableConcept>,

    /// Who/what does assessment apply to?
    pub subject: types::Reference,

    /// Where was assessment performed?
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// When was assessment made?
    /// The `RiskAssessment.occurrence[x]` choice element (0..1); see [`RiskAssessmentOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<RiskAssessmentOccurrence>,

    /// Condition assessed
    pub condition: Option<types::Reference<crate::r6::resources::Condition>>,

    /// Who did assessment?
    pub performer: Option<types::Reference>,

    /// Why was the assessment necessary?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RiskAssessmentDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    based_on: Option<types::Reference>,
    parent: Option<types::Reference>,
    status: crate::coded::Coded<crate::r6::codes::ObservationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    method: Option<types::CodeableConcept>,
    code: Option<types::CodeableConcept>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r6::choice::Slot<RiskAssessmentOccurrence>,
    condition: Option<types::Reference<crate::r6::resources::Condition>>,
    performer: Option<types::Reference>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    #[serde(default)]
    basis: Vec<types::Reference>,
    #[serde(default)]
    prediction: Vec<RiskAssessmentPrediction>,
    mitigation: Option<types::String>,
    #[serde(rename = "_mitigation")]
    mitigation_ext: Option<types::Element>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<RiskAssessmentDe> for RiskAssessment {
    fn from(v: RiskAssessmentDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            based_on: v.based_on,
            parent: v.parent,
            status: v.status,
            status_ext: v.status_ext,
            method: v.method,
            code: v.code,
            subject: v.subject,
            encounter: v.encounter,
            occurrence: v.occurrence.0,
            condition: v.condition,
            performer: v.performer,
            reason: v.reason,
            basis: v.basis,
            prediction: v.prediction,
            mitigation: v.mitigation,
            mitigation_ext: v.mitigation_ext,
            note: v.note,
        }
    }
}

/// Describes the expected outcome for the subject.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::risk_assessment::RiskAssessmentPrediction;
/// use fhir::r6::types;
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
#[serde(from = "RiskAssessmentPredictionDe")]
#[fhir_version("r6")]
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RiskAssessmentPredictionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    outcome: Option<types::CodeableConcept>,
    #[serde(flatten)]
    probability: crate::r6::choice::Slot<RiskAssessmentPredictionProbability>,
    qualitative_risk: Option<types::CodeableConcept>,
    relative_risk: Option<types::Decimal>,
    #[serde(rename = "_relativeRisk")]
    relative_risk_ext: Option<types::Element>,
    #[serde(flatten)]
    when: crate::r6::choice::Slot<RiskAssessmentPredictionWhen>,
    rationale: Option<types::String>,
    #[serde(rename = "_rationale")]
    rationale_ext: Option<types::Element>,
}

impl ::core::convert::From<RiskAssessmentPredictionDe> for RiskAssessmentPrediction {
    fn from(v: RiskAssessmentPredictionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            outcome: v.outcome,
            probability: v.probability.0,
            qualitative_risk: v.qualitative_risk,
            relative_risk: v.relative_risk,
            relative_risk_ext: v.relative_risk_ext,
            when: v.when.0,
            rationale: v.rationale,
            rationale_ext: v.rationale_ext,
        }
    }
}

/// The `RiskAssessment.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
}

/// The `RiskAssessment.prediction.probability[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RiskAssessmentPredictionProbability {
    /// `probabilityDecimal` variant.
    #[fhir("probabilityDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `probabilityRange` variant.
    #[fhir("probabilityRange")]
    Range(Box<types::Range>),
}

/// The `RiskAssessment.prediction.when[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
