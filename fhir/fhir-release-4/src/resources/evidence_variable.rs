//! EvidenceVariable
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
//!
//! Version: 4.0.1
//!
//! A population, intervention, or exposure definition
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The EvidenceVariable resource describes a "PICO" element that knowledge
/// (evidence, assertion, recommendation) is about.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::evidence_variable::EvidenceVariable;
///
/// let value = EvidenceVariable::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EvidenceVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EvidenceVariable {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this evidence variable, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the evidence variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the evidence variable
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this evidence variable (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this evidence variable (human friendly)
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

    /// Subordinate title of the EvidenceVariable
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

    /// Natural language description of the evidence variable
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for evidence variable (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// When the evidence variable was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the evidence variable was last reviewed
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the evidence variable is expected to be used
    pub effective_period: Option<types::Period>,

    /// The category of the EvidenceVariable, such as Education, Treatment,
    /// Assessment, etc.
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

    /// dichotomous | continuous | descriptive
    pub r#type: Option<crate::coded::Coded<crate::r4::codes::VariableType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// What defines the members of the evidence element
    pub characteristic: ::vec1::Vec1<EvidenceVariableCharacteristic>,
}

/// A characteristic that defines the members of the evidence element. Multiple
/// characteristics are applied with "and" semantics.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::evidence_variable::EvidenceVariableCharacteristic;
/// use fhir::r4::types;
///
/// let value = EvidenceVariableCharacteristic {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct EvidenceVariableCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Natural language description of the characteristic
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// What code or expression defines members?
    /// The `EvidenceVariable.characteristic.definition[x]` choice element (1..1); see [`EvidenceVariableCharacteristicDefinition`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub definition: Option<EvidenceVariableCharacteristicDefinition>,

    /// What code/value pairs define members?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub usage_context: Vec<types::UsageContext>,

    /// Whether the characteristic includes or excludes members
    pub exclude: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// What time period do participants cover
    /// The `EvidenceVariable.characteristic.participantEffective[x]` choice element (0..1); see [`EvidenceVariableCharacteristicParticipantEffective`].
    #[serde(flatten)]
    pub participant_effective: Option<EvidenceVariableCharacteristicParticipantEffective>,

    /// Observation time from study start
    pub time_from_start: Option<types::Duration>,

    /// mean | median | mean-of-mean | mean-of-median | median-of-mean |
    /// median-of-median
    pub group_measure: Option<crate::coded::Coded<crate::r4::codes::GroupMeasure>>,
    /// Primitive extension sibling for [`group_measure`](Self::group_measure) (FHIR `_groupMeasure`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupMeasure")]
    pub group_measure_ext: Option<types::Element>,
}

/// The `EvidenceVariable.characteristic.definition[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCharacteristicDefinition {
    /// `definitionReference` variant.
    #[fhir("definitionReference")]
    Reference(Box<types::Reference>),
    /// `definitionCanonical` variant.
    #[fhir("definitionCanonical")]
    Canonical(crate::r4::choice::Primitive<types::Canonical>),
    /// `definitionCodeableConcept` variant.
    #[fhir("definitionCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `definitionExpression` variant.
    #[fhir("definitionExpression")]
    Expression(Box<types::Expression>),
    /// `definitionDataRequirement` variant.
    #[fhir("definitionDataRequirement")]
    DataRequirement(Box<types::DataRequirement>),
    /// `definitionTriggerDefinition` variant.
    #[fhir("definitionTriggerDefinition")]
    TriggerDefinition(Box<types::TriggerDefinition>),
}

/// The `EvidenceVariable.characteristic.participantEffective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCharacteristicParticipantEffective {
    /// `participantEffectiveDateTime` variant.
    #[fhir("participantEffectiveDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
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
