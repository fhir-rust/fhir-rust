//! ActorDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ActorDefinition
//!
//! Version: 5.0.0
//!
//! ActorDefinition Resource: The ActorDefinition resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The ActorDefinition resource is used to describe an actor - a human or an
/// application that plays a role in data exchange, and that may have
/// obligations associated with the role the actor plays. Actors are the
/// participants in an exchange of information, and an ActorDefinition provides
/// a canonical, referenceable description of one such participant. It is
/// commonly used within implementation guides to formally define the systems
/// and people that a specification expects to interact.
///
/// An ActorDefinition does not describe a specific individual, organization,
/// or software product; rather it defines a role or category of participant
/// (for example a "client" or "server" system, or a "patient" or "provider"
/// person) that other conformance resources, such as CapabilityStatement or
/// StructureDefinition-based profiles, can reference. Implementation guide
/// authors use ActorDefinition to give each participant in an interoperability
/// scenario a stable, citable identity, and to document the expected
/// capabilities, behaviors, and constraints associated with fulfilling that
/// role. The `type` field distinguishes whether the actor is a `person` or a
/// `system`, and narrative fields such as `documentation` and `reference`
/// describe the actor's responsibilities and point implementers to further
/// guidance.
///
/// # Related resources
///
/// ActorDefinition is a conformance/terminology-infrastructure resource. It is
/// commonly referenced alongside `CapabilityStatement` (via `capabilities`)
/// and other canonical definitional resources within an implementation guide,
/// and it uses shared types such as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`Identifier`](crate::r5::types::Identifier), and
/// [`ContactDetail`](crate::r5::types::ContactDetail) for metadata. It is
/// distinct from resources that describe real-world participants, such as
/// [`Patient`](crate::r5::resources::patient::Patient) or `Practitioner`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::actor_definition::ActorDefinition;
/// use fhir::r5::types;
///
/// let value = ActorDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: ActorDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActorDefinition {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this actor definition, represented as a URI (globally unique), used to reference this actor from other resources
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the actor definition (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the actor definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ActorDefinition.versionAlgorithm[x]` choice element (0..1); see [`ActorDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ActorDefinitionVersionAlgorithm>,

    /// Name for this actor definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this actor definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication status of this actor definition: draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the actor
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for actor definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this actor definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Whether the actor represents a human participant or a software system: person | system
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ExamplescenarioActorType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Functionality associated with the actor
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Reference to more information about the actor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<types::Url>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`).
    #[serde(rename = "_reference")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_ext: Vec<Option<types::Element>>,

    /// Canonical reference to a CapabilityStatement describing the actor's expected behavior (if applicable)
    pub capabilities: Option<types::Canonical>,
    /// Primitive extension sibling for [`capabilities`](Self::capabilities) (FHIR `_capabilities`).
    #[serde(rename = "_capabilities")]
    pub capabilities_ext: Option<types::Element>,

    /// Definition of this actor in another context / IG
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`).
    #[serde(rename = "_derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_ext: Vec<Option<types::Element>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ActorDefinition;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
/// The `ActorDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ActorDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
