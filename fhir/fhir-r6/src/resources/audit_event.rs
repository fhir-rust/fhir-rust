//! AuditEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
//!
//! Version: 6.0.0-ballot3
//!
//! Record of an event
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of an event relevant for purposes such as operations, privacy,
/// security, maintenance, and performance analysis.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::audit_event::AuditEvent;
///
/// let value = AuditEvent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AuditEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "AuditEventDe")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// High level categorization of audit event
    pub r#type: types::CodeableConcept,

    /// Specific type of event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtype: Vec<types::CodeableConcept>,

    /// Type of action performed during the event
    pub action: Option<crate::coded::Coded<crate::r6::codes::AuditEventAction>>,
    /// Primitive extension sibling for [`action`](Self::action) (FHIR `_action`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_action")]
    pub action_ext: Option<types::Element>,

    /// emergency | alert | critical | error | warning | notice | informational
    /// | debug
    pub severity: Option<crate::coded::Coded<crate::r6::codes::AuditEventSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// When the activity occurred
    /// The `AuditEvent.occurred[x]` choice element (0..1); see [`AuditEventOccurred`].
    #[serde(flatten)]
    pub occurred: Option<AuditEventOccurred>,

    /// Time when the event was recorded
    pub recorded: types::Instant,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Whether the event succeeded or failed
    pub outcome: Option<AuditEventOutcome>,

    /// Authorization related to the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization: Vec<types::CodeableConcept>,

    /// Workflow authorization within which this event occurred
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The patient is the subject of the data used/created/updated/deleted
    /// during the activity
    pub patient: Option<types::Reference<crate::r6::resources::Patient>>,

    /// Encounter within which this event occurred or which the event is
    /// tightly associated
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Actor involved in the event
    pub agent: ::vec1::Vec1<AuditEventAgent>,

    /// Audit Event Reporter
    pub source: AuditEventSource,

    /// Data or objects used
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity: Vec<AuditEventEntity>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuditEventDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::CodeableConcept,
    #[serde(default)]
    subtype: Vec<types::CodeableConcept>,
    action: Option<crate::coded::Coded<crate::r6::codes::AuditEventAction>>,
    #[serde(rename = "_action")]
    action_ext: Option<types::Element>,
    severity: Option<crate::coded::Coded<crate::r6::codes::AuditEventSeverity>>,
    #[serde(rename = "_severity")]
    severity_ext: Option<types::Element>,
    #[serde(flatten)]
    occurred: crate::r6::choice::Slot<AuditEventOccurred>,
    recorded: types::Instant,
    #[serde(rename = "_recorded")]
    recorded_ext: Option<types::Element>,
    outcome: Option<AuditEventOutcome>,
    #[serde(default)]
    authorization: Vec<types::CodeableConcept>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    patient: Option<types::Reference<crate::r6::resources::Patient>>,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    agent: ::vec1::Vec1<AuditEventAgent>,
    source: AuditEventSource,
    #[serde(default)]
    entity: Vec<AuditEventEntity>,
}

impl ::core::convert::From<AuditEventDe> for AuditEvent {
    fn from(v: AuditEventDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            subtype: v.subtype,
            action: v.action,
            action_ext: v.action_ext,
            severity: v.severity,
            severity_ext: v.severity_ext,
            occurred: v.occurred.0,
            recorded: v.recorded,
            recorded_ext: v.recorded_ext,
            outcome: v.outcome,
            authorization: v.authorization,
            based_on: v.based_on,
            patient: v.patient,
            encounter: v.encounter,
            agent: v.agent,
            source: v.source,
            entity: v.entity,
        }
    }
}

/// An actor taking an active role in the event or activity that is logged.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::audit_event::AuditEventAgent;
/// use fhir::r6::types;
///
/// let value = AuditEventAgent {
///     requestor: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `requestor` is the name this serializes to on the wire.
/// assert_eq!(json["requestor"], ::serde_json::json!(true));
///
/// let back: AuditEventAgent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "AuditEventAgentDe")]
#[fhir_version("r6")]
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
    pub who: types::Reference,

    /// Whether user is initiator
    pub requestor: Option<types::Boolean>,
    /// Primitive extension sibling for [`requestor`](Self::requestor) (FHIR `_requestor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requestor")]
    pub requestor_ext: Option<types::Element>,

    /// The agent location when the event occurred
    pub location: Option<types::Reference<crate::r6::resources::Location>>,

    /// Policy that authorized the agent participation in the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<types::Uri>,
    /// Primitive extension sibling for [`policy`](Self::policy) (FHIR `_policy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_policy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_ext: Vec<Option<types::Element>>,

    /// This agent network location for the activity
    /// The `AuditEvent.agent.network[x]` choice element (0..1); see [`AuditEventAgentNetwork`].
    #[serde(flatten)]
    pub network: Option<AuditEventAgentNetwork>,

    /// Allowable authorization for this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization: Vec<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuditEventAgentDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: Option<types::CodeableConcept>,
    #[serde(default)]
    role: Vec<types::CodeableConcept>,
    who: types::Reference,
    requestor: Option<types::Boolean>,
    #[serde(rename = "_requestor")]
    requestor_ext: Option<types::Element>,
    location: Option<types::Reference<crate::r6::resources::Location>>,
    #[serde(default)]
    policy: Vec<types::Uri>,
    #[serde(rename = "_policy")]
    #[serde(default)]
    policy_ext: Vec<Option<types::Element>>,
    #[serde(flatten)]
    network: crate::r6::choice::Slot<AuditEventAgentNetwork>,
    #[serde(default)]
    authorization: Vec<types::CodeableConcept>,
}

impl ::core::convert::From<AuditEventAgentDe> for AuditEventAgent {
    fn from(v: AuditEventAgentDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            role: v.role,
            who: v.who,
            requestor: v.requestor,
            requestor_ext: v.requestor_ext,
            location: v.location,
            policy: v.policy,
            policy_ext: v.policy_ext,
            network: v.network.0,
            authorization: v.authorization,
        }
    }
}

/// Specific instances of data or objects that have been accessed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::audit_event::AuditEventEntity;
/// use fhir::r6::types;
///
/// let value = AuditEventEntity {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: AuditEventEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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

    /// What role the entity played
    pub role: Option<types::CodeableConcept>,

    /// Security labels on the entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security_label: Vec<types::CodeableConcept>,

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

    /// Entity is attributed to this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<AuditEventAgent>,
}

/// Tagged value pairs for conveying additional information about the entity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::audit_event::AuditEventEntityDetail;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct AuditEventEntityDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The name of the extra detail property
    pub r#type: types::CodeableConcept,

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
    r#type: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<AuditEventEntityDetailValue>,
}

impl ::core::convert::From<AuditEventEntityDetailDe> for AuditEventEntityDetail {
    fn from(v: AuditEventEntityDetailDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            value: v.value.0,
        }
    }
}

/// Indicates whether the event succeeded or failed. A free text descripiton
/// can be given in outcome.text.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::audit_event::AuditEventOutcome;
/// use fhir::r6::types;
///
/// let value = AuditEventOutcome {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AuditEventOutcome = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct AuditEventOutcome {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether the event succeeded or failed
    pub code: types::Coding,

    /// Additional outcome detail
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<types::CodeableConcept>,
}

/// The actor that is reporting the event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::audit_event::AuditEventSource;
/// use fhir::r6::types;
///
/// let value = AuditEventSource {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AuditEventSource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub site: Option<types::Reference<crate::r6::resources::Location>>,

    /// The identity of source detecting the event
    pub observer: types::Reference,

    /// The type of source where event originated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,
}

/// The `AuditEvent.occurred[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum AuditEventOccurred {
    /// `occurredPeriod` variant.
    #[fhir("occurredPeriod")]
    Period(Box<types::Period>),
    /// `occurredDateTime` variant.
    #[fhir("occurredDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
}

/// The `AuditEvent.agent.network[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum AuditEventAgentNetwork {
    /// `networkReference` variant.
    #[fhir("networkReference")]
    Reference(Box<types::Reference>),
    /// `networkUri` variant.
    #[fhir("networkUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `networkString` variant.
    #[fhir("networkString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `AuditEvent.entity.detail.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum AuditEventEntityDetailValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r6::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r6::choice::Primitive<types::Base64Binary>),
}
