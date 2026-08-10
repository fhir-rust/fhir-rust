//! DetectedIssue
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
//!
//! Version: 5.0.0
//!
//! DetectedIssue Resource: Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, gaps in care, etc.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient.
///
/// A DetectedIssue records a problem discovered during clinical decision
/// support or review, such as a drug-drug interaction, duplicate therapy,
/// ineffective treatment frequency, or a procedure-condition conflict. It
/// captures the severity, the resources implicated in the issue, supporting
/// evidence, and any mitigation steps taken to address it. In FHIR R5 it is
/// commonly produced by decision-support systems and reviewed by clinicians.
/// A single detected issue may reference one or more implicated resources
/// (such as a `MedicationRequest` or `Procedure`), and downstream workflows
/// can track whether the issue is still open or has been mitigated by
/// recording one or more `DetectedIssueMitigation` entries.
///
/// # Related resources
///
/// - The `subject` of the detected issue is typically a
///   [`Patient`](crate::r5::resources::patient::Patient).
/// - `category`, `code`, and `severity` use
///   [`CodeableConcept`](crate::r5::types::CodeableConcept) to classify the
///   nature and seriousness of the issue.
/// - The `implicated` field references the clinical resources (such as
///   `MedicationRequest`, `Procedure`, or `Condition`) that are involved in
///   the issue.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::detected_issue::DetectedIssue;
/// use fhir::r5::types;
///
/// let value = DetectedIssue {
///     detail: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `detail` is the name this serializes to on the wire.
/// assert_eq!(json["detail"], ::serde_json::json!("# Heading"));
///
/// let back: DetectedIssue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DetectedIssueDe")]
pub struct DetectedIssue {
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

    /// Unique id for the detected issue
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The workflow status of this issue: preliminary | final | entered-in-error | mitigated
    pub status: crate::r5::coded::Coded<crate::r5::codes::DetectedissueStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Broad category of the detected issue, e.g. drug-drug interaction, duplicate therapy, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Specific type of detected issue, e.g. drug-drug, duplicate therapy, etc
    pub code: Option<types::CodeableConcept>,

    /// Indicates the potential clinical seriousness of the issue: high | moderate | low
    pub severity: Option<crate::r5::coded::Coded<crate::r5::codes::DetectedissueSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`).
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// The patient, or other subject, for whom the issue was detected
    pub subject: Option<types::Reference>,

    /// Encounter detected issue is part of
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `DetectedIssue.identified[x]` choice element (0..1); see [`DetectedIssueIdentified`].
    #[serde(flatten)]
    pub identified: Option<DetectedIssueIdentified>,

    /// The provider or device that identified the issue
    pub author: Option<types::Reference>,

    /// The clinical resource(s), such as a medication order or procedure, that are implicated in the issue
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implicated: Vec<types::Reference>,

    /// Supporting evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<DetectedIssueEvidence>,

    /// Description and context
    pub detail: Option<types::Markdown>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`).
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,

    /// Authority for issue
    pub reference: Option<types::Uri>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`).
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Step taken to address
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigation: Vec<DetectedIssueMitigation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetectedIssueDe {
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
    status: crate::r5::coded::Coded<crate::r5::codes::DetectedissueStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    code: Option<types::CodeableConcept>,
    severity: Option<crate::r5::coded::Coded<crate::r5::codes::DetectedissueSeverity>>,
    #[serde(rename = "_severity")]
    severity_ext: Option<types::Element>,
    subject: Option<types::Reference>,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(flatten)]
    identified: crate::r5::choice::Slot<DetectedIssueIdentified>,
    author: Option<types::Reference>,
    #[serde(default)]
    implicated: Vec<types::Reference>,
    #[serde(default)]
    evidence: Vec<DetectedIssueEvidence>,
    detail: Option<types::Markdown>,
    #[serde(rename = "_detail")]
    detail_ext: Option<types::Element>,
    reference: Option<types::Uri>,
    #[serde(rename = "_reference")]
    reference_ext: Option<types::Element>,
    #[serde(default)]
    mitigation: Vec<DetectedIssueMitigation>,
}

impl ::core::convert::From<DetectedIssueDe> for DetectedIssue {
    fn from(v: DetectedIssueDe) -> Self {
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
            severity: v.severity,
            severity_ext: v.severity_ext,
            subject: v.subject,
            encounter: v.encounter,
            identified: v.identified.0,
            author: v.author,
            implicated: v.implicated,
            evidence: v.evidence,
            detail: v.detail,
            detail_ext: v.detail_ext,
            reference: v.reference,
            reference_ext: v.reference_ext,
            mitigation: v.mitigation,
        }
    }
}

/// Supporting evidence.
///
/// Supporting evidence or manifestations that provide the basis for identifying
/// the detected issue.
/// # Examples
///
/// ```
/// use fhir::r5::resources::detected_issue::DetectedIssueEvidence;
/// use fhir::r5::types;
///
/// let value = DetectedIssueEvidence {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DetectedIssueEvidence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssueEvidence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Manifestation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<types::Reference>,
}

/// Step taken to address.
///
/// Indicates an action that has been taken or is committed to reduce or
/// eliminate the likelihood or severity of the identified issue.
/// # Examples
///
/// ```
/// use fhir::r5::resources::detected_issue::DetectedIssueMitigation;
/// use fhir::r5::types;
///
/// let value = DetectedIssueMitigation {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DetectedIssueMitigation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssueMitigation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What mitigation?
    pub action: types::CodeableConcept,

    /// Date committed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is committing?
    pub author: Option<types::Reference>,

    /// Additional notes about the mitigation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DetectedIssue;

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
/// The `DetectedIssue.identified[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DetectedIssueIdentified {
    /// `identifiedDateTime` variant.
    #[fhir("identifiedDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `identifiedPeriod` variant.
    #[fhir("identifiedPeriod")]
    Period(Box<types::Period>),
}
