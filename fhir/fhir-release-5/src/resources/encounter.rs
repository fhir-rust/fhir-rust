//! Encounter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Encounter
//!
//! Version: 5.0.0
//!
//! Encounter Resource: An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An interaction between healthcare provider(s), and/or patient(s) for the
/// purpose of providing healthcare service(s) or assessing the health status of
/// patient(s).
///
/// In FHIR R5 the Encounter resource is the primary record of the context in
/// which healthcare is delivered. It represents a single contact between a
/// patient (or group) and the health system, spanning the full range of care
/// settings: inpatient admissions, ambulatory and outpatient visits, emergency
/// presentations, home health visits, and virtual or telehealth consultations.
/// The encounter provides the administrative and clinical framing that other
/// resources reference, and it typically progresses through a lifecycle captured
/// by its status, from planned and in-progress through to discharged, completed,
/// or cancelled.
///
/// An Encounter ties together the subject of care, the participants (such as
/// practitioners and care teams), the physical or virtual locations visited, the
/// reasons the interaction took place, the diagnoses addressed, the periods of
/// time involved, and the administrative details of admission and discharge. It
/// serves as an organizing hub in the record, so that observations, procedures,
/// medication administrations, and other clinical activities can be grouped
/// against the interaction during which they occurred. Encounters may also be
/// nested or grouped, with one encounter referenced as part of another and one
/// or more encounters recorded against a broader episode of care.
///
/// # Related resources
///
/// The subject of an encounter is commonly a [`Patient`](crate::r5::resources::patient::Patient),
/// and coded elements such as status, class, priority, and type are expressed
/// using [`CodeableConcept`](crate::r5::types::CodeableConcept). Links to other
/// resources, including participants, locations, and the service provider, are
/// carried as [`Reference`](crate::r5::types::Reference) values. Encounters are
/// frequently associated with an `EpisodeOfCare`, an `Appointment`, and clinical
/// resources such as `Condition` and `Observation`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter::Encounter;
/// use fhir::r5::types;
///
/// let value = Encounter {
///     planned_start_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `plannedStartDate` is the name this serializes to on the wire.
/// assert_eq!(json["plannedStartDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Encounter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
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

    /// Identifier(s) by which this encounter is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Current lifecycle state of the encounter, such as planned, in-progress, on-hold, discharged, completed, cancelled, discontinued, entered-in-error, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::EncounterStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of the encounter setting, such as inpatient, outpatient, ambulatory, emergency, or virtual.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<types::CodeableConcept>,

    /// Indicates the urgency of the encounter
    pub priority: Option<types::CodeableConcept>,

    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, ...)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Specific type of service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// Reference to the patient or group that is the subject of this encounter and receives the healthcare service.
    pub subject: Option<types::Reference>,

    /// The current status of the subject in relation to the Encounter
    pub subject_status: Option<types::CodeableConcept>,

    /// Episode(s) of care that this encounter should be recorded against
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub episode_of_care: Vec<types::Reference>,

    /// The request that initiated this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The group(s) that are allocated to participate in this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<types::Reference>,

    /// Another Encounter this encounter is part of
    pub part_of: Option<types::Reference>,

    /// The organization (facility) responsible for this encounter
    pub service_provider: Option<types::Reference>,

    /// The people, devices, or services that took part in the encounter, each with a role and time period, modeled by EncounterParticipant.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<EncounterParticipant>,

    /// The appointment that scheduled this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appointment: Vec<types::Reference>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_service: Vec<types::VirtualServiceDetail>,

    /// The actual start and end time of the encounter
    pub actual_period: Option<types::Period>,

    /// The planned start date/time (or admission date) of the encounter
    pub planned_start_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`planned_start_date`](Self::planned_start_date) (FHIR `_plannedStartDate`).
    #[serde(rename = "_plannedStartDate")]
    pub planned_start_date_ext: Option<types::Element>,

    /// The planned end date/time (or discharge date) of the encounter
    pub planned_end_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`planned_end_date`](Self::planned_end_date) (FHIR `_plannedEndDate`).
    #[serde(rename = "_plannedEndDate")]
    pub planned_end_date_ext: Option<types::Element>,

    /// Actual quantity of time the encounter lasted (less time absent)
    pub length: Option<types::Duration>,

    /// The list of medical reasons that are expected to be addressed during the episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<EncounterReason>,

    /// The list of diagnosis relevant to this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<EncounterDiagnosis>,

    /// The set of accounts that may be used for billing for this Encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference>,

    /// Diet preferences reported by the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diet_preference: Vec<types::CodeableConcept>,

    /// Wheelchair, translator, stretcher, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_arrangement: Vec<types::CodeableConcept>,

    /// Special courtesies (VIP, board member)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_courtesy: Vec<types::CodeableConcept>,

    /// Details about the admission to a healthcare service
    pub admission: Option<EncounterAdmission>,

    /// List of locations where the patient has been
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<EncounterLocation>,
}

/// List of participants involved in the encounter.
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter::EncounterParticipant;
/// use fhir::r5::types;
///
/// let value = EncounterParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Role of participant in encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Period of time during the encounter that the participant participated
    pub period: Option<types::Period>,

    /// The individual, device, or service participating in the encounter
    pub actor: Option<types::Reference>,
}

/// The list of medical reasons that are expected to be addressed during the
/// episode of care.
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter::EncounterReason;
/// use fhir::r5::types;
///
/// let value = EncounterReason {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterReason = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterReason {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What the reason value should be used for/as
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#use: Vec<types::CodeableConcept>,

    /// Reason the encounter takes place (core or reference)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<types::CodeableReference>,
}

/// The list of diagnosis relevant to this encounter.
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter::EncounterDiagnosis;
/// use fhir::r5::types;
///
/// let value = EncounterDiagnosis {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The diagnosis relevant to the encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<types::CodeableReference>,

    /// Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#use: Vec<types::CodeableConcept>,
}

/// Details about the admission to a healthcare service.
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter::EncounterAdmission;
/// use fhir::r5::types;
///
/// let value = EncounterAdmission {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterAdmission = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterAdmission {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pre-admission identifier
    pub pre_admission_identifier: Option<types::Identifier>,

    /// The location/organization from which the patient came before admission
    pub origin: Option<types::Reference>,

    /// From where patient was admitted (physician referral, transfer)
    pub admit_source: Option<types::CodeableConcept>,

    /// Indicates that the patient is being re-admitted
    pub re_admission: Option<types::CodeableConcept>,

    /// Location/organization to which the patient is discharged
    pub destination: Option<types::Reference>,

    /// Category or kind of location after discharge
    pub discharge_disposition: Option<types::CodeableConcept>,
}

/// List of locations where the patient has been.
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter::EncounterLocation;
/// use fhir::r5::types;
///
/// let value = EncounterLocation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterLocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location the encounter takes place
    pub location: types::Reference,

    /// planned | active | reserved | completed
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::EncounterLocationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<types::CodeableConcept>,

    /// Time period during which the patient was present at the location
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Encounter;

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
