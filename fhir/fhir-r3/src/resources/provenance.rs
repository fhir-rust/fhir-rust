//! Provenance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Provenance
//!
//!
//!
//! Who, What, When for a set of resources
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Provenance Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::provenance::Provenance;
///
/// let value = Provenance::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Provenance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
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
    pub contained: Vec<crate::r3::resources::Resource>,

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

    /// Policy or plan the activity was defined by
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<types::Uri>,
    /// Primitive extension sibling for [`policy`](Self::policy) (FHIR `_policy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_policy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_ext: Vec<Option<types::Element>>,

    /// Where the activity occurred, if relevant
    pub location: Option<types::Reference<crate::r3::resources::Location>>,

    /// Reason the activity is occurring
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::Coding>,

    /// Activity that occurred
    pub activity: Option<types::Coding>,

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
/// use fhir::r3::resources::provenance::ProvenanceAgent;
/// use fhir::r3::types;
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
#[serde(from = "ProvenanceAgentDe")]
#[fhir_version("r3")]
pub struct ProvenanceAgent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What the agents role was
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,

    /// Who participated
    /// The `Provenance.agent.who[x]` choice element (1..1); see [`ProvenanceAgentWho`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub who: Option<ProvenanceAgentWho>,

    /// Who the agent is representing
    /// The `Provenance.agent.onBehalfOf[x]` choice element (0..1); see [`ProvenanceAgentOnBehalfOf`].
    #[serde(flatten)]
    pub on_behalf_of: Option<ProvenanceAgentOnBehalfOf>,

    /// Type of relationship between agents
    pub related_agent_type: Option<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProvenanceAgentDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    role: Vec<types::CodeableConcept>,
    #[serde(flatten)]
    who: crate::r3::choice::Slot<ProvenanceAgentWho>,
    #[serde(flatten)]
    on_behalf_of: crate::r3::choice::Slot<ProvenanceAgentOnBehalfOf>,
    related_agent_type: Option<types::CodeableConcept>,
}

impl ::core::convert::From<ProvenanceAgentDe> for ProvenanceAgent {
    fn from(v: ProvenanceAgentDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            role: v.role,
            who: v.who.0,
            on_behalf_of: v.on_behalf_of.0,
            related_agent_type: v.related_agent_type,
        }
    }
}

/// An entity used in this activity.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::provenance::ProvenanceEntity;
/// use fhir::r3::types;
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
#[serde(from = "ProvenanceEntityDe")]
#[fhir_version("r3")]
pub struct ProvenanceEntity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// derivation | revision | quotation | source | removal
    pub role: crate::coded::Coded<crate::r3::codes::ProvenanceEntityRole>,
    /// Primitive extension sibling for [`role`](Self::role) (FHIR `_role`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_role")]
    pub role_ext: Option<types::Element>,

    /// Identity of entity
    /// The `Provenance.entity.what[x]` choice element (1..1); see [`ProvenanceEntityWhat`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub what: Option<ProvenanceEntityWhat>,

    /// Entity is attributed to this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent: Vec<ProvenanceAgent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProvenanceEntityDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    role: crate::coded::Coded<crate::r3::codes::ProvenanceEntityRole>,
    #[serde(rename = "_role")]
    role_ext: Option<types::Element>,
    #[serde(flatten)]
    what: crate::r3::choice::Slot<ProvenanceEntityWhat>,
    #[serde(default)]
    agent: Vec<ProvenanceAgent>,
}

impl ::core::convert::From<ProvenanceEntityDe> for ProvenanceEntity {
    fn from(v: ProvenanceEntityDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            role: v.role,
            role_ext: v.role_ext,
            what: v.what.0,
            agent: v.agent,
        }
    }
}

/// The `Provenance.agent.who[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ProvenanceAgentWho {
    /// `whoUri` variant.
    #[fhir("whoUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `whoReference` variant.
    #[fhir("whoReference")]
    Reference(Box<types::Reference>),
}

/// The `Provenance.agent.onBehalfOf[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ProvenanceAgentOnBehalfOf {
    /// `onBehalfOfUri` variant.
    #[fhir("onBehalfOfUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `onBehalfOfReference` variant.
    #[fhir("onBehalfOfReference")]
    Reference(Box<types::Reference>),
}

/// The `Provenance.entity.what[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ProvenanceEntityWhat {
    /// `whatUri` variant.
    #[fhir("whatUri")]
    Uri(crate::r3::choice::Primitive<types::Uri>),
    /// `whatReference` variant.
    #[fhir("whatReference")]
    Reference(Box<types::Reference>),
    /// `whatIdentifier` variant.
    #[fhir("whatIdentifier")]
    Identifier(Box<types::Identifier>),
}
