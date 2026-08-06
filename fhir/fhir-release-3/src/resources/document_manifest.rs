//! DocumentManifest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DocumentManifest
//!
//!
//!
//! A list that defines a set of documents
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DocumentManifest Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::document_manifest::DocumentManifest;
///
/// let value = DocumentManifest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DocumentManifest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DocumentManifest {
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique Identifier for the set of documents
    pub master_identifier: Option<types::Identifier>,

    /// Other identifiers for the manifest
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// current | superseded | entered-in-error
    pub status: crate::coded::Coded<crate::r3::codes::DocumentReferenceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of document set
    pub r#type: Option<types::CodeableConcept>,

    /// The subject of the set of documents
    pub subject: Option<types::Reference>,

    /// When this document manifest created
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Who and/or what authored the manifest
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Intended to get notified about this set of documents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipient: Vec<types::Reference>,

    /// The source system/application/software
    pub source: Option<types::Uri>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Human-readable description (title)
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The items included
    pub content: ::vec1::Vec1<DocumentManifestContent>,

    /// Related things
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<DocumentManifestRelated>,
}

/// The list of Documents included in the manifest.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::document_manifest::DocumentManifestContent;
/// use fhir::r3::types;
///
/// let value = DocumentManifestContent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DocumentManifestContent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DocumentManifestContent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Contents of this set of documents
    /// The `DocumentManifest.content.p[x]` choice element (1..1); see [`DocumentManifestContentP`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub p: Option<DocumentManifestContentP>,
}

/// Related identifiers or resources associated with the DocumentManifest.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::document_manifest::DocumentManifestRelated;
/// use fhir::r3::types;
///
/// let value = DocumentManifestRelated {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DocumentManifestRelated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DocumentManifestRelated {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifiers of things that are related
    pub identifier: Option<types::Identifier>,

    /// Related Resource
    pub r#ref: Option<types::Reference>,
}

/// The `DocumentManifest.content.p[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DocumentManifestContentP {
    /// `pAttachment` variant.
    #[fhir("pAttachment")]
    Attachment(Box<types::Attachment>),
    /// `pReference` variant.
    #[fhir("pReference")]
    Reference(Box<types::Reference>),
}
