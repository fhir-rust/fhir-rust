//! Requirements
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Requirements
//!
//! Version: 5.0.0
//!
//! Requirements Resource: The Requirements resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Requirements resource is a conformance resource that gathers a set of
/// requirement statements describing what a system, actor, or specification is
/// expected to do. Each statement carries one or more conformance verbs (SHALL,
/// SHOULD, MAY, SHOULD-NOT) together with the requirement itself expressed as
/// markdown, and statements may be derived from, refined by, or traced to other
/// statements. In FHIR R5 it is used primarily within implementation guides and
/// solution designs to formally capture stakeholder expectations, organize them
/// into a traceable hierarchy, and link each expectation to the actors that
/// carry the obligation and to the design artifacts that satisfy it. This makes
/// it a foundation for requirements traceability, gap analysis, and conformance
/// planning across an interoperability project.
///
/// Related resources: the statements typically reference an `ActorDefinition`
/// through the actor field, and Requirements resources are often bundled and
/// published alongside an `ImplementationGuide`. Common building-block types
/// used by its fields include [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`UsageContext`](crate::r5::types::UsageContext),
/// [`ContactDetail`](crate::r5::types::ContactDetail), and
/// [`Identifier`](crate::r5::types::Identifier).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::requirements::Requirements;
/// use fhir::r5::types;
///
/// let value = Requirements {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: Requirements = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
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

    /// Canonical identifier for this Requirements, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the Requirements (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the Requirements
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `Requirements.versionAlgorithm[x]` choice element (0..1); see [`RequirementsVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<RequirementsVersionAlgorithm>,

    /// Name for this Requirements (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this Requirements (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Publication lifecycle state of this Requirements: draft, active, retired, or unknown
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

    /// Natural language description summarizing what this Requirements resource covers and why
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for Requirements (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Explanation of why this Requirements is needed and the intent behind its use
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

    /// Other set of Requirements this builds on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`).
    #[serde(rename = "_derivedFrom")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_ext: Vec<Option<types::Element>>,

    /// External artifact (rule/document etc. that) created this set of requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<types::Url>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`).
    #[serde(rename = "_reference")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_ext: Vec<Option<types::Element>>,

    /// Canonical references to the ActorDefinition resources that bear these requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<types::Canonical>,
    /// Primitive extension sibling for [`actor`](Self::actor) (FHIR `_actor`).
    #[serde(rename = "_actor")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_ext: Vec<Option<types::Element>>,

    /// The individual requirement statements, each captured as a RequirementsStatement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statement: Vec<RequirementsStatement>,
}

/// A single requirement statement within a Requirements resource, carrying a
/// unique key, an optional human label, one or more conformance verbs, and the
/// actual requirement expressed as markdown. Statements may be derived from or
/// refine other statements and can reference the design artifacts that satisfy
/// them.
/// # Examples
///
/// ```
/// use fhir::r5::resources::requirements::RequirementsStatement;
/// use fhir::r5::types;
///
/// let value = RequirementsStatement {
///     derived_from: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `derivedFrom` is the name this serializes to on the wire.
/// assert_eq!(json["derivedFrom"], ::serde_json::json!("abc"));
///
/// let back: RequirementsStatement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequirementsStatement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Key that identifies this statement
    pub key: types::Id,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`).
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Short Human label for this statement
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`).
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Conformance verbs that set the strength of this requirement: SHALL, SHOULD, MAY, or SHOULD-NOT
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conformance: Vec<crate::r5::coded::Coded<crate::r5::codes::ConformanceExpectation>>,
    /// Primitive extension sibling for [`conformance`](Self::conformance) (FHIR `_conformance`).
    #[serde(rename = "_conformance")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conformance_ext: Vec<Option<types::Element>>,

    /// Set to true if requirements statement is conditional
    pub conditionality: Option<types::Boolean>,
    /// Primitive extension sibling for [`conditionality`](Self::conditionality) (FHIR `_conditionality`).
    #[serde(rename = "_conditionality")]
    pub conditionality_ext: Option<types::Element>,

    /// The actual requirement text, expressed as markdown
    pub requirement: types::Markdown,
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`).
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// Another statement this clarifies/restricts ([url#]key)
    pub derived_from: Option<types::String>,
    /// Primitive extension sibling for [`derived_from`](Self::derived_from) (FHIR `_derivedFrom`).
    #[serde(rename = "_derivedFrom")]
    pub derived_from_ext: Option<types::Element>,

    /// A larger requirement that this requirement helps to refine and enable
    pub parent: Option<types::String>,
    /// Primitive extension sibling for [`parent`](Self::parent) (FHIR `_parent`).
    #[serde(rename = "_parent")]
    pub parent_ext: Option<types::Element>,

    /// Design artifact that satisfies this requirement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub satisfied_by: Vec<types::Url>,
    /// Primitive extension sibling for [`satisfied_by`](Self::satisfied_by) (FHIR `_satisfiedBy`).
    #[serde(rename = "_satisfiedBy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub satisfied_by_ext: Vec<Option<types::Element>>,

    /// External artifact (rule/document etc. that) created this requirement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference: Vec<types::Url>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`).
    #[serde(rename = "_reference")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_ext: Vec<Option<types::Element>>,

    /// The person, organization, or other actor who requested or originated this statement
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Requirements;

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
/// The `Requirements.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum RequirementsVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
