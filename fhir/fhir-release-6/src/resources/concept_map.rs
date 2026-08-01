//! ConceptMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ConceptMap
//!
//! Version: 6.0.0-ballot3
//!
//! A map from one set of concepts to one or more other concepts
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMap;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConceptMap {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this concept map, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the concept map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the concept map
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `ConceptMap.versionAlgorithm[x]` choice element (0..1); see [`ConceptMapVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ConceptMapVersionAlgorithm>,

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

    /// Natural language description of the concept map
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// When the ConceptMap was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the ConceptMap was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// The source value set that contains the concepts that are being mapped
    /// The `ConceptMap.sourceScope[x]` choice element (0..1); see [`ConceptMapSourceScope`].
    #[serde(flatten)]
    pub source_scope: Option<ConceptMapSourceScope>,

    /// The target value set which provides context for the mappings
    /// The `ConceptMap.targetScope[x]` choice element (0..1); see [`ConceptMapTargetScope`].
    #[serde(flatten)]
    pub target_scope: Option<ConceptMapTargetScope>,

    /// Same source and target systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<ConceptMapGroup>,
}

/// An additionalAttribute defines an additional data element found in the
/// source or target data model where the data will come from or be mapped to.
/// Some mappings are based on data in addition to the source data element,
/// where codes in multiple fields are combined to a single field (or vice
/// versa).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMapAdditionalAttribute;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Formal identifier for the data element referred to in this attribte
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Why the additional attribute is defined, and/or what the data element
    /// it refers to is
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// code | Coding | string | boolean | Quantity
    pub r#type: crate::coded::Coded<crate::r6::codes::ConceptmapAttributeType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// A group of mappings that all have the same source and target system.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::concept_map::ConceptMapGroup;
///
/// let value = ConceptMapGroup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConceptMapGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Target system that the concepts are to be mapped to
    pub target: Option<types::Canonical>,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_target")]
    pub target_ext: Option<types::Element>,

    /// Mappings for a concept from the source set
    pub element: ::vec1::Vec1<ConceptMapGroupElement>,

    /// What to do when there is no mapping target for the source concept and
    /// ConceptMap.group.element.noMap is not true
    pub unmapped: Option<ConceptMapGroupUnmapped>,
}

/// Mappings for an individual concept in the source to one or more concepts in
/// the target.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMapGroupElement;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Identifies the set of concepts being mapped
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// No mapping to a target concept for this source concept
    pub no_map: Option<types::Boolean>,
    /// Primitive extension sibling for [`no_map`](Self::no_map) (FHIR `_noMap`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noMap")]
    pub no_map_ext: Option<types::Element>,

    /// Concept in target system for element
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<ConceptMapGroupElementTarget>,
}

/// A concept from the target value set that this concept maps to.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMapGroupElementTarget;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Identifies the set of target concepts
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// related-to | equivalent | source-is-narrower-than-target |
    /// source-is-broader-than-target | not-related-to
    pub relationship: crate::coded::Coded<crate::r6::codes::ConceptMapRelationship>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// Description of status/issues in mapping
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Property value for the source -> target mapping
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ConceptMapGroupElementTargetProperty>,

    /// Other properties required for this mapping
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ConceptMapGroupElementTargetDependsOn>,

    /// Other data elements that this mapping also produces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product: Vec<ConceptMapGroupElementTargetDependsOn>,
}

/// A set of additional dependencies for this mapping to hold. This mapping is
/// only applicable if the specified data attribute can be resolved, and it has
/// the specified value.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMapGroupElementTargetDependsOn;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ConceptMapGroupElementTargetDependsOn {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A reference to a mapping attribute defined in
    /// ConceptMap.additionalAttribute
    pub attribute: types::Code,
    /// Primitive extension sibling for [`attribute`](Self::attribute) (FHIR `_attribute`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_attribute")]
    pub attribute_ext: Option<types::Element>,

    /// Value of the referenced data element
    /// The `ConceptMap.group.element.target.dependsOn.value[x]` choice element (0..1); see [`ConceptMapGroupElementTargetDependsOnValue`].
    #[serde(flatten)]
    pub value: Option<ConceptMapGroupElementTargetDependsOnValue>,

    /// The mapping depends on a data element with a value from this value set
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,
}

/// A property value for this source -> target mapping.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMapGroupElementTargetProperty;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Value of the property for this concept
    /// The `ConceptMap.group.element.target.property.value[x]` choice element (1..1); see [`ConceptMapGroupElementTargetPropertyValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<ConceptMapGroupElementTargetPropertyValue>,
}

/// What to do when there is no mapping to a target concept from the source
/// concept and ConceptMap.group.element.noMap is not true. This provides the
/// "default" to be applied when there is no target concept mapping specified
/// or the expansion of ConceptMap.group.element.target.valueSet is empty.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMapGroupUnmapped;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub mode: crate::coded::Coded<crate::r6::codes::ConceptmapUnmappedMode>,
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

    /// Fixed code set when mode = fixed
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// related-to | equivalent | source-is-narrower-than-target |
    /// source-is-broader-than-target | not-related-to
    pub relationship: Option<crate::coded::Coded<crate::r6::codes::ConceptMapRelationship>>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// canonical reference to an additional ConceptMap to use for mapping if
    /// the source concept is unmapped
    pub other_map: Option<types::Canonical>,
    /// Primitive extension sibling for [`other_map`](Self::other_map) (FHIR `_otherMap`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_otherMap")]
    pub other_map_ext: Option<types::Element>,
}

/// A property defines a slot through which additional information can be
/// provided about a map from source -> target.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::concept_map::ConceptMapProperty;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ConceptMapProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies the property on the mappings, and when referred to in the
    /// $translate operation
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Formal identifier for the property
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Why the property is defined, and/or what it conveys
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Coding | string | integer | boolean | dateTime | decimal | code
    pub r#type: crate::coded::Coded<crate::r6::codes::ConceptmapPropertyType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The CodeSystem from which code values come
    pub system: Option<types::Canonical>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,
}

/// The `ConceptMap.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `ConceptMap.sourceScope[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapSourceScope {
    /// `sourceScopeUri` variant.
    #[fhir("sourceScopeUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `sourceScopeCanonical` variant.
    #[fhir("sourceScopeCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
}

/// The `ConceptMap.targetScope[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapTargetScope {
    /// `targetScopeUri` variant.
    #[fhir("targetScopeUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `targetScopeCanonical` variant.
    #[fhir("targetScopeCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
}

/// The `ConceptMap.group.element.target.dependsOn.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapGroupElementTargetDependsOnValue {
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `ConceptMap.group.element.target.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConceptMapGroupElementTargetPropertyValue {
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
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
