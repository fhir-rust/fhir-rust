//! DiagnosticReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
//!
//! Version: 6.0.0-ballot3
//!
//! A Diagnostic report - a combination of request information, atomic results,
//! images, interpretation, as well as formatted reports
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The findings and interpretation of diagnostic tests performed on patients,
/// groups of patients, products, substances, devices, and locations, and/or
/// specimens derived from these. The report includes clinical context such as
/// requesting provider information, and some mix of atomic results, images,
/// textual and coded interpretations, and formatted representation of
/// diagnostic reports. The report also includes non-clinical context such as
/// batch analysis and stability reporting of products and substances.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::diagnostic_report::DiagnosticReport;
/// use fhir::r6::types;
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
#[serde(from = "DiagnosticReportDe")]
#[fhir_version("r6")]
pub struct DiagnosticReport {
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

    /// Business identifier for report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// What was requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// registered | partial | preliminary | modified | final | amended |
    /// corrected | appended | cancelled | entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::DiagnosticReportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Service category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Name/Code for this diagnostic report
    pub code: types::CodeableConcept,

    /// The subject of the report - usually, but not always, the patient
    pub subject: Option<types::Reference>,

    /// Related DiagnosticReports
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<types::RelatedArtifact>,

    /// Encounter associated with the DiagnosticReport
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Clinically relevant time/time-period for the results that are included
    /// in the report
    /// The `DiagnosticReport.effective[x]` choice element (0..1); see [`DiagnosticReportEffective`].
    #[serde(flatten)]
    pub effective: Option<DiagnosticReportEffective>,

    /// DateTime this version was made
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// The procedure(s) from which the report was produced
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<types::Reference<crate::r6::resources::Procedure>>,

    /// Responsible Diagnostic Service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// Primary result interpreter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results_interpreter: Vec<types::Reference>,

    /// Specimens this report is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference<crate::r6::resources::Specimen>>,

    /// Observations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference<crate::r6::resources::Observation>>,

    /// Comments about the diagnostic report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Reference to full details of an analysis associated with the diagnostic
    /// report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study: Vec<types::Reference>,

    /// Additional information supporting the diagnostic report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<DiagnosticReportSupportingInfo>,

    /// Key images or data associated with this report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<DiagnosticReportMedia>,

    /// Reference to a Composition resource for the DiagnosticReport structure
    pub composition: Option<types::Reference<crate::r6::resources::Composition>>,

    /// Clinical conclusion (interpretation) of test results
    pub conclusion: Option<types::Markdown>,
    /// Primitive extension sibling for [`conclusion`](Self::conclusion) (FHIR `_conclusion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conclusion")]
    pub conclusion_ext: Option<types::Element>,

    /// Codes and/or references for the clinical conclusion of test results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conclusion_code: Vec<types::CodeableReference>,

    /// Recommendations based on findings and interpretations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recomendation: Vec<types::CodeableReference>,

    /// Entire report as issued
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presented_form: Vec<types::Attachment>,

    /// Communication initiated during the reporting process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<types::Reference<crate::r6::resources::Communication>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticReportDe {
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
    #[serde(default)]
    based_on: Vec<types::Reference>,
    status: crate::coded::Coded<crate::r6::codes::DiagnosticReportStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    code: types::CodeableConcept,
    subject: Option<types::Reference>,
    #[serde(default)]
    relates_to: Vec<types::RelatedArtifact>,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(flatten)]
    effective: crate::r6::choice::Slot<DiagnosticReportEffective>,
    issued: Option<types::Instant>,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    #[serde(default)]
    procedure: Vec<types::Reference<crate::r6::resources::Procedure>>,
    #[serde(default)]
    performer: Vec<types::Reference>,
    #[serde(default)]
    results_interpreter: Vec<types::Reference>,
    #[serde(default)]
    specimen: Vec<types::Reference<crate::r6::resources::Specimen>>,
    #[serde(default)]
    result: Vec<types::Reference<crate::r6::resources::Observation>>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    study: Vec<types::Reference>,
    #[serde(default)]
    supporting_info: Vec<DiagnosticReportSupportingInfo>,
    #[serde(default)]
    media: Vec<DiagnosticReportMedia>,
    composition: Option<types::Reference<crate::r6::resources::Composition>>,
    conclusion: Option<types::Markdown>,
    #[serde(rename = "_conclusion")]
    conclusion_ext: Option<types::Element>,
    #[serde(default)]
    conclusion_code: Vec<types::CodeableReference>,
    #[serde(default)]
    recomendation: Vec<types::CodeableReference>,
    #[serde(default)]
    presented_form: Vec<types::Attachment>,
    #[serde(default)]
    communication: Vec<types::Reference<crate::r6::resources::Communication>>,
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
            based_on: v.based_on,
            status: v.status,
            status_ext: v.status_ext,
            category: v.category,
            code: v.code,
            subject: v.subject,
            relates_to: v.relates_to,
            encounter: v.encounter,
            effective: v.effective.0,
            issued: v.issued,
            issued_ext: v.issued_ext,
            procedure: v.procedure,
            performer: v.performer,
            results_interpreter: v.results_interpreter,
            specimen: v.specimen,
            result: v.result,
            note: v.note,
            study: v.study,
            supporting_info: v.supporting_info,
            media: v.media,
            composition: v.composition,
            conclusion: v.conclusion,
            conclusion_ext: v.conclusion_ext,
            conclusion_code: v.conclusion_code,
            recomendation: v.recomendation,
            presented_form: v.presented_form,
            communication: v.communication,
        }
    }
}

/// A list of key images or data associated with this report. The images or
/// data are generally created during the diagnostic process, and may be
/// directly of the patient, or of treated specimens (i.e. slides of interest).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::diagnostic_report::DiagnosticReportMedia;
/// use fhir::r6::types;
///
/// let value = DiagnosticReportMedia {
///     comment: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comment` is the name this serializes to on the wire.
/// assert_eq!(json["comment"], ::serde_json::json!("abc"));
///
/// let back: DiagnosticReportMedia = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DiagnosticReportMedia {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Comment about the image or data (e.g. explanation)
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Reference to the image or data source
    pub link: types::Reference<crate::r6::resources::DocumentReference>,
}

/// This backbone element contains supporting information that was used in the
/// creation of the report not included in the results already included in the
/// report.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::diagnostic_report::DiagnosticReportSupportingInfo;
/// use fhir::r6::types;
///
/// let value = DiagnosticReportSupportingInfo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DiagnosticReportSupportingInfo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DiagnosticReportSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Supporting information role code
    pub r#type: types::CodeableConcept,

    /// Supporting information reference
    pub reference: types::Reference,
}

/// The `DiagnosticReport.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DiagnosticReportEffective {
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
