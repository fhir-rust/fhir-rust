//! StructureDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/StructureDefinition
//!
//!
//!
//! Structural Definition
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for StructureDefinition Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::structure_definition::StructureDefinition;
/// use fhir::r3::types;
///
/// let value = StructureDefinition {
///     fhir_version: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `fhirVersion` is the name this serializes to on the wire.
/// assert_eq!(json["fhirVersion"], ::serde_json::json!("pat-1"));
///
/// let back: StructureDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct StructureDefinition {
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

    /// Logical URI to reference this structure definition (globally unique)
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
    pub status: crate::coded::Coded<crate::r3::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date this was last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
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

    /// Context the content is intended to support
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

    /// Assist with indexing and finding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword: Vec<types::Coding>,

    /// FHIR Version this StructureDefinition targets
    pub fhir_version: Option<types::Id>,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// External specification that the content is mapped to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mapping: Vec<StructureDefinitionMapping>,

    /// primitive-type | complex-type | resource | logical
    pub kind: crate::coded::Coded<crate::r3::codes::StructureDefinitionKind>,
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

    /// resource | datatype | extension
    pub context_type: Option<crate::coded::Coded<crate::r3::codes::ExtensionContext>>,
    /// Primitive extension sibling for [`context_type`](Self::context_type) (FHIR `_contextType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextType")]
    pub context_type_ext: Option<types::Element>,

    /// Where the extension can be used in instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<types::String>,
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_context")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_ext: Vec<Option<types::Element>>,

    /// FHIRPath invariants - when the extension can be used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_invariant: Vec<types::String>,
    /// Primitive extension sibling for [`context_invariant`](Self::context_invariant) (FHIR `_contextInvariant`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextInvariant")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_invariant_ext: Vec<Option<types::Element>>,

    /// Type defined or constrained by this structure
    pub r#type: types::Code,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Definition that this type is constrained/specialized from
    pub base_definition: Option<types::Uri>,
    /// Primitive extension sibling for [`base_definition`](Self::base_definition) (FHIR `_baseDefinition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_baseDefinition")]
    pub base_definition_ext: Option<types::Element>,

    /// specialization | constraint - How relates to base definition
    pub derivation: Option<crate::coded::Coded<crate::r3::codes::TypeDerivationRule>>,
    /// Primitive extension sibling for [`derivation`](Self::derivation) (FHIR `_derivation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivation")]
    pub derivation_ext: Option<types::Element>,

    /// Snapshot view of the structure
    pub snapshot: Option<StructureDefinitionSnapshot>,

    /// Differential view of the structure
    pub differential: Option<StructureDefinitionDifferential>,
}

/// A differential view is expressed relative to the base StructureDefinition -
/// a statement of differences that it applies.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::structure_definition::StructureDefinitionDifferential;
///
/// let value = StructureDefinitionDifferential::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureDefinitionDifferential = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct StructureDefinitionDifferential {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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
/// use fhir::r3::resources::structure_definition::StructureDefinitionMapping;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct StructureDefinitionMapping {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

    /// Versions, Issues, Scope limitations etc.
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// A snapshot view is expressed in a stand alone form that can be used and
/// interpreted without considering the base StructureDefinition.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::structure_definition::StructureDefinitionSnapshot;
///
/// let value = StructureDefinitionSnapshot::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureDefinitionSnapshot = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct StructureDefinitionSnapshot {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: ::vec1::Vec1<types::ElementDefinition>,
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
