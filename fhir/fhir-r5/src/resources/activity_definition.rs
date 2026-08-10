//! ActivityDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ActivityDefinition
//!
//! Version: 5.0.0
//!
//! ActivityDefinition Resource: This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance
/// context. An ActivityDefinition describes the shape of an activity that can
/// be instantiated (for example, to create an order or care plan activity), and
/// is commonly referenced from PlanDefinition or care planning workflows in
/// FHIR R5. It carries clinical detail such as timing, dosage, participants, and
/// dynamic values used to customize the instantiated activity.
///
/// ActivityDefinition is a knowledge-artifact resource: it is authored once,
/// versioned, and published (typically alongside a `Library` of decision-support
/// logic) so that it can be applied repeatedly at the point of care. Clinical
/// decision support and order-set tooling reference an ActivityDefinition by its
/// canonical `url` and "apply" it against a specific subject to produce a
/// concrete resource, such as a `MedicationRequest`, `ServiceRequest`, or
/// `Task`, tailored to that patient's context. The `dynamic_value` elements
/// allow expressions (for example, in FHIRPath or CQL) to customize fields of
/// the generated resource at apply time, and the `participant` elements
/// describe who is expected to be involved in carrying out the activity.
///
/// Related resources / See also:
/// - [`PlanDefinition`](crate::r5::resources::plan_definition::PlanDefinition) —
///   often groups one or more ActivityDefinitions into an ordered or
///   conditional plan of care.
/// - [`Patient`](crate::r5::resources::patient::Patient) — the typical subject
///   an ActivityDefinition is applied against, referenced via
///   `subject_reference` or described via `subject_codeable_concept`.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used throughout
///   this resource (e.g. `code`, `topic`, `body_site`) to represent coded
///   clinical concepts.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::activity_definition::ActivityDefinition;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDefinition {
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

    /// Canonical identifier for this activity definition, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the activity definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the activity definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `ActivityDefinition.versionAlgorithm[x]` choice element (0..1); see [`ActivityDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ActivityDefinitionVersionAlgorithm>,

    /// Name for this activity definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this activity definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the activity definition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`).
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

    /// Publication status of this definition: draft | active | retired | unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// The `ActivityDefinition.subject[x]` choice element (0..1); see [`ActivityDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<ActivityDefinitionSubject>,

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

    /// Natural language description of the activity definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the activity definition
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

    /// When the activity definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the activity definition was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
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
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`).
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// The FHIR resource type that applying this definition will produce, e.g. MedicationRequest or Task
    pub kind: Option<types::Code>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// What profile the resource needs to conform to
    pub profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    pub profile_ext: Option<types::Element>,

    /// Detail type of activity, e.g. a specific procedure, medication, or service code
    pub code: Option<types::CodeableConcept>,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: Option<crate::r5::coded::Coded<crate::r5::codes::RequestIntent>>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if the activity should not be performed
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`).
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// The `ActivityDefinition.timing[x]` choice element (0..1); see [`ActivityDefinitionTiming`].
    #[serde(flatten)]
    pub timing: Option<ActivityDefinitionTiming>,

    /// The `ActivityDefinition.asNeeded[x]` choice element (0..1); see [`ActivityDefinitionAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<ActivityDefinitionAsNeeded>,

    /// Where it should happen
    pub location: Option<types::CodeableReference>,

    /// Who should participate in performing the defined activity, e.g. practitioner, patient, or device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<ActivityDefinitionParticipant>,

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
    /// Primitive extension sibling for [`specimen_requirement`](Self::specimen_requirement) (FHIR `_specimenRequirement`).
    #[serde(rename = "_specimenRequirement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen_requirement_ext: Vec<Option<types::Element>>,

    /// What observations are required to perform this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_requirement: Vec<types::Canonical>,
    /// Primitive extension sibling for [`observation_requirement`](Self::observation_requirement) (FHIR `_observationRequirement`).
    #[serde(rename = "_observationRequirement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_requirement_ext: Vec<Option<types::Element>>,

    /// What observations must be produced by this action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_result_requirement: Vec<types::Canonical>,
    /// Primitive extension sibling for [`observation_result_requirement`](Self::observation_result_requirement) (FHIR `_observationResultRequirement`).
    #[serde(rename = "_observationResultRequirement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_result_requirement_ext: Vec<Option<types::Element>>,

    /// Transform to apply the template
    pub transform: Option<types::Canonical>,
    /// Primitive extension sibling for [`transform`](Self::transform) (FHIR `_transform`).
    #[serde(rename = "_transform")]
    pub transform_ext: Option<types::Element>,

    /// Dynamic aspects of the definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic_value: Vec<ActivityDefinitionDynamicValue>,
}

/// Who should participate in the action being defined.
/// # Examples
///
/// ```
/// use fhir::r5::resources::activity_definition::ActivityDefinitionParticipant;
/// use fhir::r5::types;
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
pub struct ActivityDefinitionParticipant {
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

    /// E.g. Nurse, Surgeon, Parent, etc
    pub role: Option<types::CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<types::CodeableConcept>,
}

/// Dynamic aspects of the definition that are used to customize the activity
/// when it is instantiated.
/// # Examples
///
/// ```
/// use fhir::r5::resources::activity_definition::ActivityDefinitionDynamicValue;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// An expression that provides the dynamic value for the customization
    pub expression: types::Expression,
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
/// The `ActivityDefinition.asNeeded[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `ActivityDefinition.product[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionProduct {
    /// `productReference` variant.
    #[fhir("productReference")]
    Reference(Box<types::Reference>),
    /// `productCodeableConcept` variant.
    #[fhir("productCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `ActivityDefinition.subject[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `ActivityDefinition.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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

/// The `ActivityDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ActivityDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
