//! Encounter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Encounter
//!
//! Version: 6.0.0-ballot3
//!
//! An interaction during which services are provided to the patient
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An interaction between healthcare provider(s), and/or patient(s) for the
/// purpose of providing healthcare service(s) or assessing the health status
/// of patient(s).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter::Encounter;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct Encounter {
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

    /// Identifier(s) by which this encounter is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// planned | in-progress | on-hold | discharged | completed | cancelled |
    /// discontinued | entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::EncounterStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of patient encounter context - e.g. Inpatient,
    /// outpatient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<types::CodeableConcept>,

    /// Indicates the urgency of the encounter
    pub priority: Option<types::CodeableConcept>,

    /// Specific type of encounter (e.g. e-mail consultation, surgical
    /// day-care, ...)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Specific type of service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// The patient or group related to this encounter
    pub subject: Option<types::Reference>,

    /// The current status of the subject in relation to the Encounter
    pub subject_status: Option<types::CodeableConcept>,

    /// Episode(s) of care that this encounter should be recorded against
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub episode_of_care: Vec<types::Reference<crate::r6::resources::EpisodeOfCare>>,

    /// The request that initiated this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The group(s) that are allocated to participate in this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<types::Reference<crate::r6::resources::CareTeam>>,

    /// Another Encounter this encounter is part of
    pub part_of: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// The organization (facility) responsible for this encounter
    pub service_provider: Option<types::Reference<crate::r6::resources::Organization>>,

    /// List of participants involved in the encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<EncounterParticipant>,

    /// The appointment that scheduled this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appointment: Vec<types::Reference<crate::r6::resources::Appointment>>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_service: Vec<types::VirtualServiceDetail>,

    /// The actual start and end time of the encounter
    pub actual_period: Option<types::Period>,

    /// The planned start date/time (or admission date) of the encounter
    pub planned_start_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`planned_start_date`](Self::planned_start_date) (FHIR `_plannedStartDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_plannedStartDate")]
    pub planned_start_date_ext: Option<types::Element>,

    /// The planned end date/time (or discharge date) of the encounter
    pub planned_end_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`planned_end_date`](Self::planned_end_date) (FHIR `_plannedEndDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_plannedEndDate")]
    pub planned_end_date_ext: Option<types::Element>,

    /// Actual quantity of time the encounter lasted (less time absent)
    pub length: Option<types::Duration>,

    /// The list of medical reasons that are expected to be addressed during
    /// the episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<EncounterReason>,

    /// The list of diagnosis relevant to this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<EncounterDiagnosis>,

    /// The set of accounts that may be used for billing for this Encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference<crate::r6::resources::Account>>,

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

/// Details about the stay during which a healthcare service is provided. This
/// does not describe the event of admitting the patient, but rather any
/// information that is relevant from the time of admittance until the time of
/// discharge.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter::EncounterAdmission;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// The list of diagnosis relevant to this encounter.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter::EncounterDiagnosis;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Role that this diagnosis has within the encounter (e.g. admission,
    /// billing, discharge …)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#use: Vec<types::CodeableConcept>,
}

/// List of locations where the patient has been during this encounter.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter::EncounterLocation;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub location: types::Reference<crate::r6::resources::Location>,

    /// planned | active | reserved | completed
    pub status: Option<crate::coded::Coded<crate::r6::codes::EncounterLocationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The physical type of the location (usually the level in the location
    /// hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<types::CodeableConcept>,

    /// Time period during which the patient was present at the location
    pub period: Option<types::Period>,
}

/// The list of people responsible for providing the service.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter::EncounterParticipant;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::encounter::EncounterReason;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
