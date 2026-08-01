//! ClinicalImpression
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
//!
//! Version: 5.0.0
//!
//! ClinicalImpression Resource: A record of a clinical assessment performed to determine what problem(s) may affect the patient before planning treatments or management strategies.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management
/// strategies that are best to manage a patient's condition. Assessments are
/// often 1:1 with a clinical consultation / encounter, but this varies greatly
/// depending on the clinical workflow. This resource is called
/// "ClinicalImpression" rather than "ClinicalAssessment" to avoid confusion.
///
/// A `ClinicalImpression` captures the narrative and structured reasoning a
/// clinician goes through while evaluating a patient: the problems or
/// conditions considered, the evidence and findings that support or rule out
/// those problems, the prognosis, and any change in the patient's condition
/// relative to a previous assessment. It is a point-in-time snapshot of
/// clinical thinking, typically produced during or shortly after an encounter,
/// and is often used to justify subsequent orders, referrals, or care plans.
///
/// # Related resources
///
/// A `ClinicalImpression` is usually linked to the [`Patient`](crate::r5::resources::patient::Patient)
/// or group being assessed via `subject`, and may reference the `Encounter`
/// during which it was formed, the performing practitioner via
/// `performer`, and a prior `ClinicalImpression` via `previous`. Findings and
/// status information are commonly expressed using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), while supporting
/// evidence may reference `Observation`, `Condition`, or other diagnostic
/// resources through `supporting_info`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::clinical_impression::ClinicalImpression;
/// use fhir::r5::types;
///
/// let value = ClinicalImpression {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpression = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpression {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The workflow state of this assessment: preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Coded or textual reason explaining why the assessment currently has this status
    pub status_reason: Option<types::CodeableConcept>,

    /// Why/how the assessment was performed
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The [`Patient`](crate::r5::resources::patient::Patient) or group whose condition is being assessed
    pub subject: types::Reference,

    /// The Encounter during which this ClinicalImpression was created
    pub encounter: Option<types::Reference>,

    /// The `ClinicalImpression.effective[x]` choice element (0..1); see [`ClinicalImpressionEffective`].
    #[serde(flatten)]
    pub effective: Option<ClinicalImpressionEffective>,

    /// When the assessment was documented
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The clinician performing the assessment
    pub performer: Option<types::Reference>,

    /// Reference to last assessment
    pub previous: Option<types::Reference>,

    /// Relevant impressions of patient state
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problem: Vec<types::Reference>,

    /// Change in the status/pattern of a subject's condition since previously
    /// assessed, such as worsening, improving, or no change
    pub change_pattern: Option<types::CodeableConcept>,

    /// Clinical Protocol followed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<types::Uri>,
    /// Primitive extension sibling for [`protocol`](Self::protocol) (FHIR `_protocol`).
    #[serde(rename = "_protocol")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_ext: Vec<Option<types::Element>>,

    /// Summary of the assessment
    pub summary: Option<types::String>,
    /// Primitive extension sibling for [`summary`](Self::summary) (FHIR `_summary`).
    #[serde(rename = "_summary")]
    pub summary_ext: Option<types::Element>,

    /// Possible or likely findings and diagnoses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finding: Vec<ClinicalImpressionFinding>,

    /// Estimate of likely outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prognosis_codeable_concept: Vec<types::CodeableConcept>,

    /// RiskAssessment expressing likely outcome
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prognosis_reference: Vec<types::Reference>,

    /// Information supporting the clinical impression
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Comments made about the ClinicalImpression
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Possible or likely findings and diagnoses.
/// # Examples
///
/// ```
/// use fhir::r5::resources::clinical_impression::ClinicalImpressionFinding;
/// use fhir::r5::types;
///
/// let value = ClinicalImpressionFinding {
///     basis: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `basis` is the name this serializes to on the wire.
/// assert_eq!(json["basis"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpressionFinding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpressionFinding {
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
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`).
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ClinicalImpression;

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
/// The `ClinicalImpression.effective[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalImpressionEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}
