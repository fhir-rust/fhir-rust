//! DiagnosticReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
//!
//!
//!
//! A Diagnostic report - a combination of request information, atomic results,
//! images, interpretation, as well as formatted reports
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DiagnosticReport Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::diagnostic_report::DiagnosticReport;
/// use fhir::r3::types;
///
/// let value = DiagnosticReport {
///     issued: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `issued` is the name this serializes to on the wire.
/// assert_eq!(json["issued"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DiagnosticReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DiagnosticReport {
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

    /// Business identifier for report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// What was requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// registered | partial | preliminary | final +
    pub status: crate::coded::Coded<crate::r3::codes::DiagnosticReportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Service category
    pub category: Option<types::CodeableConcept>,

    /// Name/Code for this diagnostic report
    pub code: types::CodeableConcept,

    /// The subject of the report - usually, but not always, the patient
    pub subject: Option<types::Reference>,

    /// Health care event when test ordered
    pub context: Option<types::Reference>,

    /// Clinically relevant time/time-period for report
    /// The `DiagnosticReport.effective[x]` choice element (0..1); see [`DiagnosticReportEffective`].
    #[serde(flatten)]
    pub effective: Option<DiagnosticReportEffective>,

    /// DateTime this version was released
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Participants in producing the report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<DiagnosticReportPerformer>,

    /// Specimens this report is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference>,

    /// Observations - simple, or complex nested groups
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference>,

    /// Reference to full details of imaging associated with the diagnostic
    /// report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imaging_study: Vec<types::Reference>,

    /// Key images associated with this report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<DiagnosticReportImage>,

    /// Clinical Interpretation of test results
    pub conclusion: Option<types::String>,
    /// Primitive extension sibling for [`conclusion`](Self::conclusion) (FHIR `_conclusion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conclusion")]
    pub conclusion_ext: Option<types::Element>,

    /// Codes for the conclusion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coded_diagnosis: Vec<types::CodeableConcept>,

    /// Entire report as issued
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presented_form: Vec<types::Attachment>,
}

/// A list of key images associated with this report. The images are generally
/// created during the diagnostic process, and may be directly of the patient,
/// or of treated specimens (i.e. slides of interest).
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::diagnostic_report::DiagnosticReportImage;
/// use fhir::r3::types;
///
/// let value = DiagnosticReportImage {
///     comment: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comment` is the name this serializes to on the wire.
/// assert_eq!(json["comment"], ::serde_json::json!("abc"));
///
/// let back: DiagnosticReportImage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DiagnosticReportImage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Comment about the image (e.g. explanation)
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Reference to the image source
    pub link: types::Reference,
}

/// Indicates who or what participated in producing the report.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::diagnostic_report::DiagnosticReportPerformer;
/// use fhir::r3::types;
///
/// let value = DiagnosticReportPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DiagnosticReportPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DiagnosticReportPerformer {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performer
    pub role: Option<types::CodeableConcept>,

    /// Practitioner or Organization participant
    pub actor: types::Reference,
}

/// The `DiagnosticReport.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DiagnosticReportEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DiagnosticReport;

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
