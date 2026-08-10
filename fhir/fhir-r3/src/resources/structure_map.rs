//! StructureMap
//!
//! URL: http://hl7.org/fhir/StructureDefinition/StructureMap
//!
//!
//!
//! A Map of relationships between 2 structures that can be used to transform
//! data
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for StructureMap Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::structure_map::StructureMap;
///
/// let value = StructureMap::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMap = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct StructureMap {
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

    /// Logical URI to reference this structure map (globally unique)
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

    /// Natural language description of the structure map
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Context the content is intended to support
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

    /// Structure Definition used by this map
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structure: Vec<StructureMapStructure>,

    /// Other maps used by this map (canonical URLs)
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub import: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`import`](Self::import) (FHIR `_import`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_import")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import_ext: Vec<Option<types::Element>>,

    /// Named sections for reader convenience
    pub group: ::vec1::Vec1<StructureMapGroup>,
}

/// Organizes the mapping into managable chunks for human review/ease of
/// maintenance.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::structure_map::StructureMapGroup;
///
/// let value = StructureMapGroup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct StructureMapGroup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

    /// none | types | type-and-types
    pub type_mode: crate::coded::Coded<crate::r3::codes::MapGroupTypeMode>,
    /// Primitive extension sibling for [`type_mode`](Self::type_mode) (FHIR `_typeMode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_typeMode")]
    pub type_mode_ext: Option<types::Element>,

    /// Additional description/explaination for group
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Named instance provided when invoking the map
    pub input: ::vec1::Vec1<StructureMapGroupInput>,

    /// Transform Rule from source to target
    pub rule: ::vec1::Vec1<StructureMapGroupRule>,
}

/// A name assigned to an instance of data. The instance must be provided when
/// the mapping is invoked.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::structure_map::StructureMapGroupInput;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct StructureMapGroupInput {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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
    pub mode: crate::coded::Coded<crate::r3::codes::MapInputMode>,
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
/// use fhir::r3::resources::structure_map::StructureMapGroupRule;
///
/// let value = StructureMapGroupRule::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroupRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct StructureMapGroupRule {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of the rule for internal references
    pub name: types::Id,
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
/// use fhir::r3::resources::structure_map::StructureMapGroupRuleDependent;
///
/// let value = StructureMapGroupRuleDependent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureMapGroupRuleDependent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct StructureMapGroupRuleDependent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of a rule or group to apply
    pub name: types::Id,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Variable to pass to the rule or group
    pub variable: ::vec1::Vec1<types::String>,
    /// Primitive extension sibling for [`variable`](Self::variable) (FHIR `_variable`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_variable")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable_ext: Vec<Option<types::Element>>,
}

/// Source inputs to the mapping.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::structure_map::StructureMapGroupRuleSource;
/// use fhir::r3::types;
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
#[serde(from = "StructureMapGroupRuleSourceDe")]
#[fhir_version("r3")]
pub struct StructureMapGroupRuleSource {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type or variable this rule applies to
    pub context: types::Id,
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_context")]
    pub context_ext: Option<types::Element>,

    /// Specified minimum cardinality
    pub min: Option<types::Integer>,
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
    /// The `StructureMap.group.rule.source.defaultValue[x]` choice element (0..1); see [`StructureMapGroupRuleSourceDefaultValue`].
    #[serde(flatten)]
    pub default_value: Option<StructureMapGroupRuleSourceDefaultValue>,

    /// Optional field for this source
    pub element: Option<types::String>,
    /// Primitive extension sibling for [`element`](Self::element) (FHIR `_element`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_element")]
    pub element_ext: Option<types::Element>,

    /// first | not_first | last | not_last | only_one
    pub list_mode: Option<crate::coded::Coded<crate::r3::codes::MapSourceListMode>>,
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
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureMapGroupRuleSourceDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    context: types::Id,
    #[serde(rename = "_context")]
    context_ext: Option<types::Element>,
    min: Option<types::Integer>,
    #[serde(rename = "_min")]
    min_ext: Option<types::Element>,
    max: Option<types::String>,
    #[serde(rename = "_max")]
    max_ext: Option<types::Element>,
    r#type: Option<types::String>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    #[serde(flatten)]
    default_value: crate::r3::choice::Slot<StructureMapGroupRuleSourceDefaultValue>,
    element: Option<types::String>,
    #[serde(rename = "_element")]
    element_ext: Option<types::Element>,
    list_mode: Option<crate::coded::Coded<crate::r3::codes::MapSourceListMode>>,
    #[serde(rename = "_listMode")]
    list_mode_ext: Option<types::Element>,
    variable: Option<types::Id>,
    #[serde(rename = "_variable")]
    variable_ext: Option<types::Element>,
    condition: Option<types::String>,
    #[serde(rename = "_condition")]
    condition_ext: Option<types::Element>,
    check: Option<types::String>,
    #[serde(rename = "_check")]
    check_ext: Option<types::Element>,
}

impl ::core::convert::From<StructureMapGroupRuleSourceDe> for StructureMapGroupRuleSource {
    fn from(v: StructureMapGroupRuleSourceDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            context: v.context,
            context_ext: v.context_ext,
            min: v.min,
            min_ext: v.min_ext,
            max: v.max,
            max_ext: v.max_ext,
            r#type: v.r#type,
            type_ext: v.type_ext,
            default_value: v.default_value.0,
            element: v.element,
            element_ext: v.element_ext,
            list_mode: v.list_mode,
            list_mode_ext: v.list_mode_ext,
            variable: v.variable,
            variable_ext: v.variable_ext,
            condition: v.condition,
            condition_ext: v.condition_ext,
            check: v.check,
            check_ext: v.check_ext,
        }
    }
}

/// Content to create because of this mapping rule.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::structure_map::StructureMapGroupRuleTarget;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct StructureMapGroupRuleTarget {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type or variable this rule applies to
    pub context: Option<types::Id>,
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_context")]
    pub context_ext: Option<types::Element>,

    /// type | variable
    pub context_type: Option<crate::coded::Coded<crate::r3::codes::MapContextType>>,
    /// Primitive extension sibling for [`context_type`](Self::context_type) (FHIR `_contextType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextType")]
    pub context_type_ext: Option<types::Element>,

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

    /// first | share | last | collate
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub list_mode: ::fhir_core::PrimVec<crate::coded::Coded<crate::r3::codes::MapTargetListMode>>,
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
    pub transform: Option<crate::coded::Coded<crate::r3::codes::MapTransform>>,
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
/// use fhir::r3::resources::structure_map::StructureMapGroupRuleTargetParameter;
/// use fhir::r3::types;
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
#[serde(from = "StructureMapGroupRuleTargetParameterDe")]
#[fhir_version("r3")]
pub struct StructureMapGroupRuleTargetParameter {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Parameter value - variable or literal
    /// The `StructureMap.group.rule.target.parameter.value[x]` choice element (1..1); see [`StructureMapGroupRuleTargetParameterValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<StructureMapGroupRuleTargetParameterValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureMapGroupRuleTargetParameterDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    value: crate::r3::choice::Slot<StructureMapGroupRuleTargetParameterValue>,
}

impl ::core::convert::From<StructureMapGroupRuleTargetParameterDe>
    for StructureMapGroupRuleTargetParameter
{
    fn from(v: StructureMapGroupRuleTargetParameterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            value: v.value.0,
        }
    }
}

/// A structure definition used by this map. The structure definition may
/// describe instances that are converted, or the instances that are produced.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::structure_map::StructureMapStructure;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct StructureMapStructure {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical URL for structure definition
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// source | queried | target | produced
    pub mode: crate::coded::Coded<crate::r3::codes::MapModelMode>,
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

/// The `StructureMap.group.rule.source.defaultValue[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum StructureMapGroupRuleSourceDefaultValue {
    /// `defaultValueBase64Binary` variant.
    #[fhir("defaultValueBase64Binary")]
    Base64Binary(crate::r3::choice::Primitive<types::Base64Binary>),
    /// `defaultValueBoolean` variant.
    #[fhir("defaultValueBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `defaultValueCode` variant.
    #[fhir("defaultValueCode")]
    Code(crate::r3::choice::Primitive<types::Code>),
    /// `defaultValueDate` variant.
    #[fhir("defaultValueDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `defaultValueDateTime` variant.
    #[fhir("defaultValueDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `defaultValueDecimal` variant.
    #[fhir("defaultValueDecimal")]
    Decimal(crate::r3::choice::Primitive<types::Decimal>),
    /// `defaultValueId` variant.
    #[fhir("defaultValueId")]
    Id(crate::r3::choice::Primitive<types::Id>),
    /// `defaultValueInstant` variant.
    #[fhir("defaultValueInstant")]
    Instant(crate::r3::choice::Primitive<types::Instant>),
    /// `defaultValueInteger` variant.
    #[fhir("defaultValueInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
    /// `defaultValueMarkdown` variant.
    #[fhir("defaultValueMarkdown")]
    Markdown(crate::r3::choice::Primitive<types::Markdown>),
    /// `defaultValueOid` variant.
    #[fhir("defaultValueOid")]
    Oid(crate::r3::choice::Primitive<types::Oid>),
    /// `defaultValuePositiveInt` variant.
    #[fhir("defaultValuePositiveInt")]
    PositiveInt(crate::r3::choice::Primitive<types::PositiveInt>),
    /// `defaultValueString` variant.
    #[fhir("defaultValueString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `defaultValueTime` variant.
    #[fhir("defaultValueTime")]
    Time(crate::r3::choice::Primitive<types::Time>),
    /// `defaultValueUnsignedInt` variant.
    #[fhir("defaultValueUnsignedInt")]
    UnsignedInt(crate::r3::choice::Primitive<types::UnsignedInt>),
    /// `defaultValueUri` variant.
    #[fhir("defaultValueUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `defaultValueAddress` variant.
    #[fhir("defaultValueAddress")]
    Address(Box<types::Address>),
    /// `defaultValueAge` variant.
    #[fhir("defaultValueAge")]
    Age(Box<types::Age>),
    /// `defaultValueAnnotation` variant.
    #[fhir("defaultValueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `defaultValueAttachment` variant.
    #[fhir("defaultValueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `defaultValueCodeableConcept` variant.
    #[fhir("defaultValueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `defaultValueCoding` variant.
    #[fhir("defaultValueCoding")]
    Coding(Box<types::Coding>),
    /// `defaultValueContactPoint` variant.
    #[fhir("defaultValueContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `defaultValueCount` variant.
    #[fhir("defaultValueCount")]
    Count(Box<types::Count>),
    /// `defaultValueDistance` variant.
    #[fhir("defaultValueDistance")]
    Distance(Box<types::Distance>),
    /// `defaultValueDuration` variant.
    #[fhir("defaultValueDuration")]
    Duration(Box<types::Duration>),
    /// `defaultValueHumanName` variant.
    #[fhir("defaultValueHumanName")]
    HumanName(Box<types::HumanName>),
    /// `defaultValueIdentifier` variant.
    #[fhir("defaultValueIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `defaultValueMoney` variant.
    #[fhir("defaultValueMoney")]
    Money(Box<types::Money>),
    /// `defaultValuePeriod` variant.
    #[fhir("defaultValuePeriod")]
    Period(Box<types::Period>),
    /// `defaultValueQuantity` variant.
    #[fhir("defaultValueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `defaultValueRange` variant.
    #[fhir("defaultValueRange")]
    Range(Box<types::Range>),
    /// `defaultValueRatio` variant.
    #[fhir("defaultValueRatio")]
    Ratio(Box<types::Ratio>),
    /// `defaultValueReference` variant.
    #[fhir("defaultValueReference")]
    Reference(Box<types::Reference>),
    /// `defaultValueSampledData` variant.
    #[fhir("defaultValueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `defaultValueSignature` variant.
    #[fhir("defaultValueSignature")]
    Signature(Box<types::Signature>),
    /// `defaultValueTiming` variant.
    #[fhir("defaultValueTiming")]
    Timing(Box<types::Timing>),
    /// `defaultValueMeta` variant.
    #[fhir("defaultValueMeta")]
    Meta(Box<types::Meta>),
}

/// The `StructureMap.group.rule.target.parameter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum StructureMapGroupRuleTargetParameterValue {
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r3::choice::Primitive<types::Id>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r3::choice::Primitive<types::Decimal>),
}
