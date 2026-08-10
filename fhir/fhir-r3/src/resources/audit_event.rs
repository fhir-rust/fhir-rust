//! AuditEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
//!
//!
//!
//! Event record kept for security purposes
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for AuditEvent Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::audit_event::AuditEvent;
///
/// let value = AuditEvent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AuditEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AuditEvent {
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

    /// Type/identifier of event
    pub r#type: types::Coding,

    /// More specific type/id for the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtype: Vec<types::Coding>,

    /// Type of action performed during the event
    pub action: Option<crate::coded::Coded<crate::r3::codes::AuditEventAction>>,
    /// Primitive extension sibling for [`action`](Self::action) (FHIR `_action`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_action")]
    pub action_ext: Option<types::Element>,

    /// Time when the event occurred on source
    pub recorded: types::Instant,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Whether the event succeeded or failed
    pub outcome: Option<crate::coded::Coded<crate::r3::codes::AuditEventOutcome>>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// Description of the event outcome
    pub outcome_desc: Option<types::String>,
    /// Primitive extension sibling for [`outcome_desc`](Self::outcome_desc) (FHIR `_outcomeDesc`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outcomeDesc")]
    pub outcome_desc_ext: Option<types::Element>,

    /// The purposeOfUse of the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose_of_event: Vec<types::CodeableConcept>,

    /// Actor involved in the event
    pub agent: ::vec1::Vec1<AuditEventAgent>,

    /// Audit Event Reporter
    pub source: AuditEventSource,

    /// Data or objects used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity: Vec<AuditEventEntity>,
}

/// An actor taking an active role in the event or activity that is logged.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::audit_event::AuditEventAgent;
/// use fhir::r3::types;
///
/// let value = AuditEventAgent {
///     alt_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `altId` is the name this serializes to on the wire.
/// assert_eq!(json["altId"], ::serde_json::json!("abc"));
///
/// let back: AuditEventAgent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AuditEventAgent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Agent role in the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,

    /// Direct reference to resource
    pub reference: Option<types::Reference>,

    /// Unique identifier for the user
    pub user_id: Option<types::Identifier>,

    /// Alternative User id e.g. authentication
    pub alt_id: Option<types::String>,
    /// Primitive extension sibling for [`alt_id`](Self::alt_id) (FHIR `_altId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_altId")]
    pub alt_id_ext: Option<types::Element>,

    /// Human-meaningful name for the agent
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Whether user is initiator
    pub requestor: types::Boolean,
    /// Primitive extension sibling for [`requestor`](Self::requestor) (FHIR `_requestor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requestor")]
    pub requestor_ext: Option<types::Element>,

    /// Where
    pub location: Option<types::Reference<crate::r3::resources::Location>>,

    /// Policy that authorized event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<types::Uri>,
    /// Primitive extension sibling for [`policy`](Self::policy) (FHIR `_policy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_policy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_ext: Vec<Option<types::Element>>,

    /// Type of media
    pub media: Option<types::Coding>,

    /// Logical network location for application activity
    pub network: Option<AuditEventAgentNetwork>,

    /// Reason given for this user
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose_of_use: Vec<types::CodeableConcept>,
}

/// Logical network location for application activity, if the activity has a
/// network location.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::audit_event::AuditEventAgentNetwork;
/// use fhir::r3::types;
///
/// let value = AuditEventAgentNetwork {
///     address: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `address` is the name this serializes to on the wire.
/// assert_eq!(json["address"], ::serde_json::json!("abc"));
///
/// let back: AuditEventAgentNetwork = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AuditEventAgentNetwork {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier for the network access point of the user device
    pub address: Option<types::String>,
    /// Primitive extension sibling for [`address`](Self::address) (FHIR `_address`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_address")]
    pub address_ext: Option<types::Element>,

    /// The type of network access point
    pub r#type: Option<crate::coded::Coded<crate::r3::codes::NetworkType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// Specific instances of data or objects that have been accessed.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::audit_event::AuditEventEntity;
/// use fhir::r3::types;
///
/// let value = AuditEventEntity {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: AuditEventEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AuditEventEntity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific instance of object
    pub identifier: Option<types::Identifier>,

    /// Specific instance of resource
    pub reference: Option<types::Reference>,

    /// Type of entity involved
    pub r#type: Option<types::Coding>,

    /// What role the entity played
    pub role: Option<types::Coding>,

    /// Life-cycle stage for the entity
    pub lifecycle: Option<types::Coding>,

    /// Security labels on the entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Descriptor for entity
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Descriptive text
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Query parameters
    pub query: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`query`](Self::query) (FHIR `_query`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_query")]
    pub query_ext: Option<types::Element>,

    /// Additional Information about the entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<AuditEventEntityDetail>,
}

/// Tagged value pairs for conveying additional information about the entity.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::audit_event::AuditEventEntityDetail;
/// use fhir::r3::types;
///
/// let value = AuditEventEntityDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AuditEventEntityDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AuditEventEntityDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of the property
    pub r#type: types::String,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Property value
    pub value: types::Base64Binary,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The system that is reporting the event.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::audit_event::AuditEventSource;
/// use fhir::r3::types;
///
/// let value = AuditEventSource {
///     site: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `site` is the name this serializes to on the wire.
/// assert_eq!(json["site"], ::serde_json::json!("abc"));
///
/// let back: AuditEventSource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct AuditEventSource {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Logical source location within the enterprise
    pub site: Option<types::String>,
    /// Primitive extension sibling for [`site`](Self::site) (FHIR `_site`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_site")]
    pub site_ext: Option<types::Element>,

    /// The identity of source detecting the event
    pub identifier: types::Identifier,

    /// The type of source where event originated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::Coding>,
}
