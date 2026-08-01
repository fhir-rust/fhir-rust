//! ConceptMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ConceptMap
//!
//!
//!
//! A map from one set of concepts to one or more other concepts
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ConceptMap Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::concept_map::ConceptMap;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Globally unique logical id for concept map
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the concept map
    pub identifier: Option<types::Identifier>,

    /// Logical id for this version of the concept map
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Informal name for this concept map
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// draft | active | retired
    pub status: crate::coded::Coded<crate::r2::codes::ConformanceResourceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details of the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ConceptMapContact>,

    /// Date for given status
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Human language description of the concept map
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::CodeableConcept>,

    /// Why needed
    pub requirements: Option<types::String>,
    /// Primitive extension sibling for [`requirements`](Self::requirements) (FHIR `_requirements`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirements")]
    pub requirements_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::String>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Identifies the source of the concepts which are being mapped
    /// The `ConceptMap.source[x]` choice element (1..1); see [`ConceptMapSource`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub source: Option<ConceptMapSource>,

    /// Provides context to the mappings
    /// The `ConceptMap.target[x]` choice element (1..1); see [`ConceptMapTarget`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub target: Option<ConceptMapTarget>,

    /// Mappings for a concept from the source set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element: Vec<ConceptMapElement>,
}

/// Contacts to assist a user in finding and communicating with the publisher.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::concept_map::ConceptMapContact;
/// use fhir::r2::types;
///
/// let value = ConceptMapContact {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ConceptMapContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConceptMapContact {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of a individual to contact
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Contact details for individual or publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,
}

/// Mappings for an individual concept in the source to one or more concepts in
/// the target.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::concept_map::ConceptMapElement;
/// use fhir::r2::types;
///
/// let value = ConceptMapElement {
///     code_system: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `codeSystem` is the name this serializes to on the wire.
/// assert_eq!(json["codeSystem"], ::serde_json::json!("http://example.org"));
///
/// let back: ConceptMapElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConceptMapElement {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code System (if value set crosses code systems)
    pub code_system: Option<types::Uri>,
    /// Primitive extension sibling for [`code_system`](Self::code_system) (FHIR `_codeSystem`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_codeSystem")]
    pub code_system_ext: Option<types::Element>,

    /// Identifies element being mapped
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Concept in target system for element
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<ConceptMapElementTarget>,
}

/// A concept from the target value set that this concept maps to.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::concept_map::ConceptMapElementTarget;
/// use fhir::r2::types;
///
/// let value = ConceptMapElementTarget {
///     code_system: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `codeSystem` is the name this serializes to on the wire.
/// assert_eq!(json["codeSystem"], ::serde_json::json!("http://example.org"));
///
/// let back: ConceptMapElementTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConceptMapElementTarget {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// System of the target (if necessary)
    pub code_system: Option<types::Uri>,
    /// Primitive extension sibling for [`code_system`](Self::code_system) (FHIR `_codeSystem`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_codeSystem")]
    pub code_system_ext: Option<types::Element>,

    /// Code that identifies the target element
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// equivalent | equal | wider | subsumes | narrower | specializes |
    /// inexact | unmatched | disjoint
    pub equivalence: crate::coded::Coded<crate::r2::codes::ConceptMapEquivalence>,
    /// Primitive extension sibling for [`equivalence`](Self::equivalence) (FHIR `_equivalence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_equivalence")]
    pub equivalence_ext: Option<types::Element>,

    /// Description of status/issues in mapping
    pub comments: Option<types::String>,
    /// Primitive extension sibling for [`comments`](Self::comments) (FHIR `_comments`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comments")]
    pub comments_ext: Option<types::Element>,

    /// Other elements required for this mapping (from context)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ConceptMapElementTargetDependsOn>,

    /// Other concepts that this mapping also produces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product: Vec<ConceptMapElementTargetDependsOn>,
}

/// A set of additional dependencies for this mapping to hold. This mapping is
/// only applicable if the specified element can be resolved, and it has the
/// specified value.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::concept_map::ConceptMapElementTargetDependsOn;
/// use fhir::r2::types;
///
/// let value = ConceptMapElementTargetDependsOn {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ConceptMapElementTargetDependsOn = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConceptMapElementTargetDependsOn {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to element/field/ValueSet mapping depends on
    pub element: types::Uri,
    /// Primitive extension sibling for [`element`](Self::element) (FHIR `_element`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_element")]
    pub element_ext: Option<types::Element>,

    /// Code System (if necessary)
    pub code_system: types::Uri,
    /// Primitive extension sibling for [`code_system`](Self::code_system) (FHIR `_codeSystem`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_codeSystem")]
    pub code_system_ext: Option<types::Element>,

    /// Value of the referenced element
    pub code: types::String,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,
}

/// The `ConceptMap.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapSource {
    /// `sourceUri` variant.
    #[fhir("sourceUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}

/// The `ConceptMap.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapTarget {
    /// `targetUri` variant.
    #[fhir("targetUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
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
