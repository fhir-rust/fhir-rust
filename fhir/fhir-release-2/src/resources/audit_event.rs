//! AuditEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
//!
//!
//!
//! Event record kept for security purposes
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for AuditEvent Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::audit_event::AuditEvent;
///
/// let value = AuditEvent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AuditEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What was done
    pub event: AuditEventEvent,

    /// A person, a hardware device or software process
    pub participant: ::vec1::Vec1<AuditEventParticipant>,

    /// Application systems and processes
    pub source: AuditEventSource,

    /// Specific instances of data or objects that have been accessed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub object: Vec<AuditEventObject>,
}

/// Identifies the name, action type, time, and disposition of the audited
/// event.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::audit_event::AuditEventEvent;
/// use fhir::r2::types;
///
/// let value = AuditEventEvent {
///     outcome_desc: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `outcomeDesc` is the name this serializes to on the wire.
/// assert_eq!(json["outcomeDesc"], ::serde_json::json!("abc"));
///
/// let back: AuditEventEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct AuditEventEvent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub action: Option<crate::coded::Coded<crate::r2::codes::AuditEventAction>>,
    /// Primitive extension sibling for [`action`](Self::action) (FHIR `_action`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_action")]
    pub action_ext: Option<types::Element>,

    /// Time when the event occurred on source
    pub date_time: types::Instant,
    /// Primitive extension sibling for [`date_time`](Self::date_time) (FHIR `_dateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateTime")]
    pub date_time_ext: Option<types::Element>,

    /// Whether the event succeeded or failed
    pub outcome: Option<crate::coded::Coded<crate::r2::codes::AuditEventOutcome>>,
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
    pub purpose_of_event: Vec<types::Coding>,
}

/// Specific instances of data or objects that have been accessed.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::audit_event::AuditEventObject;
/// use fhir::r2::types;
///
/// let value = AuditEventObject {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: AuditEventObject = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct AuditEventObject {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific instance of object (e.g. versioned)
    pub identifier: Option<types::Identifier>,

    /// Specific instance of resource (e.g. versioned)
    pub reference: Option<types::Reference>,

    /// Type of object involved
    pub r#type: Option<types::Coding>,

    /// What role the Object played
    pub role: Option<types::Coding>,

    /// Life-cycle stage for the object
    pub lifecycle: Option<types::Coding>,

    /// Security labels applied to the object
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::Coding>,

    /// Instance-specific descriptor for Object
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

    /// Actual query for object
    pub query: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`query`](Self::query) (FHIR `_query`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_query")]
    pub query_ext: Option<types::Element>,

    /// Additional Information about the Object
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<AuditEventObjectDetail>,
}

/// Additional Information about the Object.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::audit_event::AuditEventObjectDetail;
/// use fhir::r2::types;
///
/// let value = AuditEventObjectDetail {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: AuditEventObjectDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct AuditEventObjectDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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

/// A person, a hardware device or software process.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::audit_event::AuditEventParticipant;
/// use fhir::r2::types;
///
/// let value = AuditEventParticipant {
///     alt_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `altId` is the name this serializes to on the wire.
/// assert_eq!(json["altId"], ::serde_json::json!("abc"));
///
/// let back: AuditEventParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct AuditEventParticipant {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User roles (e.g. local RBAC codes)
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

    /// Human-meaningful name for the user
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
    pub location: Option<types::Reference<crate::r2::resources::Location>>,

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
    pub network: Option<AuditEventParticipantNetwork>,

    /// Reason given for this user
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose_of_use: Vec<types::Coding>,
}

/// Logical network location for application activity, if the activity has a
/// network location.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::audit_event::AuditEventParticipantNetwork;
/// use fhir::r2::types;
///
/// let value = AuditEventParticipantNetwork {
///     address: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `address` is the name this serializes to on the wire.
/// assert_eq!(json["address"], ::serde_json::json!("abc"));
///
/// let back: AuditEventParticipantNetwork = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct AuditEventParticipantNetwork {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub r#type: Option<crate::coded::Coded<crate::r2::codes::NetworkType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// Application systems and processes.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::audit_event::AuditEventSource;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct AuditEventSource {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
