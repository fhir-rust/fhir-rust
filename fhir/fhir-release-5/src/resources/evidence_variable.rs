//! EvidenceVariable
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EvidenceVariable
//!
//! Version: 5.0.0
//!
//! EvidenceVariable Resource: The EvidenceVariable resource describes an element that knowledge (Evidence) is about.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The EvidenceVariable resource describes an element that knowledge (Evidence)
/// is about. It represents a "population", "exposure", or "outcome" concept that
/// defines characteristics used in evidence-based statistics. EvidenceVariables
/// are the building blocks referenced by Evidence and related resources to
/// describe what was studied and how it was measured.
///
/// Conceptually, an EvidenceVariable captures the precise, computable definition
/// of a PICO(TS) element (Population, Intervention/Exposure, Comparator, Outcome,
/// Timing, Setting) so that the same variable can be reused consistently across
/// multiple pieces of evidence, evidence reports, and evidence-based
/// recommendations. Each `characteristic` describes an inclusion or exclusion
/// criterion, optionally combined with other characteristics via
/// `definition_by_combination`, and may specify how a value is determined
/// (for example, from an observation, condition, or computed expression) as
/// well as the timing of that determination relative to a reference event.
/// When the variable represents a categorical or ordinal concept, `category`
/// groups the possible values, and `handling` indicates whether the variable
/// is continuous, dichotomous, ordinal, or polychotomous.
///
/// # Related resources
///
/// EvidenceVariable is typically referenced by `Evidence` and
/// `EvidenceReport` resources to identify the population, exposure, or
/// outcome under study, and its characteristics may reference clinical
/// resources such as [`Patient`](crate::r5::resources::patient::Patient),
/// `Observation`, or `Condition` via `definition_reference`. Coded concepts
/// used throughout this resource, such as `handling` and category values,
/// commonly make use of [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_variable::EvidenceVariable;
/// use fhir::r5::types;
///
/// let value = EvidenceVariable {
///     short_title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `shortTitle` is the name this serializes to on the wire.
/// assert_eq!(json["shortTitle"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariable {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this evidence variable, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the evidence variable
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the evidence variable
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `EvidenceVariable.versionAlgorithm[x]` choice element (0..1); see [`EvidenceVariableVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<EvidenceVariableVersionAlgorithm>,

    /// Name for this evidence variable (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this evidence variable (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Title for use in informal contexts
    pub short_title: Option<types::String>,
    /// Primitive extension sibling for [`short_title`](Self::short_title) (FHIR `_shortTitle`).
    #[serde(rename = "_shortTitle")]
    pub short_title_ext: Option<types::Element>,

    /// The publication status of this evidence variable: draft | active | retired | unknown.
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

    /// Natural language description of the evidence variable
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Why this EvidenceVariable is defined
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

    /// When the resource was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`).
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// When the resource was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`).
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// When the resource is expected to be used
    pub effective_period: Option<types::Period>,

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

    /// Actual or conceptual
    pub actual: Option<types::Boolean>,
    /// Primitive extension sibling for [`actual`](Self::actual) (FHIR `_actual`).
    #[serde(rename = "_actual")]
    pub actual_ext: Option<types::Element>,

    /// A defining factor of the EvidenceVariable, such as an inclusion or exclusion criterion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<EvidenceVariableCharacteristic>,

    /// The type of variable measurement: continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<crate::r5::coded::Coded<crate::r5::codes::VariableHandling>>,
    /// Primitive extension sibling for [`handling`](Self::handling) (FHIR `_handling`).
    #[serde(rename = "_handling")]
    pub handling_ext: Option<types::Element>,

    /// A grouping for ordinal or polychotomous variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<EvidenceVariableCategory>,
}

/// A defining factor of the EvidenceVariable.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_variable::EvidenceVariableCharacteristic;
/// use fhir::r5::types;
///
/// let value = EvidenceVariableCharacteristic {
///     link_id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("pat-1"));
///
/// let back: EvidenceVariableCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Label for internal linking
    pub link_id: Option<types::Id>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`).
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// Natural language description of the characteristic
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Whether the characteristic is an inclusion criterion or exclusion criterion
    pub exclude: Option<types::Boolean>,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`).
    #[serde(rename = "_exclude")]
    pub exclude_ext: Option<types::Element>,

    /// Defines the characteristic (without using type and value) by a Reference
    pub definition_reference: Option<types::Reference>,

    /// Defines the characteristic (without using type and value) by a Canonical
    pub definition_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`definition_canonical`](Self::definition_canonical) (FHIR `_definitionCanonical`).
    #[serde(rename = "_definitionCanonical")]
    pub definition_canonical_ext: Option<types::Element>,

    /// Defines the characteristic (without using type and value) by a CodeableConcept
    pub definition_codeable_concept: Option<types::CodeableConcept>,

    /// Defines the characteristic (without using type and value) by an expression
    pub definition_expression: Option<types::Expression>,

    /// Defines the characteristic (without using type and value) by an id
    pub definition_id: Option<types::Id>,
    /// Primitive extension sibling for [`definition_id`](Self::definition_id) (FHIR `_definitionId`).
    #[serde(rename = "_definitionId")]
    pub definition_id_ext: Option<types::Element>,

    /// Defines the characteristic using type and value
    pub definition_by_type_and_value:
        Option<EvidenceVariableCharacteristicDefinitionByTypeAndValue>,

    /// Used to specify how two or more characteristics are combined
    pub definition_by_combination: Option<EvidenceVariableCharacteristicDefinitionByCombination>,

    /// The `EvidenceVariable.characteristic.instances[x]` choice element (0..1); see [`EvidenceVariableCharacteristicInstances`].
    #[serde(flatten)]
    pub instances: Option<EvidenceVariableCharacteristicInstances>,

    /// The `EvidenceVariable.characteristic.duration[x]` choice element (0..1); see [`EvidenceVariableCharacteristicDuration`].
    #[serde(flatten)]
    pub duration: Option<EvidenceVariableCharacteristicDuration>,

    /// Timing in which the characteristic is determined
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time_from_event: Vec<EvidenceVariableCharacteristicTimeFromEvent>,
}

/// Defines the characteristic using type and value.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_variable::EvidenceVariableCharacteristicDefinitionByTypeAndValue;
/// use fhir::r5::types;
///
/// let value = EvidenceVariableCharacteristicDefinitionByTypeAndValue {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableCharacteristicDefinitionByTypeAndValue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicDefinitionByTypeAndValue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Expresses the type of characteristic
    pub r#type: types::CodeableConcept,

    /// Method for how the characteristic value was determined
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub method: Vec<types::CodeableConcept>,

    /// Device used for determining characteristic
    pub device: Option<types::Reference>,

    /// The `EvidenceVariable.characteristic.definitionByTypeAndValue.value[x]` choice element (0..1); see [`EvidenceVariableCharacteristicDefinitionByTypeAndValueValue`].
    #[serde(flatten)]
    pub value: Option<EvidenceVariableCharacteristicDefinitionByTypeAndValueValue>,

    /// Reference point for valueQuantity or valueRange
    pub offset: Option<types::CodeableConcept>,
}

/// Used to specify how two or more characteristics are combined.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::evidence_variable::EvidenceVariableCharacteristicDefinitionByCombination;
///
/// let value = EvidenceVariableCharacteristicDefinitionByCombination::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: EvidenceVariableCharacteristicDefinitionByCombination = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicDefinitionByCombination {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// all-of | any-of | at-least | at-most | statistical | net-effect | dataset
    pub code: crate::r5::coded::Coded<crate::r5::codes::CharacteristicCombination>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Provides the value of "n" when "at-least" or "at-most" codes are used
    pub threshold: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`threshold`](Self::threshold) (FHIR `_threshold`).
    #[serde(rename = "_threshold")]
    pub threshold_ext: Option<types::Element>,

    /// A defining factor of the characteristic
    pub characteristic: vec1::Vec1<EvidenceVariableCharacteristic>,
}

/// Timing in which the characteristic is determined.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_variable::EvidenceVariableCharacteristicTimeFromEvent;
/// use fhir::r5::types;
///
/// let value = EvidenceVariableCharacteristicTimeFromEvent {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: EvidenceVariableCharacteristicTimeFromEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicTimeFromEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Human readable description
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Used for footnotes or explanatory notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The `EvidenceVariable.characteristic.timeFromEvent.event[x]` choice element (0..1); see [`EvidenceVariableCharacteristicTimeFromEventEvent`].
    #[serde(flatten)]
    pub event: Option<EvidenceVariableCharacteristicTimeFromEventEvent>,

    /// Used to express the observation at a defined amount of time before or after the event
    pub quantity: Option<types::Quantity>,

    /// Used to express the observation within a period before and/or after the event
    pub range: Option<types::Range>,
}

/// A grouping for ordinal or polychotomous variables.
/// # Examples
///
/// ```
/// use fhir::r5::resources::evidence_variable::EvidenceVariableCategory;
/// use fhir::r5::types;
///
/// let value = EvidenceVariableCategory {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: EvidenceVariableCategory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCategory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of the grouping
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The `EvidenceVariable.category.value[x]` choice element (0..1); see [`EvidenceVariableCategoryValue`].
    #[serde(flatten)]
    pub value: Option<EvidenceVariableCategoryValue>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EvidenceVariable;

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
/// The `EvidenceVariable.category.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCategoryValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
}

/// The `EvidenceVariable.characteristic.definitionByTypeAndValue.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCharacteristicDefinitionByTypeAndValueValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueId` variant.
    #[fhir("valueId")]
    Id(crate::r5::choice::Primitive<types::Id>),
}

/// The `EvidenceVariable.characteristic.duration[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCharacteristicDuration {
    /// `durationQuantity` variant.
    #[fhir("durationQuantity")]
    Quantity(Box<types::Quantity>),
    /// `durationRange` variant.
    #[fhir("durationRange")]
    Range(Box<types::Range>),
}

/// The `EvidenceVariable.characteristic.instances[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCharacteristicInstances {
    /// `instancesQuantity` variant.
    #[fhir("instancesQuantity")]
    Quantity(Box<types::Quantity>),
    /// `instancesRange` variant.
    #[fhir("instancesRange")]
    Range(Box<types::Range>),
}

/// The `EvidenceVariable.characteristic.timeFromEvent.event[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableCharacteristicTimeFromEventEvent {
    /// `eventCodeableConcept` variant.
    #[fhir("eventCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `eventReference` variant.
    #[fhir("eventReference")]
    Reference(Box<types::Reference>),
    /// `eventDateTime` variant.
    #[fhir("eventDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `eventId` variant.
    #[fhir("eventId")]
    Id(crate::r5::choice::Primitive<types::Id>),
}

/// The `EvidenceVariable.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum EvidenceVariableVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
