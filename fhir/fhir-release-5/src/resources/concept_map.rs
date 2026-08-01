//! ConceptMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ConceptMap
//!
//! Version: 5.0.0
//!
//! ConceptMap Resource: A statement of relationships from one set of concepts to one or more other concepts.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
///
/// A ConceptMap describes how the concepts of one system relate to the
/// concepts of another. It is commonly used to translate codes between code
/// systems or value sets during data exchange and terminology operations such
/// as the `$translate` operation in FHIR R5.
///
/// Clinically and administratively, ConceptMap resources support semantic
/// interoperability: they let systems that use different coding schemes (for
/// example, two different lab code systems, or a local code system and a
/// national or international terminology such as LOINC or SNOMED CT)
/// exchange data without loss of meaning. Each mapping records not only the
/// source and target codes but also the nature of the relationship between
/// them (such as equivalent, narrower, or broader), plus optional conditions,
/// dependencies, and properties that refine when a mapping applies. A
/// ConceptMap is a canonical, versioned resource, so it can be published,
/// discovered, and reused across implementations in the same way as a
/// `CodeSystem` or `ValueSet`.
///
/// # See also
///
/// - `CodeSystem` and `ValueSet` — the terminology resources whose codes and
///   value sets are typically the source and target scopes of a ConceptMap.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) and
///   [`Coding`](crate::r5::types::Coding) — the data types most commonly
///   translated using the mappings defined here.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMap;
/// use fhir::r5::types;
///
/// let value = ConceptMap {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: ConceptMap = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMap {
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

    /// Canonical identifier for this concept map, represented as a URI (globally unique), used to reference this ConceptMap from other resources
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the concept map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the concept map
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ConceptMap.versionAlgorithm[x]` choice element (0..1); see [`ConceptMapVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ConceptMapVersionAlgorithm>,

    /// Name for this concept map (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this concept map (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown; the publication lifecycle status of this ConceptMap
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

    /// Natural language description of the concept map
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for concept map (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this concept map is defined
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

    /// When the ConceptMap was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the ConceptMap was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the ConceptMap is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// Who authored the ConceptMap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the ConceptMap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the ConceptMap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the ConceptMap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Additional documentation, citations, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Additional properties of the mapping
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ConceptMapProperty>,

    /// Definition of an additional attribute to act as a data source or target
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_attribute: Vec<ConceptMapAdditionalAttribute>,

    /// The `ConceptMap.sourceScope[x]` choice element (0..1); see [`ConceptMapSourceScope`].
    #[serde(flatten)]
    pub source_scope: Option<ConceptMapSourceScope>,

    /// The `ConceptMap.targetScope[x]` choice element (0..1); see [`ConceptMapTargetScope`].
    #[serde(flatten)]
    pub target_scope: Option<ConceptMapTargetScope>,

    /// Same source and target systems; the groups of element-to-target mappings that make up the body of the ConceptMap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<ConceptMapGroup>,
}

/// Additional properties of the mapping.
///
/// A property defines an attribute that can be attached to the mappings within
/// this ConceptMap, and that may be referred to in the `$translate` operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMapProperty;
/// use fhir::r5::types;
///
/// let value = ConceptMapProperty {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: ConceptMapProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies the property on the mappings, and when referred to in the $translate operation
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Formal identifier for the property
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`).
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Why the property is defined, and/or what it conveys
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Coding | string | integer | boolean | dateTime | decimal | code
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ConceptmapPropertyType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The CodeSystem from which code values come
    pub system: Option<types::Canonical>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,
}

/// Definition of an additional attribute to act as a data source or target.
///
/// Additional attributes describe data elements that can be used as sources or
/// targets for dependsOn and product relationships within group element targets.
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMapAdditionalAttribute;
/// use fhir::r5::types;
///
/// let value = ConceptMapAdditionalAttribute {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: ConceptMapAdditionalAttribute = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapAdditionalAttribute {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies this additional attribute through this resource
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Formal identifier for the data element referred to in this attribte
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`).
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Why the additional attribute is defined, and/or what the data element it refers to is
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// code | Coding | string | boolean | Quantity
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ConceptmapAttributeType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// Same source and target systems.
///
/// A group organizes mappings that share the same source and target code
/// systems, containing the individual element-to-target mappings.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::concept_map::ConceptMapGroup;
///
/// let value = ConceptMapGroup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConceptMapGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Source system where concepts to be mapped are defined
    pub source: Option<types::Canonical>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`).
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Target system that the concepts are to be mapped to
    pub target: Option<types::Canonical>,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`).
    #[serde(rename = "_target")]
    pub target_ext: Option<types::Element>,

    /// Mappings for a concept from the source set
    pub element: vec1::Vec1<ConceptMapGroupElement>,

    /// What to do when there is no mapping target for the source concept
    pub unmapped: Option<ConceptMapGroupUnmapped>,
}

/// Mappings for a concept from the source set.
///
/// Each element identifies a source concept and its associated target concepts
/// within the enclosing group.
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMapGroupElement;
/// use fhir::r5::types;
///
/// let value = ConceptMapGroupElement {
///     value_set: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `valueSet` is the name this serializes to on the wire.
/// assert_eq!(json["valueSet"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: ConceptMapGroupElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies element being mapped
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Display for the code
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Identifies the set of concepts being mapped
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// No mapping to a target concept for this source concept
    pub no_map: Option<types::Boolean>,
    /// Primitive extension sibling for [`no_map`](Self::no_map) (FHIR `_noMap`).
    #[serde(rename = "_noMap")]
    pub no_map_ext: Option<types::Element>,

    /// Concept in target system for element
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<ConceptMapGroupElementTarget>,
}

/// Concept in target system for element.
///
/// Describes a concept in the target system that the source element maps to,
/// including the relationship and any dependent or produced data elements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMapGroupElementTarget;
/// use fhir::r5::types;
///
/// let value = ConceptMapGroupElementTarget {
///     value_set: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `valueSet` is the name this serializes to on the wire.
/// assert_eq!(json["valueSet"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: ConceptMapGroupElementTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that identifies the target element
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Display for the code
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Identifies the set of target concepts
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: crate::r5::coded::Coded<crate::r5::codes::ConceptMapRelationship>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`).
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// Description of status/issues in mapping
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Property value for the source -> target mapping
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ConceptMapGroupElementTargetProperty>,

    /// Other properties required for this mapping
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ConceptMapGroupElementTargetDependsOn>,

    /// Other data elements that this mapping also produces.
    ///
    /// FHIR defines this element with `contentReference` to
    /// `ConceptMap.group.element.target.dependsOn`, so it has that backbone's
    /// shape — `attribute` plus a `value[x]` — rather than a bare `Element`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product: Vec<ConceptMapGroupElementTargetDependsOn>,
}

/// Property value for the source -> target mapping.
///
/// Attaches a value for a property (declared in `ConceptMap.property`) to a
/// specific source-to-target mapping.
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMapGroupElementTargetProperty;
/// use fhir::r5::types;
///
/// let value = ConceptMapGroupElementTargetProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConceptMapGroupElementTargetProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTargetProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to ConceptMap.property.code
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// The `ConceptMap.group.element.target.property.value[x]` choice element (0..1); see [`ConceptMapGroupElementTargetPropertyValue`].
    #[serde(flatten)]
    pub value: Option<ConceptMapGroupElementTargetPropertyValue>,
}

/// Other properties required for this mapping.
///
/// Indicates that a mapping is only valid when a referenced data element has a
/// particular value, allowing conditional or context-dependent mappings.
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMapGroupElementTargetDependsOn;
/// use fhir::r5::types;
///
/// let value = ConceptMapGroupElementTargetDependsOn {
///     value_set: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `valueSet` is the name this serializes to on the wire.
/// assert_eq!(json["valueSet"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: ConceptMapGroupElementTargetDependsOn = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTargetDependsOn {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A reference to a mapping attribute defined in ConceptMap.additionalAttribute
    pub attribute: types::Code,
    /// Primitive extension sibling for [`attribute`](Self::attribute) (FHIR `_attribute`).
    #[serde(rename = "_attribute")]
    pub attribute_ext: Option<types::Element>,

    /// The `ConceptMap.group.element.target.dependsOn.value[x]` choice element (0..1); see [`ConceptMapGroupElementTargetDependsOnValue`].
    #[serde(flatten)]
    pub value: Option<ConceptMapGroupElementTargetDependsOnValue>,

    /// The mapping depends on a data element with a value from this value set
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,
}

/// What to do when there is no mapping target for the source concept and
/// `ConceptMap.group.element.noMap` is not true.
///
/// Describes fallback behavior for unmapped source concepts, such as reusing
/// the source code, using a fixed code, or delegating to another ConceptMap.
/// # Examples
///
/// ```
/// use fhir::r5::resources::concept_map::ConceptMapGroupUnmapped;
/// use fhir::r5::types;
///
/// let value = ConceptMapGroupUnmapped {
///     value_set: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `valueSet` is the name this serializes to on the wire.
/// assert_eq!(json["valueSet"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: ConceptMapGroupUnmapped = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupUnmapped {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// use-source-code | fixed | other-map
    pub mode: crate::r5::coded::Coded<crate::r5::codes::ConceptmapUnmappedMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Fixed code when mode = fixed
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Display for the code
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Fixed code set when mode = fixed
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: Option<crate::r5::coded::Coded<crate::r5::codes::ConceptMapRelationship>>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`).
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// canonical reference to an additional ConceptMap to use for mapping if the source concept is unmapped
    pub other_map: Option<types::Canonical>,
    /// Primitive extension sibling for [`other_map`](Self::other_map) (FHIR `_otherMap`).
    #[serde(rename = "_otherMap")]
    pub other_map_ext: Option<types::Element>,
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
/// The `ConceptMap.group.element.target.dependsOn.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapGroupElementTargetDependsOnValue {
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r5::choice::Primitive<types::Code>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `ConceptMap.group.element.target.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapGroupElementTargetPropertyValue {
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r5::choice::Primitive<types::Code>),
}

/// The `ConceptMap.sourceScope[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapSourceScope {
    /// `sourceScopeUri` variant.
    #[fhir("sourceScopeUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `sourceScopeCanonical` variant.
    #[fhir("sourceScopeCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `ConceptMap.targetScope[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapTargetScope {
    /// `targetScopeUri` variant.
    #[fhir("targetScopeUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `targetScopeCanonical` variant.
    #[fhir("targetScopeCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `ConceptMap.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
