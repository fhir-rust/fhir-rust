//! Composition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Composition
//!
//! Version: 6.0.0-ballot3
//!
//! A set of resources composed into a single coherent clinical statement with
//! clinical attestation
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A set of healthcare-related information that is assembled together into a
/// single logical package that provides a single coherent statement of
/// meaning, establishes its own context and that has clinical attestation with
/// regard to who is making the statement. A Composition defines the structure
/// and narrative content necessary for a document. However, a Composition
/// alone does not constitute a document. Rather, the Composition must be the
/// first entry in a Bundle where Bundle.type=document, and any other resources
/// referenced from Composition must be included as subsequent entries in the
/// Bundle (for example Patient, Practitioner, Encounter, etc.).
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::composition::Composition;
///
/// let value = Composition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Composition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Composition {
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

    /// Canonical identifier for this Composition, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Version-independent identifier for the Composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// An explicitly assigned identifier of a variation of the content in the
    /// Composition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// registered | partial | preliminary | final | amended | corrected |
    /// appended | cancelled | entered-in-error | deprecated | unknown
    pub status: crate::coded::Coded<crate::r6::codes::CompositionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of composition (LOINC if possible)
    pub r#type: types::CodeableConcept,

    /// Categorization of Composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Who and/or what the composition is about
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Context of the Composition
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Composition editing time
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Who and/or what authored the composition
    pub author: ::vec1::Vec1<types::Reference>,

    /// Name for this Composition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human Readable name/title
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// For any additional notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Attests to accuracy of composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attester: Vec<CompositionAttester>,

    /// Organization which maintains the composition
    pub custodian: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Relationships to other compositions/documents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<CompositionRelatesTo>,

    /// The clinical service(s) being documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CompositionEvent>,

    /// Composition is broken into sections
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub section: Vec<CompositionSection>,
}

/// A participant who has attested to the accuracy of the composition/document.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::composition::CompositionAttester;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,

    /// Who attested the composition
    pub party: Option<types::Reference>,
}

/// The clinical service, such as a colonoscopy or an appendectomy, being
/// documented.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::composition::CompositionEvent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Relationships that this composition has with other compositions or
/// documents (FHIR or non-FHIR resources) that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::composition::CompositionRelatesTo;
/// use fhir::r6::types;
///
/// let value = CompositionRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CompositionRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CompositionRelatesTo {
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

    /// The artifact that is related to this Composition
    /// The `Composition.relatesTo.target[x]` choice element (1..1); see [`CompositionRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<CompositionRelatesToTarget>,
}

/// The root of the sections that make up the composition.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::composition::CompositionSection;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Classification of section (recommended)
    pub code: Option<types::CodeableConcept>,

    /// Who and/or what authored the section, when the section is authored by
    /// someone other than the composition.author
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Who/what the section is about, when it is not about the subject of
    /// composition
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

/// The `Composition.relatesTo.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CompositionRelatesToTarget {
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
