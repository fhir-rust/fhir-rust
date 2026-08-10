//! DetectedIssue
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
//!
//! Version: 6.0.0-ballot3
//!
//! Clinical issue with action
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient; e.g. Drug-drug
/// interaction, Ineffective treatment frequency, Procedure-condition conflict,
/// gaps in care, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::detected_issue::DetectedIssue;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DetectedIssueDe")]
#[fhir_version("r6")]
pub struct DetectedIssue {
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

    /// Business identifier for detected issue
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// preliminary | final | entered-in-error | unknown | mitigated
    pub status: crate::coded::Coded<crate::r6::codes::DetectedissueStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// High level categorization of detected issue
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Specific type of detected issue, e.g. drug-drug, duplicate therapy, etc
    pub code: Option<types::CodeableConcept>,

    /// high | moderate | low
    pub severity: Option<types::CodeableConcept>,

    /// Associated subject
    pub subject: Option<types::Reference>,

    /// Encounter the detected issue is part of
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// When detected issue occurred/is occurring
    /// The `DetectedIssue.identified[x]` choice element (0..1); see [`DetectedIssueIdentified`].
    #[serde(flatten)]
    pub identified: Option<DetectedIssueIdentified>,

    /// The provider or device that identified the issue
    pub author: Option<types::Reference>,

    /// Problem resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implicated: Vec<types::Reference>,

    /// Supporting evidence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<DetectedIssueEvidence>,

    /// Description and context
    pub detail: Option<types::Markdown>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,

    /// Authority for issue
    pub reference: Option<types::Uri>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// The quality of the evidence supporting the detected issue
    pub quality_of_evidence: Option<types::CodeableConcept>,

    /// Importance of taking action on the issue
    pub management_code: Option<types::CodeableConcept>,

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
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: crate::coded::Coded<crate::r6::codes::DetectedissueStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    code: Option<types::CodeableConcept>,
    severity: Option<types::CodeableConcept>,
    subject: Option<types::Reference>,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(flatten)]
    identified: crate::r6::choice::Slot<DetectedIssueIdentified>,
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
    quality_of_evidence: Option<types::CodeableConcept>,
    management_code: Option<types::CodeableConcept>,
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
            quality_of_evidence: v.quality_of_evidence,
            management_code: v.management_code,
            mitigation: v.mitigation,
        }
    }
}

/// Supporting evidence or manifestations that provide the basis for
/// identifying the detected issue such as a GuidanceResponse or MeasureReport.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::detected_issue::DetectedIssueEvidence;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Indicates an action that has been taken or is committed to reduce or
/// eliminate the likelihood of the risk identified by the detected issue from
/// manifesting. Can also reflect an observation of known mitigating factors
/// that may reduce/eliminate the need for any action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::detected_issue::DetectedIssueMitigation;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is committing?
    pub author: Option<types::Reference>,

    /// Additional notes about the mitigation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// The `DetectedIssue.identified[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DetectedIssueIdentified {
    /// `identifiedDateTime` variant.
    #[fhir("identifiedDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `identifiedPeriod` variant.
    #[fhir("identifiedPeriod")]
    Period(Box<types::Period>),
    /// `identifiedTiming` variant.
    #[fhir("identifiedTiming")]
    Timing(Box<types::Timing>),
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
