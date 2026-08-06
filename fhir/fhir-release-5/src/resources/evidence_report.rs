//! EvidenceReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EvidenceReport
//!
//! Version: 5.0.0
//!
//! EvidenceReport Resource: The EvidenceReport Resource is a specialized container for a collection of resources and codeable concepts, adapted to support compositions of Evidence, EvidenceVariable, and Citation resources and related concepts.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// EvidenceReport
///
/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codeable concepts, adapted to support compositions of Evidence,
/// EvidenceVariable, and Citation resources and related concepts. It functions as
/// a structured document with sections, allowing the assembly of research findings
/// and related artifacts into a reviewable, shareable report. Typical uses include
/// systematic reviews, evidence syntheses, and structured summaries of clinical
/// research.
///
/// Like the general-purpose `Composition` resource, an EvidenceReport is organized
/// as a hierarchy of sections (see [`EvidenceReportSection`]) that may carry
/// narrative text, coded entries, quantities, or references to other resources.
/// Its `subject` element characterizes the population, exposure, or other focus
/// that the report addresses, and its `relatesTo` element expresses relationships
/// (such as replaces, amends, or appends) to other reports or compositions,
/// supporting versioning and provenance of published evidence syntheses.
///
/// # Related resources
///
/// - [`Citation`](crate::r5::resources::citation::Citation) — bibliographic
///   citation for the artifacts an EvidenceReport summarizes or references.
/// - [`Evidence`](crate::r5::resources::evidence::Evidence) — a single
///   evidence concept that may be included or discussed within a report.
/// - [`EvidenceVariable`](crate::r5::resources::evidence_variable::EvidenceVariable) —
///   population, exposure, or outcome definitions referenced by the report's
///   subject characteristics.
/// - [`Composition`](crate::r5::resources::composition::Composition) — the
///   general-purpose document resource that EvidenceReport specializes.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_report::EvidenceReport;
/// use fhir::r5::types;
///
/// let value = EvidenceReport {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: EvidenceReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReport {
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

    /// Canonical identifier for this EvidenceReport, represented as a globally unique URI
    /// that allows the report to be referenced from other resources.
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// The publication lifecycle status of this report: draft | active | retired | unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Unique identifier for the evidence report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Identifiers for articles that may relate to more than one evidence report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_identifier: Vec<types::Identifier>,

    /// The `EvidenceReport.citeAs[x]` choice element (0..1); see [`EvidenceReportCiteAs`].
    #[serde(flatten)]
    pub cite_as: Option<EvidenceReportCiteAs>,

    /// Kind of report
    pub r#type: Option<types::CodeableConcept>,

    /// Used for footnotes and annotations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Link, description or reference to artifact associated with the report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Focus of the report, describing the population, exposure, or other
    /// characteristics the evidence addresses.
    pub subject: EvidenceReportSubject,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Relationships to other compositions/documents, such as prior versions
    /// this report replaces or amends.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<EvidenceReportRelatesTo>,

    /// The hierarchical sections that make up the body of the report.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<EvidenceReportSection>,
}

/// EvidenceReportSubject
///
/// Specifies the subject or focus of the report, describing the population,
/// exposures, or other characteristics that the evidence report addresses.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_report::EvidenceReportSubject;
/// use fhir::r5::types;
///
/// let value = EvidenceReportSubject {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceReportSubject = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSubject {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Characteristic
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<EvidenceReportSubjectCharacteristic>,

    /// Footnotes and/or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// EvidenceReportSubjectCharacteristic
///
/// Describes a single characteristic that defines the subject of the report, such
/// as a coded value with an optional exclusion flag and timeframe.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_report::EvidenceReportSubjectCharacteristic;
/// use fhir::r5::types;
///
/// let value = EvidenceReportSubjectCharacteristic {
///     exclude: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `exclude` is the name this serializes to on the wire.
/// assert_eq!(json["exclude"], ::serde_json::json!(true));
///
/// let back: EvidenceReportSubjectCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSubjectCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Characteristic code
    pub code: types::CodeableConcept,

    /// The `EvidenceReport.subject.characteristic.value[x]` choice element (0..1); see [`EvidenceReportSubjectCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<EvidenceReportSubjectCharacteristicValue>,

    /// Is used to express not the characteristic
    pub exclude: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`).
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// Timeframe for the characteristic
    pub period: Option<types::Period>,
}

/// EvidenceReportRelatesTo
///
/// Expresses a relationship between this evidence report and another composition
/// or document, such as a replacement, amendment, or transformation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_report::EvidenceReportRelatesTo;
/// use fhir::r5::types;
///
/// let value = EvidenceReportRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceReportRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// replaces | amends | appends | transforms | replacedWith | amendedWith | appendedWith | transformedWith
    pub code: crate::r5::coded::Coded<crate::r5::codes::ReportRelationType>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Target of the relationship
    pub target: EvidenceReportRelatesToTarget,
}

/// EvidenceReportRelatesToTarget
///
/// Identifies the target of a relationship expressed by the report, referenced by
/// URL, identifier, display text, or a resource reference.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_report::EvidenceReportRelatesToTarget;
/// use fhir::r5::types;
///
/// let value = EvidenceReportRelatesToTarget {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: EvidenceReportRelatesToTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportRelatesToTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Target of the relationship URL
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Target of the relationship Identifier
    pub identifier: Option<types::Identifier>,

    /// Target of the relationship Display
    pub display: Option<types::Markdown>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Target of the relationship Resource reference
    pub resource: Option<types::Reference>,
}

/// EvidenceReportSection
///
/// Represents a section of the composition, which may contain narrative text,
/// entries referencing other resources, and nested subsections, enabling a
/// hierarchical document structure.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_report::EvidenceReportSection;
/// use fhir::r5::types;
///
/// let value = EvidenceReportSection {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: EvidenceReportSection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSection {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for section (e.g. for ToC)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Classification of section (recommended)
    pub focus: Option<types::CodeableConcept>,

    /// Classification of section by Resource
    pub focus_reference: Option<types::Reference>,

    /// Who and/or what authored the section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Text summary of the section, for human interpretation
    pub text: Option<types::Narrative>,

    /// working | snapshot | changes
    pub mode: Option<crate::r5::coded::Coded<crate::r5::codes::ListMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Order of section entries
    pub ordered_by: Option<types::CodeableConcept>,

    /// Extensible classifiers as content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_classifier: Vec<types::CodeableConcept>,

    /// Reference to resources as content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_reference: Vec<types::Reference>,

    /// Quantity as content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_quantity: Vec<types::Quantity>,

    /// Why the section is empty
    pub empty_reason: Option<types::CodeableConcept>,

    /// Nested Section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<EvidenceReportSection>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EvidenceReport;

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
/// The `EvidenceReport.citeAs[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceReportCiteAs {
    /// `citeAsReference` variant.
    #[fhir("citeAsReference")]
    Reference(Box<types::Reference>),
    /// `citeAsMarkdown` variant.
    #[fhir("citeAsMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
}

/// The `EvidenceReport.subject.characteristic.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceReportSubjectCharacteristicValue {
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
}
