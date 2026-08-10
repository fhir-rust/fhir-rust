//! Composition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Composition
//!
//! Version: 4.0.1
//!
//! A set of resources composed into a single coherent clinical statement with
//! clinical attestation
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
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
/// use fhir::r4::resources::composition::Composition;
///
/// let value = Composition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Composition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Version-independent identifier for the Composition
    pub identifier: Option<types::Identifier>,

    /// preliminary | final | amended | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::CompositionStatus>,
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
    pub subject: Option<types::Reference>,

    /// Context of the Composition
    pub encounter: Option<types::Reference<crate::r4::resources::Encounter>>,

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
    pub custodian: Option<types::Reference<crate::r4::resources::Organization>>,

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
/// use fhir::r4::resources::composition::CompositionAttester;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub mode: crate::coded::Coded<crate::r4::codes::CompositionAttestationMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

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
/// use fhir::r4::resources::composition::CompositionEvent;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct CompositionEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
/// use fhir::r4::resources::composition::CompositionRelatesTo;
/// use fhir::r4::types;
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
#[serde(from = "CompositionRelatesToDe")]
#[fhir_version("r4")]
pub struct CompositionRelatesTo {
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
    /// The `Composition.relatesTo.target[x]` choice element (1..1); see [`CompositionRelatesToTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<CompositionRelatesToTarget>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompositionRelatesToDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: crate::coded::Coded<crate::r4::codes::DocumentRelationshipType>,
    #[serde(rename = "_code")]
    code_ext: Option<types::Element>,
    #[serde(flatten)]
    target: crate::r4::choice::Slot<CompositionRelatesToTarget>,
}

impl ::core::convert::From<CompositionRelatesToDe> for CompositionRelatesTo {
    fn from(v: CompositionRelatesToDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            code_ext: v.code_ext,
            target: v.target.0,
        }
    }
}

/// The root of the sections that make up the composition.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::composition::CompositionSection;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

    /// Who and/or what authored the section
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Who/what the section is about, when it is not about the subject of
    /// composition
    pub focus: Option<types::Reference>,

    /// Text summary of the section, for human interpretation
    pub text: Option<types::Narrative>,

    /// working | snapshot | changes
    pub mode: Option<crate::coded::Coded<crate::r4::codes::ListMode>>,
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
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum CompositionRelatesToTarget {
    /// `targetIdentifier` variant.
    #[fhir("targetIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `targetReference` variant.
    #[fhir("targetReference")]
    Reference(Box<types::Reference>),
}
