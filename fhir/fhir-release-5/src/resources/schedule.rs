//! Schedule
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Schedule
//!
//! Version: 5.0.0
//!
//! Schedule Resource: A container for slots of time that may be available for booking appointments.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A container for slots of time that may be available for booking appointments.
///
/// In FHIR R5, a Schedule is an administrative resource that describes the
/// general availability of one or more actors, such as a practitioner, a
/// location, a piece of equipment, or a patient, over a defined period of time.
/// It groups together the bookable time belonging to those actors and, together
/// with the planning horizon, bounds the window during which appointments may be
/// offered. A Schedule does not itself hold individual bookable intervals;
/// instead it acts as the organizing container that the finer-grained
/// [`Slot`](crate::r5::resources::slot::Slot) resources reference, and each Slot
/// then carries the concrete free or busy status for a specific interval.
/// Scheduling systems typically use the service category, service type, and
/// specialty to help clients find the right Schedule when searching for
/// availability, before an [`Appointment`](crate::r5::resources::appointment::Appointment)
/// is created against one of its slots.
///
/// # Related resources
///
/// See also [`Slot`](crate::r5::resources::slot::Slot),
/// [`Appointment`](crate::r5::resources::appointment::Appointment),
/// [`Practitioner`](crate::r5::resources::practitioner::Practitioner),
/// [`Location`](crate::r5::resources::location::Location), and
/// [`Patient`](crate::r5::resources::patient::Patient), any of which may appear
/// as an actor for a Schedule.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::schedule::Schedule;
///
/// let value = Schedule::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Schedule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
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

    /// External Ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this schedule is in active use; inactive schedules should not be offered for booking
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// High-level category of the service or resource this schedule provides, such as general practice or dental
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_category: Vec<types::CodeableConcept>,

    /// Specific type of service that may be booked against this schedule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// Type of specialty needed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// Human-readable label
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Resource(s), such as a practitioner, location, device, or patient, whose availability this schedule describes
    pub actor: vec1::Vec1<types::Reference>,

    /// Period of time covered by the schedule, bounding when slots and appointments may be offered
    pub planning_horizon: Option<types::Period>,

    /// Comments on availability
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}
