//! Composition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Composition
//!
//!
//!
//! A set of resources composed into a single coherent clinical statement with
//! clinical attestation
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Composition Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::composition::Composition;
///
/// let value = Composition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Composition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Composition {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Logical identifier of composition (version-independent)
    pub identifier: Option<types::Identifier>,

    /// preliminary | final | amended | entered-in-error
    pub status: crate::coded::Coded<crate::r3::codes::CompositionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of composition (LOINC if possible)
    pub r#type: types::CodeableConcept,

    /// Categorization of Composition
    pub class: Option<types::CodeableConcept>,

    /// Who and/or what the composition is about
    pub subject: types::Reference,

    /// Context of the Composition
    pub encounter: Option<types::Reference>,

    /// Composition editing time
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who and/or what authored the composition
    pub author: ::vec1::Vec1<types::Reference>,

    /// Human Readable name/title
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// As defined by affinity domain
    pub confidentiality: Option<types::Code>,
    /// Primitive extension sibling for [`confidentiality`](Self::confidentiality) (FHIR `_confidentiality`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_confidentiality")]
    pub confidentiality_ext: Option<types::Element>,

    /// Attests to accuracy of composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attester: Vec<CompositionAttester>,

    /// Organization which maintains the composition
    pub custodian: Option<types::Reference>,

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
/// ```ignore
/// use fhir::r3::resources::composition::CompositionAttester;
///
/// let value = CompositionAttester::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CompositionAttester = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CompositionAttester {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// personal | professional | legal | official
    pub mode: ::vec1::Vec1<crate::coded::Coded<crate::r3::codes::CompositionAttestationMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mode_ext: Vec<Option<types::Element>>,

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
/// use fhir::r3::resources::composition::CompositionEvent;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct CompositionEvent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code(s) that apply to the event being documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// The period covered by the documentation
    pub period: Option<types::Period>,

    /// The event(s) being documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<types::Reference>,
}

/// Relationships that this composition has with other compositions or
/// documents that already exist.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::composition::CompositionRelatesTo;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct CompositionRelatesTo {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// replaces | transforms | signs | appends
    pub code: crate::coded::Coded<crate::r3::codes::DocumentRelationshipType>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Target of the relationship
    /// The `Composition.relatesTo.target[x]` choice element (1..1); see [`CompositionRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<CompositionRelatesToTarget>,
}

/// The root of the sections that make up the composition.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::composition::CompositionSection;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct CompositionSection {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

    /// Text summary of the section, for human interpretation
    pub text: Option<types::Narrative>,

    /// working | snapshot | changes
    pub mode: Option<crate::coded::Coded<crate::r3::codes::ListMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

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
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum CompositionRelatesToTarget {
    /// `targetIdentifier` variant.
    #[fhir("targetIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `targetReference` variant.
    #[fhir("targetReference")]
    Reference(Box<types::Reference>),
}
