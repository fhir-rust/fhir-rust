//! ClinicalImpression
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
//!
//! Version: 4.3.0
//!
//! A clinical assessment performed when planning treatments and management
//! strategies for a patient
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a clinical assessment performed to determine what problem(s)
/// may affect the patient and before planning the treatments or management
/// strategies that are best to manage a patient's condition. Assessments are
/// often 1:1 with a clinical consultation / encounter, but this varies greatly
/// depending on the clinical workflow. This resource is called
/// "ClinicalImpression" rather than "ClinicalAssessment" to avoid confusion
/// with the recording of assessment tools such as Apgar score.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::clinical_impression::ClinicalImpression;
/// use fhir::r4b::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ClinicalImpressionDe")]
#[fhir_version("r4b")]
pub struct ClinicalImpression {
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// in-progress | completed | entered-in-error
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Kind of assessment performed
    pub code: Option<types::CodeableConcept>,

    /// Why/how the assessment was performed
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Patient or group assessed
    pub subject: types::Reference,

    /// Encounter created as part of
    pub encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,

    /// Time of assessment
    /// The `ClinicalImpression.effective[x]` choice element (0..1); see [`ClinicalImpressionEffective`].
    #[serde(flatten)]
    pub effective: Option<ClinicalImpressionEffective>,

    /// When the assessment was documented
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The clinician performing the assessment
    pub assessor: Option<types::Reference>,

    /// Reference to last assessment
    pub previous: Option<types::Reference<crate::r4b::resources::ClinicalImpression>>,

    /// Relevant impressions of patient state
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problem: Vec<types::Reference>,

    /// One or more sets of investigations (signs, symptoms, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub investigation: Vec<ClinicalImpressionInvestigation>,

    /// Clinical Protocol followed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol: Vec<types::Uri>,
    /// Primitive extension sibling for [`protocol`](Self::protocol) (FHIR `_protocol`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_protocol")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_ext: Vec<Option<types::Element>>,

    /// Summary of the assessment
    pub summary: Option<types::String>,
    /// Primitive extension sibling for [`summary`](Self::summary) (FHIR `_summary`):
    /// carries `id` and/or `extension` for the primitive value.
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
    pub prognosis_reference: Vec<types::Reference<crate::r4b::resources::RiskAssessment>>,

    /// Information supporting the clinical impression
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Comments made about the ClinicalImpression
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClinicalImpressionDe {
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
    contained: Vec<crate::r4b::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: types::Code,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    status_reason: Option<types::CodeableConcept>,
    code: Option<types::CodeableConcept>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,
    #[serde(flatten)]
    effective: crate::r4b::choice::Slot<ClinicalImpressionEffective>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    assessor: Option<types::Reference>,
    previous: Option<types::Reference<crate::r4b::resources::ClinicalImpression>>,
    #[serde(default)]
    problem: Vec<types::Reference>,
    #[serde(default)]
    investigation: Vec<ClinicalImpressionInvestigation>,
    #[serde(default)]
    protocol: Vec<types::Uri>,
    #[serde(rename = "_protocol")]
    #[serde(default)]
    protocol_ext: Vec<Option<types::Element>>,
    summary: Option<types::String>,
    #[serde(rename = "_summary")]
    summary_ext: Option<types::Element>,
    #[serde(default)]
    finding: Vec<ClinicalImpressionFinding>,
    #[serde(default)]
    prognosis_codeable_concept: Vec<types::CodeableConcept>,
    #[serde(default)]
    prognosis_reference: Vec<types::Reference<crate::r4b::resources::RiskAssessment>>,
    #[serde(default)]
    supporting_info: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<ClinicalImpressionDe> for ClinicalImpression {
    fn from(v: ClinicalImpressionDe) -> Self {
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
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            code: v.code,
            description: v.description,
            description_ext: v.description_ext,
            subject: v.subject,
            encounter: v.encounter,
            effective: v.effective.0,
            date: v.date,
            date_ext: v.date_ext,
            assessor: v.assessor,
            previous: v.previous,
            problem: v.problem,
            investigation: v.investigation,
            protocol: v.protocol,
            protocol_ext: v.protocol_ext,
            summary: v.summary,
            summary_ext: v.summary_ext,
            finding: v.finding,
            prognosis_codeable_concept: v.prognosis_codeable_concept,
            prognosis_reference: v.prognosis_reference,
            supporting_info: v.supporting_info,
            note: v.note,
        }
    }
}

/// Specific findings or diagnoses that were considered likely or relevant to
/// ongoing treatment.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::clinical_impression::ClinicalImpressionFinding;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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
    pub item_codeable_concept: Option<types::CodeableConcept>,

    /// What was found
    pub item_reference: Option<types::Reference>,

    /// Which investigations support finding
    pub basis: Option<types::String>,
    /// Primitive extension sibling for [`basis`](Self::basis) (FHIR `_basis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_basis")]
    pub basis_ext: Option<types::Element>,
}

/// One or more sets of investigations (signs, symptoms, etc.). The actual
/// grouping of investigations varies greatly depending on the type and context
/// of the assessment. These investigations may include data generated during
/// the assessment process, or data previously generated and recorded that is
/// pertinent to the outcomes.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::clinical_impression::ClinicalImpressionInvestigation;
/// use fhir::r4b::types;
///
/// let value = ClinicalImpressionInvestigation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpressionInvestigation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ClinicalImpressionInvestigation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A name/code for the set
    pub code: types::CodeableConcept,

    /// Record of a specific investigation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<types::Reference>,
}

/// The `ClinicalImpression.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalImpressionEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
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
