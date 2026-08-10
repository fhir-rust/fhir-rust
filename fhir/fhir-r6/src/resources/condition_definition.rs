//! ConditionDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ConditionDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! A definition of a condition
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A definition of a condition and information relevant to managing it.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::condition_definition::ConditionDefinition;
/// use fhir::r6::types;
///
/// let value = ConditionDefinition {
///     has_severity: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `hasSeverity` is the name this serializes to on the wire.
/// assert_eq!(json["hasSeverity"], ::serde_json::json!(true));
///
/// let back: ConditionDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ConditionDefinitionDe")]
#[fhir_version("r6")]
pub struct ConditionDefinition {
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

    /// Canonical identifier for this condition definition, represented as a
    /// URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the condition definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the condition definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `ConditionDefinition.versionAlgorithm[x]` choice element (0..1); see [`ConditionDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<ConditionDefinitionVersionAlgorithm>,

    /// Name for this condition definition (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this condition definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Subordinate title of the event definition
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

    /// Natural language description of the condition definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for condition definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Identification of the condition, problem or diagnosis
    pub code: types::CodeableConcept,

    /// Subjective severity of condition
    pub severity: Option<types::CodeableConcept>,

    /// Anatomical location, if relevant
    pub body_site: Option<types::CodeableConcept>,

    /// Stage/grade, usually assessed formally
    pub stage: Option<types::CodeableConcept>,

    /// Whether Severity is appropriate
    pub has_severity: Option<types::Boolean>,
    /// Primitive extension sibling for [`has_severity`](Self::has_severity) (FHIR `_hasSeverity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hasSeverity")]
    pub has_severity_ext: Option<types::Element>,

    /// Whether bodySite is appropriate
    pub has_body_site: Option<types::Boolean>,
    /// Primitive extension sibling for [`has_body_site`](Self::has_body_site) (FHIR `_hasBodySite`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hasBodySite")]
    pub has_body_site_ext: Option<types::Element>,

    /// Whether stage is appropriate
    pub has_stage: Option<types::Boolean>,
    /// Primitive extension sibling for [`has_stage`](Self::has_stage) (FHIR `_hasStage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hasStage")]
    pub has_stage_ext: Option<types::Element>,

    /// Formal Definition for the condition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub definition: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`definition`](Self::definition) (FHIR `_definition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_definition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition_ext: Vec<Option<types::Element>>,

    /// Observations particularly relevant to this condition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub observation: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`observation`](Self::observation) (FHIR `_observation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_observation")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub observation_ext: Vec<Option<types::Element>>,

    /// Medications particularly relevant for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub medication: Vec<ConditionDefinitionMedication>,

    /// Observation that suggets this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub precondition: Vec<ConditionDefinitionPrecondition>,

    /// Appropriate team for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team: Vec<types::Reference<crate::r6::resources::CareTeam>>,

    /// Questionnaire for this condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub questionnaire: Vec<ConditionDefinitionQuestionnaire>,

    /// Plan that is appropriate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plan: Vec<ConditionDefinitionPlan>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConditionDefinitionDe {
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
    version_algorithm: crate::r6::choice::Slot<ConditionDefinitionVersionAlgorithm>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    subtitle: Option<types::String>,
    #[serde(rename = "_subtitle")]
    subtitle_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
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
    code: types::CodeableConcept,
    severity: Option<types::CodeableConcept>,
    body_site: Option<types::CodeableConcept>,
    stage: Option<types::CodeableConcept>,
    has_severity: Option<types::Boolean>,
    #[serde(rename = "_hasSeverity")]
    has_severity_ext: Option<types::Element>,
    has_body_site: Option<types::Boolean>,
    #[serde(rename = "_hasBodySite")]
    has_body_site_ext: Option<types::Element>,
    has_stage: Option<types::Boolean>,
    #[serde(rename = "_hasStage")]
    has_stage_ext: Option<types::Element>,
    #[serde(default)]
    definition: ::fhir_core::PrimVec<types::Uri>,
    #[serde(rename = "_definition")]
    #[serde(default)]
    definition_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    observation: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_observation")]
    #[serde(default)]
    observation_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    medication: Vec<ConditionDefinitionMedication>,
    #[serde(default)]
    precondition: Vec<ConditionDefinitionPrecondition>,
    #[serde(default)]
    team: Vec<types::Reference<crate::r6::resources::CareTeam>>,
    #[serde(default)]
    questionnaire: Vec<ConditionDefinitionQuestionnaire>,
    #[serde(default)]
    plan: Vec<ConditionDefinitionPlan>,
}

impl ::core::convert::From<ConditionDefinitionDe> for ConditionDefinition {
    fn from(v: ConditionDefinitionDe) -> Self {
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
            status: v.status,
            status_ext: v.status_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            code: v.code,
            severity: v.severity,
            body_site: v.body_site,
            stage: v.stage,
            has_severity: v.has_severity,
            has_severity_ext: v.has_severity_ext,
            has_body_site: v.has_body_site,
            has_body_site_ext: v.has_body_site_ext,
            has_stage: v.has_stage,
            has_stage_ext: v.has_stage_ext,
            definition: v.definition,
            definition_ext: v.definition_ext,
            observation: v.observation,
            observation_ext: v.observation_ext,
            medication: v.medication,
            precondition: v.precondition,
            team: v.team,
            questionnaire: v.questionnaire,
            plan: v.plan,
        }
    }
}

/// Medications particularly relevant for this condition.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::condition_definition::ConditionDefinitionMedication;
/// use fhir::r6::types;
///
/// let value = ConditionDefinitionMedication {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConditionDefinitionMedication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConditionDefinitionMedication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Category that is relevant
    pub category: Option<types::CodeableConcept>,

    /// Code for relevant Medication
    pub code: Option<types::CodeableConcept>,
}

/// Plan that is appropriate.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::condition_definition::ConditionDefinitionPlan;
/// use fhir::r6::types;
///
/// let value = ConditionDefinitionPlan {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConditionDefinitionPlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConditionDefinitionPlan {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Use for the plan
    pub role: Option<types::CodeableConcept>,

    /// The actual plan
    pub reference: types::Reference<crate::r6::resources::PlanDefinition>,
}

/// An observation that suggests that this condition applies.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::condition_definition::ConditionDefinitionPrecondition;
/// use fhir::r6::types;
///
/// let value = ConditionDefinitionPrecondition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConditionDefinitionPrecondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ConditionDefinitionPreconditionDe")]
#[fhir_version("r6")]
pub struct ConditionDefinitionPrecondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// sensitive | specific
    pub r#type: crate::coded::Coded<crate::r6::codes::ConditionPreconditionType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Code for relevant Observation
    pub code: types::CodeableConcept,

    /// Value of Observation
    /// The `ConditionDefinition.precondition.value[x]` choice element (0..1); see [`ConditionDefinitionPreconditionValue`].
    #[serde(flatten)]
    pub value: Option<ConditionDefinitionPreconditionValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConditionDefinitionPreconditionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: crate::coded::Coded<crate::r6::codes::ConditionPreconditionType>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    code: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<ConditionDefinitionPreconditionValue>,
}

impl ::core::convert::From<ConditionDefinitionPreconditionDe> for ConditionDefinitionPrecondition {
    fn from(v: ConditionDefinitionPreconditionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            code: v.code,
            value: v.value.0,
        }
    }
}

/// Questionnaire for this condition.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::condition_definition::ConditionDefinitionQuestionnaire;
/// use fhir::r6::types;
///
/// let value = ConditionDefinitionQuestionnaire {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ConditionDefinitionQuestionnaire = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ConditionDefinitionQuestionnaire {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// preadmit | diff-diagnosis | outcome
    pub purpose: crate::coded::Coded<crate::r6::codes::ConditionQuestionnairePurpose>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Specific Questionnaire
    pub reference: types::Reference<crate::r6::resources::Questionnaire>,
}

/// The `ConditionDefinition.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `ConditionDefinition.precondition.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ConditionDefinitionPreconditionValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ConditionDefinition;

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
