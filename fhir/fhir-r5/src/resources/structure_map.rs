//! StructureMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/StructureMap
//!
//! Version: 5.0.0
//!
//! StructureMap Resource: A Map of relationships between 2 structures that can be used to transform data.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A StructureMap describes a map of relationships between two or more
/// structures that can be used to transform instance data from one form to
/// another. It defines the rules, groups, and transforms used by a FHIR
/// mapping engine to convert a source instance into a target instance. This
/// resource is central to the FHIR Mapping Language and is commonly used in
/// implementation guides to specify structural data transformations.
///
/// A StructureMap is typically authored using the FHIR Mapping Language
/// (`.map` syntax) and then compiled into this resource form, or it can be
/// authored directly as structured content. It is most often used to convert
/// data between different versions of FHIR, between FHIR and non-FHIR formats
/// (such as HL7 v2 or CDA), or between a source implementation guide profile
/// and a target profile. Each map references one or more source
/// `StructureDefinition`s via the `structure` element, organizes its
/// transformation logic into named `group` elements, and within each group
/// defines `rule`s that read from `source` elements and write to `target`
/// elements, optionally invoking transforms or dependent rules/groups. A
/// conformant FHIR mapping engine executes the map to produce the
/// transformed output instance.
///
/// # Related resources
///
/// A StructureMap's `structure` elements point to
/// [`StructureDefinition`](crate::r5::resources::structure_definition::StructureDefinition)s
/// that describe the shape of the source and target data, and its
/// `jurisdiction` uses
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values. Maps that
/// transform clinical data commonly operate on resources such as
/// [`Patient`](crate::r5::resources::patient::Patient) or
/// [`Observation`](crate::r5::resources::observation::Observation),
/// converting instances of one profile or format into another.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::structure_map::StructureMap;
///
/// let value = StructureMap::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMap = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap {
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

    /// Canonical identifier for this structure map, represented as a URI (globally unique); used to reference the map from other resources
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the structure map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the structure map
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `StructureMap.versionAlgorithm[x]` choice element (0..1); see [`StructureMapVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<StructureMapVersionAlgorithm>,

    /// Name for this structure map (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this structure map (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication lifecycle status of the map: draft | active | retired | unknown
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

    /// Natural language description of the structure map
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for structure map (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this structure map is defined
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

    /// The source and target structure definitions used by this map, with the mode in which each is used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structure: Vec<StructureMapStructure>,

    /// Other maps used by this map (canonical URLs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import: Vec<types::Canonical>,
    /// Primitive extension sibling for [`import`](Self::import) (FHIR `_import`).
    #[serde(rename = "_import")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import_ext: Vec<Option<types::Element>>,

    /// Definition of the constant value used in the map rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#const: Vec<StructureMapConst>,

    /// The named groups of transform rules that make up the executable logic of the map
    pub group: vec1::Vec1<StructureMapGroup>,
}

/// Structure Definition used by this map.
///
/// A structure declares a StructureDefinition that participates in this map,
/// along with the mode in which it is used (source, queried, target, or
/// produced) and an optional alias for referencing the type within map rules.
/// # Examples
///
/// ```
/// use fhir::r5::resources::structure_map::StructureMapStructure;
/// use fhir::r5::types;
///
/// let value = StructureMapStructure {
///     alias: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `alias` is the name this serializes to on the wire.
/// assert_eq!(json["alias"], ::serde_json::json!("abc"));
///
/// let back: StructureMapStructure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapStructure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical reference to structure definition
    pub url: types::Canonical,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// source | queried | target | produced
    pub mode: crate::r5::coded::Coded<crate::r5::codes::MapModelMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Name for type in this map
    pub alias: Option<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`).
    #[serde(rename = "_alias")]
    pub alias_ext: Option<types::Element>,

    /// Documentation on use of structure
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Definition of the constant value used in the map rules.
///
/// A const provides a named FHIRPath expression whose evaluated value can be
/// reused throughout the map rules, allowing shared literal or computed values
/// to be referenced by name.
/// # Examples
///
/// ```
/// use fhir::r5::resources::structure_map::StructureMapConst;
/// use fhir::r5::types;
///
/// let value = StructureMapConst {
///     name: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("pat-1"));
///
/// let back: StructureMapConst = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapConst {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Constant name
    pub name: Option<types::Id>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// FHIRPath exression - value of the constant
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Named sections for reader convenience.
///
/// A group organizes a set of transform rules and their named inputs. Groups
/// may extend other groups and specify a type mode, and they define the entry
/// points invoked when the map is executed.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::structure_map::StructureMapGroup;
///
/// let value = StructureMapGroup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Human-readable label
    pub name: types::Id,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Another group that this group adds rules to
    pub extends: Option<types::Id>,
    /// Primitive extension sibling for [`extends`](Self::extends) (FHIR `_extends`).
    #[serde(rename = "_extends")]
    pub extends_ext: Option<types::Element>,

    /// types | type-and-types
    pub type_mode: Option<crate::r5::coded::Coded<crate::r5::codes::MapGroupTypeMode>>,
    /// Primitive extension sibling for [`type_mode`](Self::type_mode) (FHIR `_typeMode`).
    #[serde(rename = "_typeMode")]
    pub type_mode_ext: Option<types::Element>,

    /// Additional description/explanation for group
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Named instance provided when invoking the map
    pub input: vec1::Vec1<StructureMapGroupInput>,

    /// Transform Rule from source to target
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<StructureMapGroupRule>,
}

/// Named instance provided when invoking the map.
///
/// An input declares a named data instance passed to the group when the map is
/// invoked, together with its type and its mode (source or target).
/// # Examples
///
/// ```
/// use fhir::r5::resources::structure_map::StructureMapGroupInput;
/// use fhir::r5::types;
///
/// let value = StructureMapGroupInput {
///     r#type: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("abc"));
///
/// let back: StructureMapGroupInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name for this instance of data
    pub name: types::Id,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Type for this instance of data
    pub r#type: Option<types::String>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// source | target
    pub mode: crate::r5::coded::Coded<crate::r5::codes::MapInputMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Documentation for this instance of data
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Transform Rule from source to target.
///
/// A rule defines the mapping from one or more sources to one or more targets,
/// and may itself contain nested rules and dependent rule/group invocations.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::structure_map::StructureMapGroupRule;
///
/// let value = StructureMapGroupRule::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroupRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of the rule for internal references
    pub name: Option<types::Id>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Source inputs to the mapping
    pub source: vec1::Vec1<StructureMapGroupRuleSource>,

    /// Content to create because of this mapping rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<StructureMapGroupRuleTarget>,

    /// Rules contained in this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<StructureMapGroupRule>,

    /// Which other rules to apply in the context of this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependent: Vec<StructureMapGroupRuleDependent>,

    /// Documentation for this instance of data
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Source inputs to the mapping.
///
/// A source identifies the context and optional element that a rule reads
/// from, along with cardinality, type, list-mode, and FHIRPath conditions that
/// determine whether and how the rule applies.
/// # Examples
///
/// ```
/// use fhir::r5::resources::structure_map::StructureMapGroupRuleSource;
/// use fhir::r5::types;
///
/// let value = StructureMapGroupRuleSource {
///     r#type: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("abc"));
///
/// let back: StructureMapGroupRuleSource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleSource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type or variable this rule applies to
    pub context: types::Id,
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`).
    #[serde(rename = "_context")]
    pub context_ext: Option<types::Element>,

    /// Specified minimum cardinality
    pub min: Option<types::Integer>,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`).
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Specified maximum cardinality (number or *)
    pub max: Option<types::String>,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`).
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,

    /// Rule only applies if source has this type
    pub r#type: Option<types::String>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Default value if no value exists
    pub default_value: Option<types::String>,
    /// Primitive extension sibling for [`default_value`](Self::default_value) (FHIR `_defaultValue`).
    #[serde(rename = "_defaultValue")]
    pub default_value_ext: Option<types::Element>,

    /// Optional field for this source
    pub element: Option<types::String>,
    /// Primitive extension sibling for [`element`](Self::element) (FHIR `_element`).
    #[serde(rename = "_element")]
    pub element_ext: Option<types::Element>,

    /// first | not_first | last | not_last | only_one
    pub list_mode: Option<crate::r5::coded::Coded<crate::r5::codes::MapSourceListMode>>,
    /// Primitive extension sibling for [`list_mode`](Self::list_mode) (FHIR `_listMode`).
    #[serde(rename = "_listMode")]
    pub list_mode_ext: Option<types::Element>,

    /// Named context for field, if a field is specified
    pub variable: Option<types::Id>,
    /// Primitive extension sibling for [`variable`](Self::variable) (FHIR `_variable`).
    #[serde(rename = "_variable")]
    pub variable_ext: Option<types::Element>,

    /// FHIRPath expression  - must be true or the rule does not apply
    pub condition: Option<types::String>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`).
    #[serde(rename = "_condition")]
    pub condition_ext: Option<types::Element>,

    /// FHIRPath expression  - must be true or the mapping engine throws an error instead of completing
    pub check: Option<types::String>,
    /// Primitive extension sibling for [`check`](Self::check) (FHIR `_check`).
    #[serde(rename = "_check")]
    pub check_ext: Option<types::Element>,

    /// Message to put in log if source exists (FHIRPath)
    pub log_message: Option<types::String>,
    /// Primitive extension sibling for [`log_message`](Self::log_message) (FHIR `_logMessage`).
    #[serde(rename = "_logMessage")]
    pub log_message_ext: Option<types::Element>,
}

/// Content to create because of this mapping rule.
///
/// A target describes an element to create or populate in the output, the
/// transform to apply, its parameters, and list-handling behavior for the
/// generated content.
/// # Examples
///
/// ```
/// use fhir::r5::resources::structure_map::StructureMapGroupRuleTarget;
/// use fhir::r5::types;
///
/// let value = StructureMapGroupRuleTarget {
///     list_rule_id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `listRuleId` is the name this serializes to on the wire.
/// assert_eq!(json["listRuleId"], ::serde_json::json!("pat-1"));
///
/// let back: StructureMapGroupRuleTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Variable this rule applies to
    pub context: Option<types::String>,
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`).
    #[serde(rename = "_context")]
    pub context_ext: Option<types::Element>,

    /// Field to create in the context
    pub element: Option<types::String>,
    /// Primitive extension sibling for [`element`](Self::element) (FHIR `_element`).
    #[serde(rename = "_element")]
    pub element_ext: Option<types::Element>,

    /// Named context for field, if desired, and a field is specified
    pub variable: Option<types::Id>,
    /// Primitive extension sibling for [`variable`](Self::variable) (FHIR `_variable`).
    #[serde(rename = "_variable")]
    pub variable_ext: Option<types::Element>,

    /// first | share | last | single
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list_mode: Vec<crate::r5::coded::Coded<crate::r5::codes::MapTargetListMode>>,
    /// Primitive extension sibling for [`list_mode`](Self::list_mode) (FHIR `_listMode`).
    #[serde(rename = "_listMode")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list_mode_ext: Vec<Option<types::Element>>,

    /// Internal rule reference for shared list items
    pub list_rule_id: Option<types::Id>,
    /// Primitive extension sibling for [`list_rule_id`](Self::list_rule_id) (FHIR `_listRuleId`).
    #[serde(rename = "_listRuleId")]
    pub list_rule_id_ext: Option<types::Element>,

    /// create | copy +
    pub transform: Option<crate::r5::coded::Coded<crate::r5::codes::MapTransform>>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`).
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

    /// Parameters to the transform
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<StructureMapGroupRuleTargetParameter>,
}

/// Parameters to the transform.
///
/// A parameter supplies a value to a transform, expressed as either a variable
/// reference or a literal of one of several primitive types.
/// # Examples
///
/// ```
/// use fhir::r5::resources::structure_map::StructureMapGroupRuleTargetParameter;
/// use fhir::r5::types;
///
/// let value = StructureMapGroupRuleTargetParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: StructureMapGroupRuleTargetParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleTargetParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `StructureMap.group.rule.target.parameter.value[x]` choice element (0..1); see [`StructureMapGroupRuleTargetParameterValue`].
    #[serde(flatten)]
    pub value: Option<StructureMapGroupRuleTargetParameterValue>,
}

/// Which other rules to apply in the context of this rule.
///
/// A dependent invokes another named rule or group, passing parameters that
/// bind the current context to the invoked rule or group's inputs.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::structure_map::StructureMapGroupRuleDependent;
///
/// let value = StructureMapGroupRuleDependent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroupRuleDependent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleDependent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of a rule or group to apply
    pub name: types::Id,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Parameter to pass to the rule or group
    pub parameter: vec1::Vec1<StructureMapGroupRuleTargetParameter>,
}

/// The `StructureMap.group.rule.target.parameter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum StructureMapGroupRuleTargetParameterValue {
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r5::choice::Primitive<types::Id>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}

/// The `StructureMap.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum StructureMapVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
