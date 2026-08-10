//! Condition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Condition
//!
//! Version: 6.0.0-ballot3
//!
//! Detailed information about conditions, problems or diagnoses
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A clinical condition, problem, diagnosis, or other event, situation, issue,
/// or clinical concept that has risen to a level of concern.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::condition::Condition;
/// use fhir::r6::types;
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
#[serde(from = "ConditionDe")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | recurrence | relapse | inactive | remission | resolved |
    /// unknown
    pub clinical_status: types::CodeableConcept,

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

    /// Anatomical body structure
    pub body_structure: Option<types::Reference<crate::r6::resources::BodyStructure>>,

    /// Who has the condition?
    pub subject: types::Reference,

    /// The Encounter during which this Condition was created
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Estimated or actual date, date-time, or age
    /// The `Condition.onset[x]` choice element (0..1); see [`ConditionOnset`].
    #[serde(flatten)]
    pub onset: Option<ConditionOnset>,

    /// When in resolution/remission
    /// The `Condition.abatement[x]` choice element (0..1); see [`ConditionAbatement`].
    #[serde(flatten)]
    pub abatement: Option<ConditionAbatement>,

    /// Date condition was first recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Who recorded the condition
    pub recorder: Option<types::Reference>,

    /// Person or device that asserts this condition
    pub asserter: Option<types::Reference>,

    /// Stage/grade, usually assessed formally
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage: Vec<ConditionStage>,

    /// Supporting evidence for the condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<types::CodeableReference>,

    /// Additional information about the Condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConditionDe {
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
    clinical_status: types::CodeableConcept,
    verification_status: Option<types::CodeableConcept>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    severity: Option<types::CodeableConcept>,
    code: Option<types::CodeableConcept>,
    #[serde(default)]
    body_site: Vec<types::CodeableConcept>,
    body_structure: Option<types::Reference<crate::r6::resources::BodyStructure>>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(flatten)]
    onset: crate::r6::choice::Slot<ConditionOnset>,
    #[serde(flatten)]
    abatement: crate::r6::choice::Slot<ConditionAbatement>,
    recorded_date: Option<types::DateTime>,
    #[serde(rename = "_recordedDate")]
    recorded_date_ext: Option<types::Element>,
    recorder: Option<types::Reference>,
    asserter: Option<types::Reference>,
    #[serde(default)]
    stage: Vec<ConditionStage>,
    #[serde(default)]
    evidence: Vec<types::CodeableReference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<ConditionDe> for Condition {
    fn from(v: ConditionDe) -> Self {
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
            clinical_status: v.clinical_status,
            verification_status: v.verification_status,
            category: v.category,
            severity: v.severity,
            code: v.code,
            body_site: v.body_site,
            body_structure: v.body_structure,
            subject: v.subject,
            encounter: v.encounter,
            onset: v.onset.0,
            abatement: v.abatement.0,
            recorded_date: v.recorded_date,
            recorded_date_ext: v.recorded_date_ext,
            recorder: v.recorder,
            asserter: v.asserter,
            stage: v.stage,
            evidence: v.evidence,
            note: v.note,
        }
    }
}

/// A simple summary of the stage such as "Stage 3" or "Early Onset". The
/// determination of the stage is disease-specific, such as cancer, retinopathy
/// of prematurity, kidney diseases, Alzheimer's, or Parkinson disease.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::condition::ConditionStage;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
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
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `Condition.abatement[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionAbatement {
    /// `abatementDateTime` variant.
    #[fhir("abatementDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
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
    String(crate::r6::choice::Primitive<types::String>),
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
