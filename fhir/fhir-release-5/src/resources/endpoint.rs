//! Endpoint
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Endpoint
//!
//! Version: 5.0.0
//!
//! Endpoint Resource: The technical details of an endpoint that can be used for electronic services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The technical details of an endpoint that can be used for electronic
/// services, such as for web services providing XDS.b, a REST endpoint for
/// another FHIR server, or a s/Mime email address. This may include any
/// security context information. In FHIR R5 an Endpoint describes how to
/// connect to a system, the protocols it uses, the payloads it accepts, and the
/// organization that manages it.
///
/// Endpoint is primarily an administrative/technical resource rather than a
/// clinical one: it is referenced by other resources (for example a
/// `Subscription`, a `HealthcareService`, or an exchange-related resource)
/// wherever an application needs to know where and how to send or retrieve
/// data electronically. A single Endpoint typically records the address
/// (URL or other technical address), the connection type/protocol, the
/// accepted payload types and MIME types, and the organization responsible
/// for the endpoint's operation, along with an interval during which the
/// endpoint is expected to be usable.
///
/// # See also
///
/// - [`types::Reference`] is commonly used by other resources to point at an
///   `Endpoint`.
/// - [`types::CodeableConcept`] is used here to describe the connection type
///   and environment type.
/// - [`types::Period`] describes the interval during which the endpoint is
///   expected to be operational.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::endpoint::Endpoint;
///
/// let value = Endpoint::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Endpoint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
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

    /// Identifies this endpoint across multiple systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The current operational status of the endpoint: active | suspended |
    /// error | off | entered-in-error | test
    pub status: crate::r5::coded::Coded<crate::r5::codes::EndpointStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// One or more protocols, profiles, or standards used to connect to and
    /// exchange data with this endpoint (for example a messaging or REST
    /// protocol)
    pub connection_type: vec1::Vec1<types::CodeableConcept>,

    /// A human-friendly name that this endpoint can be identified by
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Additional details about the endpoint that could be displayed as further
    /// information to identify the description beyond its name
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The type of environment(s) exposed at this endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub environment_type: Vec<types::CodeableConcept>,

    /// Organization that manages this endpoint (might not be the organization
    /// that exposes the endpoint)
    pub managing_organization: Option<types::Reference>,

    /// Contact details for source (e.g. troubleshooting)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactPoint>,

    /// Interval the endpoint is expected to be operational
    pub period: Option<types::Period>,

    /// Set of payloads that are provided by this endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<EndpointPayload>,

    /// The technical base address for connecting to this endpoint
    pub address: types::Url,
    /// Primitive extension sibling for [`address`](Self::address) (FHIR `_address`).
    #[serde(rename = "_address")]
    pub address_ext: Option<types::Element>,

    /// Usage depends on the channel type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub header: Vec<types::String>,
    /// Primitive extension sibling for [`header`](Self::header) (FHIR `_header`).
    #[serde(rename = "_header")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub header_ext: Vec<Option<types::Element>>,
}

/// Set of payloads that are provided by this endpoint. Each payload describes
/// the type of content and the MIME types that may be sent to or received from
/// the endpoint.
/// # Examples
///
/// ```
/// use fhir::r5::resources::endpoint::EndpointPayload;
/// use fhir::r5::types;
///
/// let value = EndpointPayload {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EndpointPayload = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPayload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of content that may be used at this endpoint (e.g. XDS
    /// Discharge summaries)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Mimetype to send. If not specified, the content could be anything
    /// (including no payload, if the connectionType defined this)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mime_type: Vec<types::Code>,
    /// Primitive extension sibling for [`mime_type`](Self::mime_type) (FHIR `_mimeType`).
    #[serde(rename = "_mimeType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mime_type_ext: Vec<Option<types::Element>>,
}
