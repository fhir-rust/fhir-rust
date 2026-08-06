//! ActivityDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! The definition of a specific activity to be taken, independent of any
//! particular patient or context
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance
/// context.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::activity_definition::ActivityDefinition;
/// use fhir::r6::types;
///
/// let value = ActivityDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: ActivityDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

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

    /// How to compare versions
    /// The `ActivityDefinition.versionAlgorithm[x]` choice element (0..1); see [`ActivityDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ActivityDefinitionVersionAlgorithm>,

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

    /// Name of the publisher/steward (organization or individual)
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

    /// When the activity definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the activity definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the activity definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// E.g. Education, Treatment, Assessment, etc
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

    /// Additional documentation, citations, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_artifact: Vec<types::RelatedArtifact>,

    /// Logic used by the activity definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// Kind of resource
    pub kind: Option<types::Code>,
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
    pub intent: Option<crate::coded::Coded<crate::r6::codes::RequestIntent>>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
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

    /// Preconditions for service
    /// The `ActivityDefinition.asNeeded[x]` choice element (0..1); see [`ActivityDefinitionAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<ActivityDefinitionAsNeeded>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

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
    pub specimen_requirement: Vec<types::Canonical>,
    /// Primitive extension sibling for [`specimen_requirement`](Self::specimen_requirement) (FHIR `_specimenRequirement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_specimenRequirement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen_requirement_ext: Vec<Option<types::Element>>,

    /// What observations are required to perform this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_requirement: Vec<types::Canonical>,
    /// Primitive extension sibling for [`observation_requirement`](Self::observation_requirement) (FHIR `_observationRequirement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_observationRequirement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_requirement_ext: Vec<Option<types::Element>>,

    /// What observations must be produced by this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_result_requirement: Vec<types::Canonical>,
    /// Primitive extension sibling for [`observation_result_requirement`](Self::observation_result_requirement) (FHIR `_observationResultRequirement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_observationResultRequirement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_result_requirement_ext: Vec<Option<types::Element>>,

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

/// Dynamic values that will be evaluated to produce values for elements of the
/// resulting resource. For example, if the dosage of a medication must be
/// computed based on the patient's weight, a dynamic value would be used to
/// specify an expression that calculated the weight, and the path on the
/// request resource that would contain the result.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::activity_definition::ActivityDefinitionDynamicValue;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
/// use fhir::r6::resources::activity_definition::ActivityDefinitionParticipant;
/// use fhir::r6::types;
///
/// let value = ActivityDefinitionParticipant {
///     type_canonical: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `typeCanonical` is the name this serializes to on the wire.
/// assert_eq!(json["typeCanonical"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: ActivityDefinitionParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ActivityDefinitionParticipant {
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

    /// E.g. Nurse, Surgeon, Parent, etc
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,
}

/// The `ActivityDefinition.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `ActivityDefinition.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionSubject {
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

/// The `ActivityDefinition.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionTiming {
    /// `timingTiming` variant.
    #[fhir("timingTiming")]
    Timing(Box<types::Timing>),
    /// `timingAge` variant.
    #[fhir("timingAge")]
    Age(Box<types::Age>),
    /// `timingRange` variant.
    #[fhir("timingRange")]
    Range(Box<types::Range>),
    /// `timingDuration` variant.
    #[fhir("timingDuration")]
    Duration(Box<types::Duration>),
}

/// The `ActivityDefinition.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `ActivityDefinition.product[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
