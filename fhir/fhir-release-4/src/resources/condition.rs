//! Condition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Condition
//!
//! Version: 4.0.1
//!
//! Detailed information about conditions, problems or diagnoses
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A clinical condition, problem, diagnosis, or other event, situation, issue,
/// or clinical concept that has risen to a level of concern.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::condition::Condition;
/// use fhir::r4::types;
///
/// let value = Condition {
///     recorded_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `recordedDate` is the name this serializes to on the wire.
/// assert_eq!(json["recordedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Condition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Condition {
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

    /// External Ids for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | recurrence | relapse | inactive | remission | resolved
    pub clinical_status: Option<types::CodeableConcept>,

    /// unconfirmed | provisional | differential | confirmed | refuted |
    /// entered-in-error
    pub verification_status: Option<types::CodeableConcept>,

    /// problem-list-item | encounter-diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Subjective severity of condition
    pub severity: Option<types::CodeableConcept>,

    /// Identification of the condition, problem or diagnosis
    pub code: Option<types::CodeableConcept>,

    /// Anatomical location, if relevant
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// Who has the condition?
    pub subject: types::Reference,

    /// Encounter created as part of
    pub encounter: Option<types::Reference>,

    /// Estimated or actual date, date-time, or age
    /// The `Condition.onset[x]` choice element (0..1); see [`ConditionOnset`].
    #[serde(flatten)]
    pub onset: Option<ConditionOnset>,

    /// When in resolution/remission
    /// The `Condition.abatement[x]` choice element (0..1); see [`ConditionAbatement`].
    #[serde(flatten)]
    pub abatement: Option<ConditionAbatement>,

    /// Date record was first recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Who recorded the condition
    pub recorder: Option<types::Reference>,

    /// Person who asserts this condition
    pub asserter: Option<types::Reference>,

    /// Stage/grade, usually assessed formally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage: Vec<ConditionStage>,

    /// Supporting evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<ConditionEvidence>,

    /// Additional information about the Condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Supporting evidence / manifestations that are the basis of the Condition's
/// verification status, such as evidence that confirmed or refuted the
/// condition.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::condition::ConditionEvidence;
/// use fhir::r4::types;
///
/// let value = ConditionEvidence {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConditionEvidence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ConditionEvidence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Manifestation/symptom
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Supporting information found elsewhere
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<types::Reference>,
}

/// Clinical stage or grade of a condition. May include formal severity
/// assessments.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::condition::ConditionStage;
/// use fhir::r4::types;
///
/// let value = ConditionStage {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConditionStage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ConditionStage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Simple summary (disease specific)
    pub summary: Option<types::CodeableConcept>,

    /// Formal record of assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assessment: Vec<types::Reference>,

    /// Kind of staging
    pub r#type: Option<types::CodeableConcept>,
}

/// The `Condition.onset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
    /// `onsetAge` variant.
    #[fhir("onsetAge")]
    Age(Box<types::Age>),
    /// `onsetPeriod` variant.
    #[fhir("onsetPeriod")]
    Period(Box<types::Period>),
    /// `onsetRange` variant.
    #[fhir("onsetRange")]
    Range(Box<types::Range>),
    /// `onsetString` variant.
    #[fhir("onsetString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `Condition.abatement[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionAbatement {
    /// `abatementDateTime` variant.
    #[fhir("abatementDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
    /// `abatementAge` variant.
    #[fhir("abatementAge")]
    Age(Box<types::Age>),
    /// `abatementPeriod` variant.
    #[fhir("abatementPeriod")]
    Period(Box<types::Period>),
    /// `abatementRange` variant.
    #[fhir("abatementRange")]
    Range(Box<types::Range>),
    /// `abatementString` variant.
    #[fhir("abatementString")]
    String(crate::r4::choice::Primitive<types::String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Condition;

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
