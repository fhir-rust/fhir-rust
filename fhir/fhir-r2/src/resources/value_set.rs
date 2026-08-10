//! ValueSet
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ValueSet
//!
//!
//!
//! A set of codes drawn from one or more code systems
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ValueSet Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSet;
/// use fhir::r2::types;
///
/// let value = ValueSet {
///     locked_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lockedDate` is the name this serializes to on the wire.
/// assert_eq!(json["lockedDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: ValueSet = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSet {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Globally unique logical identifier for value set
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the value set (e.g. HL7 v2 / CDA)
    pub identifier: Option<types::Identifier>,

    /// Logical identifier for this version of the value set
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Informal name for this value set
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// draft | active | retired
    pub status: crate::coded::Coded<crate::r2::codes::ConformanceResourceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details of the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ValueSetContact>,

    /// Date for given status
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Fixed date for all referenced code systems and value sets
    pub locked_date: Option<types::Date>,
    /// Primitive extension sibling for [`locked_date`](Self::locked_date) (FHIR `_lockedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lockedDate")]
    pub locked_date_ext: Option<types::Element>,

    /// Human language description of the value set
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::CodeableConcept>,

    /// Indicates whether or not any change to the content logical definition
    /// may occur
    pub immutable: Option<types::Boolean>,
    /// Primitive extension sibling for [`immutable`](Self::immutable) (FHIR `_immutable`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_immutable")]
    pub immutable_ext: Option<types::Element>,

    /// Why needed
    pub requirements: Option<types::String>,
    /// Primitive extension sibling for [`requirements`](Self::requirements) (FHIR `_requirements`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirements")]
    pub requirements_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::String>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Whether this is intended to be used with an extensible binding
    pub extensible: Option<types::Boolean>,
    /// Primitive extension sibling for [`extensible`](Self::extensible) (FHIR `_extensible`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_extensible")]
    pub extensible_ext: Option<types::Element>,

    /// An inline code system, which is part of this value set
    pub code_system: Option<ValueSetCodeSystem>,

    /// When value set includes codes from elsewhere
    pub compose: Option<ValueSetCompose>,

    /// Used when the value set is "expanded"
    pub expansion: Option<ValueSetExpansion>,
}

/// A definition of a code system, inlined into the value set (as a packaging
/// convenience). Note that the inline code system may be used from other value
/// sets by referring to its (codeSystem.system) directly.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::value_set::ValueSetCodeSystem;
///
/// let value = ValueSetCodeSystem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ValueSetCodeSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetCodeSystem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// URI to identify the code system (e.g. in Coding.system)
    pub system: types::Uri,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Version (for use in Coding.version)
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// If code comparison is case sensitive
    pub case_sensitive: Option<types::Boolean>,
    /// Primitive extension sibling for [`case_sensitive`](Self::case_sensitive) (FHIR `_caseSensitive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_caseSensitive")]
    pub case_sensitive_ext: Option<types::Element>,

    /// Concepts in the code system
    pub concept: ::vec1::Vec1<ValueSetCodeSystemConcept>,
}

/// Concepts that are in the code system. The concept definitions are
/// inherently hierarchical, but the definitions must be consulted to determine
/// what the meaning of the hierarchical relationships are.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetCodeSystemConcept;
/// use fhir::r2::types;
///
/// let value = ValueSetCodeSystemConcept {
///     r#abstract: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `abstract` is the name this serializes to on the wire.
/// assert_eq!(json["abstract"], ::serde_json::json!(true));
///
/// let back: ValueSetCodeSystemConcept = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetCodeSystemConcept {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that identifies concept
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// If this code is not for use as a real concept
    pub r#abstract: Option<types::Boolean>,
    /// Primitive extension sibling for [`r#abstract`](Self::r#abstract) (FHIR `_abstract`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_abstract")]
    pub abstract_ext: Option<types::Element>,

    /// Text to display to the user
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Formal definition
    pub definition: Option<types::String>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// Additional representations for the concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designation: Vec<ValueSetCodeSystemConceptDesignation>,

    /// Child Concepts (is-a/contains/categorizes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<ValueSetCodeSystemConcept>,
}

/// Additional representations for the concept - other languages, aliases,
/// specialized purposes, used for particular purposes, etc.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetCodeSystemConceptDesignation;
/// use fhir::r2::types;
///
/// let value = ValueSetCodeSystemConceptDesignation {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ValueSetCodeSystemConceptDesignation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetCodeSystemConceptDesignation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Human language of the designation
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Details how this designation would be used
    pub r#use: Option<types::Coding>,

    /// The text value for this designation
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A set of criteria that provide the content logical definition of the value
/// set by including or excluding codes from outside this value set.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetCompose;
/// use fhir::r2::types;
///
/// let value = ValueSetCompose {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ValueSetCompose = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetCompose {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Import the contents of another value set
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub import: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`import`](Self::import) (FHIR `_import`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_import")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub import_ext: Vec<Option<types::Element>>,

    /// Include one or more codes from a code system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<ValueSetComposeInclude>,

    /// Explicitly exclude codes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<ValueSetComposeInclude>,
}

/// Include one or more codes from a code system.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetComposeInclude;
/// use fhir::r2::types;
///
/// let value = ValueSetComposeInclude {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: ValueSetComposeInclude = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetComposeInclude {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The system the codes come from
    pub system: types::Uri,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Specific version of the code system referred to
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// A concept defined in the system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<ValueSetComposeIncludeConcept>,

    /// Select codes/concepts by their properties (including relationships)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<ValueSetComposeIncludeFilter>,
}

/// Specifies a concept to be included or excluded.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetComposeIncludeConcept;
/// use fhir::r2::types;
///
/// let value = ValueSetComposeIncludeConcept {
///     display: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `display` is the name this serializes to on the wire.
/// assert_eq!(json["display"], ::serde_json::json!("abc"));
///
/// let back: ValueSetComposeIncludeConcept = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetComposeIncludeConcept {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code or expression from system
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Test to display for this code for this value set
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Additional representations for this valueset
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designation: Vec<ValueSetCodeSystemConceptDesignation>,
}

/// Select concepts by specify a matching criteria based on the properties
/// (including relationships) defined by the system. If multiple filters are
/// specified, they SHALL all be true.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetComposeIncludeFilter;
/// use fhir::r2::types;
///
/// let value = ValueSetComposeIncludeFilter {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ValueSetComposeIncludeFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetComposeIncludeFilter {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A property defined by the code system
    pub property: types::Code,
    /// Primitive extension sibling for [`property`](Self::property) (FHIR `_property`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_property")]
    pub property_ext: Option<types::Element>,

    /// \= | is-a | is-not-a | regex | in | not-in
    pub op: crate::coded::Coded<crate::r2::codes::FilterOperator>,
    /// Primitive extension sibling for [`op`](Self::op) (FHIR `_op`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_op")]
    pub op_ext: Option<types::Element>,

    /// Code from the system, or regex criteria
    pub value: types::Code,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Contacts to assist a user in finding and communicating with the publisher.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetContact;
/// use fhir::r2::types;
///
/// let value = ValueSetContact {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ValueSetContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetContact {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of an individual to contact
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Contact details for individual or publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,
}

/// A value set can also be "expanded", where the value set is turned into a
/// simple collection of enumerated codes. This element holds the expansion, if
/// it has been performed.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetExpansion;
/// use fhir::r2::types;
///
/// let value = ValueSetExpansion {
///     total: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `total` is the name this serializes to on the wire.
/// assert_eq!(json["total"], ::serde_json::json!(42));
///
/// let back: ValueSetExpansion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetExpansion {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Uniquely identifies this expansion
    pub identifier: types::Uri,
    /// Primitive extension sibling for [`identifier`](Self::identifier) (FHIR `_identifier`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_identifier")]
    pub identifier_ext: Option<types::Element>,

    /// Time ValueSet expansion happened
    pub timestamp: types::DateTime,
    /// Primitive extension sibling for [`timestamp`](Self::timestamp) (FHIR `_timestamp`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_timestamp")]
    pub timestamp_ext: Option<types::Element>,

    /// Total number of codes in the expansion
    pub total: Option<types::Integer>,
    /// Primitive extension sibling for [`total`](Self::total) (FHIR `_total`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_total")]
    pub total_ext: Option<types::Element>,

    /// Offset at which this resource starts
    pub offset: Option<types::Integer>,
    /// Primitive extension sibling for [`offset`](Self::offset) (FHIR `_offset`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_offset")]
    pub offset_ext: Option<types::Element>,

    /// Parameter that controlled the expansion process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<ValueSetExpansionParameter>,

    /// Codes in the value set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contains: Vec<ValueSetExpansionContains>,
}

/// The codes that are contained in the value set expansion.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetExpansionContains;
/// use fhir::r2::types;
///
/// let value = ValueSetExpansionContains {
///     r#abstract: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `abstract` is the name this serializes to on the wire.
/// assert_eq!(json["abstract"], ::serde_json::json!(true));
///
/// let back: ValueSetExpansionContains = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ValueSetExpansionContains {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// System value for the code
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// If user cannot select this entry
    pub r#abstract: Option<types::Boolean>,
    /// Primitive extension sibling for [`r#abstract`](Self::r#abstract) (FHIR `_abstract`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_abstract")]
    pub abstract_ext: Option<types::Element>,

    /// Version in which this code/display is defined
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Code - if blank, this is not a selectable code
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// User display for the concept
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Codes contained under this entry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contains: Vec<ValueSetExpansionContains>,
}

/// A parameter that controlled the expansion process. These parameters may be
/// used by users of expanded value sets to check whether the expansion is
/// suitable for a particular purpose, or to pick the correct expansion.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::value_set::ValueSetExpansionParameter;
/// use fhir::r2::types;
///
/// let value = ValueSetExpansionParameter {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ValueSetExpansionParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ValueSetExpansionParameterDe")]
#[fhir_version("r2")]
pub struct ValueSetExpansionParameter {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name as assigned by the server
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Value of the named parameter
    /// The `ValueSet.expansion.parameter.value[x]` choice element (0..1); see [`ValueSetExpansionParameterValue`].
    #[serde(flatten)]
    pub value: Option<ValueSetExpansionParameterValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValueSetExpansionParameterDe {
    id: Option<types::Id>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    name: types::String,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    #[serde(flatten)]
    value: crate::r2::choice::Slot<ValueSetExpansionParameterValue>,
}

impl ::core::convert::From<ValueSetExpansionParameterDe> for ValueSetExpansionParameter {
    fn from(v: ValueSetExpansionParameterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            name: v.name,
            name_ext: v.name_ext,
            value: v.value.0,
        }
    }
}

/// The `ValueSet.expansion.parameter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ValueSetExpansionParameterValue {
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r2::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r2::choice::Primitive<types::Integer>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r2::choice::Primitive<types::Decimal>),
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r2::choice::Primitive<types::Uri>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r2::choice::Primitive<types::Code>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ValueSet;

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
