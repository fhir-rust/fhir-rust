//! TerminologyCapabilities
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TerminologyCapabilities
//!
//! Version: 4.0.1
//!
//! A statement of system capabilities
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A TerminologyCapabilities resource documents a set of capabilities
/// (behaviors) of a FHIR Terminology Server that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilities;
/// use fhir::r4::types;
///
/// let value = TerminologyCapabilities {
///     locked_date: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lockedDate` is the name this serializes to on the wire.
/// assert_eq!(json["lockedDate"], ::serde_json::json!(true));
///
/// let back: TerminologyCapabilities = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct TerminologyCapabilities {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this terminology capabilities, represented as
    /// a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business version of the terminology capabilities
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this terminology capabilities (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this terminology capabilities (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
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

    /// Date last changed
    pub date: types::DateTime,
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

    /// Natural language description of the terminology capabilities
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// instance | capability | requirements
    pub kind: crate::coded::Coded<crate::r4::codes::CapabilityStatementKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Software that is covered by this terminology capability statement
    pub software: Option<TerminologyCapabilitiesSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<TerminologyCapabilitiesImplementation>,

    /// Whether lockedDate is supported
    pub locked_date: Option<types::Boolean>,
    /// Primitive extension sibling for [`locked_date`](Self::locked_date) (FHIR `_lockedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lockedDate")]
    pub locked_date_ext: Option<types::Element>,

    /// A code system supported by the server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_system: Vec<TerminologyCapabilitiesCodeSystem>,

    /// Information about the
    /// [ValueSet/$expand](valueset-operation-expand.html) operation
    pub expansion: Option<TerminologyCapabilitiesExpansion>,

    /// explicit | all
    pub code_search: Option<crate::coded::Coded<crate::r4::codes::CodeSearchSupport>>,
    /// Primitive extension sibling for [`code_search`](Self::code_search) (FHIR `_codeSearch`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_codeSearch")]
    pub code_search_ext: Option<types::Element>,

    /// Information about the
    /// [ValueSet/$validate-code](valueset-operation-validate-code.html)
    /// operation
    pub validate_code: Option<TerminologyCapabilitiesValidateCode>,

    /// Information about the
    /// [ConceptMap/$translate](conceptmap-operation-translate.html) operation
    pub translation: Option<TerminologyCapabilitiesTranslation>,

    /// Information about the
    /// [ConceptMap/$closure](conceptmap-operation-closure.html) operation
    pub closure: Option<TerminologyCapabilitiesClosure>,
}

/// Whether the $closure operation is supported.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesClosure;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`translation`](Self::translation) (FHIR `_translation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_translation")]
    pub translation_ext: Option<types::Element>,
}

/// Identifies a code system that is supported by the server. If there is a no
/// code system URL, then this declares the general assumptions a client can
/// make about support for any CodeSystem resource.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesCodeSystem;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct TerminologyCapabilitiesCodeSystem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// URI for the Code System
    pub uri: Option<types::Canonical>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Version of Code System supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<TerminologyCapabilitiesCodeSystemVersion>,

    /// Whether subsumption is supported
    pub subsumption: Option<types::Boolean>,
    /// Primitive extension sibling for [`subsumption`](Self::subsumption) (FHIR `_subsumption`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subsumption")]
    pub subsumption_ext: Option<types::Element>,
}

/// For the code system, a list of versions that are supported by the server.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesCodeSystemVersion;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// If this is the default version for this code system
    pub is_default: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_default`](Self::is_default) (FHIR `_isDefault`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isDefault")]
    pub is_default_ext: Option<types::Element>,

    /// If compositional grammar is supported
    pub compositional: Option<types::Boolean>,
    /// Primitive extension sibling for [`compositional`](Self::compositional) (FHIR `_compositional`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compositional")]
    pub compositional_ext: Option<types::Element>,

    /// Language Displays supported
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub language: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_ext: Vec<Option<types::Element>>,

    /// Filter Properties supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter: Vec<TerminologyCapabilitiesCodeSystemVersionFilter>,

    /// Properties supported for $lookup
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub property: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`property`](Self::property) (FHIR `_property`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_property")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property_ext: Vec<Option<types::Element>>,
}

/// Filter Properties supported.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesCodeSystemVersionFilter;
///
/// let value = TerminologyCapabilitiesCodeSystemVersionFilter::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TerminologyCapabilitiesCodeSystemVersionFilter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Operations supported for the property
    pub op: ::vec1::Vec1<types::Code>,
    /// Primitive extension sibling for [`op`](Self::op) (FHIR `_op`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_op")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub op_ext: Vec<Option<types::Element>>,
}

/// Information about the [ValueSet/$expand](valueset-operation-expand.html)
/// operation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesExpansion;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`hierarchical`](Self::hierarchical) (FHIR `_hierarchical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hierarchical")]
    pub hierarchical_ext: Option<types::Element>,

    /// Whether the server supports paging on expansion
    pub paging: Option<types::Boolean>,
    /// Primitive extension sibling for [`paging`](Self::paging) (FHIR `_paging`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paging")]
    pub paging_ext: Option<types::Element>,

    /// Allow request for incomplete expansions?
    pub incomplete: Option<types::Boolean>,
    /// Primitive extension sibling for [`incomplete`](Self::incomplete) (FHIR `_incomplete`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_incomplete")]
    pub incomplete_ext: Option<types::Element>,

    /// Supported expansion parameter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<TerminologyCapabilitiesExpansionParameter>,

    /// Documentation about text searching works
    pub text_filter: Option<types::Markdown>,
    /// Primitive extension sibling for [`text_filter`](Self::text_filter) (FHIR `_textFilter`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_textFilter")]
    pub text_filter_ext: Option<types::Element>,
}

/// Supported expansion parameter.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesExpansionParameter;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct TerminologyCapabilitiesExpansionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Expansion Parameter name
    pub name: types::Code,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Description of support for parameter
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Identifies a specific implementation instance that is described by the
/// terminology capability statement - i.e. a particular installation, rather
/// than the capabilities of a software program.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesImplementation;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Base URL for the implementation
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// Software that is covered by this terminology capability statement. It is
/// used when the statement describes the capabilities of a particular software
/// version, independent of an installation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesSoftware;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Version covered by this statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,
}

/// Information about the
/// [ConceptMap/$translate](conceptmap-operation-translate.html) operation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesTranslation;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`needs_map`](Self::needs_map) (FHIR `_needsMap`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_needsMap")]
    pub needs_map_ext: Option<types::Element>,
}

/// Information about the
/// [ValueSet/$validate-code](valueset-operation-validate-code.html) operation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::terminology_capabilities::TerminologyCapabilitiesValidateCode;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`translations`](Self::translations) (FHIR `_translations`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_translations")]
    pub translations_ext: Option<types::Element>,
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
