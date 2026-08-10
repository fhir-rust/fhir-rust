//! DiagnosticReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DiagnosticReport
//!
//! Version: 5.0.0
//!
//! DiagnosticReport Resource: The findings and interpretation of diagnostic tests performed on patients, groups, products, substances, devices, locations, and/or specimens derived from these.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// DiagnosticReport
///
/// The findings and interpretation of diagnostic tests performed on patients,
/// groups of patients, products, substances, devices, and locations, and/or
/// specimens derived from these. The report includes clinical context such as
/// requesting provider information, and some mix of atomic results, images,
/// textual and coded interpretations, and formatted representations. It is
/// used to convey lab, imaging, pathology, and other diagnostic outcomes.
///
/// A DiagnosticReport is typically generated once testing on a request is
/// complete, and it acts as the aggregation point that ties together the
/// individual `Observation` results, any supporting images or documents, and
/// a narrative or coded conclusion into a single, clinically reviewable
/// report. Reports move through a lifecycle expressed by the `status` field
/// (for example `registered`, `preliminary`, `final`, and `amended`), so
/// consumers can track whether a report is still in progress or has been
/// finalized. Reports are commonly produced by laboratory, imaging,
/// pathology, cardiology, and other diagnostic services, and they are
/// referenced by ordering workflows and clinical summaries.
///
/// See also: [`Patient`](crate::r5::resources::patient::Patient) or `Group`
/// as the typical `subject` of a report, `Observation` for individual
/// result entries referenced via `result`, `ServiceRequest` for the order
/// referenced via `based_on`, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) for the coded
/// `code`, `category`, and `conclusion_code` values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::diagnostic_report::DiagnosticReport;
/// use fhir::r5::types;
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
pub struct DiagnosticReport {
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
    pub contained: Vec<crate::r5::resources::Resource>,

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

    /// registered | partial | preliminary | modified | final | amended | corrected | appended | cancelled | entered-in-error | unknown; tracks the report's lifecycle status
    pub status: crate::r5::coded::Coded<crate::r5::codes::DiagnosticReportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Service category, such as the performing department (e.g. laboratory, radiology, cardiology)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Name/Code for this diagnostic report, identifying the type of report or panel being reported
    pub code: types::CodeableConcept,

    /// The subject of the report - usually, but not always, the patient; may also reference a Group, Device, Location, or other subject
    pub subject: Option<types::Reference>,

    /// Health care event when test ordered
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `DiagnosticReport.effective[x]` choice element (0..1); see [`DiagnosticReportEffective`].
    #[serde(flatten)]
    pub effective: Option<DiagnosticReportEffective>,

    /// DateTime this version was made
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Responsible Diagnostic Service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// Primary result interpreter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results_interpreter: Vec<types::Reference>,

    /// Specimens this report is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference<crate::r5::resources::Specimen>>,

    /// Observations that make up the individual results contributing to this report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::Reference<crate::r5::resources::Observation>>,

    /// Comments about the diagnostic report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Reference to full details of an analysis associated with the diagnostic report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study: Vec<types::Reference>,

    /// Additional information supporting the diagnostic report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<DiagnosticReportSupportingInfo>,

    /// Key images or data associated with this report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<DiagnosticReportMedia>,

    /// Reference to a Composition resource for the DiagnosticReport structure
    pub composition: Option<types::Reference<crate::r5::resources::Composition>>,

    /// Clinical conclusion (interpretation) of test results, the narrative summary a clinician relies on for decision making
    pub conclusion: Option<types::Markdown>,
    /// Primitive extension sibling for [`conclusion`](Self::conclusion) (FHIR `_conclusion`).
    #[serde(rename = "_conclusion")]
    pub conclusion_ext: Option<types::Element>,

    /// Codes for the clinical conclusion of test results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conclusion_code: Vec<types::CodeableConcept>,

    /// Entire report as issued
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presented_form: Vec<types::Attachment>,
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
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    status: crate::r5::coded::Coded<crate::r5::codes::DiagnosticReportStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    code: types::CodeableConcept,
    subject: Option<types::Reference>,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(flatten)]
    effective: crate::r5::choice::Slot<DiagnosticReportEffective>,
    issued: Option<types::Instant>,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    #[serde(default)]
    performer: Vec<types::Reference>,
    #[serde(default)]
    results_interpreter: Vec<types::Reference>,
    #[serde(default)]
    specimen: Vec<types::Reference<crate::r5::resources::Specimen>>,
    #[serde(default)]
    result: Vec<types::Reference<crate::r5::resources::Observation>>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    study: Vec<types::Reference>,
    #[serde(default)]
    supporting_info: Vec<DiagnosticReportSupportingInfo>,
    #[serde(default)]
    media: Vec<DiagnosticReportMedia>,
    composition: Option<types::Reference<crate::r5::resources::Composition>>,
    conclusion: Option<types::Markdown>,
    #[serde(rename = "_conclusion")]
    conclusion_ext: Option<types::Element>,
    #[serde(default)]
    conclusion_code: Vec<types::CodeableConcept>,
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
            based_on: v.based_on,
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
            presented_form: v.presented_form,
        }
    }
}

/// DiagnosticReportSupportingInfo
///
/// Additional information supporting the diagnostic report, referencing other
/// resources that provide context for the results.
/// # Examples
///
/// ```
/// use fhir::r5::resources::diagnostic_report::DiagnosticReportSupportingInfo;
/// use fhir::r5::types;
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

/// DiagnosticReportMedia
///
/// Key images or data associated with this report, such as images of a
/// microslide or a formatted representation of the source data.
/// # Examples
///
/// ```
/// use fhir::r5::resources::diagnostic_report::DiagnosticReportMedia;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Reference to the image or data source
    pub link: types::Reference<crate::r5::resources::DocumentReference>,
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
/// The `DiagnosticReport.effective[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DiagnosticReportEffective {
    /// `effectiveDateTime` variant.
    #[fhir("effectiveDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
}
