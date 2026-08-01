//! DocumentReference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
//!
//! Version: 5.0.0
//!
//! DocumentReference Resource: A reference to a document of any kind for any purpose.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A reference to a document of any kind for any purpose.
///
/// While the term "document" implies a narrow focus, for this resource a
/// "document" encompasses any serialized object with a mime-type. It includes
/// formal patient-centric documents (CDA), clinical notes, scanned paper, and
/// non-patient specific documents like policy text, as well as photos, videos,
/// or audio recordings acquired or used in healthcare. In FHIR R5 it provides
/// metadata about the document plus one or more content attachments describing
/// where and how to access the actual bytes.
///
/// Clinically and administratively, `DocumentReference` acts as an index or
/// pointer resource: rather than embedding the full document content inline,
/// it typically references externally stored bytes (via an attachment URL or
/// inline base64 data) and carries descriptive metadata such as the document
/// type, category, authoring context, subject, time period covered, and
/// security labeling. This separation allows document management systems,
/// registries, and repositories (for example, those conforming to IHE XDS or
/// similar architectures) to catalog, discover, and retrieve documents without
/// duplicating their content in the FHIR server itself. The resource also
/// supports versioning and relationships between documents, such as
/// replacement, transformation, or appending, via its `relates_to` element.
///
/// See also: [`Patient`](crate::r5::resources::patient::Patient) or other
/// resource types are commonly referenced via the `subject` element to
/// identify who the document is about, and `types::CodeableConcept` values
/// (for example LOINC codes) are used to describe the document's `type` and
/// `category`. Related resources with similar indexing purposes include
/// `Binary` (for raw content storage) and `Composition` (for structured,
/// FHIR-native clinical documents).
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::document_reference::DocumentReference;
///
/// let value = DocumentReference::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DocumentReference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReference {
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

    /// Business identifiers for the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// An explicitly assigned identifer of a variation of the content in the DocumentReference
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Procedure that caused this media to be created
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The status of this document reference (current | superseded | entered-in-error).
    pub status: crate::r5::coded::Coded<crate::r5::codes::DocumentReferenceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    pub doc_status: Option<crate::r5::coded::Coded<crate::r5::codes::CompositionStatus>>,
    /// Primitive extension sibling for [`doc_status`](Self::doc_status) (FHIR `_docStatus`).
    #[serde(rename = "_docStatus")]
    pub doc_status_ext: Option<types::Element>,

    /// Imaging modality used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modality: Vec<types::CodeableConcept>,

    /// Kind of document, ideally coded using a LOINC document type code
    pub r#type: Option<types::CodeableConcept>,

    /// Categorization of document, such as a broad class like "clinical note"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Who/what the document is about, most often a reference to a Patient
    pub subject: Option<types::Reference>,

    /// Context of the document content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<types::Reference>,

    /// Main clinical acts documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<types::CodeableReference>,

    /// Body part included
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableReference>,

    /// Kind of facility where patient was seen
    pub facility_type: Option<types::CodeableConcept>,

    /// Additional details about where the content was created (e.g. clinical specialty)
    pub practice_setting: Option<types::CodeableConcept>,

    /// Time of service that is being documented
    pub period: Option<types::Period>,

    /// When this document reference was created
    pub date: Option<types::Instant>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who and/or what authored the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Attests to accuracy of the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attester: Vec<DocumentReferenceAttester>,

    /// Organization which maintains the document
    pub custodian: Option<types::Reference>,

    /// Relationships this document has to other documents, such as replaces or transforms
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<DocumentReferenceRelatesTo>,

    /// Human-readable description
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Document security-tags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::CodeableConcept>,

    /// The document(s) referenced, each pairing an attachment with optional format/profile details
    pub content: vec1::Vec1<DocumentReferenceContent>,
}

/// Attests to accuracy of the document.
/// # Examples
///
/// ```
/// use fhir::r5::resources::document_reference::DocumentReferenceAttester;
/// use fhir::r5::types;
///
/// let value = DocumentReferenceAttester {
///     time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `time` is the name this serializes to on the wire.
/// assert_eq!(json["time"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DocumentReferenceAttester = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceAttester {
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

    /// When the document was attested
    pub time: Option<types::DateTime>,
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`).
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,

    /// Who attested the document
    pub party: Option<types::Reference>,
}

/// Relationships to other documents.
/// # Examples
///
/// ```
/// use fhir::r5::resources::document_reference::DocumentReferenceRelatesTo;
/// use fhir::r5::types;
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
pub struct DocumentReferenceRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The relationship type with another document
    pub code: types::CodeableConcept,

    /// Target of the relationship
    pub target: types::Reference,
}

/// Document referenced.
/// # Examples
///
/// ```
/// use fhir::r5::resources::document_reference::DocumentReferenceContent;
/// use fhir::r5::types;
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

    /// Content profile rules for the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<DocumentReferenceContentProfile>,
}

/// Content profile rules for the document.
/// # Examples
///
/// ```
/// use fhir::r5::resources::document_reference::DocumentReferenceContentProfile;
/// use fhir::r5::types;
///
/// let value = DocumentReferenceContentProfile {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DocumentReferenceContentProfile = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceContentProfile {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `DocumentReference.content.profile.value[x]` choice element (0..1); see [`DocumentReferenceContentProfileValue`].
    #[serde(flatten)]
    pub value: Option<DocumentReferenceContentProfileValue>,
}

/// The `DocumentReference.content.profile.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum DocumentReferenceContentProfileValue {
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `valueCanonical` variant.
    #[fhir("valueCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}
