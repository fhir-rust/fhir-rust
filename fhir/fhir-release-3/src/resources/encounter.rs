//! Encounter
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Encounter
//!
//!
//!
//! An interaction during which services are provided to the patient
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Encounter Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::encounter::Encounter;
/// use fhir::r3::types;
///
/// let value = Encounter {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: Encounter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Encounter {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier(s) by which this encounter is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// planned | arrived | triaged | in-progress | onleave | finished |
    /// cancelled +
    pub status: crate::coded::Coded<crate::r3::codes::EncounterStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// List of past encounter statuses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_history: Vec<EncounterStatusHistory>,

    /// inpatient | outpatient | ambulatory | emergency +
    pub class: Option<types::Coding>,

    /// List of past encounter classes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class_history: Vec<EncounterClassHistory>,

    /// Specific type of encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Indicates the urgency of the encounter
    pub priority: Option<types::CodeableConcept>,

    /// The patient ro group present at the encounter
    pub subject: Option<types::Reference>,

    /// Episode(s) of care that this encounter should be recorded against
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub episode_of_care: Vec<types::Reference>,

    /// The ReferralRequest that initiated this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub incoming_referral: Vec<types::Reference>,

    /// List of participants involved in the encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<EncounterParticipant>,

    /// The appointment that scheduled this encounter
    pub appointment: Option<types::Reference>,

    /// The start and end time of the encounter
    pub period: Option<types::Period>,

    /// Quantity of time the encounter lasted (less time absent)
    pub length: Option<types::Duration>,

    /// Reason the encounter takes place (code)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// The list of diagnosis relevant to this encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<EncounterDiagnosis>,

    /// The set of accounts that may be used for billing for this Encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference>,

    /// Details about the admission to a healthcare service
    pub hospitalization: Option<EncounterHospitalization>,

    /// List of locations where the patient has been
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<EncounterLocation>,

    /// The custodian organization of this Encounter record
    pub service_provider: Option<types::Reference>,

    /// Another Encounter this encounter is part of
    pub part_of: Option<types::Reference>,
}

/// The class history permits the tracking of the encounters transitions
/// without needing to go through the resource history. This would be used for
/// a case where an admission starts of as an emergency encounter, then
/// transisions into an inpatient scenario. Doing this and not restarting a new
/// encounter ensures that any lab/diagnostic results can more easily follow
/// the patient and not require re-processing and not get lost or cancelled
/// during a kindof discharge from emergency to inpatient.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::encounter::EncounterClassHistory;
/// use fhir::r3::types;
///
/// let value = EncounterClassHistory {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterClassHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EncounterClassHistory {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// inpatient | outpatient | ambulatory | emergency +
    pub class: types::Coding,

    /// The time that the episode was in the specified class
    pub period: types::Period,
}

/// The list of diagnosis relevant to this encounter.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::encounter::EncounterDiagnosis;
/// use fhir::r3::types;
///
/// let value = EncounterDiagnosis {
///     rank: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `rank` is the name this serializes to on the wire.
/// assert_eq!(json["rank"], ::serde_json::json!(1));
///
/// let back: EncounterDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EncounterDiagnosis {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reason the encounter takes place (resource)
    pub condition: types::Reference,

    /// Role that this diagnosis has within the encounter (e.g. admission,
    /// billing, discharge …)
    pub role: Option<types::CodeableConcept>,

    /// Ranking of the diagnosis (for each role type)
    pub rank: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`rank`](Self::rank) (FHIR `_rank`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rank")]
    pub rank_ext: Option<types::Element>,
}

/// Details about the admission to a healthcare service.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::encounter::EncounterHospitalization;
/// use fhir::r3::types;
///
/// let value = EncounterHospitalization {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterHospitalization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EncounterHospitalization {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pre-admission identifier
    pub pre_admission_identifier: Option<types::Identifier>,

    /// The location from which the patient came before admission
    pub origin: Option<types::Reference>,

    /// From where patient was admitted (physician referral, transfer)
    pub admit_source: Option<types::CodeableConcept>,

    /// The type of hospital re-admission that has occurred (if any). If the
    /// value is absent, then this is not identified as a readmission
    pub re_admission: Option<types::CodeableConcept>,

    /// Diet preferences reported by the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diet_preference: Vec<types::CodeableConcept>,

    /// Special courtesies (VIP, board member)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_courtesy: Vec<types::CodeableConcept>,

    /// Wheelchair, translator, stretcher, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_arrangement: Vec<types::CodeableConcept>,

    /// Location to which the patient is discharged
    pub destination: Option<types::Reference>,

    /// Category or kind of location after discharge
    pub discharge_disposition: Option<types::CodeableConcept>,
}

/// List of locations where the patient has been during this encounter.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::encounter::EncounterLocation;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct EncounterLocation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location the encounter takes place
    pub location: types::Reference,

    /// planned | active | reserved | completed
    pub status: Option<crate::coded::Coded<crate::r3::codes::EncounterLocationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Time period during which the patient was present at the location
    pub period: Option<types::Period>,
}

/// The list of people responsible for providing the service.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::encounter::EncounterParticipant;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct EncounterParticipant {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Role of participant in encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Period of time during the encounter that the participant participated
    pub period: Option<types::Period>,

    /// Persons involved in the encounter other than the patient
    pub individual: Option<types::Reference>,
}

/// The status history permits the encounter resource to contain the status
/// history without needing to read through the historical versions of the
/// resource, or even have the server store them.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::encounter::EncounterStatusHistory;
/// use fhir::r3::types;
///
/// let value = EncounterStatusHistory {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterStatusHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EncounterStatusHistory {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// planned | arrived | triaged | in-progress | onleave | finished |
    /// cancelled +
    pub status: crate::coded::Coded<crate::r3::codes::EncounterStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The time that the episode was in the specified status
    pub period: types::Period,
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
