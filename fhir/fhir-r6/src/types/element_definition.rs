//! ElementDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ElementDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! Definition of an element in a resource or extension
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// ElementDefinition Type: Captures constraints on each element within the
/// resource, profile, or extension.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinition;
/// use fhir::r6::types;
///
/// let value = ElementDefinition {
///     slice_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sliceName` is the name this serializes to on the wire.
/// assert_eq!(json["sliceName"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ElementDefinitionDe")]
#[fhir_version("r6")]
pub struct ElementDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Path of the element in the hierarchy of elements
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// xmlAttr | xmlText | typeAttr | cdaText | xhtml
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub representation:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r6::codes::PropertyRepresentation>>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_representation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representation_ext: Vec<Option<types::Element>>,

    /// Name for this particular element (in a set of slices)
    pub slice_name: Option<types::String>,
    /// Primitive extension sibling for [`slice_name`](Self::slice_name) (FHIR `_sliceName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sliceName")]
    pub slice_name_ext: Option<types::Element>,

    /// If this slice definition constrains an inherited slice definition (or
    /// not)
    pub slice_is_constraining: Option<types::Boolean>,
    /// Primitive extension sibling for [`slice_is_constraining`](Self::slice_is_constraining) (FHIR `_sliceIsConstraining`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sliceIsConstraining")]
    pub slice_is_constraining_ext: Option<types::Element>,

    /// String to display with or prompt for element
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Corresponding codes in terminologies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// This element is sliced - slices follow
    pub slicing: Option<ElementDefinitionSlicing>,

    /// Concise definition for space-constrained presentation
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
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Requirements satisfied by this element/structure and its constraints
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
    pub min: Option<types::UnsignedInt>,
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

    /// Reference to definition of content for the element
    pub content_reference: Option<types::Uri>,
    /// Primitive extension sibling for [`content_reference`](Self::content_reference) (FHIR `_contentReference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentReference")]
    pub content_reference_ext: Option<types::Element>,

    /// Data type and Profile for this element
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<ElementDefinitionType>,

    /// Specified value if missing from instance
    /// The `ElementDefinition.defaultValue[x]` choice element (0..1); see [`ElementDefinitionDefaultValue`].
    #[serde(flatten)]
    pub default_value: Option<ElementDefinitionDefaultValue>,

    /// Implicit meaning when this element is missing
    pub meaning_when_missing: Option<types::Markdown>,
    /// Primitive extension sibling for [`meaning_when_missing`](Self::meaning_when_missing) (FHIR `_meaningWhenMissing`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_meaningWhenMissing")]
    pub meaning_when_missing_ext: Option<types::Element>,

    /// What the order of the elements means
    pub order_meaning: Option<types::String>,
    /// Primitive extension sibling for [`order_meaning`](Self::order_meaning) (FHIR `_orderMeaning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_orderMeaning")]
    pub order_meaning_ext: Option<types::Element>,

    /// Value must be exactly this
    /// The `ElementDefinition.fixed[x]` choice element (0..1); see [`ElementDefinitionFixed`].
    #[serde(flatten)]
    pub fixed: Option<ElementDefinitionFixed>,

    /// Value must have at least these property values
    /// The `ElementDefinition.pattern[x]` choice element (0..1); see [`ElementDefinitionPattern`].
    #[serde(flatten)]
    pub pattern: Option<ElementDefinitionPattern>,

    /// Example value (as defined for type)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub example: Vec<ElementDefinitionExample>,

    /// Minimum Allowed Value (for some types)
    /// The `ElementDefinition.minValue[x]` choice element (0..1); see [`ElementDefinitionMinValue`].
    #[serde(flatten)]
    pub min_value: Option<ElementDefinitionMinValue>,

    /// Maximum Allowed Value (for some types)
    /// The `ElementDefinition.maxValue[x]` choice element (0..1); see [`ElementDefinitionMaxValue`].
    #[serde(flatten)]
    pub max_value: Option<ElementDefinitionMaxValue>,

    /// Max length for string type data
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

    /// For primitives, that a value must be present - not replaced by an
    /// extension
    pub must_have_value: Option<types::Boolean>,
    /// Primitive extension sibling for [`must_have_value`](Self::must_have_value) (FHIR `_mustHaveValue`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mustHaveValue")]
    pub must_have_value_ext: Option<types::Element>,

    /// Extensions that are allowed to replace a primitive value
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub value_alternatives: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`value_alternatives`](Self::value_alternatives) (FHIR `_valueAlternatives`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueAlternatives")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_alternatives_ext: Vec<Option<types::Element>>,

    /// If the element must be supported (discouraged - see obligations)
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

    /// Reason that this element is marked as a modifier
    pub is_modifier_reason: Option<types::String>,
    /// Primitive extension sibling for [`is_modifier_reason`](Self::is_modifier_reason) (FHIR `_isModifierReason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isModifierReason")]
    pub is_modifier_reason_ext: Option<types::Element>,

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
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    path: types::String,
    #[serde(rename = "_path")]
    path_ext: Option<types::Element>,
    #[serde(default)]
    representation:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r6::codes::PropertyRepresentation>>,
    #[serde(rename = "_representation")]
    #[serde(default)]
    representation_ext: Vec<Option<types::Element>>,
    slice_name: Option<types::String>,
    #[serde(rename = "_sliceName")]
    slice_name_ext: Option<types::Element>,
    slice_is_constraining: Option<types::Boolean>,
    #[serde(rename = "_sliceIsConstraining")]
    slice_is_constraining_ext: Option<types::Element>,
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
    comment: Option<types::Markdown>,
    #[serde(rename = "_comment")]
    comment_ext: Option<types::Element>,
    requirements: Option<types::Markdown>,
    #[serde(rename = "_requirements")]
    requirements_ext: Option<types::Element>,
    #[serde(default)]
    alias: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_alias")]
    #[serde(default)]
    alias_ext: Vec<Option<types::Element>>,
    min: Option<types::UnsignedInt>,
    #[serde(rename = "_min")]
    min_ext: Option<types::Element>,
    max: Option<types::String>,
    #[serde(rename = "_max")]
    max_ext: Option<types::Element>,
    base: Option<ElementDefinitionBase>,
    content_reference: Option<types::Uri>,
    #[serde(rename = "_contentReference")]
    content_reference_ext: Option<types::Element>,
    #[serde(default)]
    r#type: Vec<ElementDefinitionType>,
    #[serde(flatten)]
    default_value: crate::r6::choice::Slot<ElementDefinitionDefaultValue>,
    meaning_when_missing: Option<types::Markdown>,
    #[serde(rename = "_meaningWhenMissing")]
    meaning_when_missing_ext: Option<types::Element>,
    order_meaning: Option<types::String>,
    #[serde(rename = "_orderMeaning")]
    order_meaning_ext: Option<types::Element>,
    #[serde(flatten)]
    fixed: crate::r6::choice::Slot<ElementDefinitionFixed>,
    #[serde(flatten)]
    pattern: crate::r6::choice::Slot<ElementDefinitionPattern>,
    #[serde(default)]
    example: Vec<ElementDefinitionExample>,
    #[serde(flatten)]
    min_value: crate::r6::choice::Slot<ElementDefinitionMinValue>,
    #[serde(flatten)]
    max_value: crate::r6::choice::Slot<ElementDefinitionMaxValue>,
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
    must_have_value: Option<types::Boolean>,
    #[serde(rename = "_mustHaveValue")]
    must_have_value_ext: Option<types::Element>,
    #[serde(default)]
    value_alternatives: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_valueAlternatives")]
    #[serde(default)]
    value_alternatives_ext: Vec<Option<types::Element>>,
    must_support: Option<types::Boolean>,
    #[serde(rename = "_mustSupport")]
    must_support_ext: Option<types::Element>,
    is_modifier: Option<types::Boolean>,
    #[serde(rename = "_isModifier")]
    is_modifier_ext: Option<types::Element>,
    is_modifier_reason: Option<types::String>,
    #[serde(rename = "_isModifierReason")]
    is_modifier_reason_ext: Option<types::Element>,
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
            modifier_extension: v.modifier_extension,
            path: v.path,
            path_ext: v.path_ext,
            representation: v.representation,
            representation_ext: v.representation_ext,
            slice_name: v.slice_name,
            slice_name_ext: v.slice_name_ext,
            slice_is_constraining: v.slice_is_constraining,
            slice_is_constraining_ext: v.slice_is_constraining_ext,
            label: v.label,
            label_ext: v.label_ext,
            code: v.code,
            slicing: v.slicing,
            short: v.short,
            short_ext: v.short_ext,
            definition: v.definition,
            definition_ext: v.definition_ext,
            comment: v.comment,
            comment_ext: v.comment_ext,
            requirements: v.requirements,
            requirements_ext: v.requirements_ext,
            alias: v.alias,
            alias_ext: v.alias_ext,
            min: v.min,
            min_ext: v.min_ext,
            max: v.max,
            max_ext: v.max_ext,
            base: v.base,
            content_reference: v.content_reference,
            content_reference_ext: v.content_reference_ext,
            r#type: v.r#type,
            default_value: v.default_value.0,
            meaning_when_missing: v.meaning_when_missing,
            meaning_when_missing_ext: v.meaning_when_missing_ext,
            order_meaning: v.order_meaning,
            order_meaning_ext: v.order_meaning_ext,
            fixed: v.fixed.0,
            pattern: v.pattern.0,
            example: v.example,
            min_value: v.min_value.0,
            max_value: v.max_value.0,
            max_length: v.max_length,
            max_length_ext: v.max_length_ext,
            condition: v.condition,
            condition_ext: v.condition_ext,
            constraint: v.constraint,
            must_have_value: v.must_have_value,
            must_have_value_ext: v.must_have_value_ext,
            value_alternatives: v.value_alternatives,
            value_alternatives_ext: v.value_alternatives_ext,
            must_support: v.must_support,
            must_support_ext: v.must_support_ext,
            is_modifier: v.is_modifier,
            is_modifier_ext: v.is_modifier_ext,
            is_modifier_reason: v.is_modifier_reason,
            is_modifier_reason_ext: v.is_modifier_reason_ext,
            is_summary: v.is_summary,
            is_summary_ext: v.is_summary_ext,
            binding: v.binding,
            mapping: v.mapping,
        }
    }
}

/// Information about the base definition of the element, provided to make it
/// unnecessary for tools to trace the deviation of the element through the
/// derived and related profiles. When the element definition is not the
/// original definition of an element - e.g. either in a constraint on another
/// type, or for elements from a super type in a snap shot - then the
/// information in provided in the element definition may be different to the
/// base definition. On the original definition of the element, it will be
/// same.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionBase;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionBase {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionBase = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ElementDefinitionBase {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Path that identifies the base element
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Min cardinality of the base element
    pub min: types::UnsignedInt,
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
/// CodeableConcept, Quantity), or the data types (string, uri).
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionBinding;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionBinding {
///     value_set: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `valueSet` is the name this serializes to on the wire.
/// assert_eq!(json["valueSet"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: ElementDefinitionBinding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ElementDefinitionBinding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// required | extensible | preferred | example | descriptive
    pub strength: crate::coded::Coded<crate::r6::codes::BindingStrength>,
    /// Primitive extension sibling for [`strength`](Self::strength) (FHIR `_strength`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_strength")]
    pub strength_ext: Option<types::Element>,

    /// Guidance on the codes to be used
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Source of value set
    pub value_set: Option<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// Additional Bindings - more rules about the binding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional: Vec<ElementDefinitionBindingAdditional>,
}

/// Additional bindings that help applications implementing this element.
/// Additional bindings do not replace the main binding but provide more
/// information and/or context.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionBindingAdditional;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionBindingAdditional {
///     short_doco: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `shortDoco` is the name this serializes to on the wire.
/// assert_eq!(json["shortDoco"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionBindingAdditional = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ElementDefinitionBindingAdditional {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Unique identifier so additional bindings to be matched across profiles
    pub key: Option<types::Id>,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// maximum | minimum | required | extensible | candidate | current |
    /// preferred | ui | starter | component
    pub purpose: crate::coded::Coded<crate::r6::codes::AdditionalBindingPurpose>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// The value set for the additional binding
    pub value_set: types::Canonical,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,

    /// Documentation of the purpose of use of the binding
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Concise documentation - for summary tables
    pub short_doco: Option<types::String>,
    /// Primitive extension sibling for [`short_doco`](Self::short_doco) (FHIR `_shortDoco`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_shortDoco")]
    pub short_doco_ext: Option<types::Element>,

    /// Qualifies the usage - jurisdiction, gender, workflow status etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage: Vec<types::UsageContext>,

    /// Whether binding can applies to all repeats, or just one
    pub any: Option<types::Boolean>,
    /// Primitive extension sibling for [`any`](Self::any) (FHIR `_any`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_any")]
    pub any_ext: Option<types::Element>,
}

/// Formal constraints such as co-occurrence and other constraints that can be
/// computationally evaluated within the context of the instance.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionConstraint;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionConstraint {
///     requirements: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `requirements` is the name this serializes to on the wire.
/// assert_eq!(json["requirements"], ::serde_json::json!("# Heading"));
///
/// let back: ElementDefinitionConstraint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ElementDefinitionConstraint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Target of 'condition' reference above
    pub key: types::Id,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Why this constraint is necessary or appropriate
    pub requirements: Option<types::Markdown>,
    /// Primitive extension sibling for [`requirements`](Self::requirements) (FHIR `_requirements`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirements")]
    pub requirements_ext: Option<types::Element>,

    /// error | warning
    pub severity: crate::coded::Coded<crate::r6::codes::ConstraintSeverity>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// Suppress warning or hint in profile
    pub suppress: Option<types::Boolean>,
    /// Primitive extension sibling for [`suppress`](Self::suppress) (FHIR `_suppress`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_suppress")]
    pub suppress_ext: Option<types::Element>,

    /// Human description of constraint
    pub human: types::String,
    /// Primitive extension sibling for [`human`](Self::human) (FHIR `_human`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_human")]
    pub human_ext: Option<types::Element>,

    /// FHIRPath expression of constraint
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// Reference to original source of constraint
    pub source: Option<types::Canonical>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,
}

/// A sample value for this element demonstrating the type of information that
/// would typically be found in the element.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionExample;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionExample {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionExample = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ElementDefinitionExampleDe")]
#[fhir_version("r6")]
pub struct ElementDefinitionExample {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Describes the purpose of this example
    pub label: types::String,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Value of Example (one of allowed types)
    /// The `ElementDefinition.example.value[x]` choice element (1..1); see [`ElementDefinitionExampleValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<ElementDefinitionExampleValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDefinitionExampleDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    label: types::String,
    #[serde(rename = "_label")]
    label_ext: Option<types::Element>,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<ElementDefinitionExampleValue>,
}

impl ::core::convert::From<ElementDefinitionExampleDe> for ElementDefinitionExample {
    fn from(v: ElementDefinitionExampleDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            label: v.label,
            label_ext: v.label_ext,
            value: v.value.0,
        }
    }
}

/// Identifies a concept from an external specification that roughly
/// corresponds to this element.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionMapping;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionMapping {
///     comment: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comment` is the name this serializes to on the wire.
/// assert_eq!(json["comment"], ::serde_json::json!("# Heading"));
///
/// let back: ElementDefinitionMapping = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ElementDefinitionMapping {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
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

    /// Comments about the mapping or its use
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
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
/// use fhir::r6::types::element_definition::ElementDefinitionSlicing;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ElementDefinitionSlicing {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Element values that are used to distinguish the slices
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub discriminator: Vec<ElementDefinitionSlicingDiscriminator>,

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
    pub rules: crate::coded::Coded<crate::r6::codes::ResourceSlicingRules>,
    /// Primitive extension sibling for [`rules`](Self::rules) (FHIR `_rules`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rules")]
    pub rules_ext: Option<types::Element>,
}

/// Designates which child elements are used to discriminate between the slices
/// when processing an instance. If one or more discriminators are provided,
/// the values of the child elements in the instance data SHALL completely
/// distinguish which slice the element in the resource matches based on the
/// allowed values for those elements in each of the slices.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionSlicingDiscriminator;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionSlicingDiscriminator {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionSlicingDiscriminator = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ElementDefinitionSlicingDiscriminator {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// value | exists | type | profile | position
    pub r#type: crate::coded::Coded<crate::r6::codes::DiscriminatorType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Path to element value
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,
}

/// The data type or resource that the value of this element is permitted to
/// be.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::element_definition::ElementDefinitionType;
/// use fhir::r6::types;
///
/// let value = ElementDefinitionType {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ElementDefinitionType = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ElementDefinitionType {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Data type or Resource (reference to definition)
    pub code: types::Uri,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Profiles (StructureDefinition or IG) - one must apply
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub profile: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// Profile (StructureDefinition or IG) on the Reference/canonical target -
    /// one must apply
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub target_profile: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`target_profile`](Self::target_profile) (FHIR `_targetProfile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetProfile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_profile_ext: Vec<Option<types::Element>>,

    /// contained | referenced | bundled - how aggregated
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub aggregation:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r6::codes::ResourceAggregationMode>>,
    /// Primitive extension sibling for [`aggregation`](Self::aggregation) (FHIR `_aggregation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_aggregation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aggregation_ext: Vec<Option<types::Element>>,

    /// either | independent | specific
    pub versioning: Option<crate::coded::Coded<crate::r6::codes::ReferenceVersionRules>>,
    /// Primitive extension sibling for [`versioning`](Self::versioning) (FHIR `_versioning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_versioning")]
    pub versioning_ext: Option<types::Element>,
}

/// The `ElementDefinition.defaultValue[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionDefaultValue {
    /// `defaultValueBase64Binary` variant.
    #[fhir("defaultValueBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `defaultValueBoolean` variant.
    #[fhir("defaultValueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `defaultValueCanonical` variant.
    #[fhir("defaultValueCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `defaultValueCode` variant.
    #[fhir("defaultValueCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `defaultValueDate` variant.
    #[fhir("defaultValueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `defaultValueDateTime` variant.
    #[fhir("defaultValueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `defaultValueDecimal` variant.
    #[fhir("defaultValueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `defaultValueId` variant.
    #[fhir("defaultValueId")]
    Id(crate::r6::choice::Primitive<types::Id>),
    /// `defaultValueInstant` variant.
    #[fhir("defaultValueInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `defaultValueInteger` variant.
    #[fhir("defaultValueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `defaultValueInteger64` variant.
    #[fhir("defaultValueInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `defaultValueMarkdown` variant.
    #[fhir("defaultValueMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `defaultValueOid` variant.
    #[fhir("defaultValueOid")]
    Oid(crate::r6::choice::Primitive<types::Oid>),
    /// `defaultValuePositiveInt` variant.
    #[fhir("defaultValuePositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `defaultValueString` variant.
    #[fhir("defaultValueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `defaultValueTime` variant.
    #[fhir("defaultValueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `defaultValueUnsignedInt` variant.
    #[fhir("defaultValueUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `defaultValueUri` variant.
    #[fhir("defaultValueUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `defaultValueUrl` variant.
    #[fhir("defaultValueUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `defaultValueUuid` variant.
    #[fhir("defaultValueUuid")]
    Uuid(crate::r6::choice::Primitive<types::Uuid>),
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
    /// `defaultValueCodeableReference` variant.
    #[fhir("defaultValueCodeableReference")]
    CodeableReference(Box<types::CodeableReference>),
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
    /// `defaultValueRatioRange` variant.
    #[fhir("defaultValueRatioRange")]
    RatioRange(Box<types::RatioRange>),
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
    /// `defaultValueContactDetail` variant.
    #[fhir("defaultValueContactDetail")]
    ContactDetail(Box<types::ContactDetail>),
    /// `defaultValueDataRequirement` variant.
    #[fhir("defaultValueDataRequirement")]
    DataRequirement(Box<types::DataRequirement>),
    /// `defaultValueExpression` variant.
    #[fhir("defaultValueExpression")]
    Expression(Box<types::Expression>),
    /// `defaultValueParameterDefinition` variant.
    #[fhir("defaultValueParameterDefinition")]
    ParameterDefinition(Box<types::ParameterDefinition>),
    /// `defaultValueRelatedArtifact` variant.
    #[fhir("defaultValueRelatedArtifact")]
    RelatedArtifact(Box<types::RelatedArtifact>),
    /// `defaultValueTriggerDefinition` variant.
    #[fhir("defaultValueTriggerDefinition")]
    TriggerDefinition(Box<types::TriggerDefinition>),
    /// `defaultValueUsageContext` variant.
    #[fhir("defaultValueUsageContext")]
    UsageContext(Box<types::UsageContext>),
    /// `defaultValueAvailability` variant.
    #[fhir("defaultValueAvailability")]
    Availability(Box<types::Availability>),
    /// `defaultValueExtendedContactDetail` variant.
    #[fhir("defaultValueExtendedContactDetail")]
    ExtendedContactDetail(Box<types::ExtendedContactDetail>),
    /// `defaultValueDosage` variant.
    #[fhir("defaultValueDosage")]
    Dosage(Box<types::Dosage>),
    /// `defaultValueMeta` variant.
    #[fhir("defaultValueMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.fixed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionFixed {
    /// `fixedBase64Binary` variant.
    #[fhir("fixedBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `fixedBoolean` variant.
    #[fhir("fixedBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `fixedCanonical` variant.
    #[fhir("fixedCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `fixedCode` variant.
    #[fhir("fixedCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `fixedDate` variant.
    #[fhir("fixedDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `fixedDateTime` variant.
    #[fhir("fixedDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `fixedDecimal` variant.
    #[fhir("fixedDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `fixedId` variant.
    #[fhir("fixedId")]
    Id(crate::r6::choice::Primitive<types::Id>),
    /// `fixedInstant` variant.
    #[fhir("fixedInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `fixedInteger` variant.
    #[fhir("fixedInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `fixedInteger64` variant.
    #[fhir("fixedInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `fixedMarkdown` variant.
    #[fhir("fixedMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `fixedOid` variant.
    #[fhir("fixedOid")]
    Oid(crate::r6::choice::Primitive<types::Oid>),
    /// `fixedPositiveInt` variant.
    #[fhir("fixedPositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `fixedString` variant.
    #[fhir("fixedString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `fixedTime` variant.
    #[fhir("fixedTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `fixedUnsignedInt` variant.
    #[fhir("fixedUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `fixedUri` variant.
    #[fhir("fixedUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `fixedUrl` variant.
    #[fhir("fixedUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `fixedUuid` variant.
    #[fhir("fixedUuid")]
    Uuid(crate::r6::choice::Primitive<types::Uuid>),
    /// `fixedAddress` variant.
    #[fhir("fixedAddress")]
    Address(Box<types::Address>),
    /// `fixedAge` variant.
    #[fhir("fixedAge")]
    Age(Box<types::Age>),
    /// `fixedAnnotation` variant.
    #[fhir("fixedAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `fixedAttachment` variant.
    #[fhir("fixedAttachment")]
    Attachment(Box<types::Attachment>),
    /// `fixedCodeableConcept` variant.
    #[fhir("fixedCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `fixedCodeableReference` variant.
    #[fhir("fixedCodeableReference")]
    CodeableReference(Box<types::CodeableReference>),
    /// `fixedCoding` variant.
    #[fhir("fixedCoding")]
    Coding(Box<types::Coding>),
    /// `fixedContactPoint` variant.
    #[fhir("fixedContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `fixedCount` variant.
    #[fhir("fixedCount")]
    Count(Box<types::Count>),
    /// `fixedDistance` variant.
    #[fhir("fixedDistance")]
    Distance(Box<types::Distance>),
    /// `fixedDuration` variant.
    #[fhir("fixedDuration")]
    Duration(Box<types::Duration>),
    /// `fixedHumanName` variant.
    #[fhir("fixedHumanName")]
    HumanName(Box<types::HumanName>),
    /// `fixedIdentifier` variant.
    #[fhir("fixedIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `fixedMoney` variant.
    #[fhir("fixedMoney")]
    Money(Box<types::Money>),
    /// `fixedPeriod` variant.
    #[fhir("fixedPeriod")]
    Period(Box<types::Period>),
    /// `fixedQuantity` variant.
    #[fhir("fixedQuantity")]
    Quantity(Box<types::Quantity>),
    /// `fixedRange` variant.
    #[fhir("fixedRange")]
    Range(Box<types::Range>),
    /// `fixedRatio` variant.
    #[fhir("fixedRatio")]
    Ratio(Box<types::Ratio>),
    /// `fixedRatioRange` variant.
    #[fhir("fixedRatioRange")]
    RatioRange(Box<types::RatioRange>),
    /// `fixedReference` variant.
    #[fhir("fixedReference")]
    Reference(Box<types::Reference>),
    /// `fixedSampledData` variant.
    #[fhir("fixedSampledData")]
    SampledData(Box<types::SampledData>),
    /// `fixedSignature` variant.
    #[fhir("fixedSignature")]
    Signature(Box<types::Signature>),
    /// `fixedTiming` variant.
    #[fhir("fixedTiming")]
    Timing(Box<types::Timing>),
    /// `fixedContactDetail` variant.
    #[fhir("fixedContactDetail")]
    ContactDetail(Box<types::ContactDetail>),
    /// `fixedDataRequirement` variant.
    #[fhir("fixedDataRequirement")]
    DataRequirement(Box<types::DataRequirement>),
    /// `fixedExpression` variant.
    #[fhir("fixedExpression")]
    Expression(Box<types::Expression>),
    /// `fixedParameterDefinition` variant.
    #[fhir("fixedParameterDefinition")]
    ParameterDefinition(Box<types::ParameterDefinition>),
    /// `fixedRelatedArtifact` variant.
    #[fhir("fixedRelatedArtifact")]
    RelatedArtifact(Box<types::RelatedArtifact>),
    /// `fixedTriggerDefinition` variant.
    #[fhir("fixedTriggerDefinition")]
    TriggerDefinition(Box<types::TriggerDefinition>),
    /// `fixedUsageContext` variant.
    #[fhir("fixedUsageContext")]
    UsageContext(Box<types::UsageContext>),
    /// `fixedAvailability` variant.
    #[fhir("fixedAvailability")]
    Availability(Box<types::Availability>),
    /// `fixedExtendedContactDetail` variant.
    #[fhir("fixedExtendedContactDetail")]
    ExtendedContactDetail(Box<types::ExtendedContactDetail>),
    /// `fixedDosage` variant.
    #[fhir("fixedDosage")]
    Dosage(Box<types::Dosage>),
    /// `fixedMeta` variant.
    #[fhir("fixedMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.pattern[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionPattern {
    /// `patternBase64Binary` variant.
    #[fhir("patternBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `patternBoolean` variant.
    #[fhir("patternBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `patternCanonical` variant.
    #[fhir("patternCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `patternCode` variant.
    #[fhir("patternCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `patternDate` variant.
    #[fhir("patternDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `patternDateTime` variant.
    #[fhir("patternDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `patternDecimal` variant.
    #[fhir("patternDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `patternId` variant.
    #[fhir("patternId")]
    Id(crate::r6::choice::Primitive<types::Id>),
    /// `patternInstant` variant.
    #[fhir("patternInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `patternInteger` variant.
    #[fhir("patternInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `patternInteger64` variant.
    #[fhir("patternInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `patternMarkdown` variant.
    #[fhir("patternMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `patternOid` variant.
    #[fhir("patternOid")]
    Oid(crate::r6::choice::Primitive<types::Oid>),
    /// `patternPositiveInt` variant.
    #[fhir("patternPositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `patternString` variant.
    #[fhir("patternString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `patternTime` variant.
    #[fhir("patternTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `patternUnsignedInt` variant.
    #[fhir("patternUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `patternUri` variant.
    #[fhir("patternUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `patternUrl` variant.
    #[fhir("patternUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `patternUuid` variant.
    #[fhir("patternUuid")]
    Uuid(crate::r6::choice::Primitive<types::Uuid>),
    /// `patternAddress` variant.
    #[fhir("patternAddress")]
    Address(Box<types::Address>),
    /// `patternAge` variant.
    #[fhir("patternAge")]
    Age(Box<types::Age>),
    /// `patternAnnotation` variant.
    #[fhir("patternAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `patternAttachment` variant.
    #[fhir("patternAttachment")]
    Attachment(Box<types::Attachment>),
    /// `patternCodeableConcept` variant.
    #[fhir("patternCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `patternCodeableReference` variant.
    #[fhir("patternCodeableReference")]
    CodeableReference(Box<types::CodeableReference>),
    /// `patternCoding` variant.
    #[fhir("patternCoding")]
    Coding(Box<types::Coding>),
    /// `patternContactPoint` variant.
    #[fhir("patternContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `patternCount` variant.
    #[fhir("patternCount")]
    Count(Box<types::Count>),
    /// `patternDistance` variant.
    #[fhir("patternDistance")]
    Distance(Box<types::Distance>),
    /// `patternDuration` variant.
    #[fhir("patternDuration")]
    Duration(Box<types::Duration>),
    /// `patternHumanName` variant.
    #[fhir("patternHumanName")]
    HumanName(Box<types::HumanName>),
    /// `patternIdentifier` variant.
    #[fhir("patternIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `patternMoney` variant.
    #[fhir("patternMoney")]
    Money(Box<types::Money>),
    /// `patternPeriod` variant.
    #[fhir("patternPeriod")]
    Period(Box<types::Period>),
    /// `patternQuantity` variant.
    #[fhir("patternQuantity")]
    Quantity(Box<types::Quantity>),
    /// `patternRange` variant.
    #[fhir("patternRange")]
    Range(Box<types::Range>),
    /// `patternRatio` variant.
    #[fhir("patternRatio")]
    Ratio(Box<types::Ratio>),
    /// `patternRatioRange` variant.
    #[fhir("patternRatioRange")]
    RatioRange(Box<types::RatioRange>),
    /// `patternReference` variant.
    #[fhir("patternReference")]
    Reference(Box<types::Reference>),
    /// `patternSampledData` variant.
    #[fhir("patternSampledData")]
    SampledData(Box<types::SampledData>),
    /// `patternSignature` variant.
    #[fhir("patternSignature")]
    Signature(Box<types::Signature>),
    /// `patternTiming` variant.
    #[fhir("patternTiming")]
    Timing(Box<types::Timing>),
    /// `patternContactDetail` variant.
    #[fhir("patternContactDetail")]
    ContactDetail(Box<types::ContactDetail>),
    /// `patternDataRequirement` variant.
    #[fhir("patternDataRequirement")]
    DataRequirement(Box<types::DataRequirement>),
    /// `patternExpression` variant.
    #[fhir("patternExpression")]
    Expression(Box<types::Expression>),
    /// `patternParameterDefinition` variant.
    #[fhir("patternParameterDefinition")]
    ParameterDefinition(Box<types::ParameterDefinition>),
    /// `patternRelatedArtifact` variant.
    #[fhir("patternRelatedArtifact")]
    RelatedArtifact(Box<types::RelatedArtifact>),
    /// `patternTriggerDefinition` variant.
    #[fhir("patternTriggerDefinition")]
    TriggerDefinition(Box<types::TriggerDefinition>),
    /// `patternUsageContext` variant.
    #[fhir("patternUsageContext")]
    UsageContext(Box<types::UsageContext>),
    /// `patternAvailability` variant.
    #[fhir("patternAvailability")]
    Availability(Box<types::Availability>),
    /// `patternExtendedContactDetail` variant.
    #[fhir("patternExtendedContactDetail")]
    ExtendedContactDetail(Box<types::ExtendedContactDetail>),
    /// `patternDosage` variant.
    #[fhir("patternDosage")]
    Dosage(Box<types::Dosage>),
    /// `patternMeta` variant.
    #[fhir("patternMeta")]
    Meta(Box<types::Meta>),
}

/// The `ElementDefinition.minValue[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionMinValue {
    /// `minValueDate` variant.
    #[fhir("minValueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `minValueDateTime` variant.
    #[fhir("minValueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `minValueInstant` variant.
    #[fhir("minValueInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `minValueTime` variant.
    #[fhir("minValueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `minValueDecimal` variant.
    #[fhir("minValueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `minValueInteger` variant.
    #[fhir("minValueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `minValueInteger64` variant.
    #[fhir("minValueInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `minValuePositiveInt` variant.
    #[fhir("minValuePositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `minValueUnsignedInt` variant.
    #[fhir("minValueUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `minValueQuantity` variant.
    #[fhir("minValueQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `ElementDefinition.maxValue[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionMaxValue {
    /// `maxValueDate` variant.
    #[fhir("maxValueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `maxValueDateTime` variant.
    #[fhir("maxValueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `maxValueInstant` variant.
    #[fhir("maxValueInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `maxValueTime` variant.
    #[fhir("maxValueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `maxValueDecimal` variant.
    #[fhir("maxValueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `maxValueInteger` variant.
    #[fhir("maxValueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `maxValueInteger64` variant.
    #[fhir("maxValueInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `maxValuePositiveInt` variant.
    #[fhir("maxValuePositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `maxValueUnsignedInt` variant.
    #[fhir("maxValueUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `maxValueQuantity` variant.
    #[fhir("maxValueQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `ElementDefinition.example.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ElementDefinitionExampleValue {
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueCanonical` variant.
    #[fhir("valueCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r6::choice::Primitive<types::Id>),
    /// `valueInstant` variant.
    #[fhir("valueInstant")]
    Instant(crate::r6::choice::Primitive<types::Instant>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueInteger64` variant.
    #[fhir("valueInteger64")]
    Integer64(crate::r6::choice::Primitive<types::Integer64>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `valueOid` variant.
    #[fhir("valueOid")]
    Oid(crate::r6::choice::Primitive<types::Oid>),
    /// `valuePositiveInt` variant.
    #[fhir("valuePositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `valueUnsignedInt` variant.
    #[fhir("valueUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `valueUrl` variant.
    #[fhir("valueUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `valueUuid` variant.
    #[fhir("valueUuid")]
    Uuid(crate::r6::choice::Primitive<types::Uuid>),
    /// `valueAddress` variant.
    #[fhir("valueAddress")]
    Address(Box<types::Address>),
    /// `valueAge` variant.
    #[fhir("valueAge")]
    Age(Box<types::Age>),
    /// `valueAnnotation` variant.
    #[fhir("valueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueCodeableReference` variant.
    #[fhir("valueCodeableReference")]
    CodeableReference(Box<types::CodeableReference>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueContactPoint` variant.
    #[fhir("valueContactPoint")]
    ContactPoint(Box<types::ContactPoint>),
    /// `valueCount` variant.
    #[fhir("valueCount")]
    Count(Box<types::Count>),
    /// `valueDistance` variant.
    #[fhir("valueDistance")]
    Distance(Box<types::Distance>),
    /// `valueDuration` variant.
    #[fhir("valueDuration")]
    Duration(Box<types::Duration>),
    /// `valueHumanName` variant.
    #[fhir("valueHumanName")]
    HumanName(Box<types::HumanName>),
    /// `valueIdentifier` variant.
    #[fhir("valueIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `valueMoney` variant.
    #[fhir("valueMoney")]
    Money(Box<types::Money>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueRatioRange` variant.
    #[fhir("valueRatioRange")]
    RatioRange(Box<types::RatioRange>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueSampledData` variant.
    #[fhir("valueSampledData")]
    SampledData(Box<types::SampledData>),
    /// `valueSignature` variant.
    #[fhir("valueSignature")]
    Signature(Box<types::Signature>),
    /// `valueTiming` variant.
    #[fhir("valueTiming")]
    Timing(Box<types::Timing>),
    /// `valueContactDetail` variant.
    #[fhir("valueContactDetail")]
    ContactDetail(Box<types::ContactDetail>),
    /// `valueDataRequirement` variant.
    #[fhir("valueDataRequirement")]
    DataRequirement(Box<types::DataRequirement>),
    /// `valueExpression` variant.
    #[fhir("valueExpression")]
    Expression(Box<types::Expression>),
    /// `valueParameterDefinition` variant.
    #[fhir("valueParameterDefinition")]
    ParameterDefinition(Box<types::ParameterDefinition>),
    /// `valueRelatedArtifact` variant.
    #[fhir("valueRelatedArtifact")]
    RelatedArtifact(Box<types::RelatedArtifact>),
    /// `valueTriggerDefinition` variant.
    #[fhir("valueTriggerDefinition")]
    TriggerDefinition(Box<types::TriggerDefinition>),
    /// `valueUsageContext` variant.
    #[fhir("valueUsageContext")]
    UsageContext(Box<types::UsageContext>),
    /// `valueAvailability` variant.
    #[fhir("valueAvailability")]
    Availability(Box<types::Availability>),
    /// `valueExtendedContactDetail` variant.
    #[fhir("valueExtendedContactDetail")]
    ExtendedContactDetail(Box<types::ExtendedContactDetail>),
    /// `valueDosage` variant.
    #[fhir("valueDosage")]
    Dosage(Box<types::Dosage>),
    /// `valueMeta` variant.
    #[fhir("valueMeta")]
    Meta(Box<types::Meta>),
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
