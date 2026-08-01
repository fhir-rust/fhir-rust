//! TerminologyCapabilities
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TerminologyCapabilities
//!
//! Version: 5.0.0
//!
//! TerminologyCapabilities Resource: A statement of the capabilities (behaviors) of a FHIR Terminology Server.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A TerminologyCapabilities resource documents a set of capabilities
/// (behaviors) of a FHIR Terminology Server that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation. In FHIR R5 it describes which code systems, value set
/// expansion behaviors, and terminology operations (such as `$validate-code`,
/// `$translate`, and `$closure`) a server supports. It is the terminology
/// analogue of the CapabilityStatement resource and is used for conformance
/// testing and capability negotiation between clients and servers.
///
/// Publishers use `TerminologyCapabilities` to advertise, for a given
/// terminology server, which code systems and their versions are known,
/// whether value set expansion supports paging or hierarchical results,
/// whether concept subsumption testing is available, and how the
/// `ValueSet/$validate-code`, `ConceptMap/$translate`, and
/// `ConceptMap/$closure` operations behave. A `kind` of `instance` describes
/// a specific running server, `capability` describes a general software
/// capability, and `requirements` describes a desired set of capabilities
/// that an implementation should satisfy. Clients typically retrieve this
/// resource from a server's `/metadata` endpoint (or a related terminology
/// service discovery mechanism) to decide, ahead of time, whether the server
/// can support the terminology operations a workflow requires.
///
/// # Related resources
///
/// - Terminology operations described here typically operate on
///   [`CodeableConcept`](crate::r5::types::CodeableConcept) and `Coding`
///   values drawn from the code systems and value sets the server supports.
/// - This resource is the terminology-focused counterpart to the general
///   `CapabilityStatement` resource, and is commonly published alongside it.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilities;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilities {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: TerminologyCapabilities = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilities {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this terminology capabilities, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the terminology capabilities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the terminology capabilities
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `TerminologyCapabilities.versionAlgorithm[x]` choice element (0..1); see [`TerminologyCapabilitiesVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<TerminologyCapabilitiesVersionAlgorithm>,

    /// Name for this terminology capabilities (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this terminology capabilities (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Publication status of this statement: draft | active | retired | unknown
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
    pub date: types::DateTime,
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

    /// Natural language description of the terminology capabilities
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for terminology capabilities (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this terminology capabilities is defined
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

    /// The way this statement is intended to be used: instance | capability | requirements
    pub kind: crate::r5::coded::Coded<crate::r5::codes::CapabilityStatementKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Identifies the software product (name and version) that this terminology capability statement describes
    pub software: Option<TerminologyCapabilitiesSoftware>,

    /// Identifies a specific deployed instance of software this statement applies to, including its base URL
    pub implementation: Option<TerminologyCapabilitiesImplementation>,

    /// Whether lockedDate is supported
    pub locked_date: Option<types::Boolean>,
    /// Primitive extension sibling for [`locked_date`](Self::locked_date) (FHIR `_lockedDate`).
    #[serde(rename = "_lockedDate")]
    pub locked_date_ext: Option<types::Element>,

    /// A code system, with its supported versions and content mode, that the server can resolve terminology operations against
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_system: Vec<TerminologyCapabilitiesCodeSystem>,

    /// Information about the ValueSet/$expand operation
    pub expansion: Option<TerminologyCapabilitiesExpansion>,

    /// in-compose | in-expansion | in-compose-or-expansion
    pub code_search: Option<crate::r5::coded::Coded<crate::r5::codes::CodeSearchSupport>>,
    /// Primitive extension sibling for [`code_search`](Self::code_search) (FHIR `_codeSearch`).
    #[serde(rename = "_codeSearch")]
    pub code_search_ext: Option<types::Element>,

    /// Information about the ValueSet/$validate-code operation
    pub validate_code: Option<TerminologyCapabilitiesValidateCode>,

    /// Information about the ConceptMap/$translate operation
    pub translation: Option<TerminologyCapabilitiesTranslation>,

    /// Information about the ConceptMap/$closure operation
    pub closure: Option<TerminologyCapabilitiesClosure>,
}

/// Software that is covered by this terminology capability statement.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesSoftware;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesSoftware {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: TerminologyCapabilitiesSoftware = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesSoftware {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A name the software is known by
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Version covered by this statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,
}

/// If this describes a specific instance.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesImplementation;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesImplementation {
///     url: Some(types::Url("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: TerminologyCapabilitiesImplementation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesImplementation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Describes this specific instance
    pub description: types::String,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Base URL for the implementation
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// A code system supported by the server.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesCodeSystem;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesCodeSystem {
///     uri: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: TerminologyCapabilitiesCodeSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for the code system, represented as a URI
    pub uri: Option<types::Canonical>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`).
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Version of Code System supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<TerminologyCapabilitiesCodeSystemVersion>,

    /// not-present | example | fragment | complete | supplement
    pub content: crate::r5::coded::Coded<crate::r5::codes::CodesystemContentMode>,
    /// Primitive extension sibling for [`content`](Self::content) (FHIR `_content`).
    #[serde(rename = "_content")]
    pub content_ext: Option<types::Element>,

    /// Whether subsumption is supported
    pub subsumption: Option<types::Boolean>,
    /// Primitive extension sibling for [`subsumption`](Self::subsumption) (FHIR `_subsumption`).
    #[serde(rename = "_subsumption")]
    pub subsumption_ext: Option<types::Element>,
}

/// Version of Code System supported.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesCodeSystemVersion;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesCodeSystemVersion {
///     is_default: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isDefault` is the name this serializes to on the wire.
/// assert_eq!(json["isDefault"], ::serde_json::json!(true));
///
/// let back: TerminologyCapabilitiesCodeSystemVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystemVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Version identifier for this version
    pub code: Option<types::String>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// If this is the default version for this code system
    pub is_default: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_default`](Self::is_default) (FHIR `_isDefault`).
    #[serde(rename = "_isDefault")]
    pub is_default_ext: Option<types::Element>,

    /// If compositional grammar is supported
    pub compositional: Option<types::Boolean>,
    /// Primitive extension sibling for [`compositional`](Self::compositional) (FHIR `_compositional`).
    #[serde(rename = "_compositional")]
    pub compositional_ext: Option<types::Element>,

    /// Language Displays supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language: Vec<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_ext: Vec<Option<types::Element>>,

    /// Filter Properties supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<TerminologyCapabilitiesCodeSystemVersionFilter>,

    /// Properties supported for $lookup
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<types::Code>,
    /// Primitive extension sibling for [`property`](Self::property) (FHIR `_property`).
    #[serde(rename = "_property")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property_ext: Vec<Option<types::Element>>,
}

/// Filter Properties supported.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesCodeSystemVersionFilter;
///
/// let value = TerminologyCapabilitiesCodeSystemVersionFilter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TerminologyCapabilitiesCodeSystemVersionFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesCodeSystemVersionFilter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code of the property supported
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Operations supported for the property
    pub op: vec1::Vec1<types::Code>,
    /// Primitive extension sibling for [`op`](Self::op) (FHIR `_op`).
    #[serde(rename = "_op")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub op_ext: Vec<Option<types::Element>>,
}

/// Information about the ValueSet/$expand operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesExpansion;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesExpansion {
///     text_filter: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `textFilter` is the name this serializes to on the wire.
/// assert_eq!(json["textFilter"], ::serde_json::json!("# Heading"));
///
/// let back: TerminologyCapabilitiesExpansion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesExpansion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether the server can return nested value sets
    pub hierarchical: Option<types::Boolean>,
    /// Primitive extension sibling for [`hierarchical`](Self::hierarchical) (FHIR `_hierarchical`).
    #[serde(rename = "_hierarchical")]
    pub hierarchical_ext: Option<types::Element>,

    /// Whether the server supports paging on expansion
    pub paging: Option<types::Boolean>,
    /// Primitive extension sibling for [`paging`](Self::paging) (FHIR `_paging`).
    #[serde(rename = "_paging")]
    pub paging_ext: Option<types::Element>,

    /// Allow request for incomplete expansions?
    pub incomplete: Option<types::Boolean>,
    /// Primitive extension sibling for [`incomplete`](Self::incomplete) (FHIR `_incomplete`).
    #[serde(rename = "_incomplete")]
    pub incomplete_ext: Option<types::Element>,

    /// Supported expansion parameter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<TerminologyCapabilitiesExpansionParameter>,

    /// Documentation about text searching works
    pub text_filter: Option<types::Markdown>,
    /// Primitive extension sibling for [`text_filter`](Self::text_filter) (FHIR `_textFilter`).
    #[serde(rename = "_textFilter")]
    pub text_filter_ext: Option<types::Element>,
}

/// Supported expansion parameter.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesExpansionParameter;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesExpansionParameter {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: TerminologyCapabilitiesExpansionParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesExpansionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of the supported expansion parameter
    pub name: types::Code,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Description of support for parameter
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Information about the ValueSet/$validate-code operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesValidateCode;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesValidateCode {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TerminologyCapabilitiesValidateCode = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesValidateCode {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether translations are validated
    pub translations: types::Boolean,
    /// Primitive extension sibling for [`translations`](Self::translations) (FHIR `_translations`).
    #[serde(rename = "_translations")]
    pub translations_ext: Option<types::Element>,
}

/// Information about the ConceptMap/$translate operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesTranslation;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesTranslation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TerminologyCapabilitiesTranslation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesTranslation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether the client must identify the map
    pub needs_map: types::Boolean,
    /// Primitive extension sibling for [`needs_map`](Self::needs_map) (FHIR `_needsMap`).
    #[serde(rename = "_needsMap")]
    pub needs_map_ext: Option<types::Element>,
}

/// Information about the ConceptMap/$closure operation.
/// # Examples
///
/// ```
/// use fhir::r5::resources::terminology_capabilities::TerminologyCapabilitiesClosure;
/// use fhir::r5::types;
///
/// let value = TerminologyCapabilitiesClosure {
///     translation: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `translation` is the name this serializes to on the wire.
/// assert_eq!(json["translation"], ::serde_json::json!(true));
///
/// let back: TerminologyCapabilitiesClosure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TerminologyCapabilitiesClosure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// If cross-system closure is supported
    pub translation: Option<types::Boolean>,
    /// Primitive extension sibling for [`translation`](Self::translation) (FHIR `_translation`).
    #[serde(rename = "_translation")]
    pub translation_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TerminologyCapabilities;

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
/// The `TerminologyCapabilities.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TerminologyCapabilitiesVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
