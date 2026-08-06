//! AuditEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AuditEvent
//!
//! Version: 5.0.0
//!
//! AuditEvent Resource: A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// AuditEvent records an event that is relevant for purposes such as
/// operations, privacy, security, maintenance, and performance analysis.
///
/// A typical AuditEvent captures who (agents) did what (code/action) to which
/// data (entities), when it occurred, where it was reported from (source), and
/// whether it succeeded (outcome). It is the FHIR R5 representation of a
/// security or activity audit log entry, commonly used to satisfy regulatory
/// accountability and traceability requirements.
///
/// AuditEvent resources are typically created automatically by systems as a
/// side effect of other operations (for example, when a record is read,
/// created, updated, or deleted, or when a user logs in or out) rather than
/// being authored directly by clinicians. They form the basis of security
/// audit trails required by regulations such as HIPAA and by security
/// frameworks like the ATNA (Audit Trail and Node Authentication) profile.
/// Because audit logs can grow very large and are queried primarily for
/// compliance and forensic investigation, servers commonly expose AuditEvent
/// through a dedicated audit-logging endpoint or repository rather than the
/// main clinical data store.
///
/// # Related resources
///
/// The `patient` and `encounter` fields commonly
/// reference [`Patient`](crate::r5::resources::patient::Patient) and
/// `Encounter` resources respectively, while `agent.who` and `entity.what`
/// use [`Reference`](crate::r5::types::Reference) to point at the actors and
/// data objects involved in the event. Coded fields such as `category`,
/// `code`, and `entity.role` use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) to describe the
/// nature of the event and its participants using standard terminologies.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::audit_event::AuditEvent;
///
/// let value = AuditEvent::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: AuditEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent {
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

    /// Type/identifier of event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Specific type of event, drawn from a coded terminology identifying what happened (e.g. login, patient record access)
    pub code: types::CodeableConcept,

    /// Type of action performed during the event: create (C), read/view (R), update (U), delete (D), or execute (E)
    pub action: Option<crate::r5::coded::Coded<crate::r5::codes::AuditEventAction>>,
    /// Primitive extension sibling for [`action`](Self::action) (FHIR `_action`).
    #[serde(rename = "_action")]
    pub action_ext: Option<types::Element>,

    /// emergency | alert | critical | error | warning | notice | informational | debug
    pub severity: Option<crate::r5::coded::Coded<crate::r5::codes::AuditEventSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`).
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// The `AuditEvent.occurred[x]` choice element (0..1); see [`AuditEventOccurred`].
    #[serde(flatten)]
    pub occurred: Option<AuditEventOccurred>,

    /// Time when the event was recorded, which may differ from when the underlying activity actually occurred
    pub recorded: types::Instant,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`).
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Whether the event succeeded or failed, along with any additional outcome detail
    pub outcome: Option<AuditEventOutcome>,

    /// Authorization related to the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization: Vec<types::CodeableConcept>,

    /// Workflow authorization within which this event occurred
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The patient is the subject of the data used/created/updated/deleted during the activity
    pub patient: Option<types::Reference>,

    /// Encounter within which this event occurred or which the event is tightly associated
    pub encounter: Option<types::Reference>,

    /// Actor(s) involved in the event, such as the user, system, or device that performed or participated in the action
    pub agent: vec1::Vec1<AuditEventAgent>,

    /// The system or application that detected and reported the event
    pub source: AuditEventSource,

    /// Data or object(s) that the event acted upon, such as a resource, record, or query
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity: Vec<AuditEventEntity>,
}

/// Whether the event succeeded or failed.
///
/// Indicates the outcome of the audited event, using a coded value plus
/// optional additional detail describing the result.
/// # Examples
///
/// ```
/// use fhir::r5::resources::audit_event::AuditEventOutcome;
/// use fhir::r5::types;
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

/// Actor involved in the event.
///
/// An agent is a person, organization, device, or software that participated in
/// the audited activity, described by its role, identity, location, and network
/// access point.
/// # Examples
///
/// ```
/// use fhir::r5::resources::audit_event::AuditEventAgent;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`requestor`](Self::requestor) (FHIR `_requestor`).
    #[serde(rename = "_requestor")]
    pub requestor_ext: Option<types::Element>,

    /// The agent location when the event occurred
    pub location: Option<types::Reference>,

    /// Policy that authorized the agent participation in the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<types::Uri>,
    /// Primitive extension sibling for [`policy`](Self::policy) (FHIR `_policy`).
    #[serde(rename = "_policy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_ext: Vec<Option<types::Element>>,

    /// The `AuditEvent.agent.network[x]` choice element (0..1); see [`AuditEventAgentNetwork`].
    #[serde(flatten)]
    pub network: Option<AuditEventAgentNetwork>,

    /// Allowable authorization for this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization: Vec<types::CodeableConcept>,
}

/// Audit Event Reporter.
///
/// Identifies the system or application that detected and reported the audited
/// event, including its logical site, the observing entity, and the type of
/// source.
/// # Examples
///
/// ```
/// use fhir::r5::resources::audit_event::AuditEventSource;
/// use fhir::r5::types;
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
    pub site: Option<types::Reference>,

    /// The identity of source detecting the event
    pub observer: types::Reference,

    /// The type of source where event originated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,
}

/// Data or objects used.
///
/// An entity describes a specific piece of data or an object that was involved
/// in the audited activity, including its role, security labels, query
/// parameters, and additional key/value details.
/// # Examples
///
/// ```
/// use fhir::r5::resources::audit_event::AuditEventEntity;
/// use fhir::r5::types;
///
/// let value = AuditEventEntity {
///     query: Some(types::Base64Binary("YWJj".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `query` is the name this serializes to on the wire.
/// assert_eq!(json["query"], ::serde_json::json!("YWJj"));
///
/// let back: AuditEventEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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

    /// Query parameters
    pub query: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`query`](Self::query) (FHIR `_query`).
    #[serde(rename = "_query")]
    pub query_ext: Option<types::Element>,

    /// Additional Information about the entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<AuditEventEntityDetail>,

    /// Entity is attributed to this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<AuditEventAgent>,
}

/// Additional Information about the entity.
///
/// A name/value pair providing extra descriptive information about the entity,
/// where the value may be one of several data types.
/// # Examples
///
/// ```
/// use fhir::r5::resources::audit_event::AuditEventEntityDetail;
/// use fhir::r5::types;
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
    pub r#type: types::CodeableConcept,

    /// The `AuditEvent.entity.detail.value[x]` choice element (0..1); see [`AuditEventEntityDetailValue`].
    #[serde(flatten)]
    pub value: Option<AuditEventEntityDetailValue>,
}

/// The `AuditEvent.agent.network[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AuditEventAgentNetwork {
    /// `networkReference` variant.
    #[fhir("networkReference")]
    Reference(Box<types::Reference>),
    /// `networkUri` variant.
    #[fhir("networkUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `networkString` variant.
    #[fhir("networkString")]
    String(crate::r5::choice::Primitive<types::String>),
}

/// The `AuditEvent.entity.detail.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueTime` variant.
    #[fhir("valueTime")]
    Time(crate::r5::choice::Primitive<types::Time>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueBase64Binary` variant.
    #[fhir("valueBase64Binary")]
    Base64Binary(crate::r5::choice::Primitive<types::Base64Binary>),
}

/// The `AuditEvent.occurred[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AuditEventOccurred {
    /// `occurredPeriod` variant.
    #[fhir("occurredPeriod")]
    Period(Box<types::Period>),
    /// `occurredDateTime` variant.
    #[fhir("occurredDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
