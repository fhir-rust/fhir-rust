//! AdverseEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
//!
//! Version: 5.0.0
//!
//! AdverseEvent Resource: An event that may be related to unintended effects on a patient or research participant.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// An event (i.e. any change to current patient status) that may be related to
/// unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects, and is used in both
/// clinical care and research safety domains.
///
/// AdverseEvent supports both direct patient safety reporting (for example,
/// medication mishaps, procedure complications, or hospital-acquired
/// infections) and clinical research pharmacovigilance, where investigators
/// must record whether an event actually occurred or was only a potential
/// occurrence (see `actuality`). It captures when the event happened, who was
/// involved, the suspected causal agents, contributing factors, and any
/// preventive or mitigating actions taken, making it a central resource for
/// safety surveillance and quality-improvement workflows.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the `subject` of
///   an adverse event is commonly a patient or research participant.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used
///   throughout this resource for coded values such as `category`, `code`,
///   `seriousness`, and `outcome`.
/// - `Encounter` and `ResearchStudy` — referenced via `encounter` and
///   `study` to relate the event to a care episode or research context.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEvent;
/// use fhir::r5::types;
///
/// let value = AdverseEvent {
///     recorded_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `recordedDate` is the name this serializes to on the wire.
/// assert_eq!(json["recordedDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AdverseEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEvent {
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

    /// Business identifier for the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The lifecycle status of the record itself: in-progress | completed |
    /// entered-in-error | unknown
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Whether the adverse event actually occurred (actual) or was only a
    /// possible or averted occurrence (potential)
    pub actuality: crate::r5::coded::Coded<crate::r5::codes::AdverseEventActuality>,
    /// Primitive extension sibling for [`actuality`](Self::actuality) (FHIR `_actuality`).
    #[serde(rename = "_actuality")]
    pub actuality_ext: Option<types::Element>,

    /// wrong-patient | procedure-mishap | medication-mishap | device |
    /// unsafe-physical-environment | hospital-aquired-infection | wrong-body-site
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Event or incident that occurred or was averted
    pub code: Option<types::CodeableConcept>,

    /// The patient or research participant impacted by the event; see also
    /// [`Patient`](crate::r5::resources::patient::Patient)
    pub subject: types::Reference,

    /// The Encounter associated with the start of the AdverseEvent
    pub encounter: Option<types::Reference>,

    /// The `AdverseEvent.occurrence[x]` choice element (0..1); see [`AdverseEventOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<AdverseEventOccurrence>,

    /// When the event was detected
    pub detected: Option<types::DateTime>,
    /// Primitive extension sibling for [`detected`](Self::detected) (FHIR `_detected`).
    #[serde(rename = "_detected")]
    pub detected_ext: Option<types::Element>,

    /// When the event was recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`).
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Effect on the subject due to this event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resulting_effect: Vec<types::Reference>,

    /// Location where adverse event occurred
    pub location: Option<types::Reference>,

    /// Seriousness or gravity of the event
    pub seriousness: Option<types::CodeableConcept>,

    /// Type of outcome from the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome: Vec<types::CodeableConcept>,

    /// Who recorded the adverse event
    pub recorder: Option<types::Reference>,

    /// Who was involved in the adverse event or the potential adverse event
    /// and what they did
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<AdverseEventParticipant>,

    /// Research study that the subject is enrolled in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub study: Vec<types::Reference>,

    /// Considered likely or probable or anticipated in the research study
    pub expected_in_research_study: Option<types::Boolean>,
    /// Primitive extension sibling for [`expected_in_research_study`](Self::expected_in_research_study) (FHIR `_expectedInResearchStudy`).
    #[serde(rename = "_expectedInResearchStudy")]
    pub expected_in_research_study_ext: Option<types::Element>,

    /// The suspected agent causing the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suspect_entity: Vec<AdverseEventSuspectEntity>,

    /// Contributing factors suspected to have increased the probability or
    /// severity of the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributing_factor: Vec<AdverseEventContributingFactor>,

    /// Preventive actions that contributed to avoiding the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preventive_action: Vec<AdverseEventPreventiveAction>,

    /// Ameliorating actions taken after the adverse event occured in order to
    /// reduce the extent of harm
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigating_action: Vec<AdverseEventMitigatingAction>,

    /// Supporting information relevant to the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<AdverseEventSupportingInfo>,

    /// Comment on adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Who was involved in the adverse event or the potential adverse event and
/// what they did.
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEventParticipant;
/// use fhir::r5::types;
///
/// let value = AdverseEventParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of involvement
    pub function: Option<types::CodeableConcept>,

    /// Who was involved in the adverse event or the potential adverse event
    pub actor: types::Reference,
}

/// The suspected agent causing the adverse event.
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEventSuspectEntity;
/// use fhir::r5::types;
///
/// let value = AdverseEventSuspectEntity {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventSuspectEntity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `AdverseEvent.suspectEntity.instance[x]` choice element (0..1); see [`AdverseEventSuspectEntityInstance`].
    #[serde(flatten)]
    pub instance: Option<AdverseEventSuspectEntityInstance>,

    /// Information on the possible cause of the event
    pub causality: Option<AdverseEventSuspectEntityCausality>,
}

/// Information on the possible cause of the event.
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEventSuspectEntityCausality;
/// use fhir::r5::types;
///
/// let value = AdverseEventSuspectEntityCausality {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventSuspectEntityCausality = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntityCausality {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Method of evaluating the relatedness of the suspected entity to the event
    pub assessment_method: Option<types::CodeableConcept>,

    /// Result of the assessment regarding the relatedness of the suspected
    /// entity to the event
    pub entity_relatedness: Option<types::CodeableConcept>,

    /// Author of the information on the possible cause of the event
    pub author: Option<types::Reference>,
}

/// Contributing factors suspected to have increased the probability or severity
/// of the adverse event.
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEventContributingFactor;
/// use fhir::r5::types;
///
/// let value = AdverseEventContributingFactor {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventContributingFactor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventContributingFactor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `AdverseEvent.contributingFactor.item[x]` choice element (0..1); see [`AdverseEventContributingFactorItem`].
    #[serde(flatten)]
    pub item: Option<AdverseEventContributingFactorItem>,
}

/// Preventive actions that contributed to avoiding the adverse event.
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEventPreventiveAction;
/// use fhir::r5::types;
///
/// let value = AdverseEventPreventiveAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventPreventiveAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventPreventiveAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `AdverseEvent.preventiveAction.item[x]` choice element (0..1); see [`AdverseEventPreventiveActionItem`].
    #[serde(flatten)]
    pub item: Option<AdverseEventPreventiveActionItem>,
}

/// Ameliorating actions taken after the adverse event occured in order to
/// reduce the extent of harm.
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEventMitigatingAction;
/// use fhir::r5::types;
///
/// let value = AdverseEventMitigatingAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventMitigatingAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventMitigatingAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `AdverseEvent.mitigatingAction.item[x]` choice element (0..1); see [`AdverseEventMitigatingActionItem`].
    #[serde(flatten)]
    pub item: Option<AdverseEventMitigatingActionItem>,
}

/// Supporting information relevant to the event.
/// # Examples
///
/// ```
/// use fhir::r5::resources::adverse_event::AdverseEventSupportingInfo;
/// use fhir::r5::types;
///
/// let value = AdverseEventSupportingInfo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AdverseEventSupportingInfo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `AdverseEvent.supportingInfo.item[x]` choice element (0..1); see [`AdverseEventSupportingInfoItem`].
    #[serde(flatten)]
    pub item: Option<AdverseEventSupportingInfoItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AdverseEvent;

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
/// The `AdverseEvent.contributingFactor.item[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventContributingFactorItem {
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `AdverseEvent.mitigatingAction.item[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventMitigatingActionItem {
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `AdverseEvent.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `AdverseEvent.preventiveAction.item[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventPreventiveActionItem {
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `AdverseEvent.supportingInfo.item[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventSupportingInfoItem {
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `AdverseEvent.suspectEntity.instance[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventSuspectEntityInstance {
    /// `instanceCodeableConcept` variant.
    #[fhir("instanceCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `instanceReference` variant.
    #[fhir("instanceReference")]
    Reference(Box<types::Reference>),
}
