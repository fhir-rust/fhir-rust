//! Endpoint
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Endpoint
//!
//! Version: 6.0.0-ballot3
//!
//! The technical details of an endpoint that can be used for electronic
//! services
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The technical details of an endpoint that can be used for electronic
/// services, such as for web services providing XDS.b, a REST endpoint for
/// another FHIR server, or a s/Mime email address. This may include any
/// security context information.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::endpoint::Endpoint;
///
/// let value = Endpoint::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Endpoint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Endpoint {
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

    /// Identifies this endpoint across multiple systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | limited | suspended | error | off | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::EndpointStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Protocol/Profile/Standard to be used with this endpoint connection
    pub connection_type: ::vec1::Vec1<types::CodeableConcept>,

    /// A name that this endpoint can be identified by
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Additional details about the endpoint that could be displayed as
    /// further information to identify the description beyond its name
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// Times the endpoint is expected to be available (including exceptions)
    pub availability: Option<types::Availability>,

    /// Set of payloads that are provided by this endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<EndpointPayload>,

    /// The technical base address for connecting to this endpoint
    pub address: types::Url,
    /// Primitive extension sibling for [`address`](Self::address) (FHIR `_address`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_address")]
    pub address_ext: Option<types::Element>,

    /// Usage depends on the channel type
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub header: Vec<types::String>,
    /// Primitive extension sibling for [`header`](Self::header) (FHIR `_header`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_header")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub header_ext: Vec<Option<types::Element>>,
}

/// The set of payloads that are provided/available at this endpoint.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::endpoint::EndpointPayload;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`mime_type`](Self::mime_type) (FHIR `_mimeType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mimeType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mime_type_ext: Vec<Option<types::Element>>,

    /// The profile that is expected at this endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`profile_canonical`](Self::profile_canonical) (FHIR `_profileCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profileCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_canonical_ext: Vec<Option<types::Element>>,

    /// The non-fhir based profile that is expected at this endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`profile_uri`](Self::profile_uri) (FHIR `_profileUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profileUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_uri_ext: Vec<Option<types::Element>>,
}
