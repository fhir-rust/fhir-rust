//! ResearchElementDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ResearchElementDefinition
//!
//! Version: 4.3.0
//!
//! A population, intervention, or exposure definition
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The ResearchElementDefinition resource describes a "PICO" element that
/// knowledge (evidence, assertion, recommendation) is about.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4b::resources::research_element_definition::ResearchElementDefinition;
///
/// let value = ResearchElementDefinition::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ResearchElementDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ResearchElementDefinition {
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

    /// Canonical identifier for this research element definition, represented
    /// as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the research element definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the research element definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this research element definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this research element definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Title for use in informal contexts
    pub short_title: Option<types::String>,
    /// Primitive extension sibling for [`short_title`](Self::short_title) (FHIR `_shortTitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_shortTitle")]
    pub short_title_ext: Option<types::Element>,

    /// Subordinate title of the ResearchElementDefinition
    pub subtitle: Option<types::String>,
    /// Primitive extension sibling for [`subtitle`](Self::subtitle) (FHIR `_subtitle`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subtitle")]
    pub subtitle_ext: Option<types::Element>,

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

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location,
    /// Device
    /// The `ResearchElementDefinition.subject[x]` choice element (0..1); see [`ResearchElementDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<ResearchElementDefinitionSubject>,

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

    /// Natural language description of the research element definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment: Vec<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment_ext: Vec<Option<types::Element>>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for research element definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this research element definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Describes the clinical usage of the ResearchElementDefinition
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

    /// When the research element definition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the research element definition was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the research element definition is expected to be used
    pub effective_period: Option<types::Period>,

    /// The category of the ResearchElementDefinition, such as Education,
    /// Treatment, Assessment, etc.
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

    /// Logic used by the ResearchElementDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<types::Canonical>,
    /// Primitive extension sibling for [`library`](Self::library) (FHIR `_library`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_library")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library_ext: Vec<Option<types::Element>>,

    /// population | exposure | outcome
    pub r#type: crate::coded::Coded<crate::r4b::codes::ResearchElementType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// dichotomous | continuous | descriptive
    pub variable_type: Option<crate::coded::Coded<crate::r4b::codes::VariableType>>,
    /// Primitive extension sibling for [`variable_type`](Self::variable_type) (FHIR `_variableType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_variableType")]
    pub variable_type_ext: Option<types::Element>,

    /// What defines the members of the research element
    pub characteristic: ::vec1::Vec1<ResearchElementDefinitionCharacteristic>,
}

/// A characteristic that defines the members of the research element. Multiple
/// characteristics are applied with "and" semantics.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::research_element_definition::ResearchElementDefinitionCharacteristic;
/// use fhir::r4b::types;
///
/// let value = ResearchElementDefinitionCharacteristic {
///     study_effective_description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `studyEffectiveDescription` is the name this serializes to on the wire.
/// assert_eq!(json["studyEffectiveDescription"], ::serde_json::json!("abc"));
///
/// let back: ResearchElementDefinitionCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ResearchElementDefinitionCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What code or expression defines members?
    /// The `ResearchElementDefinition.characteristic.definition[x]` choice element (1..1); see [`ResearchElementDefinitionCharacteristicDefinition`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub definition: Option<ResearchElementDefinitionCharacteristicDefinition>,

    /// What code/value pairs define members?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage_context: Vec<types::UsageContext>,

    /// Whether the characteristic includes or excludes members
    pub exclude: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// What unit is the outcome described in?
    pub unit_of_measure: Option<types::CodeableConcept>,

    /// What time period does the study cover
    pub study_effective_description: Option<types::String>,
    /// Primitive extension sibling for [`study_effective_description`](Self::study_effective_description) (FHIR `_studyEffectiveDescription`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_studyEffectiveDescription")]
    pub study_effective_description_ext: Option<types::Element>,

    /// What time period does the study cover
    /// The `ResearchElementDefinition.characteristic.studyEffective[x]` choice element (0..1); see [`ResearchElementDefinitionCharacteristicStudyEffective`].
    #[serde(flatten)]
    pub study_effective: Option<ResearchElementDefinitionCharacteristicStudyEffective>,

    /// Observation time from study start
    pub study_effective_time_from_start: Option<types::Duration>,

    /// mean | median | mean-of-mean | mean-of-median | median-of-mean |
    /// median-of-median
    pub study_effective_group_measure: Option<crate::coded::Coded<crate::r4b::codes::GroupMeasure>>,
    /// Primitive extension sibling for [`study_effective_group_measure`](Self::study_effective_group_measure) (FHIR `_studyEffectiveGroupMeasure`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_studyEffectiveGroupMeasure")]
    pub study_effective_group_measure_ext: Option<types::Element>,

    /// What time period do participants cover
    pub participant_effective_description: Option<types::String>,
    /// Primitive extension sibling for [`participant_effective_description`](Self::participant_effective_description) (FHIR `_participantEffectiveDescription`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_participantEffectiveDescription")]
    pub participant_effective_description_ext: Option<types::Element>,

    /// What time period do participants cover
    /// The `ResearchElementDefinition.characteristic.participantEffective[x]` choice element (0..1); see [`ResearchElementDefinitionCharacteristicParticipantEffective`].
    #[serde(flatten)]
    pub participant_effective: Option<ResearchElementDefinitionCharacteristicParticipantEffective>,

    /// Observation time from study start
    pub participant_effective_time_from_start: Option<types::Duration>,

    /// mean | median | mean-of-mean | mean-of-median | median-of-mean |
    /// median-of-median
    pub participant_effective_group_measure:
        Option<crate::coded::Coded<crate::r4b::codes::GroupMeasure>>,
    /// Primitive extension sibling for [`participant_effective_group_measure`](Self::participant_effective_group_measure) (FHIR `_participantEffectiveGroupMeasure`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_participantEffectiveGroupMeasure")]
    pub participant_effective_group_measure_ext: Option<types::Element>,
}

/// The `ResearchElementDefinition.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ResearchElementDefinitionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `ResearchElementDefinition.characteristic.definition[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ResearchElementDefinitionCharacteristicDefinition {
    /// `definitionCodeableConcept` variant.
    #[fhir("definitionCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `definitionCanonical` variant.
    #[fhir("definitionCanonical")]
    Canonical(crate::r4b::choice::Primitive<types::Canonical>),
    /// `definitionExpression` variant.
    #[fhir("definitionExpression")]
    Expression(Box<types::Expression>),
    /// `definitionDataRequirement` variant.
    #[fhir("definitionDataRequirement")]
    DataRequirement(Box<types::DataRequirement>),
}

/// The `ResearchElementDefinition.characteristic.studyEffective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ResearchElementDefinitionCharacteristicStudyEffective {
    /// `studyEffectiveDateTime` variant.
    #[fhir("studyEffectiveDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `studyEffectivePeriod` variant.
    #[fhir("studyEffectivePeriod")]
    Period(Box<types::Period>),
    /// `studyEffectiveDuration` variant.
    #[fhir("studyEffectiveDuration")]
    Duration(Box<types::Duration>),
    /// `studyEffectiveTiming` variant.
    #[fhir("studyEffectiveTiming")]
    Timing(Box<types::Timing>),
}

/// The `ResearchElementDefinition.characteristic.participantEffective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum ResearchElementDefinitionCharacteristicParticipantEffective {
    /// `participantEffectiveDateTime` variant.
    #[fhir("participantEffectiveDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `participantEffectivePeriod` variant.
    #[fhir("participantEffectivePeriod")]
    Period(Box<types::Period>),
    /// `participantEffectiveDuration` variant.
    #[fhir("participantEffectiveDuration")]
    Duration(Box<types::Duration>),
    /// `participantEffectiveTiming` variant.
    #[fhir("participantEffectiveTiming")]
    Timing(Box<types::Timing>),
}
