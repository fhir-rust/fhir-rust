//! DocumentReference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
//!
//! Version: 4.0.1
//!
//! A reference to a document
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A reference to a document of any kind for any purpose. Provides metadata
/// about the document so that the document can be discovered and managed. The
/// scope of a document is any seralized object with a mime-type, so includes
/// formal patient centric documents (CDA), cliical notes, scanned paper, and
/// non-patient specific documents like policy text.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::document_reference::DocumentReference;
///
/// let value = DocumentReference::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DocumentReference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DocumentReference {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Master Version Specific Identifier
    pub master_identifier: Option<types::Identifier>,

    /// Other identifiers for the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// current | superseded | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::DocumentReferenceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// preliminary | final | amended | entered-in-error
    pub doc_status: Option<crate::coded::Coded<crate::r4::codes::CompositionStatus>>,
    /// Primitive extension sibling for [`doc_status`](Self::doc_status) (FHIR `_docStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_docStatus")]
    pub doc_status_ext: Option<types::Element>,

    /// Kind of document (LOINC if possible)
    pub r#type: Option<types::CodeableConcept>,

    /// Categorization of document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Who/what is the subject of the document
    pub subject: Option<types::Reference>,

    /// When this document reference was created
    pub date: Option<types::Instant>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who and/or what authored the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Who/what authenticated the document
    pub authenticator: Option<types::Reference>,

    /// Organization which maintains the document
    pub custodian: Option<types::Reference>,

    /// Relationships to other documents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<DocumentReferenceRelatesTo>,

    /// Human-readable description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Document security-tags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::CodeableConcept>,

    /// Document referenced
    pub content: ::vec1::Vec1<DocumentReferenceContent>,

    /// Clinical context of document
    pub context: Option<DocumentReferenceContext>,
}

/// The document and format referenced. There may be multiple content element
/// repetitions, each with a different format.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::document_reference::DocumentReferenceContent;
/// use fhir::r4::types;
///
/// let value = DocumentReferenceContent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DocumentReferenceContent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DocumentReferenceContent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Where to access the document
    pub attachment: types::Attachment,

    /// Format/content rules for the document
    pub format: Option<types::Coding>,
}

/// The clinical context in which the document was prepared.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::document_reference::DocumentReferenceContext;
/// use fhir::r4::types;
///
/// let value = DocumentReferenceContext {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DocumentReferenceContext = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DocumentReferenceContext {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Context of the document content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encounter: Vec<types::Reference>,

    /// Main clinical acts documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<types::CodeableConcept>,

    /// Time of service that is being documented
    pub period: Option<types::Period>,

    /// Kind of facility where patient was seen
    pub facility_type: Option<types::CodeableConcept>,

    /// Additional details about where the content was created (e.g. clinical
    /// specialty)
    pub practice_setting: Option<types::CodeableConcept>,

    /// Patient demographics from source
    pub source_patient_info: Option<types::Reference>,

    /// Related identifiers or resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<types::Reference>,
}

/// Relationships that this document has with other document references that
/// already exist.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::document_reference::DocumentReferenceRelatesTo;
/// use fhir::r4::types;
///
/// let value = DocumentReferenceRelatesTo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DocumentReferenceRelatesTo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct DocumentReferenceRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// replaces | transforms | signs | appends
    pub code: crate::coded::Coded<crate::r4::codes::DocumentRelationshipType>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Target of the relationship
    pub target: types::Reference,
}
