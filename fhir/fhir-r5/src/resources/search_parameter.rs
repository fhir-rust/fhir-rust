//! SearchParameter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
//!
//! Version: 5.0.0
//!
//! SearchParameter Resource: A search parameter that defines a named search item that can be used to search/filter on a resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A search parameter that defines a named search item that can be used to
/// search or filter on a resource. SearchParameter resources describe how a
/// FHIR server exposes queryable elements of a resource, mapping a search code
/// to a FHIRPath expression that extracts the matchable values. They are a
/// foundational part of the FHIR RESTful search framework and are commonly
/// published within implementation guides and capability statements.
///
/// Conceptually, a SearchParameter binds a short `code` (the token that
/// appears in a search URL, for example `?status=active`) to the underlying
/// element(s) of a resource that the code queries against, along with the
/// data `type` (such as token, date, reference, or composite) that governs
/// how the value is parsed and compared. Servers advertise the search
/// parameters they support for each resource type in their CapabilityStatement,
/// and clients use that information to construct valid search requests.
/// Composite search parameters combine two or more component parameters,
/// each with its own subexpression, to allow matching on the relationship
/// between values (for example, an observation's code and its value together).
///
/// # See also
///
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used for the jurisdiction of a search parameter.
/// - `CapabilityStatement`, which typically enumerates the search parameters a server supports.
/// - `Observation` and other resource types, which are the common subjects of search parameter definitions.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::search_parameter::SearchParameter;
///
/// let value = SearchParameter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SearchParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchParameter {
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

    /// Canonical identifier for this search parameter, represented as a globally unique URI used to reference it from other artifacts
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the search parameter (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the search parameter
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `SearchParameter.versionAlgorithm[x]` choice element (0..1); see [`SearchParameterVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<SearchParameterVersionAlgorithm>,

    /// Computer-friendly name for this search parameter, used as the base for the machine-readable name
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this search parameter (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Original definition for the search parameter
    pub derived_from: Option<types::Canonical>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`).
    #[serde(rename = "_derivedFrom")]
    pub derived_from_ext: Option<types::Element>,

    /// Publication status of the search parameter definition: draft | active | retired | unknown
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

    /// Natural language description of the search parameter, explaining what it matches and how it is intended to be used
    pub description: types::Markdown,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for search parameter (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this search parameter is defined
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

    /// Recommended name for the parameter as it appears in a search URL, for example `?code=...`
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// The resource type(s) this search parameter applies to, such as `Patient` or `Observation`
    pub base: vec1::Vec1<types::Code>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`).
    #[serde(rename = "_base")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub base_ext: Vec<Option<types::Element>>,

    /// Data type of the search parameter's value: number | date | string | token | reference | composite | quantity | uri | special
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::SearchParamType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// FHIRPath expression, relative to the resource, that extracts the values matched by this search parameter
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`).
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// normal | phonetic | other
    pub processing_mode: Option<crate::r5::coded::Coded<crate::r5::codes::SearchProcessingmode>>,
    /// Primitive extension sibling for [`processing_mode`](Self::processing_mode) (FHIR `_processingMode`).
    #[serde(rename = "_processingMode")]
    pub processing_mode_ext: Option<types::Element>,

    /// FHIRPath expression that constraints the usage of this SearchParamete
    pub constraint: Option<types::String>,
    /// Primitive extension sibling for [`constraint`](Self::constraint) (FHIR `_constraint`).
    #[serde(rename = "_constraint")]
    pub constraint_ext: Option<types::Element>,

    /// Types of resource (if a resource reference)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<types::Code>,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`).
    #[serde(rename = "_target")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_ext: Vec<Option<types::Element>>,

    /// Allow multiple values per parameter (or)
    pub multiple_or: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiple_or`](Self::multiple_or) (FHIR `_multipleOr`).
    #[serde(rename = "_multipleOr")]
    pub multiple_or_ext: Option<types::Element>,

    /// Allow multiple parameters (and)
    pub multiple_and: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiple_and`](Self::multiple_and) (FHIR `_multipleAnd`).
    #[serde(rename = "_multipleAnd")]
    pub multiple_and_ext: Option<types::Element>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparator: Vec<crate::r5::coded::Coded<crate::r5::codes::SearchComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`).
    #[serde(rename = "_comparator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparator_ext: Vec<Option<types::Element>>,

    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<crate::r5::coded::Coded<crate::r5::codes::SearchModifierCode>>,
    /// Primitive extension sibling for [`modifier`](Self::modifier) (FHIR `_modifier`).
    #[serde(rename = "_modifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_ext: Vec<Option<types::Element>>,

    /// Chained names supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain: Vec<types::String>,
    /// Primitive extension sibling for [`chain`](Self::chain) (FHIR `_chain`).
    #[serde(rename = "_chain")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain_ext: Vec<Option<types::Element>>,

    /// For Composite resources to define the parts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<SearchParameterComponent>,
}

/// For Composite resources to define the parts. Each component points to a
/// sub search parameter definition and provides a subexpression, relative to
/// the main expression, that yields the value for that part of the composite.
/// # Examples
///
/// ```
/// use fhir::r5::resources::search_parameter::SearchParameterComponent;
/// use fhir::r5::types;
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
pub struct SearchParameterComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Defines how the part works
    pub definition: types::Canonical,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`).
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Subexpression relative to main expression
    pub expression: types::String,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`).
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,
}

/// The `SearchParameter.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SearchParameterVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
