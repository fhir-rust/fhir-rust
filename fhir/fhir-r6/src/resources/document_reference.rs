//! DocumentReference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DocumentReference
//!
//! Version: 6.0.0-ballot3
//!
//! A reference to a document
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A reference to a document of any kind for any purpose. While the term
/// “document” implies a more narrow focus, for this resource this “document”
/// encompasses *any* serialized object with a mime-type, it includes formal
/// patient-centric documents (CDA), clinical notes, scanned paper, non-patient
/// specific documents like policy text, as well as a photo, video, or audio
/// recording acquired or used in healthcare. The DocumentReference resource
/// provides metadata about the document so that the document can be discovered
/// and managed. The actual content may be inline base64 encoded data or
/// provided by direct reference.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::document_reference::DocumentReference;
///
/// let value = DocumentReference::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DocumentReference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifiers for the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// An explicitly assigned identifier of a variation of the content in the
    /// DocumentReference
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Procedure that caused this media to be created
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// current | superseded | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::DocumentReferenceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// registered | partial | preliminary | final | amended | corrected |
    /// appended | cancelled | entered-in-error | deprecated | unknown
    pub doc_status: Option<crate::coded::Coded<crate::r6::codes::CompositionStatus>>,
    /// Primitive extension sibling for [`doc_status`](Self::doc_status) (FHIR `_docStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_docStatus")]
    pub doc_status_ext: Option<types::Element>,

    /// Imaging modality used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modality: Vec<types::CodeableConcept>,

    /// Kind of document (LOINC if possible)
    pub r#type: Option<types::CodeableConcept>,

    /// Categorization of document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Who/what is the subject of the document
    pub subject: Option<types::Reference>,

    /// Encounter the document reference is part of
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<types::Reference>,

    /// Main clinical acts documented
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<types::CodeableReference>,

    /// Related identifiers or resources associated with the document reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<types::Reference>,

    /// Body part included
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableReference>,

    /// Kind of facility where patient was seen
    pub facility_type: Option<types::CodeableConcept>,

    /// Additional details about where the content was created (e.g. clinical
    /// specialty)
    pub practice_setting: Option<types::CodeableConcept>,

    /// Time of service that is being documented
    pub period: Option<types::Period>,

    /// When this document reference was created
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who and/or what authored the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Attests to accuracy of the document
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attester: Vec<DocumentReferenceAttester>,

    /// Organization which maintains the document
    pub custodian: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Relationships to other documents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relates_to: Vec<DocumentReferenceRelatesTo>,

    /// Human-readable description
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Document security-tags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::CodeableConcept>,

    /// Document referenced
    pub content: ::vec1::Vec1<DocumentReferenceContent>,
}

/// A participant who has authenticated the accuracy of the document.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::document_reference::DocumentReferenceAttester;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,

    /// Who attested the document
    pub party: Option<types::Reference>,
}

/// The document and format referenced. If there are multiple content element
/// repetitions, these must all represent the same document in different
/// format, or attachment metadata.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::document_reference::DocumentReferenceContent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// An identifier of the document constraints, encoding, structure, and
/// template that the document conforms to beyond the base format indicated in
/// the mimeType.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::document_reference::DocumentReferenceContentProfile;
/// use fhir::r6::types;
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
#[serde(from = "DocumentReferenceContentProfileDe")]
#[fhir_version("r6")]
pub struct DocumentReferenceContentProfile {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code|uri|canonical
    /// The `DocumentReference.content.profile.value[x]` choice element (1..1); see [`DocumentReferenceContentProfileValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<DocumentReferenceContentProfileValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentReferenceContentProfileDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<DocumentReferenceContentProfileValue>,
}

impl ::core::convert::From<DocumentReferenceContentProfileDe> for DocumentReferenceContentProfile {
    fn from(v: DocumentReferenceContentProfileDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
        }
    }
}

/// Relationships that this document has with other document references that
/// already exist.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::document_reference::DocumentReferenceRelatesTo;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub target: types::Reference<crate::r6::resources::DocumentReference>,
}

/// The `DocumentReference.content.profile.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DocumentReferenceContentProfileValue {
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `valueCanonical` variant.
    #[fhir("valueCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
}
