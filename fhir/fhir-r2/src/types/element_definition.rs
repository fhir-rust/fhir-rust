//! ElementDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ElementDefinition
//!
//!
//!
//! Definition of an element in a resource or extension
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ElementDefinition Type
///
/// # Examples
///
/// ```
/// use fhir::r2::types::element_definition::ElementDefinition;
/// use fhir::r2::types;
///
/// let value = ElementDefinition {
///     name_reference: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `nameReference` is the name this serializes to on the wire.
/// assert_eq!(json["nameReference"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ElementDefinitionDe")]
#[fhir_version("r2")]
pub struct ElementDefinition {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// The path of the element (see the Detailed Descriptions)
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// How this element is represented in instances
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub representation: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_representation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representation_ext: Vec<Option<types::Element>>,

    /// Name for this particular element definition (reference target)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for element to display with or prompt for element
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Defining code
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// This element is sliced - slices follow
    pub slicing: Option<ElementDefinitionSlicing>,

    /// Concise definition for xml presentation
    pub short: Option<types::String>,
    /// Primitive extension sibling for [`short`](Self::short) (FHIR `_short`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_short")]
    pub short_ext: Option<types::Element>,

    /// Full formal definition as narrative text
    pub definition: Option<types::Markdown>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Comments about the use of this element
    pub comments: Option<types::Markdown>,
    /// Primitive extension sibling for [`comments`](Self::comments) (FHIR `_comments`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comments")]
    pub comments_ext: Option<types::Element>,

    /// Why is this needed?
    pub requirements: Option<types::Markdown>,
    /// Primitive extension sibling for [`requirements`](Self::requirements) (FHIR `_requirements`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirements")]
    pub requirements_ext: Option<types::Element>,

    /// Other names
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub alias: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// Minimum Cardinality
    pub min: Option<types::Integer>,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Maximum Cardinality (a number or *)
    pub max: Option<types::String>,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,

    /// Base definition information for tools
    pub base: Option<ElementDefinitionBase>,

    /// Data type and Profile for this element
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<ElementDefinitionType>,

    /// To another element constraint (by element.name)
    pub name_reference: Option<types::String>,
    /// Primitive extension sibling for [`name_reference`](Self::name_reference) (FHIR `_nameReference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_nameReference")]
    pub name_reference_ext: Option<types::Element>,

    /// Specified value it missing from instance
    /// The `ElementDefinition.defaultValue[x]` choice element (0..1); see [`ElementDefinitionDefaultValue`].
    #[serde(flatten)]
    pub default_value: Option<ElementDefinitionDefaultValue>,

    /// Implicit meaning when this element is missing
    pub meaning_when_missing: Option<types::Markdown>,
    /// Primitive extension sibling for [`meaning_when_missing`](Self::meaning_when_missing) (FHIR `_meaningWhenMissing`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_meaningWhenMissing")]
    pub meaning_when_missing_ext: Option<types::Element>,

    /// Value must be exactly this
    /// The `ElementDefinition.fixed[x]` choice element (0..1); see [`ElementDefinitionFixed`].
    #[serde(flatten)]
    pub fixed: Option<ElementDefinitionFixed>,

    /// Value must have at least these property values
    /// The `ElementDefinition.pattern[x]` choice element (0..1); see [`ElementDefinitionPattern`].
    #[serde(flatten)]
    pub pattern: Option<ElementDefinitionPattern>,

    /// Example value: [as defined for type]
    /// The `ElementDefinition.example[x]` choice element (0..1); see [`ElementDefinitionExample`].
    #[serde(flatten)]
    pub example: Option<ElementDefinitionExample>,

    /// Minimum Allowed Value (for some types)
    /// The `ElementDefinition.minValue[x]` choice element (0..1); see [`ElementDefinitionMinValue`].
    #[serde(flatten)]
    pub min_value: Option<ElementDefinitionMinValue>,

    /// Maximum Allowed Value (for some types)
    /// The `ElementDefinition.maxValue[x]` choice element (0..1); see [`ElementDefinitionMaxValue`].
    #[serde(flatten)]
    pub max_value: Option<ElementDefinitionMaxValue>,

    /// Max length for strings
    pub max_length: Option<types::Integer>,
    /// Primitive extension sibling for [`max_length`](Self::max_length) (FHIR `_maxLength`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_maxLength")]
    pub max_length_ext: Option<types::Element>,

    /// Reference to invariant about presence
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub condition: ::fhir_core::PrimVec<types::Id>,
    /// Primitive extension sibling for [`condition`](Self::condition) (FHIR `_condition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_condition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition_ext: Vec<Option<types::Element>>,

    /// Condition that must evaluate to true
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraint: Vec<ElementDefinitionConstraint>,

    /// If the element must supported
    pub must_support: Option<types::Boolean>,
    /// Primitive extension sibling for [`must_support`](Self::must_support) (FHIR `_mustSupport`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mustSupport")]
    pub must_support_ext: Option<types::Element>,

    /// If this modifies the meaning of other elements
    pub is_modifier: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_modifier`](Self::is_modifier) (FHIR `_isModifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isModifier")]
    pub is_modifier_ext: Option<types::Element>,

    /// Include when _summary = true?
    pub is_summary: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_summary`](Self::is_summary) (FHIR `_isSummary`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isSummary")]
    pub is_summary_ext: Option<types::Element>,

    /// ValueSet details if this is coded
    pub binding: Option<ElementDefinitionBinding>,

    /// Map element to another set of definitions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mapping: Vec<ElementDefinitionMapping>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDefinitionDe {
    id: Option<types::Id>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    path: types::String,
    #[serde(rename = "_path")]
    path_ext: Option<types::Element>,
    #[serde(default)]
    representation: ::fhir_core::PrimVec<types::Code>,
    #[serde(rename = "_representation")]
    #[serde(default)]
    representation_ext: Vec<Option<types::Element>>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    label: Option<types::String>,
    #[serde(rename = "_label")]
    label_ext: Option<types::Element>,
    #[serde(default)]
    code: Vec<types::Coding>,
    slicing: Option<ElementDefinitionSlicing>,
    short: Option<types::String>,
    #[serde(rename = "_short")]
    short_ext: Option<types::Element>,
    definition: Option<types::Markdown>,
    #[serde(rename = "_definition")]
    definition_ext: Option<types::Element>,
    comments: Option<types::Markdown>,
    #[serde(rename = "_comments")]
    comments_ext: Option<types::Element>,
    requirements: Option<types::Markdown>,
    #[serde(rename = "_requirements")]
    requirements_ext: Option<types::Element>,
    #[serde(default)]
    alias: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_alias")]
    #[serde(default)]
    alias_ext: Vec<Option<types::Element>>,
    min: Option<types::Integer>,
    #[serde(rename = "_min")]
    min_ext: Option<types::Element>,
    max: Option<types::String>,
    #[serde(rename = "_max")]
    max_ext: Option<types::Element>,
    base: Option<ElementDefinitionBase>,
    #[serde(default)]
    r#type: Vec<ElementDefinitionType>,
    name_reference: Option<types::String>,
    #[serde(rename = "_nameReference")]
    name_reference_ext: Option<types::Element>,
    #[serde(flatten)]
    default_value: crate::r2::choice::Slot<ElementDefinitionDefaultValue>,
    meaning_when_missing: Option<types::Markdown>,
    #[serde(rename = "_meaningWhenMissing")]
    meaning_when_missing_ext: Option<types::Element>,
    #[serde(flatten)]
    fixed: crate::r2::choice::Slot<ElementDefinitionFixed>,
    #[serde(flatten)]
    pattern: crate::r2::choice::Slot<ElementDefinitionPattern>,
    #[serde(flatten)]
    example: crate::r2::choice::Slot<ElementDefinitionExample>,
    #[serde(flatten)]
    min_value: crate::r2::choice::Slot<ElementDefinitionMinValue>,
    #[serde(flatten)]
    max_value: crate::r2::choice::Slot<ElementDefinitionMaxValue>,
    max_length: Option<types::Integer>,
    #[serde(rename = "_maxLength")]
    max_length_ext: Option<types::Element>,
    #[serde(default)]
    condition: ::fhir_core::PrimVec<types::Id>,
    #[serde(rename = "_condition")]
    #[serde(default)]
    condition_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    constraint: Vec<ElementDefinitionConstraint>,
    must_support: Option<types::Boolean>,
    #[serde(rename = "_mustSupport")]
    must_support_ext: Option<types::Element>,
    is_modifier: Option<types::Boolean>,
    #[serde(rename = "_isModifier")]
    is_modifier_ext: Option<types::Element>,
    is_summary: Option<types::Boolean>,
    #[serde(rename = "_isSummary")]
    is_summary_ext: Option<types::Element>,
    binding: Option<ElementDefinitionBinding>,
    #[serde(default)]
    mapping: Vec<ElementDefinitionMapping>,
}

impl ::core::convert::From<ElementDefinitionDe> for ElementDefinition {
    fn from(v: ElementDefinitionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            path: v.path,
            path_ext: v.path_ext,
            representation: v.representation,
            representation_ext: v.representation_ext,
            name: v.name,
            name_ext: v.name_ext,
            label: v.label,
            label_ext: v.label_ext,
            code: v.code,
            slicing: v.slicing,
            short: v.short,
            short_ext: v.short_ext,
            definition: v.definition,
            definition_ext: v.definition_ext,
            comments: v.comments,
            comments_ext: v.comments_ext,
            requirements: v.requirements,
            requirements_ext: v.requirements_ext,
            alias: v.alias,
            alias_ext: v.alias_ext,
            min: v.min,
            min_ext: v.min_ext,
            max: v.max,
            max_ext: v.max_ext,
            base: v.base,
            r#type: v.r#type,
            name_reference: v.name_reference,
            name_reference_ext: v.name_reference_ext,
            default_value: v.default_value.0,
            meaning_when_missing: v.meaning_when_missing,
            meaning_when_missing_ext: v.meaning_when_missing_ext,
            fixed: v.fixed.0,
            pattern: v.pattern.0,
            example: v.example.0,
            min_value: v.min_value.0,
            max_value: v.max_value.0,
            max_length: v.max_length,
            max_length_ext: v.max_length_ext,
            condition: v.condition,
            condition_ext: v.condition_ext,
            constraint: v.constraint,
            must_support: v.must_support,
            must_support_ext: v.must_support_ext,
            is_modifier: v.is_modifier,
            is_modifier_ext: v.is_modifier_ext,
            is_summary: v.is_summary,
            is_summary_ext: v.is_summary_ext,
            binding: v.binding,
            mapping: v.mapping,
        }
    }
}

/// Information about the base definition of the element, provided to make it
/// unncessary for tools to trace the deviation of the element through the
/// derived and related profiles. This information is only provided where the
/// element definition represents a constraint on another element definition,
/// and must be present if there is a base element definition.
///
/// # Examples
///
/// ```
/// use fhir::r2::types::element_definition::ElementDefinitionBase;
/// use fhir::r2::types;
///
/// let value = ElementDefinitionBase {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ElementDefinitionBase = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ElementDefinitionBase {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Path that identifies the base element
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Min cardinality of the base element
    pub min: types::Integer,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Max cardinality of the base element
    pub max: types::String,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,
}

/// Binds to a value set if this element is coded (code, Coding,
/// CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r2::types::element_definition::ElementDefinitionBinding;
/// use fhir::r2::types;
///
/// let value = ElementDefinitionBinding {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionBinding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ElementDefinitionBindingDe")]
#[fhir_version("r2")]
pub struct ElementDefinitionBinding {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// required | extensible | preferred | example
    pub strength: crate::coded::Coded<crate::r2::codes::BindingStrength>,
    /// Primitive extension sibling for [`strength`](Self::strength) (FHIR `_strength`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_strength")]
    pub strength_ext: Option<types::Element>,

    /// Human explanation of the value set
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Source of value set
    /// The `ElementDefinition.binding.valueSet[x]` choice element (0..1); see [`ElementDefinitionBindingValueSet`].
    #[serde(flatten)]
    pub value_set: Option<ElementDefinitionBindingValueSet>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDefinitionBindingDe {
    id: Option<types::Id>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    strength: crate::coded::Coded<crate::r2::codes::BindingStrength>,
    #[serde(rename = "_strength")]
    strength_ext: Option<types::Element>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(flatten)]
    value_set: crate::r2::choice::Slot<ElementDefinitionBindingValueSet>,
}

impl ::core::convert::From<ElementDefinitionBindingDe> for ElementDefinitionBinding {
    fn from(v: ElementDefinitionBindingDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            strength: v.strength,
            strength_ext: v.strength_ext,
            description: v.description,
            description_ext: v.description_ext,
            value_set: v.value_set.0,
        }
    }
}

/// Formal constraints such as co-occurrence and other constraints that can be
/// computationally evaluated within the context of the instance.
///
/// # Examples
///
/// ```
/// use fhir::r2::types::element_definition::ElementDefinitionConstraint;
/// use fhir::r2::types;
///
/// let value = ElementDefinitionConstraint {
///     requirements: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `requirements` is the name this serializes to on the wire.
/// assert_eq!(json["requirements"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionConstraint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ElementDefinitionConstraint {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Target of 'condition' reference above
    pub key: types::Id,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Why this constraint necessary or appropriate
    pub requirements: Option<types::String>,
    /// Primitive extension sibling for [`requirements`](Self::requirements) (FHIR `_requirements`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirements")]
    pub requirements_ext: Option<types::Element>,

    /// error | warning
    pub severity: crate::coded::Coded<crate::r2::codes::ConstraintSeverity>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// Human description of constraint
    pub human: types::String,
    /// Primitive extension sibling for [`human`](Self::human) (FHIR `_human`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_human")]
    pub human_ext: Option<types::Element>,

    /// XPath expression of constraint
    pub xpath: types::String,
    /// Primitive extension sibling for [`xpath`](Self::xpath) (FHIR `_xpath`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_xpath")]
    pub xpath_ext: Option<types::Element>,
}

/// Identifies a concept from an external specification that roughly
/// corresponds to this element.
///
/// # Examples
///
/// ```
/// use fhir::r2::types::element_definition::ElementDefinitionMapping;
/// use fhir::r2::types;
///
/// let value = ElementDefinitionMapping {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ElementDefinitionMapping = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ElementDefinitionMapping {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Reference to mapping declaration
    pub identity: types::Id,
    /// Primitive extension sibling for [`identity`](Self::identity) (FHIR `_identity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_identity")]
    pub identity_ext: Option<types::Element>,

    /// Computable language of mapping
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Details of the mapping
    pub map: types::String,
    /// Primitive extension sibling for [`map`](Self::map) (FHIR `_map`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_map")]
    pub map_ext: Option<types::Element>,
}

/// Indicates that the element is sliced into a set of alternative definitions
/// (i.e. in a structure definition, there are multiple different constraints
/// on a single element in the base resource). Slicing can be used in any
/// resource that has cardinality ..* on the base resource, or any resource
/// with a choice of types. The set of slices is any elements that come after
/// this in the element sequence that have the same path, until a shorter path
/// occurs (the shorter path terminates the set).
///
/// # Examples
///
/// ```
/// use fhir::r2::types::element_definition::ElementDefinitionSlicing;
/// use fhir::r2::types;
///
/// let value = ElementDefinitionSlicing {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionSlicing = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ElementDefinitionSlicing {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Element values that used to distinguish the slices
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub discriminator: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`discriminator`](Self::discriminator) (FHIR `_discriminator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_discriminator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub discriminator_ext: Vec<Option<types::Element>>,

    /// Text description of how slicing works (or not)
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// If elements must be in same order as slices
    pub ordered: Option<types::Boolean>,
    /// Primitive extension sibling for [`ordered`](Self::ordered) (FHIR `_ordered`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ordered")]
    pub ordered_ext: Option<types::Element>,

    /// closed | open | openAtEnd
    pub rules: crate::coded::Coded<crate::r2::codes::ResourceSlicingRules>,
    /// Primitive extension sibling for [`rules`](Self::rules) (FHIR `_rules`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rules")]
    pub rules_ext: Option<types::Element>,
}

/// The data type or resource that the value of this element is permitted to
/// be.
///
/// # Examples
///
/// ```
/// use fhir::r2::types::element_definition::ElementDefinitionType;
/// use fhir::r2::types;
///
/// let value = ElementDefinitionType {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ElementDefinitionType = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ElementDefinitionType {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Name of Data type or Resource
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Profile (StructureDefinition) to apply (or IG)
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub profile: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// contained | referenced | bundled - how aggregated
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub aggregation:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r2::codes::ResourceAggregationMode>>,
    /// Primitive extension sibling for [`aggregation`](Self::aggregation) (FHIR `_aggregation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_aggregation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aggregation_ext: Vec<Option<types::Element>>,
}

/// The `ElementDefinition.defaultValue[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionDefaultValue {
    /// `defaultValueBoolean` variant.
    #[fhir("defaultValueBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `defaultValueInteger` variant.
    #[fhir("defaultValueInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `defaultValueDecimal` variant.
    #[fhir("defaultValueDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `defaultValueBase64Binary` variant.
    #[fhir("defaultValueBase64Binary")]
    Base64Binary(crate::r2::choice::Primitive<types::Base64Binary>),
    /// `defaultValueInstant` variant.
    #[fhir("defaultValueInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `defaultValueString` variant.
    #[fhir("defaultValueString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `defaultValueUri` variant.
    #[fhir("defaultValueUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `defaultValueDate` variant.
    #[fhir("defaultValueDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `defaultValueDateTime` variant.
    #[fhir("defaultValueDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `defaultValueTime` variant.
    #[fhir("defaultValueTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `defaultValueCode` variant.
    #[fhir("defaultValueCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
    /// `defaultValueOid` variant.
    #[fhir("defaultValueOid")]
    Oid(crate::r2::choice::Primitive<types::Oid>),
    /// `defaultValueId` variant.
    #[fhir("defaultValueId")]
    Id(crate::r2::choice::Primitive<types::Id>),
    /// `defaultValueUnsignedInt` variant.
    #[fhir("defaultValueUnsignedInt")]
    UnsignedInt(crate::r2::choice::Primitive<types::UnsignedInt>),
    /// `defaultValuePositiveInt` variant.
    #[fhir("defaultValuePositiveInt")]
    PositiveInt(crate::r2::choice::Primitive<types::PositiveInt>),
    /// `defaultValueMarkdown` variant.
    #[fhir("defaultValueMarkdown")]
    Markdown(crate::r2::choice::Primitive<types::Markdown>),
    /// `defaultValueAnnotation` variant.
    #[fhir("defaultValueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `defaultValueAttachment` variant.
    #[fhir("defaultValueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `defaultValueIdentifier` variant.
    #[fhir("defaultValueIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `defaultValueCodeableConcept` variant.
    #[fhir("defaultValueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `defaultValueCoding` variant.
    #[fhir("defaultValueCoding")]
    Coding(Box<types::Coding>),
    /// `defaultValueQuantity` variant.
    #[fhir("defaultValueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `defaultValueRange` variant.
    #[fhir("defaultValueRange")]
    Range(Box<types::Range>),
    /// `defaultValuePeriod` variant.
    #[fhir("defaultValuePeriod")]
    Period(Box<types::Period>),
    /// `defaultValueRatio` variant.
    #[fhir("defaultValueRatio")]
    Ratio(Box<types::Ratio>),
    /// `defaultValueSampledData` variant.
    #[fhir("defaultValueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `defaultValueSignature` variant.
    #[fhir("defaultValueSignature")]
    Signature(Box<types::Signature>),
    /// `defaultValueHumanName` variant.
    #[fhir("defaultValueHumanName")]
    HumanName(Box<types::HumanName>),
    /// `defaultValueAddress` variant.
    #[fhir("defaultValueAddress")]
    Address(Box<types::Address>),
    /// `defaultValueContactPoint` variant.
    #[fhir("defaultValueContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `defaultValueTiming` variant.
    #[fhir("defaultValueTiming")]
    Timing(Box<types::Timing>),
    /// `defaultValueReference` variant.
    #[fhir("defaultValueReference")]
    Reference(Box<types::Reference>),
    /// `defaultValueMeta` variant.
    #[fhir("defaultValueMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.fixed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionFixed {
    /// `fixedBoolean` variant.
    #[fhir("fixedBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `fixedInteger` variant.
    #[fhir("fixedInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `fixedDecimal` variant.
    #[fhir("fixedDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `fixedBase64Binary` variant.
    #[fhir("fixedBase64Binary")]
    Base64Binary(crate::r2::choice::Primitive<types::Base64Binary>),
    /// `fixedInstant` variant.
    #[fhir("fixedInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `fixedString` variant.
    #[fhir("fixedString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `fixedUri` variant.
    #[fhir("fixedUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `fixedDate` variant.
    #[fhir("fixedDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `fixedDateTime` variant.
    #[fhir("fixedDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `fixedTime` variant.
    #[fhir("fixedTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `fixedCode` variant.
    #[fhir("fixedCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
    /// `fixedOid` variant.
    #[fhir("fixedOid")]
    Oid(crate::r2::choice::Primitive<types::Oid>),
    /// `fixedId` variant.
    #[fhir("fixedId")]
    Id(crate::r2::choice::Primitive<types::Id>),
    /// `fixedUnsignedInt` variant.
    #[fhir("fixedUnsignedInt")]
    UnsignedInt(crate::r2::choice::Primitive<types::UnsignedInt>),
    /// `fixedPositiveInt` variant.
    #[fhir("fixedPositiveInt")]
    PositiveInt(crate::r2::choice::Primitive<types::PositiveInt>),
    /// `fixedMarkdown` variant.
    #[fhir("fixedMarkdown")]
    Markdown(crate::r2::choice::Primitive<types::Markdown>),
    /// `fixedAnnotation` variant.
    #[fhir("fixedAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `fixedAttachment` variant.
    #[fhir("fixedAttachment")]
    Attachment(Box<types::Attachment>),
    /// `fixedIdentifier` variant.
    #[fhir("fixedIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `fixedCodeableConcept` variant.
    #[fhir("fixedCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `fixedCoding` variant.
    #[fhir("fixedCoding")]
    Coding(Box<types::Coding>),
    /// `fixedQuantity` variant.
    #[fhir("fixedQuantity")]
    Quantity(Box<types::Quantity>),
    /// `fixedRange` variant.
    #[fhir("fixedRange")]
    Range(Box<types::Range>),
    /// `fixedPeriod` variant.
    #[fhir("fixedPeriod")]
    Period(Box<types::Period>),
    /// `fixedRatio` variant.
    #[fhir("fixedRatio")]
    Ratio(Box<types::Ratio>),
    /// `fixedSampledData` variant.
    #[fhir("fixedSampledData")]
    SampledData(Box<types::SampledData>),
    /// `fixedSignature` variant.
    #[fhir("fixedSignature")]
    Signature(Box<types::Signature>),
    /// `fixedHumanName` variant.
    #[fhir("fixedHumanName")]
    HumanName(Box<types::HumanName>),
    /// `fixedAddress` variant.
    #[fhir("fixedAddress")]
    Address(Box<types::Address>),
    /// `fixedContactPoint` variant.
    #[fhir("fixedContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `fixedTiming` variant.
    #[fhir("fixedTiming")]
    Timing(Box<types::Timing>),
    /// `fixedReference` variant.
    #[fhir("fixedReference")]
    Reference(Box<types::Reference>),
    /// `fixedMeta` variant.
    #[fhir("fixedMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.pattern[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionPattern {
    /// `patternBoolean` variant.
    #[fhir("patternBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `patternInteger` variant.
    #[fhir("patternInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `patternDecimal` variant.
    #[fhir("patternDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `patternBase64Binary` variant.
    #[fhir("patternBase64Binary")]
    Base64Binary(crate::r2::choice::Primitive<types::Base64Binary>),
    /// `patternInstant` variant.
    #[fhir("patternInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `patternString` variant.
    #[fhir("patternString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `patternUri` variant.
    #[fhir("patternUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `patternDate` variant.
    #[fhir("patternDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `patternDateTime` variant.
    #[fhir("patternDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `patternTime` variant.
    #[fhir("patternTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `patternCode` variant.
    #[fhir("patternCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
    /// `patternOid` variant.
    #[fhir("patternOid")]
    Oid(crate::r2::choice::Primitive<types::Oid>),
    /// `patternId` variant.
    #[fhir("patternId")]
    Id(crate::r2::choice::Primitive<types::Id>),
    /// `patternUnsignedInt` variant.
    #[fhir("patternUnsignedInt")]
    UnsignedInt(crate::r2::choice::Primitive<types::UnsignedInt>),
    /// `patternPositiveInt` variant.
    #[fhir("patternPositiveInt")]
    PositiveInt(crate::r2::choice::Primitive<types::PositiveInt>),
    /// `patternMarkdown` variant.
    #[fhir("patternMarkdown")]
    Markdown(crate::r2::choice::Primitive<types::Markdown>),
    /// `patternAnnotation` variant.
    #[fhir("patternAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `patternAttachment` variant.
    #[fhir("patternAttachment")]
    Attachment(Box<types::Attachment>),
    /// `patternIdentifier` variant.
    #[fhir("patternIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `patternCodeableConcept` variant.
    #[fhir("patternCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `patternCoding` variant.
    #[fhir("patternCoding")]
    Coding(Box<types::Coding>),
    /// `patternQuantity` variant.
    #[fhir("patternQuantity")]
    Quantity(Box<types::Quantity>),
    /// `patternRange` variant.
    #[fhir("patternRange")]
    Range(Box<types::Range>),
    /// `patternPeriod` variant.
    #[fhir("patternPeriod")]
    Period(Box<types::Period>),
    /// `patternRatio` variant.
    #[fhir("patternRatio")]
    Ratio(Box<types::Ratio>),
    /// `patternSampledData` variant.
    #[fhir("patternSampledData")]
    SampledData(Box<types::SampledData>),
    /// `patternSignature` variant.
    #[fhir("patternSignature")]
    Signature(Box<types::Signature>),
    /// `patternHumanName` variant.
    #[fhir("patternHumanName")]
    HumanName(Box<types::HumanName>),
    /// `patternAddress` variant.
    #[fhir("patternAddress")]
    Address(Box<types::Address>),
    /// `patternContactPoint` variant.
    #[fhir("patternContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `patternTiming` variant.
    #[fhir("patternTiming")]
    Timing(Box<types::Timing>),
    /// `patternReference` variant.
    #[fhir("patternReference")]
    Reference(Box<types::Reference>),
    /// `patternMeta` variant.
    #[fhir("patternMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.example[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionExample {
    /// `exampleBoolean` variant.
    #[fhir("exampleBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `exampleInteger` variant.
    #[fhir("exampleInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `exampleDecimal` variant.
    #[fhir("exampleDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `exampleBase64Binary` variant.
    #[fhir("exampleBase64Binary")]
    Base64Binary(crate::r2::choice::Primitive<types::Base64Binary>),
    /// `exampleInstant` variant.
    #[fhir("exampleInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `exampleString` variant.
    #[fhir("exampleString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `exampleUri` variant.
    #[fhir("exampleUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `exampleDate` variant.
    #[fhir("exampleDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `exampleDateTime` variant.
    #[fhir("exampleDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `exampleTime` variant.
    #[fhir("exampleTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `exampleCode` variant.
    #[fhir("exampleCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
    /// `exampleOid` variant.
    #[fhir("exampleOid")]
    Oid(crate::r2::choice::Primitive<types::Oid>),
    /// `exampleId` variant.
    #[fhir("exampleId")]
    Id(crate::r2::choice::Primitive<types::Id>),
    /// `exampleUnsignedInt` variant.
    #[fhir("exampleUnsignedInt")]
    UnsignedInt(crate::r2::choice::Primitive<types::UnsignedInt>),
    /// `examplePositiveInt` variant.
    #[fhir("examplePositiveInt")]
    PositiveInt(crate::r2::choice::Primitive<types::PositiveInt>),
    /// `exampleMarkdown` variant.
    #[fhir("exampleMarkdown")]
    Markdown(crate::r2::choice::Primitive<types::Markdown>),
    /// `exampleAnnotation` variant.
    #[fhir("exampleAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `exampleAttachment` variant.
    #[fhir("exampleAttachment")]
    Attachment(Box<types::Attachment>),
    /// `exampleIdentifier` variant.
    #[fhir("exampleIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `exampleCodeableConcept` variant.
    #[fhir("exampleCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `exampleCoding` variant.
    #[fhir("exampleCoding")]
    Coding(Box<types::Coding>),
    /// `exampleQuantity` variant.
    #[fhir("exampleQuantity")]
    Quantity(Box<types::Quantity>),
    /// `exampleRange` variant.
    #[fhir("exampleRange")]
    Range(Box<types::Range>),
    /// `examplePeriod` variant.
    #[fhir("examplePeriod")]
    Period(Box<types::Period>),
    /// `exampleRatio` variant.
    #[fhir("exampleRatio")]
    Ratio(Box<types::Ratio>),
    /// `exampleSampledData` variant.
    #[fhir("exampleSampledData")]
    SampledData(Box<types::SampledData>),
    /// `exampleSignature` variant.
    #[fhir("exampleSignature")]
    Signature(Box<types::Signature>),
    /// `exampleHumanName` variant.
    #[fhir("exampleHumanName")]
    HumanName(Box<types::HumanName>),
    /// `exampleAddress` variant.
    #[fhir("exampleAddress")]
    Address(Box<types::Address>),
    /// `exampleContactPoint` variant.
    #[fhir("exampleContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `exampleTiming` variant.
    #[fhir("exampleTiming")]
    Timing(Box<types::Timing>),
    /// `exampleReference` variant.
    #[fhir("exampleReference")]
    Reference(Box<types::Reference>),
    /// `exampleMeta` variant.
    #[fhir("exampleMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.minValue[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionMinValue {
    /// `minValueBoolean` variant.
    #[fhir("minValueBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `minValueInteger` variant.
    #[fhir("minValueInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `minValueDecimal` variant.
    #[fhir("minValueDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `minValueBase64Binary` variant.
    #[fhir("minValueBase64Binary")]
    Base64Binary(crate::r2::choice::Primitive<types::Base64Binary>),
    /// `minValueInstant` variant.
    #[fhir("minValueInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `minValueString` variant.
    #[fhir("minValueString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `minValueUri` variant.
    #[fhir("minValueUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `minValueDate` variant.
    #[fhir("minValueDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `minValueDateTime` variant.
    #[fhir("minValueDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `minValueTime` variant.
    #[fhir("minValueTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `minValueCode` variant.
    #[fhir("minValueCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
    /// `minValueOid` variant.
    #[fhir("minValueOid")]
    Oid(crate::r2::choice::Primitive<types::Oid>),
    /// `minValueId` variant.
    #[fhir("minValueId")]
    Id(crate::r2::choice::Primitive<types::Id>),
    /// `minValueUnsignedInt` variant.
    #[fhir("minValueUnsignedInt")]
    UnsignedInt(crate::r2::choice::Primitive<types::UnsignedInt>),
    /// `minValuePositiveInt` variant.
    #[fhir("minValuePositiveInt")]
    PositiveInt(crate::r2::choice::Primitive<types::PositiveInt>),
    /// `minValueMarkdown` variant.
    #[fhir("minValueMarkdown")]
    Markdown(crate::r2::choice::Primitive<types::Markdown>),
    /// `minValueAnnotation` variant.
    #[fhir("minValueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `minValueAttachment` variant.
    #[fhir("minValueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `minValueIdentifier` variant.
    #[fhir("minValueIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `minValueCodeableConcept` variant.
    #[fhir("minValueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `minValueCoding` variant.
    #[fhir("minValueCoding")]
    Coding(Box<types::Coding>),
    /// `minValueQuantity` variant.
    #[fhir("minValueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `minValueRange` variant.
    #[fhir("minValueRange")]
    Range(Box<types::Range>),
    /// `minValuePeriod` variant.
    #[fhir("minValuePeriod")]
    Period(Box<types::Period>),
    /// `minValueRatio` variant.
    #[fhir("minValueRatio")]
    Ratio(Box<types::Ratio>),
    /// `minValueSampledData` variant.
    #[fhir("minValueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `minValueSignature` variant.
    #[fhir("minValueSignature")]
    Signature(Box<types::Signature>),
    /// `minValueHumanName` variant.
    #[fhir("minValueHumanName")]
    HumanName(Box<types::HumanName>),
    /// `minValueAddress` variant.
    #[fhir("minValueAddress")]
    Address(Box<types::Address>),
    /// `minValueContactPoint` variant.
    #[fhir("minValueContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `minValueTiming` variant.
    #[fhir("minValueTiming")]
    Timing(Box<types::Timing>),
    /// `minValueReference` variant.
    #[fhir("minValueReference")]
    Reference(Box<types::Reference>),
    /// `minValueMeta` variant.
    #[fhir("minValueMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.maxValue[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionMaxValue {
    /// `maxValueBoolean` variant.
    #[fhir("maxValueBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `maxValueInteger` variant.
    #[fhir("maxValueInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `maxValueDecimal` variant.
    #[fhir("maxValueDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `maxValueBase64Binary` variant.
    #[fhir("maxValueBase64Binary")]
    Base64Binary(crate::r2::choice::Primitive<types::Base64Binary>),
    /// `maxValueInstant` variant.
    #[fhir("maxValueInstant")]
    Instant(crate::r2::choice::Primitive<types::Instant>),
    /// `maxValueString` variant.
    #[fhir("maxValueString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `maxValueUri` variant.
    #[fhir("maxValueUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `maxValueDate` variant.
    #[fhir("maxValueDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `maxValueDateTime` variant.
    #[fhir("maxValueDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `maxValueTime` variant.
    #[fhir("maxValueTime")]
    Time(crate::r2::choice::Primitive<types::Time>),
    /// `maxValueCode` variant.
    #[fhir("maxValueCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
    /// `maxValueOid` variant.
    #[fhir("maxValueOid")]
    Oid(crate::r2::choice::Primitive<types::Oid>),
    /// `maxValueId` variant.
    #[fhir("maxValueId")]
    Id(crate::r2::choice::Primitive<types::Id>),
    /// `maxValueUnsignedInt` variant.
    #[fhir("maxValueUnsignedInt")]
    UnsignedInt(crate::r2::choice::Primitive<types::UnsignedInt>),
    /// `maxValuePositiveInt` variant.
    #[fhir("maxValuePositiveInt")]
    PositiveInt(crate::r2::choice::Primitive<types::PositiveInt>),
    /// `maxValueMarkdown` variant.
    #[fhir("maxValueMarkdown")]
    Markdown(crate::r2::choice::Primitive<types::Markdown>),
    /// `maxValueAnnotation` variant.
    #[fhir("maxValueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `maxValueAttachment` variant.
    #[fhir("maxValueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `maxValueIdentifier` variant.
    #[fhir("maxValueIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `maxValueCodeableConcept` variant.
    #[fhir("maxValueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `maxValueCoding` variant.
    #[fhir("maxValueCoding")]
    Coding(Box<types::Coding>),
    /// `maxValueQuantity` variant.
    #[fhir("maxValueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `maxValueRange` variant.
    #[fhir("maxValueRange")]
    Range(Box<types::Range>),
    /// `maxValuePeriod` variant.
    #[fhir("maxValuePeriod")]
    Period(Box<types::Period>),
    /// `maxValueRatio` variant.
    #[fhir("maxValueRatio")]
    Ratio(Box<types::Ratio>),
    /// `maxValueSampledData` variant.
    #[fhir("maxValueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `maxValueSignature` variant.
    #[fhir("maxValueSignature")]
    Signature(Box<types::Signature>),
    /// `maxValueHumanName` variant.
    #[fhir("maxValueHumanName")]
    HumanName(Box<types::HumanName>),
    /// `maxValueAddress` variant.
    #[fhir("maxValueAddress")]
    Address(Box<types::Address>),
    /// `maxValueContactPoint` variant.
    #[fhir("maxValueContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `maxValueTiming` variant.
    #[fhir("maxValueTiming")]
    Timing(Box<types::Timing>),
    /// `maxValueReference` variant.
    #[fhir("maxValueReference")]
    Reference(Box<types::Reference>),
    /// `maxValueMeta` variant.
    #[fhir("maxValueMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.binding.valueSet[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionBindingValueSet {
    /// `valueSetUri` variant.
    #[fhir("valueSetUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `valueSetReference` variant.
    #[fhir("valueSetReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ElementDefinition;

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
