//! ArtifactAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
//!
//! Version: 5.0.0
//!
//! ArtifactAssessment Resource: This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ArtifactAssessment provides one or more comments, classifiers, or ratings
/// about a Resource and supports attribution and rights management metadata for
/// the added content. It is used in FHIR R5 to capture reviews, editorial
/// commentary, quality classifiers, and quantitative ratings that are attached
/// to a referenced or canonical artifact. Assessments carry workflow status and
/// disposition to track how comments are processed over their lifecycle. In
/// practice this resource supports evidence appraisal and editorial workflows,
/// such as peer review comments on a guideline, quality ratings on a piece of
/// evidence, or structured responses (including proposed changes) to a
/// published artifact, allowing reviewers and publishers to record, triage,
/// and resolve feedback in a consistent, machine-readable way.
///
/// The artifact under assessment may be identified by reference, canonical
/// URL, or plain URI, and the substance of the assessment is carried in one or
/// more [`ArtifactAssessmentContent`] entries, each of which can itself be
/// classified using a [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// and nested to build multi-part reviews.
///
/// See also: `Citation`, `Basic`, and other assessed resource types, which are
/// commonly referenced from `artifact_reference` or `artifact_canonical`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::artifact_assessment::ArtifactAssessment;
/// use fhir::r5::types;
///
/// let value = ArtifactAssessment {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: ArtifactAssessment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ArtifactAssessmentDe")]
pub struct ArtifactAssessment {
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

    /// Additional identifier for the artifact assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A short title for the assessment for use in displaying and selecting
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The `ArtifactAssessment.citeAs[x]` choice element (0..1); see [`ArtifactAssessmentCiteAs`].
    #[serde(flatten)]
    pub cite_as: Option<ArtifactAssessmentCiteAs>,

    /// Date the artifact assessment was last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// When the artifact assessment was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the artifact assessment was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// The `ArtifactAssessment.artifact[x]` choice element (0..1); see [`ArtifactAssessmentArtifact`].
    #[serde(flatten)]
    pub artifact: Option<ArtifactAssessmentArtifact>,

    /// The comment, classifier, or rating content that makes up the assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content: Vec<ArtifactAssessmentContent>,

    /// submitted | triaged | waiting-for-input | resolved-no-change | resolved-change-required | deferred | duplicate | applied | published | entered-in-error
    pub workflow_status:
        Option<crate::r5::coded::Coded<crate::r5::codes::ArtifactassessmentWorkflowStatus>>,
    /// Primitive extension sibling for [`workflow_status`](Self::workflow_status) (FHIR `_workflowStatus`).
    #[serde(rename = "_workflowStatus")]
    pub workflow_status_ext: Option<types::Element>,

    /// unresolved | not-persuasive | persuasive | persuasive-with-modification | not-persuasive-with-modification
    pub disposition:
        Option<crate::r5::coded::Coded<crate::r5::codes::ArtifactassessmentDisposition>>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`).
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ArtifactAssessmentDe {
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
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    #[serde(flatten)]
    cite_as: crate::r5::choice::Slot<ArtifactAssessmentCiteAs>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    approval_date: Option<types::Date>,
    #[serde(rename = "_approvalDate")]
    approval_date_ext: Option<types::Element>,
    last_review_date: Option<types::Date>,
    #[serde(rename = "_lastReviewDate")]
    last_review_date_ext: Option<types::Element>,
    #[serde(flatten)]
    artifact: crate::r5::choice::Slot<ArtifactAssessmentArtifact>,
    #[serde(default)]
    content: Vec<ArtifactAssessmentContent>,
    workflow_status:
        Option<crate::r5::coded::Coded<crate::r5::codes::ArtifactassessmentWorkflowStatus>>,
    #[serde(rename = "_workflowStatus")]
    workflow_status_ext: Option<types::Element>,
    disposition: Option<crate::r5::coded::Coded<crate::r5::codes::ArtifactassessmentDisposition>>,
    #[serde(rename = "_disposition")]
    disposition_ext: Option<types::Element>,
}

impl ::core::convert::From<ArtifactAssessmentDe> for ArtifactAssessment {
    fn from(v: ArtifactAssessmentDe) -> Self {
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
            title: v.title,
            title_ext: v.title_ext,
            cite_as: v.cite_as.0,
            date: v.date,
            date_ext: v.date_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            approval_date: v.approval_date,
            approval_date_ext: v.approval_date_ext,
            last_review_date: v.last_review_date,
            last_review_date_ext: v.last_review_date_ext,
            artifact: v.artifact.0,
            content: v.content,
            workflow_status: v.workflow_status,
            workflow_status_ext: v.workflow_status_ext,
            disposition: v.disposition,
            disposition_ext: v.disposition_ext,
        }
    }
}

/// Comment, classifier, or rating content of an [`ArtifactAssessment`].
///
/// A content entry captures a single unit of assessment such as a comment, a
/// classifier, or a quantitative rating, along with attribution and rights
/// management metadata. Content entries can be nested via the `component`
/// field to build structured, multi-part assessments.
/// # Examples
///
/// ```
/// use fhir::r5::resources::artifact_assessment::ArtifactAssessmentContent;
/// use fhir::r5::types;
///
/// let value = ArtifactAssessmentContent {
///     free_to_share: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `freeToShare` is the name this serializes to on the wire.
/// assert_eq!(json["freeToShare"], ::serde_json::json!(true));
///
/// let back: ArtifactAssessmentContent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactAssessmentContent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// comment | classifier | rating | container | response | change-request
    pub information_type:
        Option<crate::r5::coded::Coded<crate::r5::codes::ArtifactassessmentInformationType>>,
    /// Primitive extension sibling for [`information_type`](Self::information_type) (FHIR `_informationType`).
    #[serde(rename = "_informationType")]
    pub information_type_ext: Option<types::Element>,

    /// Brief summary of the content
    pub summary: Option<types::Markdown>,
    /// Primitive extension sibling for [`summary`](Self::summary) (FHIR `_summary`).
    #[serde(rename = "_summary")]
    pub summary_ext: Option<types::Element>,

    /// What type of content
    pub r#type: Option<types::CodeableConcept>,

    /// Rating, classifier, or assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// Quantitative rating
    pub quantity: Option<types::Quantity>,

    /// Who authored the content
    pub author: Option<types::Reference>,

    /// What the comment is directed to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<types::Uri>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path_ext: Vec<Option<types::Element>>,

    /// Additional information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Acceptable to publicly share the resource content
    pub free_to_share: Option<types::Boolean>,
    /// Primitive extension sibling for [`free_to_share`](Self::free_to_share) (FHIR `_freeToShare`).
    #[serde(rename = "_freeToShare")]
    pub free_to_share_ext: Option<types::Element>,

    /// Contained content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ArtifactAssessmentContent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ArtifactAssessment;

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
/// The `ArtifactAssessment.artifact[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ArtifactAssessmentArtifact {
    /// `artifactReference` variant.
    #[fhir("artifactReference")]
    Reference(Box<types::Reference>),
    /// `artifactCanonical` variant.
    #[fhir("artifactCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
    /// `artifactUri` variant.
    #[fhir("artifactUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
}

/// The `ArtifactAssessment.citeAs[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ArtifactAssessmentCiteAs {
    /// `citeAsReference` variant.
    #[fhir("citeAsReference")]
    Reference(Box<types::Reference>),
    /// `citeAsMarkdown` variant.
    #[fhir("citeAsMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
}
