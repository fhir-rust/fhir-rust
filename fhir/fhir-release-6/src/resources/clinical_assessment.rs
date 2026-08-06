//! ClinicalAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClinicalAssessment
//!
//! Version: 6.0.0-ballot3
//!
//! A clinical assessment performed when planning treatments and management
//! strategies for a patient
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a clinical assessment performed to determine what problem(s)
/// may affect the patient and before planning the treatments or management
/// strategies that are best to manage a patient's condition. Assessments are
/// often 1:1 with a clinical consultation / encounter, but this varies greatly
/// depending on the clinical workflow. This resource is called
/// "ClinicalAssessment" rather than "ClinicalAssessment" to avoid confusion
/// with the recording of assessment tools such as Apgar score.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::clinical_assessment::ClinicalAssessment;
/// use fhir::r6::types;
///
/// let value = ClinicalAssessment {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: ClinicalAssessment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClinicalAssessment {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed |
    /// entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Why/how the assessment was performed
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Patient or group assessed
    pub subject: types::Reference,

    /// The Encounter during which this ClinicalAssessment was created
    pub encounter: Option<types::Reference>,

    /// Time of assessment
    /// The `ClinicalAssessment.effective[x]` choice element (0..1); see [`ClinicalAssessmentEffective`].
    #[serde(flatten)]
    pub effective: Option<ClinicalAssessmentEffective>,

    /// When the assessment was documented
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The clinician performing the assessment
    pub performer: Option<types::Reference>,

    /// Reference to last assessment
    pub previous: Option<types::Reference>,

    /// Relevant assessments of patient state
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problem: Vec<types::Reference>,

    /// Change in the status/pattern of a subject's condition since previously
    /// assessed, such as worsening, improving, or no change
    pub change_pattern: Option<types::CodeableConcept>,

    /// Clinical Protocol followed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<types::Uri>,
    /// Primitive extension sibling for [`protocol`](Self::protocol) (FHIR `_protocol`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_protocol")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_ext: Vec<Option<types::Element>>,

    /// Text summary of the assessment
    pub summary: Option<types::Markdown>,
    /// Primitive extension sibling for [`summary`](Self::summary) (FHIR `_summary`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_summary")]
    pub summary_ext: Option<types::Element>,

    /// Possible or likely findings and diagnoses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finding: Vec<ClinicalAssessmentFinding>,

    /// Estimate of likely outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prognosis_codeable_concept: Vec<types::CodeableConcept>,

    /// RiskAssessment expressing likely outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prognosis_reference: Vec<types::Reference>,

    /// Information supporting the clinical assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Comments made about the ClinicalAssessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Specific findings or diagnoses that were considered likely or relevant to
/// ongoing treatment.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::clinical_assessment::ClinicalAssessmentFinding;
/// use fhir::r6::types;
///
/// let value = ClinicalAssessmentFinding {
///     basis: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `basis` is the name this serializes to on the wire.
/// assert_eq!(json["basis"], ::serde_json::json!("abc"));
///
/// let back: ClinicalAssessmentFinding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClinicalAssessmentFinding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What was found
    pub item: Option<types::CodeableReference>,

    /// Which investigations support finding
    pub basis: Option<types::String>,
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,
}

/// The `ClinicalAssessment.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalAssessmentEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ClinicalAssessment;

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
