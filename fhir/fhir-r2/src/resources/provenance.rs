//! Provenance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Provenance
//!
//!
//!
//! Who, What, When for a set of resources
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Provenance Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::provenance::Provenance;
///
/// let value = Provenance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Provenance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Provenance {
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

    /// Target Reference(s) (usually version specific)
    pub target: ::vec1::Vec1<types::Reference>,

    /// When the activity occurred
    pub period: Option<types::Period>,

    /// When the activity was recorded / updated
    pub recorded: types::Instant,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Reason the activity is occurring
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Activity that occurred
    pub activity: Option<types::CodeableConcept>,

    /// Where the activity occurred, if relevant
    pub location: Option<types::Reference<crate::r2::resources::Location>>,

    /// Policy or plan the activity was defined by
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<types::Uri>,
    /// Primitive extension sibling for [`policy`](Self::policy) (FHIR `_policy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_policy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_ext: Vec<Option<types::Element>>,

    /// Agents involved in creating resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<ProvenanceAgent>,

    /// An entity used in this activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity: Vec<ProvenanceEntity>,

    /// Signature on target
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<types::Signature>,
}

/// An agent takes a role in an activity such that the agent can be assigned
/// some degree of responsibility for the activity taking place. An agent can
/// be a person, an organization, software, or other entities that may be
/// ascribed responsibility.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::provenance::ProvenanceAgent;
/// use fhir::r2::types;
///
/// let value = ProvenanceAgent {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ProvenanceAgent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ProvenanceAgent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What the agents involvement was
    pub role: types::Coding,

    /// Individual, device or organization playing role
    pub actor: Option<types::Reference>,

    /// Authorization-system identifier for the agent
    pub user_id: Option<types::Identifier>,

    /// Track delegation between agents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_agent: Vec<ProvenanceAgentRelatedAgent>,
}

/// A relationship between two the agents referenced in this resource. This is
/// defined to allow for explicit description of the delegation between agents.
/// For example, this human author used this device, or one person acted on
/// another's behest.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::provenance::ProvenanceAgentRelatedAgent;
/// use fhir::r2::types;
///
/// let value = ProvenanceAgentRelatedAgent {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ProvenanceAgentRelatedAgent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ProvenanceAgentRelatedAgent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of relationship between agents
    pub r#type: types::CodeableConcept,

    /// Reference to other agent in this resource by identifier
    pub target: types::Uri,
    /// Primitive extension sibling for [`target`](Self::target) (FHIR `_target`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_target")]
    pub target_ext: Option<types::Element>,
}

/// An entity used in this activity.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::provenance::ProvenanceEntity;
/// use fhir::r2::types;
///
/// let value = ProvenanceEntity {
///     display: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `display` is the name this serializes to on the wire.
/// assert_eq!(json["display"], ::serde_json::json!("abc"));
///
/// let back: ProvenanceEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ProvenanceEntity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// derivation | revision | quotation | source
    pub role: crate::coded::Coded<crate::r2::codes::ProvenanceEntityRole>,
    /// Primitive extension sibling for [`role`](Self::role) (FHIR `_role`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_role")]
    pub role_ext: Option<types::Element>,

    /// The type of resource in this entity
    pub r#type: types::Coding,

    /// Identity of entity
    pub reference: types::Uri,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Human description of entity
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Entity is attributed to this agent
    pub agent: Option<ProvenanceAgent>,
}
