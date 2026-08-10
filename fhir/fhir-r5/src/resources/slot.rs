//! Slot
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Slot
//!
//! Version: 5.0.0
//!
//! Slot Resource: A slot of time on a schedule that may be available for booking appointments.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Slot represents a single, discrete interval of time on a Schedule that may be
/// available for booking appointments. In the FHIR R5 scheduling workflow, a
/// Schedule defines the overall availability of a resource such as a practitioner,
/// device, room, or healthcare service over a planning horizon, and that horizon
/// is subdivided into Slot instances that each carry a precise start and end
/// instant along with a status such as `free`, `busy`, `busy-unavailable`,
/// `busy-tentative`, or `entered-in-error`. Scheduling and patient-access systems
/// query these slots to present bookable openings, and an Appointment is created
/// against one or more free slots, at which point their status is typically
/// updated to reflect the reservation. A slot may further narrow what can be
/// booked by describing the service category, service type, specialty, and
/// appointment type, and it may be flagged as overbooked when demand exceeds the
/// nominal capacity of the interval.
///
/// # Related resources
///
/// A slot is owned by a [`Schedule`](crate::r5::resources::schedule::Schedule) via
/// its `schedule` reference, and it is consumed when creating an
/// [`Appointment`](crate::r5::resources::appointment::Appointment). Its
/// classification fields are typed as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::slot::Slot;
/// use fhir::r5::types;
///
/// let value = Slot {
///     overbooked: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `overbooked` is the name this serializes to on the wire.
/// assert_eq!(json["overbooked"], ::serde_json::json!(true));
///
/// let back: Slot = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
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
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A broad categorization of the service that is to be performed during this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_category: Vec<types::CodeableConcept>,

    /// The type of appointments that can be booked into this slot (ideally this would be an identifiable service - which is at a location, rather than the location itself). If provided then this overrides the value provided on the Schedule resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// The style of appointment or patient that may be booked in the slot (not service type)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appointment_type: Vec<types::CodeableConcept>,

    /// Reference to the Schedule resource that this slot subdivides and reports availability for; this reference is required
    pub schedule: types::Reference<crate::r5::resources::Schedule>,

    /// Availability status of the interval: busy | free | busy-unavailable | busy-tentative | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::Slotstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The instant, including date, time, and timezone offset, at which this slot's interval begins
    pub start: types::Instant,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`).
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// The instant, including date, time, and timezone offset, at which this slot's interval concludes
    pub end: types::Instant,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`).
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// This slot has already been overbooked, appointments are unlikely to be accepted for this time
    pub overbooked: Option<types::Boolean>,
    /// Primitive extension sibling for [`overbooked`](Self::overbooked) (FHIR `_overbooked`).
    #[serde(rename = "_overbooked")]
    pub overbooked_ext: Option<types::Element>,

    /// Comments on the slot to describe any extended information. Such as custom constraints on the slot
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Slot;

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
