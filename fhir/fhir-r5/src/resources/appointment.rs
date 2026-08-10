//! Appointment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Appointment
//!
//! Version: 5.0.0
//!
//! Appointment Resource: A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one
/// or more Encounter(s).
///
/// In FHIR R5, an Appointment records the scheduling of a healthcare event and
/// the participants (patients, practitioners, related persons, locations, and
/// devices) that are expected to be involved. It captures the administrative
/// coordination of care rather than the clinical care itself: the proposed or
/// agreed date and time, the service category and service type to be performed,
/// the specialty required, the reason for the visit, and the acceptance status
/// of each participant. Its status moves through a defined lifecycle (proposed,
/// pending, booked, arrived, fulfilled, cancelled, noshow, entered-in-error,
/// checked-in, and waitlist) that drives scheduling workflows. An Appointment
/// may fill one or more Slots exposed by a Schedule, may request a specific
/// period when an exact time is not yet fixed, and may define a recurrence
/// template so that a repeating series of visits is generated from a single
/// booking. When a booked Appointment actually takes place it typically gives
/// rise to one or more Encounters that document the care delivered, and it can
/// also carry virtual service connection details for telehealth visits.
///
/// # Related resources
///
/// See also [`Patient`](crate::r5::resources::patient::Patient) and
/// [`Practitioner`](crate::r5::resources::practitioner::Practitioner) as typical
/// participants, [`Slot`](crate::r5::resources::slot::Slot) and
/// [`Schedule`](crate::r5::resources::schedule::Schedule) for the availability
/// this booking consumes, and
/// [`Encounter`](crate::r5::resources::encounter::Encounter) for the clinical
/// event that may result. Coded fields use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and links to other
/// resources use [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::appointment::Appointment;
///
/// let value = Appointment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Appointment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Appointment {
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

    /// Lifecycle state of the appointment that drives scheduling workflow: proposed | pending | booked | arrived | fulfilled | cancelled | noshow | entered-in-error | checked-in | waitlist
    pub status: crate::r5::coded::Coded<crate::r5::codes::Appointmentstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The coded reason for the appointment being cancelled
    pub cancellation_reason: Option<types::CodeableConcept>,

    /// Classification when becoming an encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<types::CodeableConcept>,

    /// A broad categorization of the service that is to be performed during this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_category: Vec<types::CodeableConcept>,

    /// The specific service that is to be performed during this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// The style of appointment or patient that has been booked in the slot (not service type)
    pub appointment_type: Option<types::CodeableConcept>,

    /// Reason this appointment is scheduled
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Used to make informed decisions if needing to re-prioritize
    pub priority: Option<types::CodeableConcept>,

    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Appointment replaced by this Appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference<crate::r5::resources::Appointment>>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_service: Vec<types::VirtualServiceDetail>,

    /// Additional information to support the appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// The previous appointment in a series
    pub previous_appointment: Option<types::Reference<crate::r5::resources::Appointment>>,

    /// The originating appointment in a recurring set of appointments
    pub originating_appointment: Option<types::Reference<crate::r5::resources::Appointment>>,

    /// Date and time when the appointment is scheduled to begin, once a fixed time has been set
    pub start: Option<types::Instant>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`).
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// Date and time when the appointment is scheduled to conclude, once a fixed time has been set
    pub end: Option<types::Instant>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`).
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Can be less than start/end (e.g. estimate)
    pub minutes_duration: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`minutes_duration`](Self::minutes_duration) (FHIR `_minutesDuration`).
    #[serde(rename = "_minutesDuration")]
    pub minutes_duration_ext: Option<types::Element>,

    /// Potential date/time interval(s) requested to allocate the appointment within
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_period: Vec<types::Period>,

    /// The Slot resource(s) that this appointment is filling, drawn from a Schedule's published availability
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slot: Vec<types::Reference<crate::r5::resources::Slot>>,

    /// The set of accounts that may be used for billing for this Appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference<crate::r5::resources::Account>>,

    /// The date that this appointment was initially created
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// When the appointment was cancelled
    pub cancellation_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`cancellation_date`](Self::cancellation_date) (FHIR `_cancellationDate`).
    #[serde(rename = "_cancellationDate")]
    pub cancellation_date_ext: Option<types::Element>,

    /// Additional comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Detailed information and instructions for the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patient_instruction: Vec<types::CodeableReference>,

    /// The request this appointment is allocated to assess
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The patient or group of patients that this appointment primarily concerns
    pub subject: Option<types::Reference>,

    /// The actors expected to attend, each with a role, required flag, and acceptance status
    pub participant: vec1::Vec1<AppointmentParticipant>,

    /// The sequence number in the recurrence
    pub recurrence_id: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`recurrence_id`](Self::recurrence_id) (FHIR `_recurrenceId`).
    #[serde(rename = "_recurrenceId")]
    pub recurrence_id_ext: Option<types::Element>,

    /// Indicates that this appointment varies from a recurrence pattern
    pub occurrence_changed: Option<types::Boolean>,
    /// Primitive extension sibling for [`occurrence_changed`](Self::occurrence_changed) (FHIR `_occurrenceChanged`).
    #[serde(rename = "_occurrenceChanged")]
    pub occurrence_changed_ext: Option<types::Element>,

    /// Details of the recurrence pattern/template used to generate occurrences
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurrence_template: Vec<AppointmentRecurrenceTemplate>,
}

/// Participants involved in appointment.
///
/// Lists an actor (individual, device, location, or service) expected to take
/// part in the appointment, along with their role, participation period, whether
/// they are required, and their acceptance status.
/// # Examples
///
/// ```
/// use fhir::r5::resources::appointment::AppointmentParticipant;
/// use fhir::r5::types;
///
/// let value = AppointmentParticipant {
///     required: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `required` is the name this serializes to on the wire.
/// assert_eq!(json["required"], ::serde_json::json!(true));
///
/// let back: AppointmentParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Role of participant in the appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Participation period of the actor
    pub period: Option<types::Period>,

    /// The individual, device, location, or service participating in the appointment
    pub actor: Option<types::Reference>,

    /// The participant is required to attend (optional when false)
    pub required: Option<types::Boolean>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`).
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// accepted | declined | tentative | needs-action
    pub status: crate::r5::coded::Coded<crate::r5::codes::Participationstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,
}

/// Details of the recurrence pattern/template used to generate occurrences.
///
/// Describes how a recurring set of appointments repeats: the timezone,
/// recurrence type, end conditions, and the weekly, monthly, or yearly template
/// that governs occurrence generation, plus any excluded dates or recurrence ids.
/// # Examples
///
/// ```
/// use fhir::r5::resources::appointment::AppointmentRecurrenceTemplate;
/// use fhir::r5::types;
///
/// let value = AppointmentRecurrenceTemplate {
///     last_occurrence_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lastOccurrenceDate` is the name this serializes to on the wire.
/// assert_eq!(json["lastOccurrenceDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: AppointmentRecurrenceTemplate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The timezone of the occurrences
    pub timezone: Option<types::CodeableConcept>,

    /// The frequency of the recurrence
    pub recurrence_type: types::CodeableConcept,

    /// The date when the recurrence should end
    pub last_occurrence_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_occurrence_date`](Self::last_occurrence_date) (FHIR `_lastOccurrenceDate`).
    #[serde(rename = "_lastOccurrenceDate")]
    pub last_occurrence_date_ext: Option<types::Element>,

    /// The number of planned occurrences
    pub occurrence_count: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`occurrence_count`](Self::occurrence_count) (FHIR `_occurrenceCount`).
    #[serde(rename = "_occurrenceCount")]
    pub occurrence_count_ext: Option<types::Element>,

    /// Specific dates for a recurring set of appointments (no template)
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub occurrence_date: ::fhir_core::PrimVec<types::Date>,
    /// Primitive extension sibling for [`occurrence_date`](Self::occurrence_date) (FHIR `_occurrenceDate`).
    #[serde(rename = "_occurrenceDate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub occurrence_date_ext: Vec<Option<types::Element>>,

    /// Information about weekly recurring appointments
    pub weekly_template: Option<AppointmentRecurrenceTemplateWeeklyTemplate>,

    /// Information about monthly recurring appointments
    pub monthly_template: Option<AppointmentRecurrenceTemplateMonthlyTemplate>,

    /// Information about yearly recurring appointments
    pub yearly_template: Option<AppointmentRecurrenceTemplateYearlyTemplate>,

    /// Any dates that should be excluded from the series
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub excluding_date: ::fhir_core::PrimVec<types::Date>,
    /// Primitive extension sibling for [`excluding_date`](Self::excluding_date) (FHIR `_excludingDate`).
    #[serde(rename = "_excludingDate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluding_date_ext: Vec<Option<types::Element>>,

    /// Any recurrence IDs that should be excluded from the recurrence
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub excluding_recurrence_id: ::fhir_core::PrimVec<types::PositiveInt>,
    /// Primitive extension sibling for [`excluding_recurrence_id`](Self::excluding_recurrence_id) (FHIR `_excludingRecurrenceId`).
    #[serde(rename = "_excludingRecurrenceId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluding_recurrence_id_ext: Vec<Option<types::Element>>,
}

/// Information about weekly recurring appointments.
///
/// Specifies which days of the week the appointment recurs on and how many
/// weeks apart each recurrence is spaced.
/// # Examples
///
/// ```
/// use fhir::r5::resources::appointment::AppointmentRecurrenceTemplateWeeklyTemplate;
/// use fhir::r5::types;
///
/// let value = AppointmentRecurrenceTemplateWeeklyTemplate {
///     week_interval: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `weekInterval` is the name this serializes to on the wire.
/// assert_eq!(json["weekInterval"], ::serde_json::json!(1));
///
/// let back: AppointmentRecurrenceTemplateWeeklyTemplate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateWeeklyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Recurs on Mondays
    pub monday: Option<types::Boolean>,
    /// Primitive extension sibling for [`monday`](Self::monday) (FHIR `_monday`).
    #[serde(rename = "_monday")]
    pub monday_ext: Option<types::Element>,

    /// Recurs on Tuesday
    pub tuesday: Option<types::Boolean>,
    /// Primitive extension sibling for [`tuesday`](Self::tuesday) (FHIR `_tuesday`).
    #[serde(rename = "_tuesday")]
    pub tuesday_ext: Option<types::Element>,

    /// Recurs on Wednesday
    pub wednesday: Option<types::Boolean>,
    /// Primitive extension sibling for [`wednesday`](Self::wednesday) (FHIR `_wednesday`).
    #[serde(rename = "_wednesday")]
    pub wednesday_ext: Option<types::Element>,

    /// Recurs on Thursday
    pub thursday: Option<types::Boolean>,
    /// Primitive extension sibling for [`thursday`](Self::thursday) (FHIR `_thursday`).
    #[serde(rename = "_thursday")]
    pub thursday_ext: Option<types::Element>,

    /// Recurs on Friday
    pub friday: Option<types::Boolean>,
    /// Primitive extension sibling for [`friday`](Self::friday) (FHIR `_friday`).
    #[serde(rename = "_friday")]
    pub friday_ext: Option<types::Element>,

    /// Recurs on Saturday
    pub saturday: Option<types::Boolean>,
    /// Primitive extension sibling for [`saturday`](Self::saturday) (FHIR `_saturday`).
    #[serde(rename = "_saturday")]
    pub saturday_ext: Option<types::Element>,

    /// Recurs on Sunday
    pub sunday: Option<types::Boolean>,
    /// Primitive extension sibling for [`sunday`](Self::sunday) (FHIR `_sunday`).
    #[serde(rename = "_sunday")]
    pub sunday_ext: Option<types::Element>,

    /// Recurs every nth week
    pub week_interval: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`week_interval`](Self::week_interval) (FHIR `_weekInterval`).
    #[serde(rename = "_weekInterval")]
    pub week_interval_ext: Option<types::Element>,
}

/// Information about monthly recurring appointments.
///
/// Specifies whether the appointment recurs on a particular day of the month or
/// on a particular weekday within a particular week of the month, and how many
/// months apart each recurrence is spaced.
/// # Examples
///
/// ```
/// use fhir::r5::resources::appointment::AppointmentRecurrenceTemplateMonthlyTemplate;
/// use fhir::r5::types;
///
/// let value = AppointmentRecurrenceTemplateMonthlyTemplate {
///     day_of_month: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dayOfMonth` is the name this serializes to on the wire.
/// assert_eq!(json["dayOfMonth"], ::serde_json::json!(1));
///
/// let back: AppointmentRecurrenceTemplateMonthlyTemplate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateMonthlyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Recurs on a specific day of the month
    pub day_of_month: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`day_of_month`](Self::day_of_month) (FHIR `_dayOfMonth`).
    #[serde(rename = "_dayOfMonth")]
    pub day_of_month_ext: Option<types::Element>,

    /// Indicates which week of the month the appointment should occur
    pub nth_week_of_month: Option<types::Coding>,

    /// Indicates which day of the week the appointment should occur
    pub day_of_week: Option<types::Coding>,

    /// Recurs every nth month
    pub month_interval: types::PositiveInt,
    /// Primitive extension sibling for [`month_interval`](Self::month_interval) (FHIR `_monthInterval`).
    #[serde(rename = "_monthInterval")]
    pub month_interval_ext: Option<types::Element>,
}

/// Information about yearly recurring appointments.
///
/// Specifies how many years apart each recurrence of the appointment is spaced.
/// # Examples
///
/// ```
/// use fhir::r5::resources::appointment::AppointmentRecurrenceTemplateYearlyTemplate;
/// use fhir::r5::types;
///
/// let value = AppointmentRecurrenceTemplateYearlyTemplate {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AppointmentRecurrenceTemplateYearlyTemplate = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateYearlyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Recurs every nth year
    pub year_interval: types::PositiveInt,
    /// Primitive extension sibling for [`year_interval`](Self::year_interval) (FHIR `_yearInterval`).
    #[serde(rename = "_yearInterval")]
    pub year_interval_ext: Option<types::Element>,
}
