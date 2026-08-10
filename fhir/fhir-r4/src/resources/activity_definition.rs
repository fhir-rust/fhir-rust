//! ActivityDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
//!
//! Version: 4.0.1
//!
//! The definition of a specific activity to be taken, independent of any
//! particular patient or context
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance
/// context.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::activity_definition::ActivityDefinition;
/// use fhir::r4::types;
///
/// let value = ActivityDefinition {
///     approval_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `approvalDate` is the name this serializes to on the wire.
/// assert_eq!(json["approvalDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: ActivityDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ActivityDefinitionDe")]
#[fhir_version("r4")]
pub struct ActivityDefinition {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this activity definition, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the activity definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the activity definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this activity definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this activity definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the activity definition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
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

    /// Type of individual the activity definition is intended for
    /// The `ActivityDefinition.subject[x]` choice element (0..1); see [`ActivityDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<ActivityDefinitionSubject>,

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

    /// Natural language description of the activity definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for activity definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this activity definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the activity definition
    pub usage: Option<types::String>,
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

    /// When the activity definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the activity definition was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the activity definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc.
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

    /// Additional documentation, citations, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Logic used by the activity definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub library: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// Kind of resource
    pub kind: Option<crate::coded::Coded<crate::r4::codes::RequestResourceTypes>>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// What profile the resource needs to conform to
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,

    /// Detail type of activity
    pub code: Option<types::CodeableConcept>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: Option<crate::coded::Coded<crate::r4::codes::RequestIntent>>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r4::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if the activity should not be performed
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// When activity is to occur
    /// The `ActivityDefinition.timing[x]` choice element (0..1); see [`ActivityDefinitionTiming`].
    #[serde(flatten)]
    pub timing: Option<ActivityDefinitionTiming>,

    /// Where it should happen
    pub location: Option<types::Reference<crate::r4::resources::Location>>,

    /// Who should participate in the action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<ActivityDefinitionParticipant>,

    /// What's administered/supplied
    /// The `ActivityDefinition.product[x]` choice element (0..1); see [`ActivityDefinitionProduct`].
    #[serde(flatten)]
    pub product: Option<ActivityDefinitionProduct>,

    /// How much is administered/consumed/supplied
    pub quantity: Option<types::Quantity>,

    /// Detailed dosage instructions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage: Vec<types::Dosage>,

    /// What part of body to perform on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// What specimens are required to perform this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen_requirement: Vec<types::Reference<crate::r4::resources::SpecimenDefinition>>,

    /// What observations are required to perform this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_requirement: Vec<types::Reference<crate::r4::resources::ObservationDefinition>>,

    /// What observations must be produced by this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_result_requirement:
        Vec<types::Reference<crate::r4::resources::ObservationDefinition>>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

    /// Dynamic aspects of the definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic_value: Vec<ActivityDefinitionDynamicValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivityDefinitionDe {
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
    contained: Vec<crate::r4::resources::Resource>,
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
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    subtitle: Option<types::String>,
    #[serde(rename = "_subtitle")]
    subtitle_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    #[serde(flatten)]
    subject: crate::r4::choice::Slot<ActivityDefinitionSubject>,
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
    usage: Option<types::String>,
    #[serde(rename = "_usage")]
    usage_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
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
    kind: Option<crate::coded::Coded<crate::r4::codes::RequestResourceTypes>>,
    #[serde(rename = "_kind")]
    kind_ext: Option<types::Element>,
    profile: Option<types::Canonical>,
    #[serde(rename = "_profile")]
    profile_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    intent: Option<crate::coded::Coded<crate::r4::codes::RequestIntent>>,
    #[serde(rename = "_intent")]
    intent_ext: Option<types::Element>,
    priority: Option<crate::coded::Coded<crate::r4::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    do_not_perform: Option<types::Boolean>,
    #[serde(rename = "_doNotPerform")]
    do_not_perform_ext: Option<types::Element>,
    #[serde(flatten)]
    timing: crate::r4::choice::Slot<ActivityDefinitionTiming>,
    location: Option<types::Reference<crate::r4::resources::Location>>,
    #[serde(default)]
    participant: Vec<ActivityDefinitionParticipant>,
    #[serde(flatten)]
    product: crate::r4::choice::Slot<ActivityDefinitionProduct>,
    quantity: Option<types::Quantity>,
    #[serde(default)]
    dosage: Vec<types::Dosage>,
    #[serde(default)]
    body_site: Vec<types::CodeableConcept>,
    #[serde(default)]
    specimen_requirement: Vec<types::Reference<crate::r4::resources::SpecimenDefinition>>,
    #[serde(default)]
    observation_requirement: Vec<types::Reference<crate::r4::resources::ObservationDefinition>>,
    #[serde(default)]
    observation_result_requirement:
        Vec<types::Reference<crate::r4::resources::ObservationDefinition>>,
    transform: Option<types::Canonical>,
    #[serde(rename = "_transform")]
    transform_ext: Option<types::Element>,
    #[serde(default)]
    dynamic_value: Vec<ActivityDefinitionDynamicValue>,
}

impl ::core::convert::From<ActivityDefinitionDe> for ActivityDefinition {
    fn from(v: ActivityDefinitionDe) -> Self {
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
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            subtitle: v.subtitle,
            subtitle_ext: v.subtitle_ext,
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
            kind: v.kind,
            kind_ext: v.kind_ext,
            profile: v.profile,
            profile_ext: v.profile_ext,
            code: v.code,
            intent: v.intent,
            intent_ext: v.intent_ext,
            priority: v.priority,
            priority_ext: v.priority_ext,
            do_not_perform: v.do_not_perform,
            do_not_perform_ext: v.do_not_perform_ext,
            timing: v.timing.0,
            location: v.location,
            participant: v.participant,
            product: v.product.0,
            quantity: v.quantity,
            dosage: v.dosage,
            body_site: v.body_site,
            specimen_requirement: v.specimen_requirement,
            observation_requirement: v.observation_requirement,
            observation_result_requirement: v.observation_result_requirement,
            transform: v.transform,
            transform_ext: v.transform_ext,
            dynamic_value: v.dynamic_value,
        }
    }
}

/// Dynamic values that will be evaluated to produce values for elements of the
/// resulting resource. For example, if the dosage of a medication must be
/// computed based on the patient's weight, a dynamic value would be used to
/// specify an expression that calculated the weight, and the path on the
/// request resource that would contain the result.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::activity_definition::ActivityDefinitionDynamicValue;
/// use fhir::r4::types;
///
/// let value = ActivityDefinitionDynamicValue {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ActivityDefinitionDynamicValue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ActivityDefinitionDynamicValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The path to the element to be set dynamically
    pub path: types::String,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// An expression that provides the dynamic value for the customization
    pub expression: types::Expression,
}

/// Indicates who should participate in performing the action described.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::activity_definition::ActivityDefinitionParticipant;
/// use fhir::r4::types;
///
/// let value = ActivityDefinitionParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ActivityDefinitionParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ActivityDefinitionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// patient | practitioner | related-person | device
    pub r#type: crate::coded::Coded<crate::r4::codes::ActionParticipantType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// E.g. Nurse, Surgeon, Parent, etc.
    pub role: Option<types::CodeableConcept>,
}

/// The `ActivityDefinition.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `ActivityDefinition.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionTiming {
    /// `timingTiming` variant.
    #[fhir("timingTiming")]
    Timing(Box<types::Timing>),
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
    /// `timingAge` variant.
    #[fhir("timingAge")]
    Age(Box<types::Age>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
    /// `timingRange` variant.
    #[fhir("timingRange")]
    Range(Box<types::Range>),
    /// `timingDuration` variant.
    #[fhir("timingDuration")]
    Duration(Box<types::Duration>),
}

/// The `ActivityDefinition.product[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionProduct {
    /// `productReference` variant.
    #[fhir("productReference")]
    Reference(Box<types::Reference>),
    /// `productCodeableConcept` variant.
    #[fhir("productCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ActivityDefinition;

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
