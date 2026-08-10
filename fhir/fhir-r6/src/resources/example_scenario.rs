//! ExampleScenario
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExampleScenario
//!
//! Version: 6.0.0-ballot3
//!
//! A computable description of a set of actors and the interactions between
//! those actors
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A walkthrough of a workflow showing the interaction between systems and the
/// instances shared, possibly including the evolution of instances over time.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenario;
/// use fhir::r6::types;
///
/// let value = ExampleScenario {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenario = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ExampleScenarioDe")]
#[fhir_version("r6")]
pub struct ExampleScenario {
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

    /// Canonical identifier for this example scenario, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the example scenario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the example scenario
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `ExampleScenario.versionAlgorithm[x]` choice element (0..1); see [`ExampleScenarioVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ExampleScenarioVersionAlgorithm>,

    /// Name for this example scenario (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this example scenario (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the ExampleScenario
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for example scenario (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// The purpose of the example, e.g. to illustrate a scenario
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Individual involved in exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ExampleScenarioActor>,

    /// Data used in the scenario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ExampleScenarioInstance>,

    /// Major process within scenario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process: Vec<ExampleScenarioProcess>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleScenarioDe {
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
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r6::choice::Slot<ExampleScenarioVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    #[serde(default)]
    actor: Vec<ExampleScenarioActor>,
    #[serde(default)]
    instance: Vec<ExampleScenarioInstance>,
    #[serde(default)]
    process: Vec<ExampleScenarioProcess>,
}

impl ::core::convert::From<ExampleScenarioDe> for ExampleScenario {
    fn from(v: ExampleScenarioDe) -> Self {
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
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            actor: v.actor,
            instance: v.instance,
            process: v.process,
        }
    }
}

/// A system or person who shares or receives an instance within the scenario.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioActor;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioActor {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: ExampleScenarioActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExampleScenarioActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// ID or acronym of the actor
    pub key: types::String,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// system | non-system | individual | patient | practitioner |
    /// related-person | device | collective | care-team | group |
    /// healthcare-service | organization
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::ActordefinitionActorType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Label for actor when rendering
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Details about actor
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Formal definition of actor
    pub definition: Option<types::Canonical>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    pub definition_ext: Option<types::Element>,
}

/// A single data collection that is shared as part of the scenario.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioInstance;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioInstance {
///     structure_version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `structureVersion` is the name this serializes to on the wire.
/// assert_eq!(json["structureVersion"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ExampleScenarioInstanceDe")]
#[fhir_version("r6")]
pub struct ExampleScenarioInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// ID or acronym of the instance
    pub key: types::String,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Data structure for example
    pub structure_type: types::Coding,

    /// E.g. 4.0.1
    pub structure_version: Option<types::String>,
    /// Primitive extension sibling for [`structure_version`](Self::structure_version) (FHIR `_structureVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_structureVersion")]
    pub structure_version_ext: Option<types::Element>,

    /// Rules instance adheres to
    /// The `ExampleScenario.instance.structureProfile[x]` choice element (0..1); see [`ExampleScenarioInstanceStructureProfile`].
    #[serde(flatten)]
    pub structure_profile: Option<ExampleScenarioInstanceStructureProfile>,

    /// Label for instance
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Human-friendly description of the instance
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Example instance data
    pub content: Option<types::Reference>,

    /// Snapshot of instance that changes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<ExampleScenarioInstanceVersion>,

    /// Resources contained in the instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained_instance: Vec<ExampleScenarioInstanceContainedInstance>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExampleScenarioInstanceDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    key: types::String,
    #[serde(rename = "_key")]
    key_ext: Option<types::Element>,
    structure_type: types::Coding,
    structure_version: Option<types::String>,
    #[serde(rename = "_structureVersion")]
    structure_version_ext: Option<types::Element>,
    #[serde(flatten)]
    structure_profile: crate::r6::choice::Slot<ExampleScenarioInstanceStructureProfile>,
    title: types::String,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    content: Option<types::Reference>,
    #[serde(default)]
    version: Vec<ExampleScenarioInstanceVersion>,
    #[serde(default)]
    contained_instance: Vec<ExampleScenarioInstanceContainedInstance>,
}

impl ::core::convert::From<ExampleScenarioInstanceDe> for ExampleScenarioInstance {
    fn from(v: ExampleScenarioInstanceDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            key: v.key,
            key_ext: v.key_ext,
            structure_type: v.structure_type,
            structure_version: v.structure_version,
            structure_version_ext: v.structure_version_ext,
            structure_profile: v.structure_profile.0,
            title: v.title,
            title_ext: v.title_ext,
            description: v.description,
            description_ext: v.description_ext,
            content: v.content,
            version: v.version,
            contained_instance: v.contained_instance,
        }
    }
}

/// References to other instances that can be found within this instance (e.g.
/// the observations contained in a bundle).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioInstanceContainedInstance;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioInstanceContainedInstance {
///     version_reference: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `versionReference` is the name this serializes to on the wire.
/// assert_eq!(json["versionReference"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioInstanceContainedInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExampleScenarioInstanceContainedInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Key of contained instance
    pub instance_reference: types::String,
    /// Primitive extension sibling for [`instance_reference`](Self::instance_reference) (FHIR `_instanceReference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instanceReference")]
    pub instance_reference_ext: Option<types::Element>,

    /// Key of contained instance version
    pub version_reference: Option<types::String>,
    /// Primitive extension sibling for [`version_reference`](Self::version_reference) (FHIR `_versionReference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_versionReference")]
    pub version_reference_ext: Option<types::Element>,
}

/// Represents the instance as it was at a specific time-point.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioInstanceVersion;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioInstanceVersion {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: ExampleScenarioInstanceVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExampleScenarioInstanceVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// ID or acronym of the version
    pub key: types::String,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Label for instance version
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Details about version
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Example instance version data
    pub content: Option<types::Reference>,
}

/// A group of operations that represents a significant step within a scenario.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioProcess;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioProcess {
///     pre_conditions: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preConditions` is the name this serializes to on the wire.
/// assert_eq!(json["preConditions"], ::serde_json::json!("# Heading"));
///
/// let back: ExampleScenarioProcess = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExampleScenarioProcess {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for procss
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Human-friendly description of the process
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Status before process starts
    pub pre_conditions: Option<types::Markdown>,
    /// Primitive extension sibling for [`pre_conditions`](Self::pre_conditions) (FHIR `_preConditions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preConditions")]
    pub pre_conditions_ext: Option<types::Element>,

    /// Status after successful completion
    pub post_conditions: Option<types::Markdown>,
    /// Primitive extension sibling for [`post_conditions`](Self::post_conditions) (FHIR `_postConditions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_postConditions")]
    pub post_conditions_ext: Option<types::Element>,

    /// Event within of the process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<ExampleScenarioProcessStep>,
}

/// A significant action that occurs as part of the process.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioProcessStep;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioProcessStep {
///     number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `number` is the name this serializes to on the wire.
/// assert_eq!(json["number"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioProcessStep = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExampleScenarioProcessStep {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Sequential number of the step
    pub number: Option<types::String>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// Step is nested process
    pub process: Option<ExampleScenarioProcess>,

    /// Step is nested workflow
    pub workflow: Option<types::Canonical>,
    /// Primitive extension sibling for [`workflow`](Self::workflow) (FHIR `_workflow`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_workflow")]
    pub workflow_ext: Option<types::Element>,

    /// Step is simple action
    pub operation: Option<ExampleScenarioProcessStepOperation>,

    /// Alternate non-typical step action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternative: Vec<ExampleScenarioProcessStepAlternative>,

    /// Pause in the flow?
    pub pause: Option<types::Boolean>,
    /// Primitive extension sibling for [`pause`](Self::pause) (FHIR `_pause`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_pause")]
    pub pause_ext: Option<types::Element>,
}

/// Indicates an alternative step that can be taken instead of the sub-process,
/// scenario or operation. E.g. to represent
/// non-happy-path/exceptional/atypical circumstances.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioProcessStepAlternative;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioProcessStepAlternative {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: ExampleScenarioProcessStepAlternative = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExampleScenarioProcessStepAlternative {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for alternative
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Human-readable description of option
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Alternative action(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<ExampleScenarioProcessStep>,
}

/// The step represents a single operation invoked on receiver by sender.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::example_scenario::ExampleScenarioProcessStepOperation;
/// use fhir::r6::types;
///
/// let value = ExampleScenarioProcessStepOperation {
///     initiator_active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `initiatorActive` is the name this serializes to on the wire.
/// assert_eq!(json["initiatorActive"], ::serde_json::json!(true));
///
/// let back: ExampleScenarioProcessStepOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ExampleScenarioProcessStepOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Kind of action
    pub r#type: Option<types::Coding>,

    /// Label for step
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Who starts the operation
    pub initiator: Option<types::String>,
    /// Primitive extension sibling for [`initiator`](Self::initiator) (FHIR `_initiator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_initiator")]
    pub initiator_ext: Option<types::Element>,

    /// Who receives the operation
    pub receiver: Option<types::String>,
    /// Primitive extension sibling for [`receiver`](Self::receiver) (FHIR `_receiver`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_receiver")]
    pub receiver_ext: Option<types::Element>,

    /// Human-friendly description of the operation
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Initiator stays active?
    pub initiator_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`initiator_active`](Self::initiator_active) (FHIR `_initiatorActive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_initiatorActive")]
    pub initiator_active_ext: Option<types::Element>,

    /// Receiver stays active?
    pub receiver_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`receiver_active`](Self::receiver_active) (FHIR `_receiverActive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_receiverActive")]
    pub receiver_active_ext: Option<types::Element>,

    /// Instance transmitted on invocation
    pub request: Option<ExampleScenarioInstanceContainedInstance>,

    /// Instance transmitted on invocation response
    pub response: Option<ExampleScenarioInstanceContainedInstance>,
}

/// The `ExampleScenario.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ExampleScenarioVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `ExampleScenario.instance.structureProfile[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ExampleScenarioInstanceStructureProfile {
    /// `structureProfileCanonical` variant.
    #[fhir("structureProfileCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `structureProfileUri` variant.
    #[fhir("structureProfileUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExampleScenario;

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
