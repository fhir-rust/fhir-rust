//! EncounterHistory
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EncounterHistory
//!
//! Version: 5.0.0
//!
//! EncounterHistory Resource: A record of significant events/milestones key data throughout the history of an Encounter
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of significant events/milestones and key data throughout the
/// history of an Encounter.
///
/// EncounterHistory captures a snapshot of an Encounter's status and related
/// values at a particular point in time, allowing systems to preserve the
/// evolution of an encounter without repeatedly updating and versioning the
/// full Encounter resource. Each instance references the associated Encounter
/// and records the status, class, timing, and location details as they applied
/// at that moment. In FHIR R5 this supports auditing and reconstructing the
/// progression of a patient's encounter over time.
///
/// Because an Encounter can change status, class, or location many times over
/// its lifecycle (for example moving from `planned` to `in-progress` to
/// `completed`, or transferring between wards), systems can create a new
/// EncounterHistory record each time such a change occurs rather than
/// overwriting the prior state on the Encounter itself. This yields an
/// append-only audit trail that supports retrospective review, billing
/// reconciliation, and reporting on how long a patient spent in each state or
/// location during their stay.
///
/// # Related resources
///
/// - [`Encounter`](crate::r5::resources::encounter::Encounter): the resource
///   whose evolving status this record captures a snapshot of.
/// - [`Patient`](crate::r5::resources::patient::Patient): typically the
///   subject of the referenced Encounter.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept): used for the
///   `class`, `type`, and `service_type` classifications.
/// - `Location`: referenced from each
///   [`EncounterHistoryLocation`] entry to describe where the patient was.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter_history::EncounterHistory;
/// use fhir::r5::types;
///
/// let value = EncounterHistory {
///     planned_start_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `plannedStartDate` is the name this serializes to on the wire.
/// assert_eq!(json["plannedStartDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: EncounterHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterHistory {
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

    /// A reference to the Encounter whose status/class/location this record snapshots
    pub encounter: Option<types::Reference>,

    /// Identifier(s) by which this encounter is known
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The status of the encounter at this point in its history: planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::EncounterStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of the patient encounter at this point in time (e.g. inpatient, outpatient, ambulatory, emergency)
    pub class: types::CodeableConcept,

    /// Specific type of encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Specific type of service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// The patient or group related to this encounter
    pub subject: Option<types::Reference>,

    /// The current status of the subject in relation to the Encounter
    pub subject_status: Option<types::CodeableConcept>,

    /// The actual start and end time associated with this set of values associated with the encounter
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

    /// Location of the patient at this point in the encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<EncounterHistoryLocation>,
}

/// Location of the patient at this point in the encounter.
///
/// Records where the patient was located during this segment of the encounter,
/// including a reference to the Location resource and an optional description of
/// the physical type of that location (such as bed, room, ward, or virtual).
/// # Examples
///
/// ```
/// use fhir::r5::resources::encounter_history::EncounterHistoryLocation;
/// use fhir::r5::types;
///
/// let value = EncounterHistoryLocation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EncounterHistoryLocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EncounterHistoryLocation {
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

    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EncounterHistory;

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
