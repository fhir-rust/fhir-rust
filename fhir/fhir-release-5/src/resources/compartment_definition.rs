//! CompartmentDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CompartmentDefinition
//!
//! Version: 5.0.0
//!
//! CompartmentDefinition Resource: A compartment definition that defines how resources are accessed on a server.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A compartment definition that defines how resources are accessed on a
/// server.
///
/// CompartmentDefinition describes a logical grouping of resources (a
/// compartment) such as Patient, Encounter, or Practitioner, and specifies
/// which search parameters relate each resource type to that compartment. It
/// is used in FHIR R5 to drive access, filtering, and `$everything`-style
/// operations by identifying the resources that belong together.
///
/// A server declares its supported compartments as a set of
/// CompartmentDefinition resources so that clients and other systems can
/// discover, for a given compartment code (for example `Patient`), which
/// resource types participate in that compartment and which search
/// parameter on each resource type links it back to the compartment's
/// membership resource. This underpins compartment-based search (such as
/// `GET /Patient/{id}/*`) and access-control policies that scope visibility
/// to the resources belonging to a particular patient, practitioner, or
/// other compartment-defining resource.
///
/// See also: the compartment's membership resource is typically
/// [`Patient`](crate::r5::resources::patient::Patient) or a similar resource
/// such as `Encounter`, `Practitioner`, `RelatedPerson`, `Device`, or
/// `EpisodeOfCare`; compartment membership rules are commonly consumed
/// alongside `SearchParameter` definitions and access-control mechanisms
/// such as `CapabilityStatement`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::compartment_definition::CompartmentDefinition;
/// use fhir::r5::types;
///
/// let value = CompartmentDefinition {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: CompartmentDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompartmentDefinition {
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

    /// Canonical identifier for this compartment definition, represented as a
    /// URI (globally unique); used to reference this definition from other
    /// artifacts such as a `CapabilityStatement`
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business version of the compartment definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `CompartmentDefinition.versionAlgorithm[x]` choice element (0..1); see [`CompartmentDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<CompartmentDefinitionVersionAlgorithm>,

    /// Name for this compartment definition (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this compartment definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
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

    /// Natural language description of the compartment definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Why this compartment definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Patient | Encounter | RelatedPerson | Practitioner | Device |
    /// EpisodeOfCare; identifies which resource type defines membership in
    /// this compartment
    pub code: crate::r5::coded::Coded<crate::r5::codes::CompartmentType>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Whether the compartment search syntax (`GET [base]/[type]/{id}/*`) is
    /// supported for this compartment
    pub search: types::Boolean,
    /// Primitive extension sibling for [`search`](Self::search) (FHIR `_search`).
    #[serde(rename = "_search")]
    pub search_ext: Option<types::Element>,

    /// How each resource type is related to the compartment, including the
    /// search parameter that links it to the membership resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<CompartmentDefinitionResource>,
}

/// How a resource is related to the compartment.
///
/// Information about how a resource is related to the compartment.
/// # Examples
///
/// ```
/// use fhir::r5::resources::compartment_definition::CompartmentDefinitionResource;
/// use fhir::r5::types;
///
/// let value = CompartmentDefinitionResource {
///     start_param: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `startParam` is the name this serializes to on the wire.
/// assert_eq!(json["startParam"], ::serde_json::json!("http://example.org"));
///
/// let back: CompartmentDefinitionResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CompartmentDefinitionResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of resource type
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Search Parameter Name, or chained parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub param: Vec<types::String>,
    /// Primitive extension sibling for [`param`](Self::param) (FHIR `_param`).
    #[serde(rename = "_param")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub param_ext: Vec<Option<types::Element>>,

    /// Additional documentation about the resource and compartment
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// Search Param for interpreting $everything.start
    pub start_param: Option<types::Uri>,
    /// Primitive extension sibling for [`start_param`](Self::start_param) (FHIR `_startParam`).
    #[serde(rename = "_startParam")]
    pub start_param_ext: Option<types::Element>,

    /// Search Param for interpreting $everything.end
    pub end_param: Option<types::Uri>,
    /// Primitive extension sibling for [`end_param`](Self::end_param) (FHIR `_endParam`).
    #[serde(rename = "_endParam")]
    pub end_param_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CompartmentDefinition;

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
/// The `CompartmentDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CompartmentDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
