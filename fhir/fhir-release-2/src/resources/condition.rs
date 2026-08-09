//! Condition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Condition
//!
//!
//!
//! Detailed information about conditions, problems or diagnoses
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Condition Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::condition::Condition;
/// use fhir::r2::types;
///
/// let value = Condition {
///     date_recorded: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dateRecorded` is the name this serializes to on the wire.
/// assert_eq!(json["dateRecorded"], ::serde_json::json!("2019-11-01"));
///
/// let back: Condition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who has the condition?
    pub patient: types::Reference<crate::r2::resources::Patient>,

    /// Encounter when condition first asserted
    pub encounter: Option<types::Reference<crate::r2::resources::Encounter>>,

    /// Person who asserts this condition
    pub asserter: Option<types::Reference>,

    /// When first entered
    pub date_recorded: Option<types::Date>,
    /// Primitive extension sibling for [`date_recorded`](Self::date_recorded) (FHIR `_dateRecorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateRecorded")]
    pub date_recorded_ext: Option<types::Element>,

    /// Identification of the condition, problem or diagnosis
    pub code: types::CodeableConcept,

    /// complaint | symptom | finding | diagnosis
    pub category: Option<types::CodeableConcept>,

    /// active | relapse | remission | resolved
    pub clinical_status: Option<types::Code>,
    /// Primitive extension sibling for [`clinical_status`](Self::clinical_status) (FHIR `_clinicalStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_clinicalStatus")]
    pub clinical_status_ext: Option<types::Element>,

    /// provisional | differential | confirmed | refuted | entered-in-error |
    /// unknown
    pub verification_status: crate::coded::Coded<crate::r2::codes::ConditionVerStatus>,
    /// Primitive extension sibling for [`verification_status`](Self::verification_status) (FHIR `_verificationStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_verificationStatus")]
    pub verification_status_ext: Option<types::Element>,

    /// Subjective severity of condition
    pub severity: Option<types::CodeableConcept>,

    /// Estimated or actual date, date-time, or age
    /// The `Condition.onset[x]` choice element (0..1); see [`ConditionOnset`].
    #[serde(flatten)]
    pub onset: Option<ConditionOnset>,

    /// If/when in resolution/remission
    /// The `Condition.abatement[x]` choice element (0..1); see [`ConditionAbatement`].
    #[serde(flatten)]
    pub abatement: Option<ConditionAbatement>,

    /// Stage/grade, usually assessed formally
    pub stage: Option<ConditionStage>,

    /// Supporting evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<ConditionEvidence>,

    /// Anatomical location, if relevant
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// Additional information about the Condition
    pub notes: Option<types::String>,
    /// Primitive extension sibling for [`notes`](Self::notes) (FHIR `_notes`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_notes")]
    pub notes_ext: Option<types::Element>,
}

/// Supporting Evidence / manifestations that are the basis on which this
/// condition is suspected or confirmed.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::condition::ConditionEvidence;
/// use fhir::r2::types;
///
/// let value = ConditionEvidence {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ConditionEvidence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConditionEvidence {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Manifestation/symptom
    pub code: Option<types::CodeableConcept>,

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
/// use fhir::r2::resources::condition::ConditionStage;
/// use fhir::r2::types;
///
/// let value = ConditionStage {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ConditionStage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConditionStage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionOnset {
    /// `onsetDateTime` variant.
    #[fhir("onsetDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `onsetQuantity` variant.
    #[fhir("onsetQuantity")]
    Quantity(Box<types::Quantity>),
    /// `onsetPeriod` variant.
    #[fhir("onsetPeriod")]
    Period(Box<types::Period>),
    /// `onsetRange` variant.
    #[fhir("onsetRange")]
    Range(Box<types::Range>),
    /// `onsetString` variant.
    #[fhir("onsetString")]
    String(crate::r2::choice::Primitive<types::String>),
}

/// The `Condition.abatement[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionAbatement {
    /// `abatementDateTime` variant.
    #[fhir("abatementDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `abatementQuantity` variant.
    #[fhir("abatementQuantity")]
    Quantity(Box<types::Quantity>),
    /// `abatementBoolean` variant.
    #[fhir("abatementBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `abatementPeriod` variant.
    #[fhir("abatementPeriod")]
    Period(Box<types::Period>),
    /// `abatementRange` variant.
    #[fhir("abatementRange")]
    Range(Box<types::Range>),
    /// `abatementString` variant.
    #[fhir("abatementString")]
    String(crate::r2::choice::Primitive<types::String>),
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
