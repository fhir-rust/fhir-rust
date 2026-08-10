//! ExampleScenario
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExampleScenario
//!
//! Version: 4.3.0
//!
//! Example of workflow instance
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Example of workflow instance.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenario;
/// use fhir::r4b::types;
///
/// let value = ExampleScenario {
///     url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: ExampleScenario = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
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
    pub contained: Vec<crate::r4b::resources::Resource>,

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

    /// Name for this example scenario (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
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

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for example scenario (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// The purpose of the example, e.g. to illustrate a scenario
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Actor participating in the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ExampleScenarioActor>,

    /// Each resource and each version that is present in the workflow
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ExampleScenarioInstance>,

    /// Each major process - a group of operations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process: Vec<ExampleScenarioProcess>,

    /// Another nested workflow
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflow: Vec<types::Canonical>,
    /// Primitive extension sibling for [`workflow`](Self::workflow) (FHIR `_workflow`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_workflow")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflow_ext: Vec<Option<types::Element>>,
}

/// Actor participating in the resource.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioActor;
/// use fhir::r4b::types;
///
/// let value = ExampleScenarioActor {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
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
    pub actor_id: types::String,
    /// Primitive extension sibling for [`actor_id`](Self::actor_id) (FHIR `_actorId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actorId")]
    pub actor_id_ext: Option<types::Element>,

    /// person | entity
    pub r#type: crate::coded::Coded<crate::r4b::codes::ExamplescenarioActorType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The name of the actor as shown in the page
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The description of the actor
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Each resource and each version that is present in the workflow.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioInstance;
/// use fhir::r4b::types;
///
/// let value = ExampleScenarioInstance {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ExampleScenarioInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The id of the resource for referencing
    pub resource_id: types::String,
    /// Primitive extension sibling for [`resource_id`](Self::resource_id) (FHIR `_resourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resourceId")]
    pub resource_id_ext: Option<types::Element>,

    /// The type of the resource
    pub resource_type: crate::coded::Coded<crate::r4b::codes::ResourceTypes>,
    /// Primitive extension sibling for [`resource_type`](Self::resource_type) (FHIR `_resourceType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resourceType")]
    pub resource_type_ext: Option<types::Element>,

    /// A short name for the resource instance
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Human-friendly description of the resource instance
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// A specific version of the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<ExampleScenarioInstanceVersion>,

    /// Resources contained in the instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained_instance: Vec<ExampleScenarioInstanceContainedInstance>,
}

/// Resources contained in the instance (e.g. the observations contained in a
/// bundle).
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioInstanceContainedInstance;
/// use fhir::r4b::types;
///
/// let value = ExampleScenarioInstanceContainedInstance {
///     version_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `versionId` is the name this serializes to on the wire.
/// assert_eq!(json["versionId"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioInstanceContainedInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ExampleScenarioInstanceContainedInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Each resource contained in the instance
    pub resource_id: types::String,
    /// Primitive extension sibling for [`resource_id`](Self::resource_id) (FHIR `_resourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resourceId")]
    pub resource_id_ext: Option<types::Element>,

    /// A specific version of a resource contained in the instance
    pub version_id: Option<types::String>,
    /// Primitive extension sibling for [`version_id`](Self::version_id) (FHIR `_versionId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_versionId")]
    pub version_id_ext: Option<types::Element>,
}

/// A specific version of the resource.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioInstanceVersion;
/// use fhir::r4b::types;
///
/// let value = ExampleScenarioInstanceVersion {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioInstanceVersion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ExampleScenarioInstanceVersion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identifier of a specific version of a resource
    pub version_id: types::String,
    /// Primitive extension sibling for [`version_id`](Self::version_id) (FHIR `_versionId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_versionId")]
    pub version_id_ext: Option<types::Element>,

    /// The description of the resource version
    pub description: types::Markdown,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Each major process - a group of operations.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioProcess;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct ExampleScenarioProcess {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The diagram title of the group of operations
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// A longer description of the group of operations
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Description of initial status before the process starts
    pub pre_conditions: Option<types::Markdown>,
    /// Primitive extension sibling for [`pre_conditions`](Self::pre_conditions) (FHIR `_preConditions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preConditions")]
    pub pre_conditions_ext: Option<types::Element>,

    /// Description of final status after the process ends
    pub post_conditions: Option<types::Markdown>,
    /// Primitive extension sibling for [`post_conditions`](Self::post_conditions) (FHIR `_postConditions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_postConditions")]
    pub post_conditions_ext: Option<types::Element>,

    /// Each step of the process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<ExampleScenarioProcessStep>,
}

/// Each step of the process.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioProcessStep;
/// use fhir::r4b::types;
///
/// let value = ExampleScenarioProcessStep {
///     pause: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `pause` is the name this serializes to on the wire.
/// assert_eq!(json["pause"], ::serde_json::json!(true));
///
/// let back: ExampleScenarioProcessStep = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ExampleScenarioProcessStep {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Nested process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process: Vec<ExampleScenarioProcess>,

    /// If there is a pause in the flow
    pub pause: Option<types::Boolean>,
    /// Primitive extension sibling for [`pause`](Self::pause) (FHIR `_pause`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_pause")]
    pub pause_ext: Option<types::Element>,

    /// Each interaction or action
    pub operation: Option<ExampleScenarioProcessStepOperation>,

    /// Alternate non-typical step action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternative: Vec<ExampleScenarioProcessStepAlternative>,
}

/// Indicates an alternative step that can be taken instead of the operations
/// on the base step in exceptional/atypical circumstances.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioProcessStepAlternative;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

    /// A human-readable description of each option
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// What happens in each alternative option
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<ExampleScenarioProcessStep>,
}

/// Each interaction or action.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::example_scenario::ExampleScenarioProcessStepOperation;
/// use fhir::r4b::types;
///
/// let value = ExampleScenarioProcessStepOperation {
///     r#type: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("abc"));
///
/// let back: ExampleScenarioProcessStepOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ExampleScenarioProcessStepOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The sequential number of the interaction
    pub number: types::String,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// The type of operation - CRUD
    pub r#type: Option<types::String>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The human-friendly name of the interaction
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Who starts the transaction
    pub initiator: Option<types::String>,
    /// Primitive extension sibling for [`initiator`](Self::initiator) (FHIR `_initiator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_initiator")]
    pub initiator_ext: Option<types::Element>,

    /// Who receives the transaction
    pub receiver: Option<types::String>,
    /// Primitive extension sibling for [`receiver`](Self::receiver) (FHIR `_receiver`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_receiver")]
    pub receiver_ext: Option<types::Element>,

    /// A comment to be inserted in the diagram
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Whether the initiator is deactivated right after the transaction
    pub initiator_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`initiator_active`](Self::initiator_active) (FHIR `_initiatorActive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_initiatorActive")]
    pub initiator_active_ext: Option<types::Element>,

    /// Whether the receiver is deactivated right after the transaction
    pub receiver_active: Option<types::Boolean>,
    /// Primitive extension sibling for [`receiver_active`](Self::receiver_active) (FHIR `_receiverActive`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_receiverActive")]
    pub receiver_active_ext: Option<types::Element>,

    /// Each resource instance used by the initiator
    pub request: Option<ExampleScenarioInstanceContainedInstance>,

    /// Each resource instance used by the responder
    pub response: Option<ExampleScenarioInstanceContainedInstance>,
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
