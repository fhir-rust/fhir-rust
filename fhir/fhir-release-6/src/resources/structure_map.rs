//! StructureMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/StructureMap
//!
//! Version: 6.0.0-ballot3
//!
//! A Map of relationships between 2 structures that can be used to transform
//! data
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A Map of relationships between 2 structures that can be used to transform
/// data.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::structure_map::StructureMap;
///
/// let value = StructureMap::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMap = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct StructureMap {
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

    /// Canonical identifier for this structure map, represented as a URI
    /// (globally unique)
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the structure map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the structure map
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `StructureMap.versionAlgorithm[x]` choice element (0..1); see [`StructureMapVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<StructureMapVersionAlgorithm>,

    /// Name for this structure map (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this structure map (human friendly)
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

    /// Natural language description of the structure map
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// Structure Definition used by this map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structure: Vec<StructureMapStructure>,

    /// Other maps used by this map (canonical URLs)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import: Vec<types::Canonical>,
    /// Primitive extension sibling for [`import`](Self::import) (FHIR `_import`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_import")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import_ext: Vec<Option<types::Element>>,

    /// Definition of the constant value used in the map rules
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#const: Vec<StructureMapConst>,

    /// Named sections for reader convenience
    pub group: ::vec1::Vec1<StructureMapGroup>,
}

/// Definition of a constant value used in the map rules.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_map::StructureMapConst;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// FHIRPath exression - value of the constant
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Organizes the mapping into managable chunks for human review/ease of
/// maintenance.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::structure_map::StructureMapGroup;
///
/// let value = StructureMapGroup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Another group that this group adds rules to
    pub extends: Option<types::Id>,
    /// Primitive extension sibling for [`extends`](Self::extends) (FHIR `_extends`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_extends")]
    pub extends_ext: Option<types::Element>,

    /// types | type-and-types
    pub type_mode: Option<crate::coded::Coded<crate::r6::codes::MapGroupTypeMode>>,
    /// Primitive extension sibling for [`type_mode`](Self::type_mode) (FHIR `_typeMode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_typeMode")]
    pub type_mode_ext: Option<types::Element>,

    /// Additional description/explanation for group
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Named instance provided when invoking the map
    pub input: ::vec1::Vec1<StructureMapGroupInput>,

    /// Transform Rule from source to target
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<StructureMapGroupRule>,
}

/// A name assigned to an instance of data. The instance must be provided when
/// the mapping is invoked.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_map::StructureMapGroupInput;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Type for this instance of data
    pub r#type: Option<types::String>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// source | target
    pub mode: crate::coded::Coded<crate::r6::codes::MapInputMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Documentation for this instance of data
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Transform Rule from source to target.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::structure_map::StructureMapGroupRule;
///
/// let value = StructureMapGroupRule::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroupRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Source inputs to the mapping
    pub source: ::vec1::Vec1<StructureMapGroupRuleSource>,

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
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Which other rules to apply in the context of this rule.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::structure_map::StructureMapGroupRuleDependent;
///
/// let value = StructureMapGroupRuleDependent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroupRuleDependent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Parameter to pass to the rule or group
    pub parameter: ::vec1::Vec1<StructureMapGroupRuleTargetParameter>,
}

/// Source inputs to the mapping.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_map::StructureMapGroupRuleSource;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_context")]
    pub context_ext: Option<types::Element>,

    /// Specified minimum cardinality
    pub min: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Specified maximum cardinality (number or *)
    pub max: Option<types::String>,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,

    /// Rule only applies if source has this type
    pub r#type: Option<types::String>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Default value if no value exists
    pub default_value: Option<types::String>,
    /// Primitive extension sibling for [`default_value`](Self::default_value) (FHIR `_defaultValue`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_defaultValue")]
    pub default_value_ext: Option<types::Element>,

    /// Optional field for this source
    pub element: Option<types::String>,
    /// Primitive extension sibling for [`element`](Self::element) (FHIR `_element`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_element")]
    pub element_ext: Option<types::Element>,

    /// first | not_first | last | not_last | only_one
    pub list_mode: Option<crate::coded::Coded<crate::r6::codes::MapSourceListMode>>,
    /// Primitive extension sibling for [`list_mode`](Self::list_mode) (FHIR `_listMode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_listMode")]
    pub list_mode_ext: Option<types::Element>,

    /// Named context for field, if a field is specified
    pub variable: Option<types::Id>,
    /// Primitive extension sibling for [`variable`](Self::variable) (FHIR `_variable`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_variable")]
    pub variable_ext: Option<types::Element>,

    /// FHIRPath expression - must be true or the rule does not apply
    pub condition: Option<types::String>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_condition")]
    pub condition_ext: Option<types::Element>,

    /// FHIRPath expression - must be true or the mapping engine throws an
    /// error instead of completing
    pub check: Option<types::String>,
    /// Primitive extension sibling for [`check`](Self::check) (FHIR `_check`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_check")]
    pub check_ext: Option<types::Element>,

    /// Message to put in log if source exists (FHIRPath)
    pub log_message: Option<types::String>,
    /// Primitive extension sibling for [`log_message`](Self::log_message) (FHIR `_logMessage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_logMessage")]
    pub log_message_ext: Option<types::Element>,
}

/// Content to create because of this mapping rule.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_map::StructureMapGroupRuleTarget;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_context")]
    pub context_ext: Option<types::Element>,

    /// Field to create in the context
    pub element: Option<types::String>,
    /// Primitive extension sibling for [`element`](Self::element) (FHIR `_element`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_element")]
    pub element_ext: Option<types::Element>,

    /// Named context for field, if desired, and a field is specified
    pub variable: Option<types::Id>,
    /// Primitive extension sibling for [`variable`](Self::variable) (FHIR `_variable`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_variable")]
    pub variable_ext: Option<types::Element>,

    /// first | share | last | single
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list_mode: Vec<crate::coded::Coded<crate::r6::codes::MapTargetListMode>>,
    /// Primitive extension sibling for [`list_mode`](Self::list_mode) (FHIR `_listMode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_listMode")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list_mode_ext: Vec<Option<types::Element>>,

    /// Internal rule reference for shared list items
    pub list_rule_id: Option<types::Id>,
    /// Primitive extension sibling for [`list_rule_id`](Self::list_rule_id) (FHIR `_listRuleId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_listRuleId")]
    pub list_rule_id_ext: Option<types::Element>,

    /// create | copy +
    pub transform: Option<crate::coded::Coded<crate::r6::codes::MapTransform>>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

    /// Parameters to the transform
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<StructureMapGroupRuleTargetParameter>,
}

/// Parameters to the transform.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_map::StructureMapGroupRuleTargetParameter;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct StructureMapGroupRuleTargetParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Parameter value - variable or literal
    /// The `StructureMap.group.rule.target.parameter.value[x]` choice element (1..1); see [`StructureMapGroupRuleTargetParameterValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<StructureMapGroupRuleTargetParameterValue>,
}

/// A structure definition used by this map. The structure definition may
/// describe instances that are converted, or the instances that are produced.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::structure_map::StructureMapStructure;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// source | queried | target | produced
    pub mode: crate::coded::Coded<crate::r6::codes::MapModelMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Name for type in this map
    pub alias: Option<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alias")]
    pub alias_ext: Option<types::Element>,

    /// Documentation on use of structure
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// The `StructureMap.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum StructureMapVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `StructureMap.group.rule.target.parameter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum StructureMapGroupRuleTargetParameterValue {
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r6::choice::Primitive<types::Id>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
}
