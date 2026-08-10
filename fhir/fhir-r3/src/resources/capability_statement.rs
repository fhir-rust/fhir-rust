//! CapabilityStatement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CapabilityStatement
//!
//!
//!
//! A statement of system capabilities
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for CapabilityStatement Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::capability_statement::CapabilityStatement;
///
/// let value = CapabilityStatement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CapabilityStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatement {
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

    /// Logical URI to reference this capability statement (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business version of the capability statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this capability statement (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this capability statement (human friendly)
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

    /// Natural language description of the capability statement
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for capability statement (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this capability statement is defined
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
    pub kind: crate::coded::Coded<crate::r3::codes::CapabilityStatementKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Canonical URL of another capability statement this implements
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`instantiates`](Self::instantiates) (FHIR `_instantiates`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiates")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_ext: Vec<Option<types::Element>>,

    /// Software that is covered by this capability statement
    pub software: Option<CapabilityStatementSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<CapabilityStatementImplementation>,

    /// FHIR Version the system uses
    pub fhir_version: types::Id,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// no | extensions | elements | both
    pub accept_unknown: crate::coded::Coded<crate::r3::codes::UnknownContentCode>,
    /// Primitive extension sibling for [`accept_unknown`](Self::accept_unknown) (FHIR `_acceptUnknown`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_acceptUnknown")]
    pub accept_unknown_ext: Option<types::Element>,

    /// formats supported (xml | json | ttl | mime type)
    pub format: ::vec1::Vec1<types::Code>,
    /// Primitive extension sibling for [`format`](Self::format) (FHIR `_format`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_format")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub format_ext: Vec<Option<types::Element>>,

    /// Patch formats supported
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub patch_format: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`patch_format`](Self::patch_format) (FHIR `_patchFormat`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_patchFormat")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patch_format_ext: Vec<Option<types::Element>>,

    /// Implementation guides supported
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub implementation_guide: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`implementation_guide`](Self::implementation_guide) (FHIR `_implementationGuide`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_implementationGuide")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implementation_guide_ext: Vec<Option<types::Element>>,

    /// Profiles for use cases supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Reference<crate::r3::resources::StructureDefinition>>,

    /// If the endpoint is a RESTful one
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rest: Vec<CapabilityStatementRest>,

    /// If messaging is supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messaging: Vec<CapabilityStatementMessaging>,

    /// Document definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document: Vec<CapabilityStatementDocument>,
}

/// A document definition.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementDocument;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementDocument {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementDocument = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementDocument {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// producer | consumer
    pub mode: crate::coded::Coded<crate::r3::codes::DocumentMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Description of document support
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Constraint on a resource used in the document
    pub profile: types::Reference<crate::r3::resources::StructureDefinition>,
}

/// Identifies a specific implementation instance that is described by the
/// capability statement - i.e. a particular installation, rather than the
/// capabilities of a software program.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementImplementation;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementImplementation {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: CapabilityStatementImplementation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementImplementation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Describes this specific instance
    pub description: types::String,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Base URL for the installation
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// A description of the messaging capabilities of the solution.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementMessaging;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementMessaging {
///     reliable_cache: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reliableCache` is the name this serializes to on the wire.
/// assert_eq!(json["reliableCache"], ::serde_json::json!(0));
///
/// let back: CapabilityStatementMessaging = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementMessaging {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Where messages should be sent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<CapabilityStatementMessagingEndpoint>,

    /// Reliable Message Cache Length (min)
    pub reliable_cache: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`reliable_cache`](Self::reliable_cache) (FHIR `_reliableCache`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reliableCache")]
    pub reliable_cache_ext: Option<types::Element>,

    /// Messaging interface behavior details
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Messages supported by this system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supported_message: Vec<CapabilityStatementMessagingSupportedMessage>,

    /// Declare support for this event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CapabilityStatementMessagingEvent>,
}

/// An endpoint (network accessible address) to which messages and/or replies
/// are to be sent.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementMessagingEndpoint;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementMessagingEndpoint {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementMessagingEndpoint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementMessagingEndpoint {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// http | ftp | mllp +
    pub protocol: types::Coding,

    /// Network address or identifier of the end-point
    pub address: types::Uri,
    /// Primitive extension sibling for [`address`](Self::address) (FHIR `_address`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_address")]
    pub address_ext: Option<types::Element>,
}

/// A description of the solution's support for an event at this end-point.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementMessagingEvent;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementMessagingEvent {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementMessagingEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementMessagingEvent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Event type
    pub code: types::Coding,

    /// Consequence | Currency | Notification
    pub category: Option<crate::coded::Coded<crate::r3::codes::MessageSignificanceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// sender | receiver
    pub mode: crate::coded::Coded<crate::r3::codes::EventCapabilityMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Resource that's focus of message
    pub focus: crate::coded::Coded<crate::r3::codes::ResourceTypes>,
    /// Primitive extension sibling for [`focus`](Self::focus) (FHIR `_focus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_focus")]
    pub focus_ext: Option<types::Element>,

    /// Profile that describes the request
    pub request: types::Reference<crate::r3::resources::StructureDefinition>,

    /// Profile that describes the response
    pub response: types::Reference<crate::r3::resources::StructureDefinition>,

    /// Endpoint-specific event documentation
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// References to message definitions for messages this system can send or
/// receive.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementMessagingSupportedMessage;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementMessagingSupportedMessage {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementMessagingSupportedMessage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementMessagingSupportedMessage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// sender | receiver
    pub mode: crate::coded::Coded<crate::r3::codes::EventCapabilityMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Message supported by this system
    pub definition: types::Reference<crate::r3::resources::MessageDefinition>,
}

/// A definition of the restful capabilities of the solution, if any.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementRest;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementRest {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementRest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRest {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// client | server
    pub mode: crate::coded::Coded<crate::r3::codes::RestfulCapabilityMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// General description of implementation
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Information about security of implementation
    pub security: Option<CapabilityStatementRestSecurity>,

    /// Resource served on the REST interface
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<CapabilityStatementRestResource>,

    /// What operations are supported?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction: Vec<CapabilityStatementRestInteraction>,

    /// Search parameters for searching all resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_param: Vec<CapabilityStatementRestResourceSearchParam>,

    /// Definition of an operation or a custom query
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation: Vec<CapabilityStatementRestOperation>,

    /// Compartments served/used by system
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub compartment: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`compartment`](Self::compartment) (FHIR `_compartment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compartment")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compartment_ext: Vec<Option<types::Element>>,
}

/// A specification of restful operations supported by the system.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementRestInteraction;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementRestInteraction {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementRestInteraction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRestInteraction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// transaction | batch | search-system | history-system
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Anything special about operation behavior
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Definition of an operation or a named query together with its parameters
/// and their meaning and type.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementRestOperation;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementRestOperation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementRestOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRestOperation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name by which the operation/query is invoked
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The defined operation/query
    pub definition: types::Reference<crate::r3::resources::OperationDefinition>,
}

/// A specification of the restful capabilities of the solution for a specific
/// resource type.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::capability_statement::CapabilityStatementRestResource;
///
/// let value = CapabilityStatementRestResource::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CapabilityStatementRestResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRestResource {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A resource type that is supported
    pub r#type: crate::coded::Coded<crate::r3::codes::ResourceTypes>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Base System profile for all uses of resource
    pub profile: Option<types::Reference<crate::r3::resources::StructureDefinition>>,

    /// Additional information about the use of the resource type
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// What operations are supported?
    pub interaction: ::vec1::Vec1<CapabilityStatementRestResourceInteraction>,

    /// no-version | versioned | versioned-update
    pub versioning: Option<crate::coded::Coded<crate::r3::codes::VersioningPolicy>>,
    /// Primitive extension sibling for [`versioning`](Self::versioning) (FHIR `_versioning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_versioning")]
    pub versioning_ext: Option<types::Element>,

    /// Whether vRead can return past versions
    pub read_history: Option<types::Boolean>,
    /// Primitive extension sibling for [`read_history`](Self::read_history) (FHIR `_readHistory`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_readHistory")]
    pub read_history_ext: Option<types::Element>,

    /// If update can commit to a new identity
    pub update_create: Option<types::Boolean>,
    /// Primitive extension sibling for [`update_create`](Self::update_create) (FHIR `_updateCreate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_updateCreate")]
    pub update_create_ext: Option<types::Element>,

    /// If allows/uses conditional create
    pub conditional_create: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_create`](Self::conditional_create) (FHIR `_conditionalCreate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conditionalCreate")]
    pub conditional_create_ext: Option<types::Element>,

    /// not-supported | modified-since | not-match | full-support
    pub conditional_read: Option<crate::coded::Coded<crate::r3::codes::ConditionalReadStatus>>,
    /// Primitive extension sibling for [`conditional_read`](Self::conditional_read) (FHIR `_conditionalRead`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conditionalRead")]
    pub conditional_read_ext: Option<types::Element>,

    /// If allows/uses conditional update
    pub conditional_update: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_update`](Self::conditional_update) (FHIR `_conditionalUpdate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conditionalUpdate")]
    pub conditional_update_ext: Option<types::Element>,

    /// not-supported | single | multiple - how conditional delete is supported
    pub conditional_delete: Option<crate::coded::Coded<crate::r3::codes::ConditionalDeleteStatus>>,
    /// Primitive extension sibling for [`conditional_delete`](Self::conditional_delete) (FHIR `_conditionalDelete`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conditionalDelete")]
    pub conditional_delete_ext: Option<types::Element>,

    /// literal | logical | resolves | enforced | local
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub reference_policy:
        ::fhir_core::PrimVec<crate::coded::Coded<crate::r3::codes::ReferenceHandlingPolicy>>,
    /// Primitive extension sibling for [`reference_policy`](Self::reference_policy) (FHIR `_referencePolicy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_referencePolicy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_policy_ext: Vec<Option<types::Element>>,

    /// _include values supported by the server
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub search_include: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`search_include`](Self::search_include) (FHIR `_searchInclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_include_ext: Vec<Option<types::Element>>,

    /// _revinclude values supported by the server
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub search_rev_include: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`search_rev_include`](Self::search_rev_include) (FHIR `_searchRevInclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchRevInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_rev_include_ext: Vec<Option<types::Element>>,

    /// Search parameters supported by implementation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_param: Vec<CapabilityStatementRestResourceSearchParam>,
}

/// Identifies a restful operation supported by the solution.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementRestResourceInteraction;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementRestResourceInteraction {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: CapabilityStatementRestResourceInteraction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRestResourceInteraction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// read | vread | update | patch | delete | history-instance |
    /// history-type | create | search-type
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Anything special about operation behavior
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Search parameters for implementations to support and/or make use of -
/// either references to ones defined in the specification, or additional ones
/// defined for/by the implementation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementRestResourceSearchParam;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementRestResourceSearchParam {
///     definition: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `definition` is the name this serializes to on the wire.
/// assert_eq!(json["definition"], ::serde_json::json!("http://example.org"));
///
/// let back: CapabilityStatementRestResourceSearchParam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRestResourceSearchParam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of search parameter
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Source of definition for parameter
    pub definition: Option<types::Uri>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,

    /// number | date | string | token | reference | composite | quantity | uri
    pub r#type: crate::coded::Coded<crate::r3::codes::SearchParamType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Server-specific usage
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// Information about security implementation from an interface perspective -
/// what a client needs to know.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementRestSecurity;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementRestSecurity {
///     cors: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `cors` is the name this serializes to on the wire.
/// assert_eq!(json["cors"], ::serde_json::json!(true));
///
/// let back: CapabilityStatementRestSecurity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRestSecurity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adds CORS Headers (http://enable-cors.org/)
    pub cors: Option<types::Boolean>,
    /// Primitive extension sibling for [`cors`](Self::cors) (FHIR `_cors`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cors")]
    pub cors_ext: Option<types::Element>,

    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service: Vec<types::CodeableConcept>,

    /// General description of how security works
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Certificates associated with security profiles
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificate: Vec<CapabilityStatementRestSecurityCertificate>,
}

/// Certificates associated with security profiles.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementRestSecurityCertificate;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementRestSecurityCertificate {
///     r#type: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("final"));
///
/// let back: CapabilityStatementRestSecurityCertificate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementRestSecurityCertificate {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Mime type for certificates
    pub r#type: Option<types::Code>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Actual certificate
    pub blob: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`blob`](Self::blob) (FHIR `_blob`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_blob")]
    pub blob_ext: Option<types::Element>,
}

/// Software that is covered by this capability statement. It is used when the
/// capability statement describes the capabilities of a particular software
/// version, independent of an installation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::capability_statement::CapabilityStatementSoftware;
/// use fhir::r3::types;
///
/// let value = CapabilityStatementSoftware {
///     release_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `releaseDate` is the name this serializes to on the wire.
/// assert_eq!(json["releaseDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CapabilityStatementSoftware = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct CapabilityStatementSoftware {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

    /// Date this version released
    pub release_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`release_date`](Self::release_date) (FHIR `_releaseDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_releaseDate")]
    pub release_date_ext: Option<types::Element>,
}
