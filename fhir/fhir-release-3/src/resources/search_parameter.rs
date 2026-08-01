//! SearchParameter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
//!
//!
//!
//! Search Parameter for a resource
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for SearchParameter Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::search_parameter::SearchParameter;
///
/// let value = SearchParameter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SearchParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct SearchParameter {
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

    /// Logical URI to reference this search parameter (globally unique)
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business version of the search parameter
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this search parameter (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

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

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for search parameter (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this search parameter is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Code used in URL
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// The resource type(s) this search parameter applies to
    pub base: ::vec1::Vec1<crate::coded::Coded<crate::r3::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_base")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub base_ext: Vec<Option<types::Element>>,

    /// number | date | string | token | reference | composite | quantity | uri
    pub r#type: crate::coded::Coded<crate::r3::codes::SearchParamType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Original Definition for the search parameter
    pub derived_from: Option<types::Uri>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivedFrom")]
    pub derived_from_ext: Option<types::Element>,

    /// Natural language description of the search parameter
    pub description: types::Markdown,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// FHIRPath expression that extracts the values
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// XPath that extracts the values
    pub xpath: Option<types::String>,
    /// Primitive extension sibling for [`xpath`](Self::xpath) (FHIR `_xpath`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_xpath")]
    pub xpath_ext: Option<types::Element>,

    /// normal | phonetic | nearby | distance | other
    pub xpath_usage: Option<crate::coded::Coded<crate::r3::codes::SearchXpathUsage>>,
    /// Primitive extension sibling for [`xpath_usage`](Self::xpath_usage) (FHIR `_xpathUsage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_xpathUsage")]
    pub xpath_usage_ext: Option<types::Element>,

    /// Types of resource (if a resource reference)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<crate::coded::Coded<crate::r3::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_target")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_ext: Vec<Option<types::Element>>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparator: Vec<crate::coded::Coded<crate::r3::codes::SearchComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comparator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparator_ext: Vec<Option<types::Element>>,

    /// missing | exact | contains | not | text | in | not-in | below | above |
    /// type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<crate::coded::Coded<crate::r3::codes::SearchModifierCode>>,
    /// Primitive extension sibling for [`modifier`](Self::modifier) (FHIR `_modifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_ext: Vec<Option<types::Element>>,

    /// Chained names supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain: Vec<types::String>,
    /// Primitive extension sibling for [`chain`](Self::chain) (FHIR `_chain`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_chain")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain_ext: Vec<Option<types::Element>>,

    /// For Composite resources to define the parts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<SearchParameterComponent>,
}

/// Used to define the parts of a composite search parameter.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::search_parameter::SearchParameterComponent;
/// use fhir::r3::types;
///
/// let value = SearchParameterComponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SearchParameterComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct SearchParameterComponent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Defines how the part works
    pub definition: types::Reference,

    /// Subexpression relative to main expression
    pub expression: types::String,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,
}
