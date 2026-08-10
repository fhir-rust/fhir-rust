//! PlanDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PlanDefinition
//!
//! Version: 5.0.0
//!
//! PlanDefinition Resource: This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// PlanDefinition Resource.
///
/// This resource allows for the definition of various types of plans as a
/// sharable, consumable, and executable artifact. The resource is general
/// enough to support the description of a broad range of clinical and
/// non-clinical artifacts such as clinical decision support rules, order sets,
/// protocols, and drug quality specifications.
///
/// In FHIR R5 a PlanDefinition is a definitional resource: it describes the
/// intended structure of care or work rather than any specific instance of it
/// having occurred for a particular subject. Authors compose a plan from a
/// hierarchy of actions, each of which can carry conditions, timing,
/// participants, inputs and outputs, and relationships to other actions,
/// together with goals the plan is meant to achieve. Because the definition is
/// computable, decision support engines and workflow systems can apply a
/// PlanDefinition to a patient's context to generate concrete request
/// resources, such as a CarePlan, RequestOrchestration, or Task, and to drive
/// event-condition-action rules. This makes PlanDefinition a foundational
/// building block for knowledge artifacts that are published, versioned, and
/// shared across organizations.
///
/// Related resources: an ActivityDefinition supplies the reusable detail for
/// individual actions, a Library carries the logic and terminology the plan
/// relies on, and applying a plan commonly targets a subject such as a
/// [`Patient`](crate::r5::resources::patient::Patient). Many fields are typed
/// with shared datatypes such as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinition;
/// use fhir::r5::types;
///
/// let value = PlanDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "PlanDefinitionDe")]
pub struct PlanDefinition {
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

    /// Canonical identifier for this plan definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the plan definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the plan definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `PlanDefinition.versionAlgorithm[x]` choice element (0..1); see [`PlanDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<PlanDefinitionVersionAlgorithm>,

    /// Name for this plan definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this plan definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the plan definition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`).
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// High-level kind of artifact this plan represents, such as order-set, clinical-protocol, eca-rule, or workflow-definition.
    pub r#type: Option<types::CodeableConcept>,

    /// Publication lifecycle state of the plan definition: draft, active, retired, or unknown; this field is required.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// The `PlanDefinition.subject[x]` choice element (0..1); see [`PlanDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<PlanDefinitionSubject>,

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

    /// Natural language description of the plan definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for plan definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this plan definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the plan
    pub usage: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`).
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

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

    /// When the plan definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the plan definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the plan definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic: Vec<types::CodeableConcept>,

    /// Who authored the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::ContactDetail>,

    /// Who edited the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub editor: Vec<types::ContactDetail>,

    /// Who reviewed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer: Vec<types::ContactDetail>,

    /// Who endorsed the content
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endorser: Vec<types::ContactDetail>,

    /// Additional documentation, citations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Logic used by the plan definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`).
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// The clinical or business goals the plan is intended to accomplish, against which its actions can be measured.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<PlanDefinitionGoal>,

    /// The actors, such as roles or participant types, that take part in carrying out the plan.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<PlanDefinitionActor>,

    /// The ordered, possibly nested actions that make up the plan and define what should be done and when.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<PlanDefinitionAction>,

    /// The `PlanDefinition.asNeeded[x]` choice element (0..1); see [`PlanDefinitionAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<PlanDefinitionAsNeeded>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlanDefinitionDe {
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
    contained: Vec<crate::r5::resources::Resource>,
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
    version_algorithm: crate::r5::choice::Slot<PlanDefinitionVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    subtitle: Option<types::String>,
    #[serde(rename = "_subtitle")]
    subtitle_ext: Option<types::Element>,
    r#type: Option<types::CodeableConcept>,
    status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    #[serde(flatten)]
    subject: crate::r5::choice::Slot<PlanDefinitionSubject>,
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
    usage: Option<types::Markdown>,
    #[serde(rename = "_usage")]
    usage_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    approval_date: Option<types::Date>,
    #[serde(rename = "_approvalDate")]
    approval_date_ext: Option<types::Element>,
    last_review_date: Option<types::Date>,
    #[serde(rename = "_lastReviewDate")]
    last_review_date_ext: Option<types::Element>,
    effective_period: Option<types::Period>,
    #[serde(default)]
    topic: Vec<types::CodeableConcept>,
    #[serde(default)]
    author: Vec<types::ContactDetail>,
    #[serde(default)]
    editor: Vec<types::ContactDetail>,
    #[serde(default)]
    reviewer: Vec<types::ContactDetail>,
    #[serde(default)]
    endorser: Vec<types::ContactDetail>,
    #[serde(default)]
    related_artifact: Vec<types::RelatedArtifact>,
    #[serde(default)]
    library: Vec<types::Canonical>,
    #[serde(rename = "_library")]
    #[serde(default)]
    library_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    goal: Vec<PlanDefinitionGoal>,
    #[serde(default)]
    actor: Vec<PlanDefinitionActor>,
    #[serde(default)]
    action: Vec<PlanDefinitionAction>,
    #[serde(flatten)]
    as_needed: crate::r5::choice::Slot<PlanDefinitionAsNeeded>,
}

impl ::core::convert::From<PlanDefinitionDe> for PlanDefinition {
    fn from(v: PlanDefinitionDe) -> Self {
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
            subtitle: v.subtitle,
            subtitle_ext: v.subtitle_ext,
            r#type: v.r#type,
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            subject: v.subject.0,
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
            usage: v.usage,
            usage_ext: v.usage_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            approval_date: v.approval_date,
            approval_date_ext: v.approval_date_ext,
            last_review_date: v.last_review_date,
            last_review_date_ext: v.last_review_date_ext,
            effective_period: v.effective_period,
            topic: v.topic,
            author: v.author,
            editor: v.editor,
            reviewer: v.reviewer,
            endorser: v.endorser,
            related_artifact: v.related_artifact,
            library: v.library,
            library_ext: v.library_ext,
            goal: v.goal,
            actor: v.actor,
            action: v.action,
            as_needed: v.as_needed.0,
        }
    }
}

/// What the plan is trying to accomplish.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionGoal;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionGoal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionGoal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionGoal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// E.g. Treatment, dietary, behavioral
    pub category: Option<types::CodeableConcept>,

    /// Code or text describing the goal
    pub description: types::CodeableConcept,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<types::CodeableConcept>,

    /// When goal pursuit begins
    pub start: Option<types::CodeableConcept>,

    /// What does the goal address
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::CodeableConcept>,

    /// Supporting documentation for the goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documentation: Vec<types::RelatedArtifact>,

    /// Target outcome for the goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<PlanDefinitionGoalTarget>,
}

/// Target outcome for the goal.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionGoalTarget;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionGoalTarget {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionGoalTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "PlanDefinitionGoalTargetDe")]
pub struct PlanDefinitionGoalTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The parameter whose value is to be tracked
    pub measure: Option<types::CodeableConcept>,

    /// The `PlanDefinition.goal.target.detail[x]` choice element (0..1); see [`PlanDefinitionGoalTargetDetail`].
    #[serde(flatten)]
    pub detail: Option<PlanDefinitionGoalTargetDetail>,

    /// Reach goal within
    pub due: Option<types::Duration>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlanDefinitionGoalTargetDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    measure: Option<types::CodeableConcept>,
    #[serde(flatten)]
    detail: crate::r5::choice::Slot<PlanDefinitionGoalTargetDetail>,
    due: Option<types::Duration>,
}

impl ::core::convert::From<PlanDefinitionGoalTargetDe> for PlanDefinitionGoalTarget {
    fn from(v: PlanDefinitionGoalTargetDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            measure: v.measure,
            detail: v.detail.0,
            due: v.due,
        }
    }
}

/// Actors within the plan.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::plan_definition::PlanDefinitionActor;
///
/// let value = PlanDefinitionActor::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PlanDefinitionActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Describes the actor
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who or what can be this actor
    pub option: vec1::Vec1<PlanDefinitionActorOption>,
}

/// Who or what can be this actor.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActorOption;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActorOption {
///     type_canonical: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `typeCanonical` is the name this serializes to on the wire.
/// assert_eq!(json["typeCanonical"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: PlanDefinitionActorOption = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActorOption {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::ActionParticipantType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`).
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,
}

/// Action defined by the plan.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionAction;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionAction {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "PlanDefinitionActionDe")]
pub struct PlanDefinitionAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique id for the action in the PlanDefinition
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`).
    #[serde(rename = "_prefix")]
    pub prefix_ext: Option<types::Element>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Brief description of the action
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::Markdown>,
    /// Primitive extension sibling for [`text_equivalent`](Self::text_equivalent) (FHIR `_textEquivalent`).
    #[serde(rename = "_textEquivalent")]
    pub text_equivalent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Code representing the meaning of the action or sub-actions
    pub code: Option<types::CodeableConcept>,

    /// Why the action should be performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Supporting documentation for the intended performer of the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub documentation: Vec<types::RelatedArtifact>,

    /// What goals this action supports
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal_id: Vec<types::Id>,
    /// Primitive extension sibling for [`goal_id`](Self::goal_id) (FHIR `_goalId`).
    #[serde(rename = "_goalId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal_id_ext: Vec<Option<types::Element>>,

    /// The `PlanDefinition.action.subject[x]` choice element (0..1); see [`PlanDefinitionActionSubject`].
    #[serde(flatten)]
    pub subject: Option<PlanDefinitionActionSubject>,

    /// When the action should be triggered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trigger: Vec<types::TriggerDefinition>,

    /// Whether or not the action is applicable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<PlanDefinitionActionCondition>,

    /// Input data requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<PlanDefinitionActionInput>,

    /// Output data definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<PlanDefinitionActionOutput>,

    /// Relationship to another action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_action: Vec<PlanDefinitionActionRelatedAction>,

    /// The `PlanDefinition.action.timing[x]` choice element (0..1); see [`PlanDefinitionActionTiming`].
    #[serde(flatten)]
    pub timing: Option<PlanDefinitionActionTiming>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

    /// Who should participate in the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<PlanDefinitionActionParticipant>,

    /// create | update | remove | fire-event
    pub r#type: Option<types::CodeableConcept>,

    /// visual-group | logical-group | sentence-group
    pub grouping_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionGroupingBehavior>>,
    /// Primitive extension sibling for [`grouping_behavior`](Self::grouping_behavior) (FHIR `_groupingBehavior`).
    #[serde(rename = "_groupingBehavior")]
    pub grouping_behavior_ext: Option<types::Element>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionSelectionBehavior>>,
    /// Primitive extension sibling for [`selection_behavior`](Self::selection_behavior) (FHIR `_selectionBehavior`).
    #[serde(rename = "_selectionBehavior")]
    pub selection_behavior_ext: Option<types::Element>,

    /// must | could | must-unless-documented
    pub required_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionRequiredBehavior>>,
    /// Primitive extension sibling for [`required_behavior`](Self::required_behavior) (FHIR `_requiredBehavior`).
    #[serde(rename = "_requiredBehavior")]
    pub required_behavior_ext: Option<types::Element>,

    /// yes | no
    pub precheck_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionPrecheckBehavior>>,
    /// Primitive extension sibling for [`precheck_behavior`](Self::precheck_behavior) (FHIR `_precheckBehavior`).
    #[serde(rename = "_precheckBehavior")]
    pub precheck_behavior_ext: Option<types::Element>,

    /// single | multiple
    pub cardinality_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionCardinalityBehavior>>,
    /// Primitive extension sibling for [`cardinality_behavior`](Self::cardinality_behavior) (FHIR `_cardinalityBehavior`).
    #[serde(rename = "_cardinalityBehavior")]
    pub cardinality_behavior_ext: Option<types::Element>,

    /// The `PlanDefinition.action.definition[x]` choice element (0..1); see [`PlanDefinitionActionDefinition`].
    #[serde(flatten)]
    pub definition: Option<PlanDefinitionActionDefinition>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`).
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

    /// Dynamic aspects of the definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic_value: Vec<PlanDefinitionActionDynamicValue>,

    /// A sub-action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<PlanDefinitionAction>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlanDefinitionActionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    link_id: Option<types::String>,
    #[serde(rename = "_linkId")]
    link_id_ext: Option<types::Element>,
    prefix: Option<types::String>,
    #[serde(rename = "_prefix")]
    prefix_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    text_equivalent: Option<types::Markdown>,
    #[serde(rename = "_textEquivalent")]
    text_equivalent_ext: Option<types::Element>,
    priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    #[serde(default)]
    reason: Vec<types::CodeableConcept>,
    #[serde(default)]
    documentation: Vec<types::RelatedArtifact>,
    #[serde(default)]
    goal_id: Vec<types::Id>,
    #[serde(rename = "_goalId")]
    #[serde(default)]
    goal_id_ext: Vec<Option<types::Element>>,
    #[serde(flatten)]
    subject: crate::r5::choice::Slot<PlanDefinitionActionSubject>,
    #[serde(default)]
    trigger: Vec<types::TriggerDefinition>,
    #[serde(default)]
    condition: Vec<PlanDefinitionActionCondition>,
    #[serde(default)]
    input: Vec<PlanDefinitionActionInput>,
    #[serde(default)]
    output: Vec<PlanDefinitionActionOutput>,
    #[serde(default)]
    related_action: Vec<PlanDefinitionActionRelatedAction>,
    #[serde(flatten)]
    timing: crate::r5::choice::Slot<PlanDefinitionActionTiming>,
    location: Option<types::CodeableReference>,
    #[serde(default)]
    participant: Vec<PlanDefinitionActionParticipant>,
    r#type: Option<types::CodeableConcept>,
    grouping_behavior: Option<crate::r5::coded::Coded<crate::r5::codes::ActionGroupingBehavior>>,
    #[serde(rename = "_groupingBehavior")]
    grouping_behavior_ext: Option<types::Element>,
    selection_behavior: Option<crate::r5::coded::Coded<crate::r5::codes::ActionSelectionBehavior>>,
    #[serde(rename = "_selectionBehavior")]
    selection_behavior_ext: Option<types::Element>,
    required_behavior: Option<crate::r5::coded::Coded<crate::r5::codes::ActionRequiredBehavior>>,
    #[serde(rename = "_requiredBehavior")]
    required_behavior_ext: Option<types::Element>,
    precheck_behavior: Option<crate::r5::coded::Coded<crate::r5::codes::ActionPrecheckBehavior>>,
    #[serde(rename = "_precheckBehavior")]
    precheck_behavior_ext: Option<types::Element>,
    cardinality_behavior:
        Option<crate::r5::coded::Coded<crate::r5::codes::ActionCardinalityBehavior>>,
    #[serde(rename = "_cardinalityBehavior")]
    cardinality_behavior_ext: Option<types::Element>,
    #[serde(flatten)]
    definition: crate::r5::choice::Slot<PlanDefinitionActionDefinition>,
    transform: Option<types::Canonical>,
    #[serde(rename = "_transform")]
    transform_ext: Option<types::Element>,
    #[serde(default)]
    dynamic_value: Vec<PlanDefinitionActionDynamicValue>,
    #[serde(default)]
    action: Vec<PlanDefinitionAction>,
}

impl ::core::convert::From<PlanDefinitionActionDe> for PlanDefinitionAction {
    fn from(v: PlanDefinitionActionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            link_id: v.link_id,
            link_id_ext: v.link_id_ext,
            prefix: v.prefix,
            prefix_ext: v.prefix_ext,
            title: v.title,
            title_ext: v.title_ext,
            description: v.description,
            description_ext: v.description_ext,
            text_equivalent: v.text_equivalent,
            text_equivalent_ext: v.text_equivalent_ext,
            priority: v.priority,
            priority_ext: v.priority_ext,
            code: v.code,
            reason: v.reason,
            documentation: v.documentation,
            goal_id: v.goal_id,
            goal_id_ext: v.goal_id_ext,
            subject: v.subject.0,
            trigger: v.trigger,
            condition: v.condition,
            input: v.input,
            output: v.output,
            related_action: v.related_action,
            timing: v.timing.0,
            location: v.location,
            participant: v.participant,
            r#type: v.r#type,
            grouping_behavior: v.grouping_behavior,
            grouping_behavior_ext: v.grouping_behavior_ext,
            selection_behavior: v.selection_behavior,
            selection_behavior_ext: v.selection_behavior_ext,
            required_behavior: v.required_behavior,
            required_behavior_ext: v.required_behavior_ext,
            precheck_behavior: v.precheck_behavior,
            precheck_behavior_ext: v.precheck_behavior_ext,
            cardinality_behavior: v.cardinality_behavior,
            cardinality_behavior_ext: v.cardinality_behavior_ext,
            definition: v.definition.0,
            transform: v.transform,
            transform_ext: v.transform_ext,
            dynamic_value: v.dynamic_value,
            action: v.action,
        }
    }
}

/// Whether or not the action is applicable.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionCondition;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionCondition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// applicability | start | stop
    pub kind: crate::r5::coded::Coded<crate::r5::codes::ActionConditionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Boolean-valued expression
    pub expression: Option<types::Expression>,
}

/// Input data requirements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionInput;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionInput {
///     related_data: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `relatedData` is the name this serializes to on the wire.
/// assert_eq!(json["relatedData"], ::serde_json::json!("pat-1"));
///
/// let back: PlanDefinitionActionInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::Id>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`).
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// Output data definition.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionOutput;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionOutput {
///     related_data: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `relatedData` is the name this serializes to on the wire.
/// assert_eq!(json["relatedData"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionOutput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::String>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`).
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// Relationship to another action.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionRelatedAction;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionRelatedAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionRelatedAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "PlanDefinitionActionRelatedActionDe")]
pub struct PlanDefinitionActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What action is this related to
    pub target_id: types::Id,
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`).
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub relationship: crate::r5::coded::Coded<crate::r5::codes::ActionRelationshipType>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`).
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub end_relationship: Option<crate::r5::coded::Coded<crate::r5::codes::ActionRelationshipType>>,
    /// Primitive extension sibling for [`end_relationship`](Self::end_relationship) (FHIR `_endRelationship`).
    #[serde(rename = "_endRelationship")]
    pub end_relationship_ext: Option<types::Element>,

    /// The `PlanDefinition.action.relatedAction.offset[x]` choice element (0..1); see [`PlanDefinitionActionRelatedActionOffset`].
    #[serde(flatten)]
    pub offset: Option<PlanDefinitionActionRelatedActionOffset>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlanDefinitionActionRelatedActionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    target_id: types::Id,
    #[serde(rename = "_targetId")]
    target_id_ext: Option<types::Element>,
    relationship: crate::r5::coded::Coded<crate::r5::codes::ActionRelationshipType>,
    #[serde(rename = "_relationship")]
    relationship_ext: Option<types::Element>,
    end_relationship: Option<crate::r5::coded::Coded<crate::r5::codes::ActionRelationshipType>>,
    #[serde(rename = "_endRelationship")]
    end_relationship_ext: Option<types::Element>,
    #[serde(flatten)]
    offset: crate::r5::choice::Slot<PlanDefinitionActionRelatedActionOffset>,
}

impl ::core::convert::From<PlanDefinitionActionRelatedActionDe>
    for PlanDefinitionActionRelatedAction
{
    fn from(v: PlanDefinitionActionRelatedActionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            target_id: v.target_id,
            target_id_ext: v.target_id_ext,
            relationship: v.relationship,
            relationship_ext: v.relationship_ext,
            end_relationship: v.end_relationship,
            end_relationship_ext: v.end_relationship_ext,
            offset: v.offset.0,
        }
    }
}

/// Who should participate in the action.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionParticipant;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionParticipant {
///     actor_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `actorId` is the name this serializes to on the wire.
/// assert_eq!(json["actorId"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What actor
    pub actor_id: Option<types::String>,
    /// Primitive extension sibling for [`actor_id`](Self::actor_id) (FHIR `_actorId`).
    #[serde(rename = "_actorId")]
    pub actor_id_ext: Option<types::Element>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::ActionParticipantType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`).
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,
}

/// Dynamic aspects of the definition.
/// # Examples
///
/// ```
/// use fhir::r5::resources::plan_definition::PlanDefinitionActionDynamicValue;
/// use fhir::r5::types;
///
/// let value = PlanDefinitionActionDynamicValue {
///     path: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `path` is the name this serializes to on the wire.
/// assert_eq!(json["path"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionDynamicValue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionDynamicValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The path to the element to be set dynamically
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// An expression that provides the dynamic value for the customization
    pub expression: Option<types::Expression>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PlanDefinition;

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
/// The `PlanDefinition.action.definition[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionDefinition {
    /// `definitionCanonical` variant.
    #[fhir("definitionCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
    /// `definitionUri` variant.
    #[fhir("definitionUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
}

/// The `PlanDefinition.action.relatedAction.offset[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionRelatedActionOffset {
    /// `offsetDuration` variant.
    #[fhir("offsetDuration")]
    Duration(Box<types::Duration>),
    /// `offsetRange` variant.
    #[fhir("offsetRange")]
    Range(Box<types::Range>),
}

/// The `PlanDefinition.action.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
    /// `subjectCanonical` variant.
    #[fhir("subjectCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `PlanDefinition.action.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionTiming {
    /// `timingAge` variant.
    #[fhir("timingAge")]
    Age(Box<types::Age>),
    /// `timingDuration` variant.
    #[fhir("timingDuration")]
    Duration(Box<types::Duration>),
    /// `timingRange` variant.
    #[fhir("timingRange")]
    Range(Box<types::Range>),
    /// `timingTiming` variant.
    #[fhir("timingTiming")]
    Timing(Box<types::Timing>),
}

/// The `PlanDefinition.asNeeded[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `PlanDefinition.goal.target.detail[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionGoalTargetDetail {
    /// `detailQuantity` variant.
    #[fhir("detailQuantity")]
    Quantity(Box<types::Quantity>),
    /// `detailRange` variant.
    #[fhir("detailRange")]
    Range(Box<types::Range>),
    /// `detailCodeableConcept` variant.
    #[fhir("detailCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `detailString` variant.
    #[fhir("detailString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `detailBoolean` variant.
    #[fhir("detailBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `detailInteger` variant.
    #[fhir("detailInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `detailRatio` variant.
    #[fhir("detailRatio")]
    Ratio(Box<types::Ratio>),
}

/// The `PlanDefinition.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
    /// `subjectCanonical` variant.
    #[fhir("subjectCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `PlanDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
