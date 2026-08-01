//! Composition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Composition
//!
//! Version: 5.0.0
//!
//! Composition Resource: A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A set of healthcare-related information that is assembled together into a
/// single logical package.
///
/// A Composition establishes a single coherent statement of meaning, defines
/// its own context, and carries clinical attestation regarding who is making
/// the statement. It provides the structure and narrative content necessary for
/// a clinical document, organized into sections. In FHIR R5 a Composition alone
/// does not constitute a document; it is typically the first entry in a Bundle
/// of type "document" that packages the referenced resources together.
///
/// Compositions are used to represent discharge summaries, consultation notes,
/// diagnostic reports, care plans, and other attested clinical or administrative
/// documents. Each Composition has exactly one `subject` context (commonly a
/// patient) though implementations may reference multiple related subjects, a
/// responsible `author`, a `status` reflecting its lifecycle, and one or more
/// `section` elements that hold the narrative and referenced entries making up
/// the body of the document. Once a Composition is part of a signed or attested
/// document, its content is generally considered immutable; a new `version` or a
/// document addendum is used to record subsequent changes.
///
/// # Related resources
///
/// A Composition typically references a [`Patient`](crate::r5::resources::patient::Patient)
/// or other resource as its subject, is authored and attested by practitioners or
/// organizations via `Reference`, and classifies its type and section codes using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept). See also the `DocumentReference`
/// and `Bundle` resources, which are commonly used together with Composition to
/// index and package a complete clinical document.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::composition::Composition;
///
/// let value = Composition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Composition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Composition {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this Composition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Version-independent identifier for the Composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// An explicitly assigned identifer of a variation of the content in the Composition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The workflow/clinical status of this composition: registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::CompositionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of composition, such as a discharge summary or progress note (LOINC code preferred if possible)
    pub r#type: types::CodeableConcept,

    /// Categorization of Composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Who and/or what the composition is about, most often the patient or subject of the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Context of the Composition
    pub encounter: Option<types::Reference>,

    /// Composition editing time, the time the document was last logically attested or edited
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Who and/or what authored the composition, such as a practitioner or organization
    pub author: vec1::Vec1<types::Reference>,

    /// Name for this Composition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human readable name/title, for example "Discharge Summary"
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// For any additional notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Attests to accuracy of composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attester: Vec<CompositionAttester>,

    /// Organization which maintains the composition
    pub custodian: Option<types::Reference>,

    /// Relationships to other compositions/documents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<types::RelatedArtifact>,

    /// The clinical service(s) being documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CompositionEvent>,

    /// Composition is broken into sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<CompositionSection>,
}

/// Attests to accuracy of composition.
/// # Examples
///
/// ```
/// use fhir::r5::resources::composition::CompositionAttester;
/// use fhir::r5::types;
///
/// let value = CompositionAttester {
///     time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `time` is the name this serializes to on the wire.
/// assert_eq!(json["time"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CompositionAttester = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompositionAttester {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// personal | professional | legal | official
    pub mode: types::CodeableConcept,

    /// When the composition was attested
    pub time: Option<types::DateTime>,
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`).
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,

    /// Who attested the composition
    pub party: Option<types::Reference>,
}

/// The clinical service(s) being documented.
/// # Examples
///
/// ```
/// use fhir::r5::resources::composition::CompositionEvent;
/// use fhir::r5::types;
///
/// let value = CompositionEvent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CompositionEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompositionEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The period covered by the documentation
    pub period: Option<types::Period>,

    /// The event(s) being documented, as code(s), reference(s), or both
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<types::CodeableReference>,
}

/// Composition is broken into sections.
/// # Examples
///
/// ```
/// use fhir::r5::resources::composition::CompositionSection;
/// use fhir::r5::types;
///
/// let value = CompositionSection {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: CompositionSection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompositionSection {
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
    pub code: Option<types::CodeableConcept>,

    /// Who and/or what authored the section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Who/what the section is about, when it is not about the subject of composition
    pub focus: Option<types::Reference>,

    /// Text summary of the section, for human interpretation
    pub text: Option<types::Narrative>,

    /// Order of section entries
    pub ordered_by: Option<types::CodeableConcept>,

    /// A reference to data that supports this section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<types::Reference>,

    /// Why the section is empty
    pub empty_reason: Option<types::CodeableConcept>,

    /// Nested Section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<CompositionSection>,
}
