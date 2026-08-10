//! Condition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Condition
//!
//!
//!
//! Detailed information about conditions, problems or diagnoses
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Condition Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::condition::Condition;
/// use fhir::r3::types;
///
/// let value = Condition {
///     asserted_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `assertedDate` is the name this serializes to on the wire.
/// assert_eq!(json["assertedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Condition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Condition {
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | recurrence | inactive | remission | resolved
    pub clinical_status: Option<crate::coded::Coded<crate::r3::codes::ConditionClinical>>,
    /// Primitive extension sibling for [`clinical_status`](Self::clinical_status) (FHIR `_clinicalStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_clinicalStatus")]
    pub clinical_status_ext: Option<types::Element>,

    /// provisional | differential | confirmed | refuted | entered-in-error |
    /// unknown
    pub verification_status: Option<crate::coded::Coded<crate::r3::codes::ConditionVerStatus>>,
    /// Primitive extension sibling for [`verification_status`](Self::verification_status) (FHIR `_verificationStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_verificationStatus")]
    pub verification_status_ext: Option<types::Element>,

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

    /// Encounter or episode when condition first asserted
    pub context: Option<types::Reference>,

    /// Estimated or actual date, date-time, or age
    /// The `Condition.onset[x]` choice element (0..1); see [`ConditionOnset`].
    #[serde(flatten)]
    pub onset: Option<ConditionOnset>,

    /// If/when in resolution/remission
    /// The `Condition.abatement[x]` choice element (0..1); see [`ConditionAbatement`].
    #[serde(flatten)]
    pub abatement: Option<ConditionAbatement>,

    /// Date record was believed accurate
    pub asserted_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`asserted_date`](Self::asserted_date) (FHIR `_assertedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_assertedDate")]
    pub asserted_date_ext: Option<types::Element>,

    /// Person who asserts this condition
    pub asserter: Option<types::Reference>,

    /// Stage/grade, usually assessed formally
    pub stage: Option<ConditionStage>,

    /// Supporting evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<ConditionEvidence>,

    /// Additional information about the Condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Supporting Evidence / manifestations that are the basis on which this
/// condition is suspected or confirmed.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::condition::ConditionEvidence;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ConditionEvidence {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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
/// use fhir::r3::resources::condition::ConditionStage;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ConditionStage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Simple summary (disease specific)
    pub summary: Option<types::CodeableConcept>,

    /// Formal record of assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assessment: Vec<types::Reference>,
}

/// The `Condition.onset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
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
    String(crate::r3::choice::Primitive<types::String>),
}

/// The `Condition.abatement[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionAbatement {
    /// `abatementDateTime` variant.
    #[fhir("abatementDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `abatementAge` variant.
    #[fhir("abatementAge")]
    Age(Box<types::Age>),
    /// `abatementBoolean` variant.
    #[fhir("abatementBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `abatementPeriod` variant.
    #[fhir("abatementPeriod")]
    Period(Box<types::Period>),
    /// `abatementRange` variant.
    #[fhir("abatementRange")]
    Range(Box<types::Range>),
    /// `abatementString` variant.
    #[fhir("abatementString")]
    String(crate::r3::choice::Primitive<types::String>),
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
