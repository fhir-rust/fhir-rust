//! SearchParameter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SearchParameter
//!
//! Version: 6.0.0-ballot3
//!
//! Search parameter for a resource
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A search parameter that defines a named search item that can be used to
/// search/filter on a resource.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::search_parameter::SearchParameter;
///
/// let value = SearchParameter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: SearchParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SearchParameterDe")]
#[fhir_version("r6")]
pub struct SearchParameter {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this search parameter, represented as a URI
    /// (globally unique)
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the search parameter (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the search parameter
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `SearchParameter.versionAlgorithm[x]` choice element (0..1); see [`SearchParameterVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<SearchParameterVersionAlgorithm>,

    /// Name for this search parameter (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this search parameter (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Original definition for the search parameter
    pub derived_from: Option<types::Canonical>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivedFrom")]
    pub derived_from_ext: Option<types::Element>,

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

    /// Natural language description of the search parameter
    pub description: types::Markdown,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// Recommended name for parameter in search url
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Additional recommended names for parameter in search url
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub alias_code: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`alias_code`](Self::alias_code) (FHIR `_aliasCode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_aliasCode")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_code_ext: Vec<Option<types::Element>>,

    /// The resource type(s) this search parameter applies to
    pub base: ::vec1::Vec1<types::Code>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_base")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub base_ext: Vec<Option<types::Element>>,

    /// number | date | string | token | reference | composite | quantity | uri
    /// | special | resource
    pub r#type: crate::coded::Coded<crate::r6::codes::SearchParamType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// FHIRPath expression that extracts the values
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// normal | phonetic | other
    pub processing_mode: Option<crate::coded::Coded<crate::r6::codes::SearchProcessingmode>>,
    /// Primitive extension sibling for [`processing_mode`](Self::processing_mode) (FHIR `_processingMode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_processingMode")]
    pub processing_mode_ext: Option<types::Element>,

    /// FHIRPath expression that constraints the usage of this SearchParamete
    pub constraint: Option<types::String>,
    /// Primitive extension sibling for [`constraint`](Self::constraint) (FHIR `_constraint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_constraint")]
    pub constraint_ext: Option<types::Element>,

    /// Types of resource (if a resource reference)
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub target: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_target")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_ext: Vec<Option<types::Element>>,

    /// Allow multiple values per parameter (or)
    pub multiple_or: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiple_or`](Self::multiple_or) (FHIR `_multipleOr`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_multipleOr")]
    pub multiple_or_ext: Option<types::Element>,

    /// Allow multiple parameters (and)
    pub multiple_and: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiple_and`](Self::multiple_and) (FHIR `_multipleAnd`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_multipleAnd")]
    pub multiple_and_ext: Option<types::Element>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub comparator: ::fhir_core::PrimVec<crate::coded::Coded<crate::r6::codes::SearchComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comparator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comparator_ext: Vec<Option<types::Element>>,

    /// missing | exact | contains | not | text | in | not-in | below | above |
    /// type | identifier | of-type | code-text | text-advanced | iterate
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub modifier: ::fhir_core::PrimVec<crate::coded::Coded<crate::r6::codes::SearchModifierCode>>,
    /// Primitive extension sibling for [`modifier`](Self::modifier) (FHIR `_modifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_ext: Vec<Option<types::Element>>,

    /// Chained names supported
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub chain: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`chain`](Self::chain) (FHIR `_chain`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_chain")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain_ext: Vec<Option<types::Element>>,

    /// For Composite resources to define the parts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<SearchParameterComponent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchParameterDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: types::Uri,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r6::choice::Slot<SearchParameterVersionAlgorithm>,
    name: types::String,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    derived_from: Option<types::Canonical>,
    #[serde(rename = "_derivedFrom")]
    derived_from_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    description: types::Markdown,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    code: types::Code,
    #[serde(rename = "_code")]
    code_ext: Option<types::Element>,
    #[serde(default)]
    alias_code: ::fhir_core::PrimVec<types::Code>,
    #[serde(rename = "_aliasCode")]
    #[serde(default)]
    alias_code_ext: Vec<Option<types::Element>>,
    base: ::vec1::Vec1<types::Code>,
    #[serde(rename = "_base")]
    #[serde(default)]
    base_ext: Vec<Option<types::Element>>,
    r#type: crate::coded::Coded<crate::r6::codes::SearchParamType>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    expression: Option<types::String>,
    #[serde(rename = "_expression")]
    expression_ext: Option<types::Element>,
    processing_mode: Option<crate::coded::Coded<crate::r6::codes::SearchProcessingmode>>,
    #[serde(rename = "_processingMode")]
    processing_mode_ext: Option<types::Element>,
    constraint: Option<types::String>,
    #[serde(rename = "_constraint")]
    constraint_ext: Option<types::Element>,
    #[serde(default)]
    target: ::fhir_core::PrimVec<types::Code>,
    #[serde(rename = "_target")]
    #[serde(default)]
    target_ext: Vec<Option<types::Element>>,
    multiple_or: Option<types::Boolean>,
    #[serde(rename = "_multipleOr")]
    multiple_or_ext: Option<types::Element>,
    multiple_and: Option<types::Boolean>,
    #[serde(rename = "_multipleAnd")]
    multiple_and_ext: Option<types::Element>,
    #[serde(default)]
    comparator: ::fhir_core::PrimVec<crate::coded::Coded<crate::r6::codes::SearchComparator>>,
    #[serde(rename = "_comparator")]
    #[serde(default)]
    comparator_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    modifier: ::fhir_core::PrimVec<crate::coded::Coded<crate::r6::codes::SearchModifierCode>>,
    #[serde(rename = "_modifier")]
    #[serde(default)]
    modifier_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    chain: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_chain")]
    #[serde(default)]
    chain_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    component: Vec<SearchParameterComponent>,
}

impl ::core::convert::From<SearchParameterDe> for SearchParameter {
    fn from(v: SearchParameterDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            derived_from: v.derived_from,
            derived_from_ext: v.derived_from_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            code: v.code,
            code_ext: v.code_ext,
            alias_code: v.alias_code,
            alias_code_ext: v.alias_code_ext,
            base: v.base,
            base_ext: v.base_ext,
            r#type: v.r#type,
            type_ext: v.type_ext,
            expression: v.expression,
            expression_ext: v.expression_ext,
            processing_mode: v.processing_mode,
            processing_mode_ext: v.processing_mode_ext,
            constraint: v.constraint,
            constraint_ext: v.constraint_ext,
            target: v.target,
            target_ext: v.target_ext,
            multiple_or: v.multiple_or,
            multiple_or_ext: v.multiple_or_ext,
            multiple_and: v.multiple_and,
            multiple_and_ext: v.multiple_and_ext,
            comparator: v.comparator,
            comparator_ext: v.comparator_ext,
            modifier: v.modifier,
            modifier_ext: v.modifier_ext,
            chain: v.chain,
            chain_ext: v.chain_ext,
            component: v.component,
        }
    }
}

/// Used to define the parts of a composite search parameter.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::search_parameter::SearchParameterComponent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Subexpression relative to main expression
    pub expression: types::String,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,
}

/// The `SearchParameter.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SearchParameterVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
