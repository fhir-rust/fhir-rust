//! StructureDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/StructureDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! Structural Definition
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A definition of a FHIR structure. This resource is used to describe the
/// underlying resources, data types defined in FHIR, and also for describing
/// extensions and constraints on resources and data types.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_definition::StructureDefinition;
/// use fhir::r6::types;
///
/// let value = StructureDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: StructureDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "StructureDefinitionDe")]
#[fhir_version("r6")]
pub struct StructureDefinition {
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

    /// Canonical identifier for this structure definition, represented as a
    /// URI (globally unique)
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the structure definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the structure definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `StructureDefinition.versionAlgorithm[x]` choice element (0..1); see [`StructureDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<StructureDefinitionVersionAlgorithm>,

    /// Name for this structure definition (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this structure definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the structure definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for structure definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this structure definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Assist with indexing and finding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<types::Coding>,

    /// FHIR Version this StructureDefinition targets
    pub fhir_version: Option<crate::coded::Coded<crate::r6::codes::FhirVersion>>,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// External specification that the content is mapped to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mapping: Vec<StructureDefinitionMapping>,

    /// primitive-type | complex-type | resource | logical
    pub kind: crate::coded::Coded<crate::r6::codes::StructureDefinitionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Whether the structure is abstract
    pub r#abstract: types::Boolean,
    /// Primitive extension sibling for [`r#abstract`](Self::r#abstract) (FHIR `_abstract`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_abstract")]
    pub abstract_ext: Option<types::Element>,

    /// If an extension, where it can be used in instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<StructureDefinitionContext>,

    /// FHIRPath invariants - when the extension can be used
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub context_invariant: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`context_invariant`](Self::context_invariant) (FHIR `_contextInvariant`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextInvariant")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_invariant_ext: Vec<Option<types::Element>>,

    /// Type defined or constrained by this structure
    pub r#type: types::Uri,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Definition that this type is constrained/specialized from
    pub base_definition: Option<types::Canonical>,
    /// Primitive extension sibling for [`base_definition`](Self::base_definition) (FHIR `_baseDefinition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_baseDefinition")]
    pub base_definition_ext: Option<types::Element>,

    /// specialization | constraint - How relates to base definition
    pub derivation: Option<crate::coded::Coded<crate::r6::codes::TypeDerivationRule>>,
    /// Primitive extension sibling for [`derivation`](Self::derivation) (FHIR `_derivation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivation")]
    pub derivation_ext: Option<types::Element>,

    /// Snapshot view of the structure
    pub snapshot: Option<StructureDefinitionSnapshot>,

    /// Differential view of the structure
    pub differential: Option<StructureDefinitionDifferential>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureDefinitionDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: types::Uri,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r6::choice::Slot<StructureDefinitionVersionAlgorithm>,
    name: types::String,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    #[serde(default)]
    keyword: Vec<types::Coding>,
    fhir_version: Option<crate::coded::Coded<crate::r6::codes::FhirVersion>>,
    #[serde(rename = "_fhirVersion")]
    fhir_version_ext: Option<types::Element>,
    #[serde(default)]
    mapping: Vec<StructureDefinitionMapping>,
    kind: crate::coded::Coded<crate::r6::codes::StructureDefinitionKind>,
    #[serde(rename = "_kind")]
    kind_ext: Option<types::Element>,
    r#abstract: types::Boolean,
    #[serde(rename = "_abstract")]
    abstract_ext: Option<types::Element>,
    #[serde(default)]
    context: Vec<StructureDefinitionContext>,
    #[serde(default)]
    context_invariant: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_contextInvariant")]
    #[serde(default)]
    context_invariant_ext: Vec<Option<types::Element>>,
    r#type: types::Uri,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    base_definition: Option<types::Canonical>,
    #[serde(rename = "_baseDefinition")]
    base_definition_ext: Option<types::Element>,
    derivation: Option<crate::coded::Coded<crate::r6::codes::TypeDerivationRule>>,
    #[serde(rename = "_derivation")]
    derivation_ext: Option<types::Element>,
    snapshot: Option<StructureDefinitionSnapshot>,
    differential: Option<StructureDefinitionDifferential>,
}

impl ::core::convert::From<StructureDefinitionDe> for StructureDefinition {
    fn from(v: StructureDefinitionDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            keyword: v.keyword,
            fhir_version: v.fhir_version,
            fhir_version_ext: v.fhir_version_ext,
            mapping: v.mapping,
            kind: v.kind,
            kind_ext: v.kind_ext,
            r#abstract: v.r#abstract,
            abstract_ext: v.abstract_ext,
            context: v.context,
            context_invariant: v.context_invariant,
            context_invariant_ext: v.context_invariant_ext,
            r#type: v.r#type,
            type_ext: v.type_ext,
            base_definition: v.base_definition,
            base_definition_ext: v.base_definition_ext,
            derivation: v.derivation,
            derivation_ext: v.derivation_ext,
            snapshot: v.snapshot,
            differential: v.differential,
        }
    }
}

/// Identifies the types of resource or data type elements to which the
/// extension can be applied. For more guidance on using the 'context' element,
/// see the [defining extensions page](defining-extensions.html#context).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_definition::StructureDefinitionContext;
/// use fhir::r6::types;
///
/// let value = StructureDefinitionContext {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: StructureDefinitionContext = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct StructureDefinitionContext {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// fhirpath | element | extension
    pub r#type: crate::coded::Coded<crate::r6::codes::ExtensionContextType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Where the extension can be used in instances
    pub expression: types::String,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,
}

/// A differential view is expressed relative to the base StructureDefinition -
/// a statement of differences that it applies.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::structure_definition::StructureDefinitionDifferential;
///
/// let value = StructureDefinitionDifferential::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureDefinitionDifferential = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct StructureDefinitionDifferential {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: ::vec1::Vec1<types::ElementDefinition>,
}

/// An external specification that the content is mapped to.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_definition::StructureDefinitionMapping;
/// use fhir::r6::types;
///
/// let value = StructureDefinitionMapping {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: StructureDefinitionMapping = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct StructureDefinitionMapping {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Internal id when this mapping is used
    pub identity: types::Id,
    /// Primitive extension sibling for [`identity`](Self::identity) (FHIR `_identity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_identity")]
    pub identity_ext: Option<types::Element>,

    /// Identifies what this mapping refers to
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Names what this mapping refers to
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Versions, Issues, Scope limitations etc
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// A snapshot view is expressed in a standalone form that can be used and
/// interpreted without considering the base StructureDefinition.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::structure_definition::StructureDefinitionSnapshot;
///
/// let value = StructureDefinitionSnapshot::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureDefinitionSnapshot = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct StructureDefinitionSnapshot {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: ::vec1::Vec1<types::ElementDefinition>,
}

/// The `StructureDefinition.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum StructureDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = StructureDefinition;

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
