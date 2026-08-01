//! DetectedIssue
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
//!
//! Version: 4.0.1
//!
//! Clinical issue with action
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient; e.g. Drug-drug
/// interaction, Ineffective treatment frequency, Procedure-condition conflict,
/// etc.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::detected_issue::DetectedIssue;
/// use fhir::r4::types;
///
/// let value = DetectedIssue {
///     detail: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `detail` is the name this serializes to on the wire.
/// assert_eq!(json["detail"], ::serde_json::json!("abc"));
///
/// let back: DetectedIssue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for the detected issue
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// registered | preliminary | final | amended +
    pub status: crate::coded::Coded<crate::r4::codes::ObservationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Issue Category, e.g. drug-drug, duplicate therapy, etc.
    pub code: Option<types::CodeableConcept>,

    /// high | moderate | low
    pub severity: Option<crate::coded::Coded<crate::r4::codes::DetectedissueSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// Associated patient
    pub patient: Option<types::Reference>,

    /// When identified
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
    pub detail: Option<types::String>,
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

    /// Step taken to address
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigation: Vec<DetectedIssueMitigation>,
}

/// Supporting evidence or manifestations that provide the basis for
/// identifying the detected issue such as a GuidanceResponse or MeasureReport.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::detected_issue::DetectedIssueEvidence;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
/// use fhir::r4::resources::detected_issue::DetectedIssueMitigation;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
}

/// The `DetectedIssue.identified[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum DetectedIssueIdentified {
    /// `identifiedDateTime` variant.
    #[fhir("identifiedDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
    /// `identifiedPeriod` variant.
    #[fhir("identifiedPeriod")]
    Period(Box<types::Period>),
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
