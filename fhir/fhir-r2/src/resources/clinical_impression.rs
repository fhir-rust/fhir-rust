//! ClinicalImpression
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClinicalImpression
//!
//!
//!
//! A clinical assessment performed when planning treatments and management
//! strategies for a patient
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ClinicalImpression Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::clinical_impression::ClinicalImpression;
/// use fhir::r2::types;
///
/// let value = ClinicalImpression {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ClinicalImpression = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClinicalImpression {
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

    /// The patient being assessed
    pub patient: types::Reference<crate::r2::resources::Patient>,

    /// The clinician performing the assessment
    pub assessor: Option<types::Reference<crate::r2::resources::Practitioner>>,

    /// in-progress | completed | entered-in-error
    pub status: crate::coded::Coded<crate::r2::codes::ClinicalImpressionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// When the assessment occurred
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Why/how the assessment was performed
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Reference to last assessment
    pub previous: Option<types::Reference<crate::r2::resources::ClinicalImpression>>,

    /// General assessment of patient state
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problem: Vec<types::Reference>,

    /// Request or event that necessitated this assessment
    /// The `ClinicalImpression.trigger[x]` choice element (0..1); see [`ClinicalImpressionTrigger`].
    #[serde(flatten)]
    pub trigger: Option<ClinicalImpressionTrigger>,

    /// One or more sets of investigations (signs, symptions, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub investigations: Vec<ClinicalImpressionInvestigations>,

    /// Clinical Protocol followed
    pub protocol: Option<types::Uri>,
    /// Primitive extension sibling for [`protocol`](Self::protocol) (FHIR `_protocol`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_protocol")]
    pub protocol_ext: Option<types::Element>,

    /// Summary of the assessment
    pub summary: Option<types::String>,
    /// Primitive extension sibling for [`summary`](Self::summary) (FHIR `_summary`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_summary")]
    pub summary_ext: Option<types::Element>,

    /// Possible or likely findings and diagnoses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub finding: Vec<ClinicalImpressionFinding>,

    /// Diagnoses/conditions resolved since previous assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resolved: Vec<types::CodeableConcept>,

    /// Diagnosis considered not possible
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ruled_out: Vec<ClinicalImpressionRuledOut>,

    /// Estimate of likely outcome
    pub prognosis: Option<types::String>,
    /// Primitive extension sibling for [`prognosis`](Self::prognosis) (FHIR `_prognosis`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prognosis")]
    pub prognosis_ext: Option<types::Element>,

    /// Plan of action after assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plan: Vec<types::Reference>,

    /// Actions taken during assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::Reference>,
}

/// Specific findings or diagnoses that was considered likely or relevant to
/// ongoing treatment.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::clinical_impression::ClinicalImpressionFinding;
/// use fhir::r2::types;
///
/// let value = ClinicalImpressionFinding {
///     cause: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `cause` is the name this serializes to on the wire.
/// assert_eq!(json["cause"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpressionFinding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClinicalImpressionFinding {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific text or code for finding
    pub item: types::CodeableConcept,

    /// Which investigations support finding
    pub cause: Option<types::String>,
    /// Primitive extension sibling for [`cause`](Self::cause) (FHIR `_cause`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cause")]
    pub cause_ext: Option<types::Element>,
}

/// One or more sets of investigations (signs, symptions, etc.). The actual
/// grouping of investigations vary greatly depending on the type and context
/// of the assessment. These investigations may include data generated during
/// the assessment process, or data previously generated and recorded that is
/// pertinent to the outcomes.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::clinical_impression::ClinicalImpressionInvestigations;
/// use fhir::r2::types;
///
/// let value = ClinicalImpressionInvestigations {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClinicalImpressionInvestigations = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClinicalImpressionInvestigations {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A name/code for the set
    pub code: types::CodeableConcept,

    /// Record of a specific investigation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<types::Reference>,
}

/// Diagnosis considered not possible.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::clinical_impression::ClinicalImpressionRuledOut;
/// use fhir::r2::types;
///
/// let value = ClinicalImpressionRuledOut {
///     reason: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reason` is the name this serializes to on the wire.
/// assert_eq!(json["reason"], ::serde_json::json!("abc"));
///
/// let back: ClinicalImpressionRuledOut = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClinicalImpressionRuledOut {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific text of code for diagnosis
    pub item: types::CodeableConcept,

    /// Grounds for elimination
    pub reason: Option<types::String>,
    /// Primitive extension sibling for [`reason`](Self::reason) (FHIR `_reason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reason")]
    pub reason_ext: Option<types::Element>,
}

/// The `ClinicalImpression.trigger[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ClinicalImpressionTrigger {
    /// `triggerCodeableConcept` variant.
    #[fhir("triggerCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `triggerReference` variant.
    #[fhir("triggerReference")]
    Reference(Box<types::Reference>),
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
