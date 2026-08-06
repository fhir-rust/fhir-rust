//! Provenance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Provenance
//!
//! Version: 6.0.0-ballot3
//!
//! Who, What, When for a set of resources
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Provenance of a resource is a record that describes entities and processes
/// involved in producing and delivering or otherwise influencing that
/// resource. Provenance provides a critical foundation for assessing
/// authenticity, enabling trust, and allowing reproducibility. Provenance
/// assertions are a form of contextual metadata and can themselves become
/// important records with their own provenance. Provenance statement indicates
/// clinical significance in terms of confidence in authenticity, reliability,
/// and trustworthiness, integrity, and stage in lifecycle (e.g. Document
/// Completion - has the artifact been legally authenticated), all of which may
/// impact security, privacy, and trust policies.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::provenance::Provenance;
///
/// let value = Provenance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Provenance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Provenance {
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

    /// Target Reference(s) (usually version specific)
    pub target: ::vec1::Vec1<types::Reference>,

    /// When the activity occurred
    /// The `Provenance.occurred[x]` choice element (0..1); see [`ProvenanceOccurred`].
    #[serde(flatten)]
    pub occurred: Option<ProvenanceOccurred>,

    /// When the activity was recorded / updated
    pub recorded: Option<types::Instant>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Policy or plan the activity was defined by
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<types::Uri>,
    /// Primitive extension sibling for [`policy`](Self::policy) (FHIR `_policy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_policy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_ext: Vec<Option<types::Element>>,

    /// Where the activity occurred
    pub location: Option<types::Reference>,

    /// Authorization (purposeOfUse) related to the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization: Vec<types::CodeableReference>,

    /// Why was the event performed?
    pub why: Option<types::Markdown>,
    /// Primitive extension sibling for [`why`](Self::why) (FHIR `_why`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_why")]
    pub why_ext: Option<types::Element>,

    /// Activity that occurred
    pub activity: Option<types::CodeableConcept>,

    /// Workflow authorization within which this event occurred
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The patient is the subject of the data created/updated (.target) by the
    /// activity
    pub patient: Option<types::Reference>,

    /// Encounter within which this event occurred or which the event is
    /// tightly associated
    pub encounter: Option<types::Reference>,

    /// Actor involved
    pub agent: ::vec1::Vec1<ProvenanceAgent>,

    /// An entity used in this activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity: Vec<ProvenanceEntity>,

    /// Signature on target
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<types::Signature>,
}

/// An actor taking a role in an activity for which it can be assigned some
/// degree of responsibility for the activity taking place.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::provenance::ProvenanceAgent;
/// use fhir::r6::types;
///
/// let value = ProvenanceAgent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ProvenanceAgent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ProvenanceAgent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// How the agent participated
    pub r#type: Option<types::CodeableConcept>,

    /// What the agents role was
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,

    /// The agent that participated in the event
    pub who: types::Reference,

    /// The agent that delegated
    pub on_behalf_of: Option<types::Reference>,
}

/// An entity used in this activity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::provenance::ProvenanceEntity;
/// use fhir::r6::types;
///
/// let value = ProvenanceEntity {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ProvenanceEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ProvenanceEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// revision | quotation | source | instantiates | removal
    pub role: crate::coded::Coded<crate::r6::codes::ProvenanceEntityRole>,
    /// Primitive extension sibling for [`role`](Self::role) (FHIR `_role`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_role")]
    pub role_ext: Option<types::Element>,

    /// Identity of entity
    pub what: types::Reference,

    /// Entity is attributed to this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<ProvenanceAgent>,
}

/// The `Provenance.occurred[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ProvenanceOccurred {
    /// `occurredPeriod` variant.
    #[fhir("occurredPeriod")]
    Period(Box<types::Period>),
    /// `occurredDateTime` variant.
    #[fhir("occurredDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
}
