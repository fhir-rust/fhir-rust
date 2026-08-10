//! AdverseEvent
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AdverseEvent
//!
//! Version: 6.0.0-ballot3
//!
//! An event that may be related to unintended effects on a patient or research
//! participant
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An event (i.e. any change to current patient status) that may be related to
/// unintended effects on a patient or research participant. The unintended
/// effects may require additional monitoring, treatment, hospitalization, or
/// may result in death. The AdverseEvent resource also extends to potential or
/// avoided events that could have had such effects. There are two major
/// domains where the AdverseEvent resource is expected to be used. One is in
/// clinical care reported adverse events and the other is in reporting adverse
/// events in clinical research trial management. Adverse events can be
/// reported by healthcare providers, patients, caregivers or by medical
/// products manufacturers. Given the differences between these two concepts,
/// we recommend consulting the domain specific implementation guides when
/// implementing the AdverseEvent Resource. The implementation guides include
/// specific extensions, value sets and constraints.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::adverse_event::AdverseEvent;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct AdverseEvent {
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

    /// Business identifier for the event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// in-progress | completed | entered-in-error | unknown
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// actual | potential
    pub actuality: crate::coded::Coded<crate::r6::codes::AdverseEventActuality>,
    /// Primitive extension sibling for [`actuality`](Self::actuality) (FHIR `_actuality`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actuality")]
    pub actuality_ext: Option<types::Element>,

    /// wrong-patient | procedure-mishap | medication-mishap | device |
    /// unsafe-physical-environment | hospital-aquired-infection |
    /// wrong-body-site
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Event or incident that occurred or was averted
    pub code: Option<types::CodeableConcept>,

    /// Subject impacted by event
    pub subject: types::Reference,

    /// The Encounter associated with the start of the AdverseEvent
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// When the cause of the AdverseEvent occurred
    /// The `AdverseEvent.cause[x]` choice element (0..1); see [`AdverseEventCause`].
    #[serde(flatten)]
    pub cause: Option<AdverseEventCause>,

    /// When the effect of the AdverseEvent occurred
    /// The `AdverseEvent.effect[x]` choice element (0..1); see [`AdverseEventEffect`].
    #[serde(flatten)]
    pub effect: Option<AdverseEventEffect>,

    /// When the event was detected
    pub detected: Option<types::DateTime>,
    /// Primitive extension sibling for [`detected`](Self::detected) (FHIR `_detected`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detected")]
    pub detected_ext: Option<types::Element>,

    /// When the event was recorded
    pub recorded_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded_date`](Self::recorded_date) (FHIR `_recordedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recordedDate")]
    pub recorded_date_ext: Option<types::Element>,

    /// Effect on the subject due to this event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resulting_effect: Vec<types::CodeableReference>,

    /// Location where adverse event occurred
    pub location: Option<types::Reference<crate::r6::resources::Location>>,

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
    pub study: Vec<types::Reference<crate::r6::resources::ResearchStudy>>,

    /// Considered likely or probable or anticipated in the research study
    pub expected_in_research_study: Option<types::Boolean>,
    /// Primitive extension sibling for [`expected_in_research_study`](Self::expected_in_research_study) (FHIR `_expectedInResearchStudy`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expectedInResearchStudy")]
    pub expected_in_research_study_ext: Option<types::Element>,

    /// The suspected agent causing the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suspect_entity: Vec<AdverseEventSuspectEntity>,

    /// Contributing factors suspected to have increased the probability or
    /// severity of the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributing_factor: Vec<types::CodeableReference>,

    /// Preventive actions that contributed to avoiding the adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preventive_action: Vec<types::CodeableReference>,

    /// Ameliorating actions taken after the adverse event occurred in order to
    /// reduce the extent of harm
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigating_action: Vec<types::CodeableReference>,

    /// Subject medical history or document relevant to this adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::CodeableReference>,

    /// Comment on adverse event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Indicates who or what participated in the adverse event and how they were
/// involved.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::adverse_event::AdverseEventParticipant;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Describes the entity that is suspected to have caused the adverse event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::adverse_event::AdverseEventSuspectEntity;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct AdverseEventSuspectEntity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Refers to the specific entity that caused the adverse event
    pub instance: types::CodeableReference,

    /// Information on the possible cause of the event
    pub causality: Option<AdverseEventSuspectEntityCausality>,
}

/// Information on the possible cause of the event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::adverse_event::AdverseEventSuspectEntityCausality;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct AdverseEventSuspectEntityCausality {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Method of evaluating the relatedness of the suspected entity to the
    /// event
    pub assessment_method: Option<types::CodeableConcept>,

    /// Result of the assessment regarding the relatedness of the suspected
    /// entity to the event
    pub entity_relatedness: Option<types::CodeableConcept>,

    /// Author of the information on the possible cause of the event
    pub author: Option<types::Reference>,
}

/// The `AdverseEvent.cause[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventCause {
    /// `causeDateTime` variant.
    #[fhir("causeDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `causePeriod` variant.
    #[fhir("causePeriod")]
    Period(Box<types::Period>),
}

/// The `AdverseEvent.effect[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum AdverseEventEffect {
    /// `effectDateTime` variant.
    #[fhir("effectDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `effectPeriod` variant.
    #[fhir("effectPeriod")]
    Period(Box<types::Period>),
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
