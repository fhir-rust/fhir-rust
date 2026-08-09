//! Conformance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Conformance
//!
//!
//!
//! A conformance statement
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Conformance Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::conformance::Conformance;
///
/// let value = Conformance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Conformance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Conformance {
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

    /// Logical uri to reference this statement
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Logical id for this version of the statement
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Informal name for this conformance statement
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// draft | active | retired
    pub status: Option<crate::coded::Coded<crate::r2::codes::ConformanceResourceStatus>>,
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

    /// Name of the publisher (Organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details of the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ConformanceContact>,

    /// Publication Date(/time)
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Human description of the conformance statement
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Why is this needed?
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

    /// instance | capability | requirements
    pub kind: crate::coded::Coded<crate::r2::codes::ConformanceStatementKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Software that is covered by this conformance statement
    pub software: Option<ConformanceSoftware>,

    /// If this describes a specific instance
    pub implementation: Option<ConformanceImplementation>,

    /// FHIR Version the system uses
    pub fhir_version: types::Id,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// no | extensions | elements | both
    pub accept_unknown: crate::coded::Coded<crate::r2::codes::UnknownContentCode>,
    /// Primitive extension sibling for [`accept_unknown`](Self::accept_unknown) (FHIR `_acceptUnknown`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_acceptUnknown")]
    pub accept_unknown_ext: Option<types::Element>,

    /// formats supported (xml | json | mime type)
    pub format: ::vec1::Vec1<types::Code>,
    /// Primitive extension sibling for [`format`](Self::format) (FHIR `_format`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_format")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub format_ext: Vec<Option<types::Element>>,

    /// Profiles for use cases supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Reference<crate::r2::resources::StructureDefinition>>,

    /// If the endpoint is a RESTful one
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rest: Vec<ConformanceRest>,

    /// If messaging is supported
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messaging: Vec<ConformanceMessaging>,

    /// Document definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document: Vec<ConformanceDocument>,
}

/// Contacts to assist a user in finding and communicating with the publisher.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceContact;
/// use fhir::r2::types;
///
/// let value = ConformanceContact {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ConformanceContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceContact {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of a individual to contact
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Contact details for individual or publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,
}

/// A document definition.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceDocument;
/// use fhir::r2::types;
///
/// let value = ConformanceDocument {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: ConformanceDocument = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceDocument {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// producer | consumer
    pub mode: crate::coded::Coded<crate::r2::codes::DocumentMode>,
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
    pub profile: types::Reference<crate::r2::resources::StructureDefinition>,
}

/// Identifies a specific implementation instance that is described by the
/// conformance statement - i.e. a particular installation, rather than the
/// capabilities of a software program.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceImplementation;
/// use fhir::r2::types;
///
/// let value = ConformanceImplementation {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: ConformanceImplementation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceImplementation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
/// ```ignore
/// use fhir::r2::resources::conformance::ConformanceMessaging;
///
/// let value = ConformanceMessaging::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConformanceMessaging = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceMessaging {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A messaging service end-point
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<ConformanceMessagingEndpoint>,

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

    /// Declare support for this event
    pub event: ::vec1::Vec1<ConformanceMessagingEvent>,
}

/// An endpoint (network accessible address) to which messages and/or replies
/// are to be sent.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceMessagingEndpoint;
/// use fhir::r2::types;
///
/// let value = ConformanceMessagingEndpoint {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ConformanceMessagingEndpoint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceMessagingEndpoint {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// http | ftp | mllp +
    pub protocol: types::Coding,

    /// Address of end-point
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
/// use fhir::r2::resources::conformance::ConformanceMessagingEvent;
/// use fhir::r2::types;
///
/// let value = ConformanceMessagingEvent {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: ConformanceMessagingEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceMessagingEvent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Event type
    pub code: types::Coding,

    /// Consequence | Currency | Notification
    pub category: Option<crate::coded::Coded<crate::r2::codes::MessageSignificanceCategory>>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// sender | receiver
    pub mode: crate::coded::Coded<crate::r2::codes::MessageConformanceEventMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Resource that's focus of message
    pub focus: crate::coded::Coded<crate::r2::codes::ResourceTypes>,
    /// Primitive extension sibling for [`focus`](Self::focus) (FHIR `_focus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_focus")]
    pub focus_ext: Option<types::Element>,

    /// Profile that describes the request
    pub request: types::Reference<crate::r2::resources::StructureDefinition>,

    /// Profile that describes the response
    pub response: types::Reference<crate::r2::resources::StructureDefinition>,

    /// Endpoint-specific event documentation
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,
}

/// A definition of the restful capabilities of the solution, if any.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::conformance::ConformanceRest;
///
/// let value = ConformanceRest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConformanceRest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRest {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// client | server
    pub mode: crate::coded::Coded<crate::r2::codes::RestfulConformanceMode>,
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
    pub security: Option<ConformanceRestSecurity>,

    /// Resource served on the REST interface
    pub resource: ::vec1::Vec1<ConformanceRestResource>,

    /// What operations are supported?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interaction: Vec<ConformanceRestInteraction>,

    /// not-supported | batch | transaction | both
    pub transaction_mode: Option<crate::coded::Coded<crate::r2::codes::TransactionMode>>,
    /// Primitive extension sibling for [`transaction_mode`](Self::transaction_mode) (FHIR `_transactionMode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_transactionMode")]
    pub transaction_mode_ext: Option<types::Element>,

    /// Search params for searching all resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_param: Vec<ConformanceRestResourceSearchParam>,

    /// Definition of an operation or a custom query
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation: Vec<ConformanceRestOperation>,

    /// Compartments served/used by system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compartment: Vec<types::Uri>,
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
/// use fhir::r2::resources::conformance::ConformanceRestInteraction;
/// use fhir::r2::types;
///
/// let value = ConformanceRestInteraction {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: ConformanceRestInteraction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRestInteraction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// transaction | search-system | history-system
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

/// Definition of an operation or a named query and with its parameters and
/// their meaning and type.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceRestOperation;
/// use fhir::r2::types;
///
/// let value = ConformanceRestOperation {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ConformanceRestOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRestOperation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub definition: types::Reference<crate::r2::resources::OperationDefinition>,
}

/// A specification of the restful capabilities of the solution for a specific
/// resource type.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::conformance::ConformanceRestResource;
///
/// let value = ConformanceRestResource::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ConformanceRestResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRestResource {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A resource type that is supported
    pub r#type: crate::coded::Coded<crate::r2::codes::ResourceTypes>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Base System profile for all uses of resource
    pub profile: Option<types::Reference<crate::r2::resources::StructureDefinition>>,

    /// What operations are supported?
    pub interaction: ::vec1::Vec1<ConformanceRestResourceInteraction>,

    /// no-version | versioned | versioned-update
    pub versioning: Option<crate::coded::Coded<crate::r2::codes::VersioningPolicy>>,
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

    /// If allows/uses conditional update
    pub conditional_update: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditional_update`](Self::conditional_update) (FHIR `_conditionalUpdate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conditionalUpdate")]
    pub conditional_update_ext: Option<types::Element>,

    /// not-supported | single | multiple - how conditional delete is supported
    pub conditional_delete: Option<crate::coded::Coded<crate::r2::codes::ConditionalDeleteStatus>>,
    /// Primitive extension sibling for [`conditional_delete`](Self::conditional_delete) (FHIR `_conditionalDelete`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_conditionalDelete")]
    pub conditional_delete_ext: Option<types::Element>,

    /// _include values supported by the server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_include: Vec<types::String>,
    /// Primitive extension sibling for [`search_include`](Self::search_include) (FHIR `_searchInclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_include_ext: Vec<Option<types::Element>>,

    /// _revinclude values supported by the server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_rev_include: Vec<types::String>,
    /// Primitive extension sibling for [`search_rev_include`](Self::search_rev_include) (FHIR `_searchRevInclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchRevInclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_rev_include_ext: Vec<Option<types::Element>>,

    /// Search params supported by implementation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub search_param: Vec<ConformanceRestResourceSearchParam>,
}

/// Identifies a restful operation supported by the solution.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceRestResourceInteraction;
/// use fhir::r2::types;
///
/// let value = ConformanceRestResourceInteraction {
///     documentation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("abc"));
///
/// let back: ConformanceRestResourceInteraction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRestResourceInteraction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// read | vread | update | delete | history-instance | validate |
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
/// use fhir::r2::resources::conformance::ConformanceRestResourceSearchParam;
/// use fhir::r2::types;
///
/// let value = ConformanceRestResourceSearchParam {
///     definition: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `definition` is the name this serializes to on the wire.
/// assert_eq!(json["definition"], ::serde_json::json!("http://example.org"));
///
/// let back: ConformanceRestResourceSearchParam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRestResourceSearchParam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub r#type: crate::coded::Coded<crate::r2::codes::SearchParamType>,
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

    /// Types of resource (if a resource reference)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<crate::coded::Coded<crate::r2::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_target")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_ext: Vec<Option<types::Element>>,

    /// missing | exact | contains | not | text | in | not-in | below | above |
    /// type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<crate::coded::Coded<crate::r2::codes::SearchModifierCode>>,
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
}

/// Information about security implementation from an interface perspective -
/// what a client needs to know.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceRestSecurity;
/// use fhir::r2::types;
///
/// let value = ConformanceRestSecurity {
///     cors: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `cors` is the name this serializes to on the wire.
/// assert_eq!(json["cors"], ::serde_json::json!(true));
///
/// let back: ConformanceRestSecurity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRestSecurity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub certificate: Vec<ConformanceRestSecurityCertificate>,
}

/// Certificates associated with security profiles.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceRestSecurityCertificate;
/// use fhir::r2::types;
///
/// let value = ConformanceRestSecurityCertificate {
///     r#type: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("final"));
///
/// let back: ConformanceRestSecurityCertificate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceRestSecurityCertificate {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Mime type for certificate
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

/// Software that is covered by this conformance statement. It is used when the
/// conformance statement describes the capabilities of a particular software
/// version, independent of an installation.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::conformance::ConformanceSoftware;
/// use fhir::r2::types;
///
/// let value = ConformanceSoftware {
///     release_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `releaseDate` is the name this serializes to on the wire.
/// assert_eq!(json["releaseDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ConformanceSoftware = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ConformanceSoftware {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
