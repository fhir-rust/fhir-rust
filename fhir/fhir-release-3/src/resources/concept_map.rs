//! ConceptMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ConceptMap
//!
//!
//!
//! A map from one set of concepts to one or more other concepts
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ConceptMap Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::concept_map::ConceptMap;
/// use fhir::r3::types;
///
/// let value = ConceptMap {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: ConceptMap = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConceptMap {
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

    /// Logical URI to reference this concept map (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the concept map
    pub identifier: Option<types::Identifier>,

    /// Business version of the concept map
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this concept map (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this concept map (human friendly)
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

    /// Natural language description of the concept map
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for concept map (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this concept map is defined
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

    /// Identifies the source of the concepts which are being mapped
    /// The `ConceptMap.source[x]` choice element (0..1); see [`ConceptMapSource`].
    #[serde(flatten)]
    pub source: Option<ConceptMapSource>,

    /// Provides context to the mappings
    /// The `ConceptMap.target[x]` choice element (0..1); see [`ConceptMapTarget`].
    #[serde(flatten)]
    pub target: Option<ConceptMapTarget>,

    /// Same source and target systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<ConceptMapGroup>,
}

/// A group of mappings that all have the same source and target system.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::concept_map::ConceptMapGroup;
///
/// let value = ConceptMapGroup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConceptMapGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConceptMapGroup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code System (if value set crosses code systems)
    pub source: Option<types::Uri>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Specific version of the code system
    pub source_version: Option<types::String>,
    /// Primitive extension sibling for [`source_version`](Self::source_version) (FHIR `_sourceVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sourceVersion")]
    pub source_version_ext: Option<types::Element>,

    /// System of the target (if necessary)
    pub target: Option<types::Uri>,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_target")]
    pub target_ext: Option<types::Element>,

    /// Specific version of the code system
    pub target_version: Option<types::String>,
    /// Primitive extension sibling for [`target_version`](Self::target_version) (FHIR `_targetVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetVersion")]
    pub target_version_ext: Option<types::Element>,

    /// Mappings for a concept from the source set
    pub element: ::vec1::Vec1<ConceptMapGroupElement>,

    /// When no match in the mappings
    pub unmapped: Option<ConceptMapGroupUnmapped>,
}

/// Mappings for an individual concept in the source to one or more concepts in
/// the target.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::concept_map::ConceptMapGroupElement;
/// use fhir::r3::types;
///
/// let value = ConceptMapGroupElement {
///     code: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `code` is the name this serializes to on the wire.
/// assert_eq!(json["code"], ::serde_json::json!("final"));
///
/// let back: ConceptMapGroupElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConceptMapGroupElement {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies element being mapped
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Display for the code
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Concept in target system for element
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<ConceptMapGroupElementTarget>,
}

/// A concept from the target value set that this concept maps to.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::concept_map::ConceptMapGroupElementTarget;
/// use fhir::r3::types;
///
/// let value = ConceptMapGroupElementTarget {
///     code: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `code` is the name this serializes to on the wire.
/// assert_eq!(json["code"], ::serde_json::json!("final"));
///
/// let back: ConceptMapGroupElementTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConceptMapGroupElementTarget {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that identifies the target element
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Display for the code
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// relatedto | equivalent | equal | wider | subsumes | narrower |
    /// specializes | inexact | unmatched | disjoint
    pub equivalence: Option<crate::coded::Coded<crate::r3::codes::ConceptMapEquivalence>>,
    /// Primitive extension sibling for [`equivalence`](Self::equivalence) (FHIR `_equivalence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_equivalence")]
    pub equivalence_ext: Option<types::Element>,

    /// Description of status/issues in mapping
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Other elements required for this mapping (from context)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ConceptMapGroupElementTargetDependsOn>,

    /// Other concepts that this mapping also produces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product: Vec<ConceptMapGroupElementTargetDependsOn>,
}

/// A set of additional dependencies for this mapping to hold. This mapping is
/// only applicable if the specified element can be resolved, and it has the
/// specified value.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::concept_map::ConceptMapGroupElementTargetDependsOn;
/// use fhir::r3::types;
///
/// let value = ConceptMapGroupElementTargetDependsOn {
///     system: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `system` is the name this serializes to on the wire.
/// assert_eq!(json["system"], ::serde_json::json!("http://example.org"));
///
/// let back: ConceptMapGroupElementTargetDependsOn = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConceptMapGroupElementTargetDependsOn {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to property mapping depends on
    pub property: types::Uri,
    /// Primitive extension sibling for [`property`](Self::property) (FHIR `_property`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_property")]
    pub property_ext: Option<types::Element>,

    /// Code System (if necessary)
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Value of the referenced element
    pub code: types::String,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Display for the code
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,
}

/// What to do when there is no match in the mappings in the group.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::concept_map::ConceptMapGroupUnmapped;
/// use fhir::r3::types;
///
/// let value = ConceptMapGroupUnmapped {
///     code: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `code` is the name this serializes to on the wire.
/// assert_eq!(json["code"], ::serde_json::json!("final"));
///
/// let back: ConceptMapGroupUnmapped = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ConceptMapGroupUnmapped {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// provided | fixed | other-map
    pub mode: crate::coded::Coded<crate::r3::codes::ConceptmapUnmappedMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Fixed code when mode = fixed
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Display for the code
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Canonical URL for other concept map
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// The `ConceptMap.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapSource {
    /// `sourceUri` variant.
    #[fhir("sourceUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}

/// The `ConceptMap.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapTarget {
    /// `targetUri` variant.
    #[fhir("targetUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `targetReference` variant.
    #[fhir("targetReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ConceptMap;

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
