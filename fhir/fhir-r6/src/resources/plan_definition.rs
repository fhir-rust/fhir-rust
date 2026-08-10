//! PlanDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PlanDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! The definition of a plan for a series of actions, independent of any
//! specific patient or context
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource allows for the definition of various types of plans as a
/// sharable, consumable, and executable artifact. The resource is general
/// enough to support the description of a broad range of clinical and
/// non-clinical artifacts such as clinical decision support rules, order sets,
/// protocols, and drug quality specifications.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinition;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "PlanDefinitionDe")]
#[fhir_version("r6")]
pub struct PlanDefinition {
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

    /// Canonical identifier for this plan definition, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the plan definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the plan definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `PlanDefinition.versionAlgorithm[x]` choice element (0..1); see [`PlanDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<PlanDefinitionVersionAlgorithm>,

    /// Name for this plan definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this plan definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the plan definition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// order-set | clinical-protocol | eca-rule | workflow-definition
    pub r#type: Option<types::CodeableConcept>,

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

    /// Type of individual the plan definition is focused on
    /// The `PlanDefinition.subject[x]` choice element (0..1); see [`PlanDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<PlanDefinitionSubject>,

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

    /// Natural language description of the plan definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
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
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the plan
    pub usage: Option<types::Markdown>,
    /// Primitive extension sibling for [`usage`](Self::usage) (FHIR `_usage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_usage")]
    pub usage_ext: Option<types::Element>,

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

    /// When the plan definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the plan definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
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
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub library: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// What the plan is trying to accomplish
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<PlanDefinitionGoal>,

    /// Actors within the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<PlanDefinitionActor>,

    /// Action defined by the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<PlanDefinitionAction>,

    /// Preconditions for service
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
    version_algorithm: crate::r6::choice::Slot<PlanDefinitionVersionAlgorithm>,
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
    status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    #[serde(flatten)]
    subject: crate::r6::choice::Slot<PlanDefinitionSubject>,
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
    library: ::fhir_core::PrimVec<types::Canonical>,
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
    as_needed: crate::r6::choice::Slot<PlanDefinitionAsNeeded>,
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

/// An action or group of actions to be taken as part of the plan. For example,
/// in clinical care, an action would be to prescribe a particular indicated
/// medication, or perform a particular test as appropriate. In pharmaceutical
/// quality, an action would be the test that needs to be performed on a drug
/// product as defined in the quality specification.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionAction;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prefix")]
    pub prefix_ext: Option<types::Element>,

    /// User-visible title
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Brief description of the action
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Static text equivalent of the action, used if the dynamic aspects
    /// cannot be interpreted by the receiving system
    pub text_equivalent: Option<types::Markdown>,
    /// Primitive extension sibling for [`text_equivalent`](Self::text_equivalent) (FHIR `_textEquivalent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_textEquivalent")]
    pub text_equivalent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
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
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub goal_id: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`goal_id`](Self::goal_id) (FHIR `_goalId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_goalId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal_id_ext: Vec<Option<types::Element>>,

    /// Type of individual the action is focused on
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

    /// When the action should take place
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
    pub grouping_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionGroupingBehavior>>,
    /// Primitive extension sibling for [`grouping_behavior`](Self::grouping_behavior) (FHIR `_groupingBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupingBehavior")]
    pub grouping_behavior_ext: Option<types::Element>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    pub selection_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionSelectionBehavior>>,
    /// Primitive extension sibling for [`selection_behavior`](Self::selection_behavior) (FHIR `_selectionBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_selectionBehavior")]
    pub selection_behavior_ext: Option<types::Element>,

    /// must | could | must-unless-documented
    pub required_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionRequiredBehavior>>,
    /// Primitive extension sibling for [`required_behavior`](Self::required_behavior) (FHIR `_requiredBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requiredBehavior")]
    pub required_behavior_ext: Option<types::Element>,

    /// yes | no
    pub precheck_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionPrecheckBehavior>>,
    /// Primitive extension sibling for [`precheck_behavior`](Self::precheck_behavior) (FHIR `_precheckBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precheckBehavior")]
    pub precheck_behavior_ext: Option<types::Element>,

    /// single | multiple
    pub cardinality_behavior:
        Option<crate::coded::Coded<crate::r6::codes::ActionCardinalityBehavior>>,
    /// Primitive extension sibling for [`cardinality_behavior`](Self::cardinality_behavior) (FHIR `_cardinalityBehavior`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cardinalityBehavior")]
    pub cardinality_behavior_ext: Option<types::Element>,

    /// Description of the activity to be performed
    /// The `PlanDefinition.action.definition[x]` choice element (0..1); see [`PlanDefinitionActionDefinition`].
    #[serde(flatten)]
    pub definition: Option<PlanDefinitionActionDefinition>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`):
    /// carries `id` and/or `extension` for the primitive value.
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
    priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    #[serde(default)]
    reason: Vec<types::CodeableConcept>,
    #[serde(default)]
    documentation: Vec<types::RelatedArtifact>,
    #[serde(default)]
    goal_id: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_goalId")]
    #[serde(default)]
    goal_id_ext: Vec<Option<types::Element>>,
    #[serde(flatten)]
    subject: crate::r6::choice::Slot<PlanDefinitionActionSubject>,
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
    timing: crate::r6::choice::Slot<PlanDefinitionActionTiming>,
    location: Option<types::CodeableReference>,
    #[serde(default)]
    participant: Vec<PlanDefinitionActionParticipant>,
    r#type: Option<types::CodeableConcept>,
    grouping_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionGroupingBehavior>>,
    #[serde(rename = "_groupingBehavior")]
    grouping_behavior_ext: Option<types::Element>,
    selection_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionSelectionBehavior>>,
    #[serde(rename = "_selectionBehavior")]
    selection_behavior_ext: Option<types::Element>,
    required_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionRequiredBehavior>>,
    #[serde(rename = "_requiredBehavior")]
    required_behavior_ext: Option<types::Element>,
    precheck_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionPrecheckBehavior>>,
    #[serde(rename = "_precheckBehavior")]
    precheck_behavior_ext: Option<types::Element>,
    cardinality_behavior: Option<crate::coded::Coded<crate::r6::codes::ActionCardinalityBehavior>>,
    #[serde(rename = "_cardinalityBehavior")]
    cardinality_behavior_ext: Option<types::Element>,
    #[serde(flatten)]
    definition: crate::r6::choice::Slot<PlanDefinitionActionDefinition>,
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

/// An expression that describes applicability criteria or start/stop
/// conditions for the action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionActionCondition;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub kind: crate::coded::Coded<crate::r6::codes::ActionConditionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Boolean-valued expression
    pub expression: Option<types::Expression>,
}

/// Customizations that should be applied to the statically defined resource.
/// For example, if the dosage of a medication must be computed based on the
/// patient's weight, a customization would be used to specify an expression
/// that calculated the weight, and the path on the resource that would contain
/// the result.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionActionDynamicValue;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// An expression that provides the dynamic value for the customization
    pub expression: Option<types::Expression>,
}

/// Defines input data requirements for the action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionActionInput;
/// use fhir::r6::types;
///
/// let value = PlanDefinitionActionInput {
///     related_data: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `relatedData` is the name this serializes to on the wire.
/// assert_eq!(json["relatedData"], ::serde_json::json!("abc"));
///
/// let back: PlanDefinitionActionInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::String>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// Defines the outputs of the action, if any.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionActionOutput;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What data is provided
    pub requirement: Option<types::DataRequirement>,

    /// What data is provided
    pub related_data: Option<types::String>,
    /// Primitive extension sibling for [`related_data`](Self::related_data) (FHIR `_relatedData`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relatedData")]
    pub related_data_ext: Option<types::Element>,
}

/// Indicates who should participate in performing the action described.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionActionParticipant;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`actor_id`](Self::actor_id) (FHIR `_actorId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actorId")]
    pub actor_id_ext: Option<types::Element>,

    /// careteam | device | group | healthcareservice | location | organization
    /// | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::ActionParticipantType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,
}

/// A relationship to another action such as "before" or "30-60 minutes after
/// start of".
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionActionRelatedAction;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub target_id: types::String,
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start
    /// | concurrent-with-end | after | after-start | after-end
    pub relationship: crate::coded::Coded<crate::r6::codes::ActionRelationshipType>,
    /// Primitive extension sibling for [`relationship`](Self::relationship) (FHIR `_relationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relationship")]
    pub relationship_ext: Option<types::Element>,

    /// before | before-start | before-end | concurrent | concurrent-with-start
    /// | concurrent-with-end | after | after-start | after-end
    pub end_relationship: Option<crate::coded::Coded<crate::r6::codes::ActionRelationshipType>>,
    /// Primitive extension sibling for [`end_relationship`](Self::end_relationship) (FHIR `_endRelationship`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_endRelationship")]
    pub end_relationship_ext: Option<types::Element>,

    /// Time offset for the relationship
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
    target_id: types::String,
    #[serde(rename = "_targetId")]
    target_id_ext: Option<types::Element>,
    relationship: crate::coded::Coded<crate::r6::codes::ActionRelationshipType>,
    #[serde(rename = "_relationship")]
    relationship_ext: Option<types::Element>,
    end_relationship: Option<crate::coded::Coded<crate::r6::codes::ActionRelationshipType>>,
    #[serde(rename = "_endRelationship")]
    end_relationship_ext: Option<types::Element>,
    #[serde(flatten)]
    offset: crate::r6::choice::Slot<PlanDefinitionActionRelatedActionOffset>,
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

/// Actors represent the individuals or groups involved in the execution of the
/// defined set of activities.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::plan_definition::PlanDefinitionActor;
///
/// let value = PlanDefinitionActor::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PlanDefinitionActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Describes the actor
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who or what can be this actor
    pub option: ::vec1::Vec1<PlanDefinitionActorOption>,
}

/// The characteristics of the candidates that could serve as the actor.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionActorOption;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct PlanDefinitionActorOption {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// careteam | device | group | healthcareservice | location | organization
    /// | patient | practitioner | practitionerrole | relatedperson
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::ActionParticipantType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`type_canonical`](Self::type_canonical) (FHIR `_typeCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_typeCanonical")]
    pub type_canonical_ext: Option<types::Element>,

    /// Who or what can participate
    pub type_reference: Option<types::Reference>,

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<types::CodeableConcept>,
}

/// A goal describes an expected outcome that activities within the plan are
/// intended to achieve. For example, weight loss, restoring an activity of
/// daily living, obtaining herd immunity via immunization, meeting a process
/// improvement objective, meeting the acceptance criteria for a test as
/// specified by a quality specification, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionGoal;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Indicates what should be done and within what timeframe.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::plan_definition::PlanDefinitionGoalTarget;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// The target value to be achieved
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
    detail: crate::r6::choice::Slot<PlanDefinitionGoalTargetDetail>,
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

/// The `PlanDefinition.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `PlanDefinition.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
}

/// The `PlanDefinition.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `PlanDefinition.action.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
}

/// The `PlanDefinition.action.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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

/// The `PlanDefinition.action.definition[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionDefinition {
    /// `definitionCanonical` variant.
    #[fhir("definitionCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `definitionUri` variant.
    #[fhir("definitionUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

/// The `PlanDefinition.action.relatedAction.offset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PlanDefinitionActionRelatedActionOffset {
    /// `offsetDuration` variant.
    #[fhir("offsetDuration")]
    Duration(Box<types::Duration>),
    /// `offsetRange` variant.
    #[fhir("offsetRange")]
    Range(Box<types::Range>),
}

/// The `PlanDefinition.goal.target.detail[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    String(crate::r6::choice::Primitive<types::String>),
    /// `detailBoolean` variant.
    #[fhir("detailBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `detailInteger` variant.
    #[fhir("detailInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `detailRatio` variant.
    #[fhir("detailRatio")]
    Ratio(Box<types::Ratio>),
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
