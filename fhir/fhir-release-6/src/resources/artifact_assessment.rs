//! ArtifactAssessment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ArtifactAssessment
//!
//! Version: 6.0.0-ballot3
//!
//! Adds metadata-supported comments, classifiers or ratings related to a
//! Resource
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This Resource provides one or more comments, classifiers or ratings about a
/// Resource and supports attribution and rights management metadata for the
/// added content.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::artifact_assessment::ArtifactAssessment;
/// use fhir::r6::types;
///
/// let value = ArtifactAssessment {
///     cite_as: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `citeAs` is the name this serializes to on the wire.
/// assert_eq!(json["citeAs"], ::serde_json::json!("# Heading"));
///
/// let back: ArtifactAssessment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ArtifactAssessment {
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

    /// Additional identifier for the artifact assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A label for use in displaying and selecting the artifact assessment
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// How to cite the comment or rating
    pub cite_as: Option<types::Markdown>,
    /// Primitive extension sibling for [`cite_as`](Self::cite_as) (FHIR `_citeAs`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_citeAs")]
    pub cite_as_ext: Option<types::Element>,

    /// The artifact assessed, commented upon or rated
    /// The `ArtifactAssessment.artifact[x]` choice element (1..1); see [`ArtifactAssessmentArtifact`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub artifact: Option<ArtifactAssessmentArtifact>,

    /// Relationship to other Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<ArtifactAssessmentRelatesTo>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// When the artifact assessment was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the artifact assessment was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// Comment, classifier, or rating content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content: Vec<ArtifactAssessmentContent>,

    /// submitted | triaged | waiting-for-input | resolved-no-change |
    /// resolved-change-required | deferred | duplicate | applied | published |
    /// entered-in-error
    pub workflow_status:
        Option<crate::coded::Coded<crate::r6::codes::ArtifactassessmentWorkflowStatus>>,
    /// Primitive extension sibling for [`workflow_status`](Self::workflow_status) (FHIR `_workflowStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_workflowStatus")]
    pub workflow_status_ext: Option<types::Element>,

    /// unresolved | not-persuasive | persuasive | persuasive-with-modification
    /// | not-persuasive-with-modification
    pub disposition: Option<crate::coded::Coded<crate::r6::codes::ArtifactassessmentDisposition>>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,
}

/// A component comment, classifier, or rating of the artifact.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::artifact_assessment::ArtifactAssessmentContent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ArtifactAssessmentContent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Brief summary of the content
    pub summary: Option<types::Markdown>,
    /// Primitive extension sibling for [`summary`](Self::summary) (FHIR `_summary`):
    /// carries `id` and/or `extension` for the primitive value.
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// What the comment is directed to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<types::Uri>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path_ext: Vec<Option<types::Element>>,

    /// Relationship to other Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<ArtifactAssessmentRelatesTo>,

    /// Acceptable to publicly share the content
    pub free_to_share: Option<types::Boolean>,
    /// Primitive extension sibling for [`free_to_share`](Self::free_to_share) (FHIR `_freeToShare`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_freeToShare")]
    pub free_to_share_ext: Option<types::Element>,

    /// Comment, classifier, or rating content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ArtifactAssessmentContent>,
}

/// Relationship that this ArtifactAssessment has with other FHIR or non-FHIR
/// resources that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::artifact_assessment::ArtifactAssessmentRelatesTo;
/// use fhir::r6::types;
///
/// let value = ArtifactAssessmentRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ArtifactAssessmentRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ArtifactAssessmentRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// documentation | justification | predecessor | successor | derived-from
    /// | depends-on | composed-of | part-of | amends | amended-with | appends
    /// | appended-with | cites | cited-by | comments-on | comment-in |
    /// contains | contained-in | corrects | correction-in | replaces |
    /// replaced-with | retracts | retracted-by | signs | similar-to | supports
    /// | supported-with | transforms | transformed-into | transformed-with |
    /// specification-of | created-with | cite-as | summarizes
    pub r#type: crate::coded::Coded<crate::r6::codes::ArtifactRelationshipType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The artifact that is related to this ArtifactAssessment
    /// The `ArtifactAssessment.relatesTo.target[x]` choice element (1..1); see [`ArtifactAssessmentRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<ArtifactAssessmentRelatesToTarget>,
}

/// The `ArtifactAssessment.artifact[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ArtifactAssessmentArtifact {
    /// `artifactReference` variant.
    #[fhir("artifactReference")]
    Reference(Box<types::Reference>),
    /// `artifactCanonical` variant.
    #[fhir("artifactCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `artifactUri` variant.
    #[fhir("artifactUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

/// The `ArtifactAssessment.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ArtifactAssessmentRelatesToTarget {
    /// `targetUri` variant.
    #[fhir("targetUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `targetAttachment` variant.
    #[fhir("targetAttachment")]
    Attachment(Box<types::Attachment>),
    /// `targetCanonical` variant.
    #[fhir("targetCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `targetReference` variant.
    #[fhir("targetReference")]
    Reference(Box<types::Reference>),
    /// `targetMarkdown` variant.
    #[fhir("targetMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
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
