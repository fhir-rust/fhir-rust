//! GraphDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GraphDefinition
//!
//! Version: 5.0.0
//!
//! GraphDefinition Resource: A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A formal computable definition of a graph of resources.
///
/// A GraphDefinition describes a coherent set of resources that form a graph by
/// following references from a starting node. It defines the set of nodes
/// (potential targets) and the links (rules) between them, including cardinality
/// and compartment consistency constraints. In FHIR R5 it is used by servers and
/// tooling to drive operations such as `$graph` and `$graphql`, and to document
/// and validate the shape of interconnected resource collections.
///
/// Authors typically use a GraphDefinition to formally specify which resources
/// should be retrieved together when following references from a starting
/// resource instance, for example when a client asks a server to bundle a
/// `Patient` together with related clinical resources in a single response.
/// This is especially useful for defining reusable, machine-checkable
/// "resource bundles" for exchange scenarios such as prior authorization,
/// document composition, or data extraction for analytics, without requiring
/// each consuming system to hand-code the traversal logic.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) is a common starting
///   node for many graph definitions.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) and other data
///   types referenced by nodes and links are defined in `types`.
/// - `StructureDefinition` and `CompartmentDefinition` are related artifacts
///   that a GraphDefinition may draw on when constraining node profiles and
///   compartment rules.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::graph_definition::GraphDefinition;
/// use fhir::r5::types;
///
/// let value = GraphDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: GraphDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinition {
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
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this graph definition, represented as a URI (globally unique), used to reference it from other artifacts
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the GraphDefinition (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the graph definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `GraphDefinition.versionAlgorithm[x]` choice element (0..1); see [`GraphDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<GraphDefinitionVersionAlgorithm>,

    /// Name for this graph definition (computer friendly), used as the identifier within the code
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this graph definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Publication status of this graph definition: draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the graph definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for graph definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this graph definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Id of the node that traversal of this graph starts at
    pub start: Option<types::Id>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`).
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// The set of nodes that are potential targets for links within this graph
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub node: Vec<GraphDefinitionNode>,

    /// The rules about how references between resources are followed to build this graph
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<GraphDefinitionLink>,
}

/// Potential target for the link.
///
/// A node identifies a resource type (and optional profile) that may be a target
/// within the graph, referenced by links via its internal `node_id`.
/// # Examples
///
/// ```
/// use fhir::r5::resources::graph_definition::GraphDefinitionNode;
/// use fhir::r5::types;
///
/// let value = GraphDefinitionNode {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: GraphDefinitionNode = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionNode {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Internal ID - target for link references
    pub node_id: types::Id,
    /// Primitive extension sibling for [`node_id`](Self::node_id) (FHIR `_nodeId`).
    #[serde(rename = "_nodeId")]
    pub node_id_ext: Option<types::Element>,

    /// Why this node is specified
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Type of resource this link refers to
    pub r#type: types::Code,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Profile for the target resource
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,
}

/// Links this graph makes rules about.
///
/// A link connects a source node to a target node, optionally along a path in the
/// resource, and expresses cardinality and compartment consistency rules between
/// the connected resources.
/// # Examples
///
/// ```
/// use fhir::r5::resources::graph_definition::GraphDefinitionLink;
/// use fhir::r5::types;
///
/// let value = GraphDefinitionLink {
///     slice_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sliceName` is the name this serializes to on the wire.
/// assert_eq!(json["sliceName"], ::serde_json::json!("abc"));
///
/// let back: GraphDefinitionLink = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Why this link is specified
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Minimum occurrences for this link
    pub min: Option<types::Integer>,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`).
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Maximum occurrences for this link
    pub max: Option<types::String>,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`).
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,

    /// Source Node for this link
    pub source_id: types::Id,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`).
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,

    /// Path in the resource that contains the link
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Which slice (if profiled)
    pub slice_name: Option<types::String>,
    /// Primitive extension sibling for [`slice_name`](Self::slice_name) (FHIR `_sliceName`).
    #[serde(rename = "_sliceName")]
    pub slice_name_ext: Option<types::Element>,

    /// Target Node for this link
    pub target_id: types::Id,
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`).
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// Criteria for reverse lookup
    pub params: Option<types::String>,
    /// Primitive extension sibling for [`params`](Self::params) (FHIR `_params`).
    #[serde(rename = "_params")]
    pub params_ext: Option<types::Element>,

    /// Compartment Consistency Rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compartment: Vec<GraphDefinitionLinkCompartment>,
}

/// Compartment Consistency Rules.
///
/// A compartment rule constrains how a compartment (such as Patient or Encounter)
/// must relate across the linked resources, using a rule and optional FHIRPath
/// expression.
/// # Examples
///
/// ```
/// use fhir::r5::resources::graph_definition::GraphDefinitionLinkCompartment;
/// use fhir::r5::types;
///
/// let value = GraphDefinitionLinkCompartment {
///     expression: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `expression` is the name this serializes to on the wire.
/// assert_eq!(json["expression"], ::serde_json::json!("abc"));
///
/// let back: GraphDefinitionLinkCompartment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLinkCompartment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// where | requires
    pub r#use: crate::r5::coded::Coded<crate::r5::codes::GraphCompartmentUse>,
    /// Primitive extension sibling for [`use`](Self::r#use) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// identical | matching | different | custom
    pub rule: crate::r5::coded::Coded<crate::r5::codes::GraphCompartmentRule>,
    /// Primitive extension sibling for [`rule`](Self::rule) (FHIR `_rule`).
    #[serde(rename = "_rule")]
    pub rule_ext: Option<types::Element>,

    /// Patient | Encounter | RelatedPerson | Practitioner | Device | EpisodeOfCare
    pub code: crate::r5::coded::Coded<crate::r5::codes::CompartmentType>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Custom rule, as a FHIRPath expression
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`).
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// Documentation for FHIRPath expression
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = GraphDefinition;

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
/// The `GraphDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum GraphDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
