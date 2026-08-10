//! ValueSet
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ValueSet
//!
//! Version: 5.0.0
//!
//! ValueSet Resource: A ValueSet resource instance specifies a set of codes drawn from one or more code systems, intended for use in a particular context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link
/// between CodeSystem definitions and their use in coded elements, providing the
/// content logical definition (compose) that selects codes and an optional
/// pre-computed expansion listing the resulting concepts. In FHIR R5 it is a
/// canonical, publishable terminology artifact used throughout the specification
/// to constrain the allowed values of coded elements.
///
/// Clinically and administratively, a ValueSet is the mechanism by which
/// implementers bind a narrower, purpose-specific set of terms (for example,
/// permissible marital-status codes, or laboratory result codes for a
/// particular panel) to a coded data element on a resource, such as an
/// element bound via a terminology binding in a profile or extension.
/// Rather than reimplementing every concept from scratch, a value set's
/// `compose` element assembles its content by including or excluding codes
/// from one or more code systems (or by referencing other value sets), while
/// the optional `expansion` element records the fully enumerated list of
/// concepts that satisfy that definition at a point in time. Servers commonly
/// use the `$expand`, `$validate-code`, and `$lookup` terminology operations
/// against a ValueSet to drive form pick-lists, validate submitted codes, and
/// support decision support and reporting logic.
///
/// See also: value sets typically draw their content from one or more
/// [`CodeSystem`](crate::r5::resources::code_system::CodeSystem) resources,
/// and the codes they constrain are frequently carried on other resources
/// using the [`CodeableConcept`](crate::r5::types::CodeableConcept) and
/// [`Coding`](crate::r5::types::Coding) data types.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSet;
/// use fhir::r5::types;
///
/// let value = ValueSet {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: ValueSet = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet {
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

    /// Canonical identifier for this value set, represented as a URI (globally unique); used to reference this value set from bindings and other artifacts
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the value set (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the value set
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ValueSet.versionAlgorithm[x]` choice element (0..1); see [`ValueSetVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ValueSetVersionAlgorithm>,

    /// Name for this value set (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this value set (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Publication lifecycle status of this value set: draft | active | retired | unknown
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

    /// Natural language description of the value set
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for value set (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Indicates whether or not any change to the content logical definition may occur
    pub immutable: Option<types::Boolean>,
    /// Primitive extension sibling for [`immutable`](Self::immutable) (FHIR `_immutable`).
    #[serde(rename = "_immutable")]
    pub immutable_ext: Option<types::Element>,

    /// Why this value set is defined
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

    /// When the ValueSet was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the ValueSet was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the ValueSet is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// Who authored the ValueSet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the ValueSet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the ValueSet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the ValueSet
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Additional documentation, citations, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Content logical definition of the value set (CLD); the rules that determine which codes are included or excluded
    pub compose: Option<ValueSetCompose>,

    /// Used when the value set is "expanded"; holds the pre-computed, fully enumerated list of concepts
    pub expansion: Option<ValueSetExpansion>,

    /// Description of the semantic space the Value Set Expansion is intended to cover and should further clarify the text in ValueSet.description
    pub scope: Option<ValueSetScope>,
}

/// Content logical definition of the value set (CLD).
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::value_set::ValueSetCompose;
///
/// let value = ValueSetCompose::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ValueSetCompose = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetCompose {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Fixed date for references with no specified version (transitive)
    pub locked_date: Option<types::Date>,
    /// Primitive extension sibling for [`locked_date`](Self::locked_date) (FHIR `_lockedDate`).
    #[serde(rename = "_lockedDate")]
    pub locked_date_ext: Option<types::Element>,

    /// Whether inactive codes are in the value set
    pub inactive: Option<types::Boolean>,
    /// Primitive extension sibling for [`inactive`](Self::inactive) (FHIR `_inactive`).
    #[serde(rename = "_inactive")]
    pub inactive_ext: Option<types::Element>,

    /// Include one or more codes from a code system or other value set(s)
    pub include: vec1::Vec1<ValueSetComposeInclude>,

    /// Explicitly exclude codes from a code system or other value sets
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<ValueSetComposeInclude>,

    /// Property to return if client doesn't override
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<types::String>,
    /// Primitive extension sibling for [`property`](Self::property) (FHIR `_property`).
    #[serde(rename = "_property")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property_ext: Vec<Option<types::Element>>,
}

/// Include one or more codes from a code system or other value set(s).
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetComposeInclude;
/// use fhir::r5::types;
///
/// let value = ValueSetComposeInclude {
///     system: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `system` is the name this serializes to on the wire.
/// assert_eq!(json["system"], ::serde_json::json!("http://example.org"));
///
/// let back: ValueSetComposeInclude = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeInclude {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The system the codes come from
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Specific version of the code system referred to
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// A concept defined in the system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub concept: Vec<ValueSetComposeIncludeConcept>,

    /// Select codes/concepts by their properties (including relationships)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<ValueSetComposeIncludeFilter>,

    /// Select the contents included in this value set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_set: Vec<types::Canonical>,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_set_ext: Vec<Option<types::Element>>,

    /// A copyright statement for the specific code system included in the value set
    pub copyright: Option<types::String>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,
}

/// A concept defined in the system.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetComposeIncludeConcept;
/// use fhir::r5::types;
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
pub struct ValueSetComposeIncludeConcept {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code or expression from system
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Text to display for this code for this value set in this valueset
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Additional representations for this concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designation: Vec<ValueSetComposeIncludeConceptDesignation>,
}

/// Additional representations for this concept.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetComposeIncludeConceptDesignation;
/// use fhir::r5::types;
///
/// let value = ValueSetComposeIncludeConceptDesignation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ValueSetComposeIncludeConceptDesignation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeConceptDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Human language of the designation
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Types of uses of designations
    pub r#use: Option<types::Coding>,

    /// Additional ways how this designation would be used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_use: Vec<types::Coding>,

    /// The text value for this designation
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Select codes/concepts by their properties (including relationships).
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetComposeIncludeFilter;
/// use fhir::r5::types;
///
/// let value = ValueSetComposeIncludeFilter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ValueSetComposeIncludeFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A property/filter defined by the code system
    pub property: types::Code,
    /// Primitive extension sibling for [`property`](Self::property) (FHIR `_property`).
    #[serde(rename = "_property")]
    pub property_ext: Option<types::Element>,

    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | child-of | descendent-leaf | exists
    pub op: crate::r5::coded::Coded<crate::r5::codes::FilterOperator>,
    /// Primitive extension sibling for [`op`](Self::op) (FHIR `_op`).
    #[serde(rename = "_op")]
    pub op_ext: Option<types::Element>,

    /// Code from the system, or regex criteria, or boolean value for exists
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Used when the value set is "expanded".
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetExpansion;
/// use fhir::r5::types;
///
/// let value = ValueSetExpansion {
///     identifier: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `identifier` is the name this serializes to on the wire.
/// assert_eq!(json["identifier"], ::serde_json::json!("http://example.org"));
///
/// let back: ValueSetExpansion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies the value set expansion (business identifier)
    pub identifier: Option<types::Uri>,
    /// Primitive extension sibling for [`identifier`](Self::identifier) (FHIR `_identifier`).
    #[serde(rename = "_identifier")]
    pub identifier_ext: Option<types::Element>,

    /// Opaque urls for paging through expansion results
    pub next: Option<types::Uri>,
    /// Primitive extension sibling for [`next`](Self::next) (FHIR `_next`).
    #[serde(rename = "_next")]
    pub next_ext: Option<types::Element>,

    /// Time ValueSet expansion happened
    pub timestamp: types::DateTime,
    /// Primitive extension sibling for [`timestamp`](Self::timestamp) (FHIR `_timestamp`).
    #[serde(rename = "_timestamp")]
    pub timestamp_ext: Option<types::Element>,

    /// Total number of codes in the expansion
    pub total: Option<types::Integer>,
    /// Primitive extension sibling for [`total`](Self::total) (FHIR `_total`).
    #[serde(rename = "_total")]
    pub total_ext: Option<types::Element>,

    /// Offset at which this resource starts
    pub offset: Option<types::Integer>,
    /// Primitive extension sibling for [`offset`](Self::offset) (FHIR `_offset`).
    #[serde(rename = "_offset")]
    pub offset_ext: Option<types::Element>,

    /// Parameter that controlled the expansion process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<ValueSetExpansionParameter>,

    /// Additional information supplied about each concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ValueSetExpansionProperty>,

    /// Codes in the value set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contains: Vec<ValueSetExpansionContains>,
}

/// Parameter that controlled the expansion process.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetExpansionParameter;
/// use fhir::r5::types;
///
/// let value = ValueSetExpansionParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ValueSetExpansionParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name as assigned by the client or server
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The `ValueSet.expansion.parameter.value[x]` choice element (0..1); see [`ValueSetExpansionParameterValue`].
    #[serde(flatten)]
    pub value: Option<ValueSetExpansionParameterValue>,
}

/// Additional information supplied about each concept.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetExpansionProperty;
/// use fhir::r5::types;
///
/// let value = ValueSetExpansionProperty {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: ValueSetExpansionProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifies the property on the concepts, and when referred to in operations
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Formal identifier for the property
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`).
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,
}

/// Codes in the value set.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetExpansionContains;
/// use fhir::r5::types;
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
pub struct ValueSetExpansionContains {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// System value for the code
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// If user cannot select this entry
    pub r#abstract: Option<types::Boolean>,
    /// Primitive extension sibling for [`abstract`](Self::r#abstract) (FHIR `_abstract`).
    #[serde(rename = "_abstract")]
    pub abstract_ext: Option<types::Element>,

    /// If concept is inactive in the code system
    pub inactive: Option<types::Boolean>,
    /// Primitive extension sibling for [`inactive`](Self::inactive) (FHIR `_inactive`).
    #[serde(rename = "_inactive")]
    pub inactive_ext: Option<types::Element>,

    /// Version in which this code/display is defined
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Code - if blank, this is not a selectable code
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// User display for the concept
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Additional representations for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub designation: Vec<ValueSetComposeIncludeConceptDesignation>,

    /// Property value for the concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ValueSetExpansionContainsProperty>,

    /// Codes contained under this entry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contains: Vec<ValueSetExpansionContains>,
}

/// Property value for the concept.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetExpansionContainsProperty;
/// use fhir::r5::types;
///
/// let value = ValueSetExpansionContainsProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ValueSetExpansionContainsProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContainsProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to ValueSet.expansion.property.code
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// The `ValueSet.expansion.contains.property.value[x]` choice element (0..1); see [`ValueSetExpansionContainsPropertyValue`].
    #[serde(flatten)]
    pub value: Option<ValueSetExpansionContainsPropertyValue>,

    /// SubProperty value for the concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_property: Vec<ValueSetExpansionContainsPropertySubProperty>,
}

/// SubProperty value for the concept.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetExpansionContainsPropertySubProperty;
/// use fhir::r5::types;
///
/// let value = ValueSetExpansionContainsPropertySubProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ValueSetExpansionContainsPropertySubProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContainsPropertySubProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to ValueSet.expansion.property.code
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// The `ValueSet.expansion.contains.property.subProperty.value[x]` choice element (0..1); see [`ValueSetExpansionContainsPropertySubPropertyValue`].
    #[serde(flatten)]
    pub value: Option<ValueSetExpansionContainsPropertySubPropertyValue>,
}

/// Description of the semantic space the Value Set Expansion is intended to cover.
/// # Examples
///
/// ```
/// use fhir::r5::resources::value_set::ValueSetScope;
/// use fhir::r5::types;
///
/// let value = ValueSetScope {
///     inclusion_criteria: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `inclusionCriteria` is the name this serializes to on the wire.
/// assert_eq!(json["inclusionCriteria"], ::serde_json::json!("abc"));
///
/// let back: ValueSetScope = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetScope {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Criteria describing which concepts or codes should be included and why
    pub inclusion_criteria: Option<types::String>,
    /// Primitive extension sibling for [`inclusion_criteria`](Self::inclusion_criteria) (FHIR `_inclusionCriteria`).
    #[serde(rename = "_inclusionCriteria")]
    pub inclusion_criteria_ext: Option<types::Element>,

    /// Criteria describing which concepts or codes should be excluded and why
    pub exclusion_criteria: Option<types::String>,
    /// Primitive extension sibling for [`exclusion_criteria`](Self::exclusion_criteria) (FHIR `_exclusionCriteria`).
    #[serde(rename = "_exclusionCriteria")]
    pub exclusion_criteria_ext: Option<types::Element>,
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
/// The `ValueSet.expansion.contains.property.subProperty.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ValueSetExpansionContainsPropertySubPropertyValue {
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r5::choice::Primitive<types::Code>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
}

/// The `ValueSet.expansion.contains.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ValueSetExpansionContainsPropertyValue {
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r5::choice::Primitive<types::Code>),
    /// `valueCoding` variant.
    #[fhir("valueCoding")]
    Coding(Box<types::Coding>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
}

/// The `ValueSet.expansion.parameter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ValueSetExpansionParameterValue {
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
    /// `valueUri` variant.
    #[fhir("valueUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `valueCode` variant.
    #[fhir("valueCode")]
    Code(crate::r5::choice::Primitive<types::Code>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}

/// The `ValueSet.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ValueSetVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
