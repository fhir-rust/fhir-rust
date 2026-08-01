//! Condition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Condition
//!
//! Version: 5.0.0
//!
//! Condition Resource: A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A clinical condition, problem, diagnosis, or other event, situation, issue,
/// or clinical concept that has risen to a level of concern.
///
/// The Condition resource is used to record detailed information about a
/// condition, problem, diagnosis, or other health matter that is of concern.
/// It can be used to record conditions that a patient has, is at risk of, or is
/// being managed for, and is central to problem lists, encounter diagnoses, and
/// clinical assessments in FHIR R5.
///
/// Typical uses include capturing a patient's active and historical problem
/// list, documenting a diagnosis reached during an encounter, and tracking the
/// clinical course of a condition over time through fields such as
/// `clinical_status`, `verification_status`, `onset`, `abatement`, and `stage`.
/// A `Condition` always refers back to the `subject` (usually a patient) for
/// whom the condition is asserted, and it may be linked to the `encounter`
/// during which it was identified or recorded.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the individual who is typically the subject of a condition.
/// - `Encounter` — the clinical encounter during which the condition was noted or assessed.
/// - `Observation` — clinical findings that may support or evidence a condition.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used throughout this resource for coded status, category, severity, and body site values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::condition::Condition;
/// use fhir::r5::types;
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
pub struct Condition {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The clinical status of the condition, e.g. active | recurrence | relapse | inactive | remission | resolved | unknown.
    pub clinical_status: types::CodeableConcept,

    /// The degree of confidence in the assertion, e.g. unconfirmed | provisional | differential | confirmed | refuted | entered-in-error.
    pub verification_status: Option<types::CodeableConcept>,

    /// A categorization of the condition, e.g. problem-list-item | encounter-diagnosis.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Subjective severity of condition
    pub severity: Option<types::CodeableConcept>,

    /// Identification of the condition, problem, or diagnosis, typically drawn from a terminology such as ICD-10 or SNOMED CT.
    pub code: Option<types::CodeableConcept>,

    /// Anatomical location, if relevant
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// The patient or group who has the condition; a reference to the relevant [`Patient`](crate::r5::resources::patient::Patient) (or group).
    pub subject: types::Reference,

    /// The encounter during which this condition was created or asserted
    pub encounter: Option<types::Reference>,

    /// The `Condition.onset[x]` choice element (0..1); see [`ConditionOnset`].
    #[serde(flatten)]
    pub onset: Option<ConditionOnset>,

    /// The `Condition.abatement[x]` choice element (0..1); see [`ConditionAbatement`].
    #[serde(flatten)]
    pub abatement: Option<ConditionAbatement>,

    /// Date condition was first recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`).
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Who or what participated in the activities related to the condition and how they were involved
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<ConditionParticipant>,

    /// Stage/grade, usually assessed formally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage: Vec<ConditionStage>,

    /// Supporting evidence for the verification status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<types::CodeableReference>,

    /// Additional information about the Condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Who or what participated in the activities related to the condition and how
/// they were involved.
/// # Examples
///
/// ```
/// use fhir::r5::resources::condition::ConditionParticipant;
/// use fhir::r5::types;
///
/// let value = ConditionParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConditionParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConditionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who or what participated in the activities related to the condition
    pub actor: types::Reference,
}

/// Clinical stage or grade of a condition, usually assessed formally.
/// # Examples
///
/// ```
/// use fhir::r5::resources::condition::ConditionStage;
/// use fhir::r5::types;
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
/// The `Condition.abatement[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ConditionAbatement {
    /// `abatementDateTime` variant.
    #[fhir("abatementDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
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
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `Condition.onset[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ConditionOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
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
    String(crate::r5::choice::Primitive<types::String>),
}
