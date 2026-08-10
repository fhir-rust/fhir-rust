//! AuditEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
//!
//! Version: 4.0.1
//!
//! Event record kept for security purposes
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of an event made for purposes of maintaining a security log.
/// Typical uses include detection of intrusion attempts and monitoring for
/// inappropriate usage.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::audit_event::AuditEvent;
///
/// let value = AuditEvent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AuditEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct AuditEvent {
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

    /// Type/identifier of event
    pub r#type: types::Coding,

    /// More specific type/id for the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtype: Vec<types::Coding>,

    /// Type of action performed during the event
    pub action: Option<crate::coded::Coded<crate::r4::codes::AuditEventAction>>,
    /// Primitive extension sibling for [`action`](Self::action) (FHIR `_action`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_action")]
    pub action_ext: Option<types::Element>,

    /// When the activity occurred
    pub period: Option<types::Period>,

    /// Time when the event was recorded
    pub recorded: types::Instant,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Whether the event succeeded or failed
    pub outcome: Option<crate::coded::Coded<crate::r4::codes::AuditEventOutcome>>,
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
/// use fhir::r4::resources::audit_event::AuditEventAgent;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct AuditEventAgent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// How agent participated
    pub r#type: Option<types::CodeableConcept>,

    /// Agent role in the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,

    /// Identifier of who
    pub who: Option<types::Reference>,

    /// Alternative User identity
    pub alt_id: Option<types::String>,
    /// Primitive extension sibling for [`alt_id`](Self::alt_id) (FHIR `_altId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_altId")]
    pub alt_id_ext: Option<types::Element>,

    /// Human friendly name for the agent
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
    pub location: Option<types::Reference<crate::r4::resources::Location>>,

    /// Policy that authorized event
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub policy: ::fhir_core::PrimVec<types::Uri>,
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
/// use fhir::r4::resources::audit_event::AuditEventAgentNetwork;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct AuditEventAgentNetwork {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier for the network access point of the user device
    pub address: Option<types::String>,
    /// Primitive extension sibling for [`address`](Self::address) (FHIR `_address`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_address")]
    pub address_ext: Option<types::Element>,

    /// The type of network access point
    pub r#type: Option<crate::coded::Coded<crate::r4::codes::NetworkType>>,
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
/// use fhir::r4::resources::audit_event::AuditEventEntity;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct AuditEventEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific instance of resource
    pub what: Option<types::Reference>,

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
/// use fhir::r4::resources::audit_event::AuditEventEntityDetail;
/// use fhir::r4::types;
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
#[serde(from = "AuditEventEntityDetailDe")]
#[fhir_version("r4")]
pub struct AuditEventEntityDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of the property
    pub r#type: types::String,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Property value
    /// The `AuditEvent.entity.detail.value[x]` choice element (1..1); see [`AuditEventEntityDetailValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<AuditEventEntityDetailValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuditEventEntityDetailDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::String,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    #[serde(flatten)]
    value: crate::r4::choice::Slot<AuditEventEntityDetailValue>,
}

impl ::core::convert::From<AuditEventEntityDetailDe> for AuditEventEntityDetail {
    fn from(v: AuditEventEntityDetailDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            value: v.value.0,
        }
    }
}

/// The system that is reporting the event.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::audit_event::AuditEventSource;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct AuditEventSource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Logical source location within the enterprise
    pub site: Option<types::String>,
    /// Primitive extension sibling for [`site`](Self::site) (FHIR `_site`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_site")]
    pub site_ext: Option<types::Element>,

    /// The identity of source detecting the event
    pub observer: types::Reference,

    /// The type of source where event originated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::Coding>,
}

/// The `AuditEvent.entity.detail.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum AuditEventEntityDetailValue {
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r4::choice::Primitive<types::String>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r4::choice::Primitive<types::Base64Binary>),
}
