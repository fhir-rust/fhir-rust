//! ExampleScenario
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExampleScenario
//!
//! Version: 5.0.0
//!
//! ExampleScenario Resource: A walkthrough of a workflow showing the interaction between systems and the instances shared, possibly including the evolution of instances over time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ExampleScenario
///
/// A walkthrough of a workflow showing the interaction between systems and the
/// instances shared, possibly including the evolution of instances over time.
/// It is used to describe expected system behaviour by illustrating actors,
/// example instances of data, and the ordered processes and steps through which
/// those instances are exchanged. This makes it a documentation and testing
/// artifact rather than something used directly in production data exchange.
///
/// In FHIR R5, `ExampleScenario` is a canonical, conformance resource used
/// mainly by implementation guide authors, tooling vendors, and reviewers to
/// illustrate how a set of exchanges should work end to end. An actor
/// (`ExampleScenarioActor`) represents a person or system taking part; an
/// instance (`ExampleScenarioInstance`) represents a piece of example data,
/// optionally with versions showing how it changes over time
/// (`ExampleScenarioInstanceVersion`); and a process (`ExampleScenarioProcess`)
/// groups the ordered steps (`ExampleScenarioProcessStep`) that move those
/// instances between actors, including nested processes, referenced workflows,
/// simple request/response operations (`ExampleScenarioProcessStepOperation`),
/// and alternative branches (`ExampleScenarioProcessStepAlternative`). Because
/// it is a narrative and testing artifact rather than clinical data, it is
/// typically published alongside profiles and capability statements in an
/// implementation guide rather than exchanged during routine patient care.
///
/// # Related resources
///
/// The example instances referenced within a scenario commonly represent other
/// FHIR resources, such as [`Patient`](crate::r5::resources::patient::Patient),
/// and scenario elements such as jurisdiction make use of shared data types
/// like [`CodeableConcept`](crate::r5::types::CodeableConcept). See also the
/// related conformance resources `StructureDefinition` and
/// `ImplementationGuide`, which `ExampleScenario` is often published alongside.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenario;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario {
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

    /// Canonical identifier for this example scenario, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the example scenario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the example scenario
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ExampleScenario.versionAlgorithm[x]` choice element (0..1); see [`ExampleScenarioVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ExampleScenarioVersionAlgorithm>,

    /// To be removed?
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this example scenario (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication lifecycle status of this scenario: draft | active | retired | unknown.
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

    /// Natural language description summarizing the purpose and content of the ExampleScenario.
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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

    /// The people or systems (actors) that participate in and exchange data within the scenario.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ExampleScenarioActor>,

    /// The example data instances, and their versions over time, that are exchanged in the scenario.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ExampleScenarioInstance>,

    /// The major, ordered processes composed of steps that describe how the scenario unfolds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process: Vec<ExampleScenarioProcess>,
}

/// ExampleScenarioActor
///
/// A system or person who shares or receives an instance within the scenario.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioActor;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`).
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// person | system
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ExamplescenarioActorType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Label for actor when rendering
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Details about actor
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// ExampleScenarioInstance
///
/// A single piece of example data used within the scenario, described by its
/// structure and optionally versioned over time.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioInstance;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`).
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Data structure for example
    pub structure_type: types::Coding,

    /// E.g. 4.0.1
    pub structure_version: Option<types::String>,
    /// Primitive extension sibling for [`structure_version`](Self::structure_version) (FHIR `_structureVersion`).
    #[serde(rename = "_structureVersion")]
    pub structure_version_ext: Option<types::Element>,

    /// The `ExampleScenario.instance.structureProfile[x]` choice element (0..1); see [`ExampleScenarioInstanceStructureProfile`].
    #[serde(flatten)]
    pub structure_profile: Option<ExampleScenarioInstanceStructureProfile>,

    /// Label for instance
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Human-friendly description of the instance
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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

/// ExampleScenarioInstanceVersion
///
/// A specific snapshot of an instance representing how it appears at a
/// particular point in the workflow.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioInstanceVersion;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`).
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Label for instance version
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Details about version
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Example instance version data
    pub content: Option<types::Reference>,
}

/// ExampleScenarioInstanceContainedInstance
///
/// A reference from one instance to another instance (and optionally a specific
/// version of it) that it contains.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioInstanceContainedInstance;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`instance_reference`](Self::instance_reference) (FHIR `_instanceReference`).
    #[serde(rename = "_instanceReference")]
    pub instance_reference_ext: Option<types::Element>,

    /// Key of contained instance version
    pub version_reference: Option<types::String>,
    /// Primitive extension sibling for [`version_reference`](Self::version_reference) (FHIR `_versionReference`).
    #[serde(rename = "_versionReference")]
    pub version_reference_ext: Option<types::Element>,
}

/// ExampleScenarioProcess
///
/// A major process within the scenario, comprising an ordered set of steps and
/// the conditions that hold before and after it runs.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioProcess;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Human-friendly description of the process
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Status before process starts
    pub pre_conditions: Option<types::Markdown>,
    /// Primitive extension sibling for [`pre_conditions`](Self::pre_conditions) (FHIR `_preConditions`).
    #[serde(rename = "_preConditions")]
    pub pre_conditions_ext: Option<types::Element>,

    /// Status after successful completion
    pub post_conditions: Option<types::Markdown>,
    /// Primitive extension sibling for [`post_conditions`](Self::post_conditions) (FHIR `_postConditions`).
    #[serde(rename = "_postConditions")]
    pub post_conditions_ext: Option<types::Element>,

    /// Event within of the process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<ExampleScenarioProcessStep>,
}

/// ExampleScenarioProcessStep
///
/// A single event within a process, which may itself be a nested process, a
/// referenced workflow, a simple operation, or a pause, and may carry
/// alternative branches.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioProcessStep;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`).
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// Step is nested process
    pub process: Option<ExampleScenarioProcess>,

    /// Step is nested workflow
    pub workflow: Option<types::Canonical>,
    /// Primitive extension sibling for [`workflow`](Self::workflow) (FHIR `_workflow`).
    #[serde(rename = "_workflow")]
    pub workflow_ext: Option<types::Element>,

    /// Step is simple action
    pub operation: Option<ExampleScenarioProcessStepOperation>,

    /// Alternate non-typical step action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternative: Vec<ExampleScenarioProcessStepAlternative>,

    /// Pause in the flow?
    pub pause: Option<types::Boolean>,
    /// Primitive extension sibling for [`pause`](Self::pause) (FHIR `_pause`).
    #[serde(rename = "_pause")]
    pub pause_ext: Option<types::Element>,
}

/// ExampleScenarioProcessStepOperation
///
/// A simple action within a step describing an exchange between an initiator and
/// a receiver, including the request and response instances involved.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioProcessStepOperation;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Who starts the operation
    pub initiator: Option<types::String>,
    /// Primitive extension sibling for [`initiator`](Self::initiator) (FHIR `_initiator`).
    #[serde(rename = "_initiator")]
    pub initiator_ext: Option<types::Element>,

    /// Who receives the operation
    pub receiver: Option<types::String>,
    /// Primitive extension sibling for [`receiver`](Self::receiver) (FHIR `_receiver`).
    #[serde(rename = "_receiver")]
    pub receiver_ext: Option<types::Element>,

    /// Human-friendly description of the operation
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Initiator stays active?
    pub initiator_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`initiator_active`](Self::initiator_active) (FHIR `_initiatorActive`).
    #[serde(rename = "_initiatorActive")]
    pub initiator_active_ext: Option<types::Element>,

    /// Receiver stays active?
    pub receiver_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`receiver_active`](Self::receiver_active) (FHIR `_receiverActive`).
    #[serde(rename = "_receiverActive")]
    pub receiver_active_ext: Option<types::Element>,

    /// Instance transmitted on invocation
    pub request: Option<ExampleScenarioInstanceContainedInstance>,

    /// Instance transmitted on invocation response
    pub response: Option<ExampleScenarioInstanceContainedInstance>,
}

/// ExampleScenarioProcessStepAlternative
///
/// An alternate, non-typical set of steps that may occur in place of the main
/// step under described conditions.
/// # Examples
///
/// ```
/// use fhir::r5::resources::example_scenario::ExampleScenarioProcessStepAlternative;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Human-readable description of option
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Alternative action(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<ExampleScenarioProcessStep>,
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
/// The `ExampleScenario.instance.structureProfile[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExampleScenarioInstanceStructureProfile {
    /// `structureProfileCanonical` variant.
    #[fhir("structureProfileCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
    /// `structureProfileUri` variant.
    #[fhir("structureProfileUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
}

/// The `ExampleScenario.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExampleScenarioVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
