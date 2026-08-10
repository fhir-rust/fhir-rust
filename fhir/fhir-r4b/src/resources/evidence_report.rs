//! EvidenceReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EvidenceReport
//!
//! Version: 4.3.0
//!
//! A EvidenceReport
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The EvidenceReport Resource is a specialized container for a collection of
/// resources and codable concepts, adapted to support compositions of
/// Evidence, EvidenceVariable, and Citation resources and related concepts.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence_report::EvidenceReport;
/// use fhir::r4b::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct EvidenceReport {
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this EvidenceReport, represented as a globally
    /// unique URI
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Unique identifier for the evidence report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Identifiers for articles that may relate to more than one evidence
    /// report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_identifier: Vec<types::Identifier>,

    /// Citation for this report
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

    /// Focus of the report
    pub subject: EvidenceReportSubject,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// Relationships to other compositions/documents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<EvidenceReportRelatesTo>,

    /// Composition is broken into sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<EvidenceReportSection>,
}

/// Relationships that this composition has with other compositions or
/// documents that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence_report::EvidenceReportRelatesTo;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct EvidenceReportRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// replaces | amends | appends | transforms | replacedWith | amendedWith |
    /// appendedWith | transformedWith
    pub code: crate::coded::Coded<crate::r4b::codes::ReportRelationType>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Target of the relationship
    /// The `EvidenceReport.relatesTo.target[x]` choice element (1..1); see [`EvidenceReportRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<EvidenceReportRelatesToTarget>,
}

/// The root of the sections that make up the composition.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence_report::EvidenceReportSection;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
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
    pub mode: Option<crate::coded::Coded<crate::r4b::codes::ListMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
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

/// Specifies the subject or focus of the report. Answers "What is this report
/// about?".
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence_report::EvidenceReportSubject;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

/// Characteristic.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::evidence_report::EvidenceReportSubjectCharacteristic;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

    /// Characteristic value
    /// The `EvidenceReport.subject.characteristic.value[x]` choice element (1..1); see [`EvidenceReportSubjectCharacteristicValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<EvidenceReportSubjectCharacteristicValue>,

    /// Is used to express not the characteristic
    pub exclude: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// Timeframe for the characteristic
    pub period: Option<types::Period>,
}

/// The `EvidenceReport.citeAs[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceReportCiteAs {
    /// `citeAsReference` variant.
    #[fhir("citeAsReference")]
    Reference(Box<types::Reference>),
    /// `citeAsMarkdown` variant.
    #[fhir("citeAsMarkdown")]
    Markdown(crate::r4b::choice::Primitive<types::Markdown>),
}

/// The `EvidenceReport.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceReportRelatesToTarget {
    /// `targetIdentifier` variant.
    #[fhir("targetIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `targetReference` variant.
    #[fhir("targetReference")]
    Reference(Box<types::Reference>),
}

/// The `EvidenceReport.subject.characteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
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
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
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
