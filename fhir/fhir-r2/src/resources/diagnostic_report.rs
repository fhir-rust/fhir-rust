//! DiagnosticReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
//!
//!
//!
//! A Diagnostic report - a combination of request information, atomic results,
//! images, interpretation, as well as formatted reports
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DiagnosticReport Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::diagnostic_report::DiagnosticReport;
/// use fhir::r2::types;
///
/// let value = DiagnosticReport {
///     conclusion: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `conclusion` is the name this serializes to on the wire.
/// assert_eq!(json["conclusion"], ::serde_json::json!("abc"));
///
/// let back: DiagnosticReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DiagnosticReportDe")]
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id for external references to this report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// registered | partial | final | corrected | appended | cancelled |
    /// entered-in-error
    pub status: crate::coded::Coded<crate::r2::codes::DiagnosticReportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Service category
    pub category: Option<types::CodeableConcept>,

    /// Name/Code for this diagnostic report
    pub code: types::CodeableConcept,

    /// The subject of the report, usually, but not always, the patient
    pub subject: types::Reference,

    /// Health care event when test ordered
    pub encounter: Option<types::Reference<crate::r2::resources::Encounter>>,

    /// Clinically Relevant time/time-period for report
    /// The `DiagnosticReport.effective[x]` choice element (1..1); see [`DiagnosticReportEffective`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub effective: Option<DiagnosticReportEffective>,

    /// DateTime this version was released
    pub issued: types::Instant,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Responsible Diagnostic Service
    pub performer: types::Reference,

    /// What was requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<types::Reference>,

    /// Specimens this report is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference<crate::r2::resources::Specimen>>,

    /// Observations - simple, or complex nested groups
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference<crate::r2::resources::Observation>>,

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticReportDe {
    id: Option<types::Id>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r2::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: crate::coded::Coded<crate::r2::codes::DiagnosticReportStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    category: Option<types::CodeableConcept>,
    code: types::CodeableConcept,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r2::resources::Encounter>>,
    #[serde(flatten)]
    effective: crate::r2::choice::Slot<DiagnosticReportEffective>,
    issued: types::Instant,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    performer: types::Reference,
    #[serde(default)]
    request: Vec<types::Reference>,
    #[serde(default)]
    specimen: Vec<types::Reference<crate::r2::resources::Specimen>>,
    #[serde(default)]
    result: Vec<types::Reference<crate::r2::resources::Observation>>,
    #[serde(default)]
    imaging_study: Vec<types::Reference>,
    #[serde(default)]
    image: Vec<DiagnosticReportImage>,
    conclusion: Option<types::String>,
    #[serde(rename = "_conclusion")]
    conclusion_ext: Option<types::Element>,
    #[serde(default)]
    coded_diagnosis: Vec<types::CodeableConcept>,
    #[serde(default)]
    presented_form: Vec<types::Attachment>,
}

impl ::core::convert::From<DiagnosticReportDe> for DiagnosticReport {
    fn from(v: DiagnosticReportDe) -> Self {
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
            category: v.category,
            code: v.code,
            subject: v.subject,
            encounter: v.encounter,
            effective: v.effective.0,
            issued: v.issued,
            issued_ext: v.issued_ext,
            performer: v.performer,
            request: v.request,
            specimen: v.specimen,
            result: v.result,
            imaging_study: v.imaging_study,
            image: v.image,
            conclusion: v.conclusion,
            conclusion_ext: v.conclusion_ext,
            coded_diagnosis: v.coded_diagnosis,
            presented_form: v.presented_form,
        }
    }
}

/// A list of key images associated with this report. The images are generally
/// created during the diagnostic process, and may be directly of the patient,
/// or of treated specimens (i.e. slides of interest).
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::diagnostic_report::DiagnosticReportImage;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct DiagnosticReportImage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub link: types::Reference<crate::r2::resources::Media>,
}

/// The `DiagnosticReport.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum DiagnosticReportEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
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
