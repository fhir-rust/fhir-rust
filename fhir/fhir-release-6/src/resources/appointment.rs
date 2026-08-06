//! Appointment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Appointment
//!
//! Version: 6.0.0-ballot3
//!
//! A booking of a healthcare event among patient(s), practitioner(s), related
//! person(s) and/or device(s) for a specific date/time. This may result in one
//! or more Encounter(s)
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one
/// or more Encounter(s).
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::appointment::Appointment;
///
/// let value = Appointment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Appointment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Appointment {
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

    /// External Ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// proposed | pending | booked | arrived | fulfilled | cancelled | noshow
    /// | entered-in-error | checked-in | waitlist
    pub status: crate::coded::Coded<crate::r6::codes::Appointmentstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The coded reason for the appointment being cancelled
    pub cancellation_reason: Option<types::CodeableConcept>,

    /// Classification when becoming an encounter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<types::CodeableConcept>,

    /// A broad categorization of the service that is to be performed during
    /// this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_category: Vec<types::CodeableConcept>,

    /// The specific service that is to be performed during this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableReference>,

    /// The specialty of a practitioner that would be required to perform the
    /// service requested in this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// The style of appointment or patient that has been booked in the slot
    /// (not service type)
    pub appointment_type: Option<types::CodeableConcept>,

    /// Reason this appointment is scheduled
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Used to make informed decisions if needing to re-prioritize
    pub priority: Option<types::CodeableConcept>,

    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Appointment replaced by this Appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_service: Vec<types::VirtualServiceDetail>,

    /// Additional information to support the appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// The previous appointment in a series
    pub previous_appointment: Option<types::Reference>,

    /// The originating appointment in a recurring set of appointments
    pub originating_appointment: Option<types::Reference>,

    /// When appointment is to take place
    pub start: Option<types::Instant>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// When appointment is to conclude
    pub end: Option<types::Instant>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Can be less than start/end (e.g. estimate)
    pub minutes_duration: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`minutes_duration`](Self::minutes_duration) (FHIR `_minutesDuration`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_minutesDuration")]
    pub minutes_duration_ext: Option<types::Element>,

    /// Potential date/time interval(s) requested to allocate the appointment
    /// within
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_period: Vec<types::Period>,

    /// The slots that this appointment is filling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slot: Vec<types::Reference>,

    /// The set of accounts that may be used for billing for this Appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference>,

    /// The date that this appointment was initially created
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// When the appointment was cancelled
    pub cancellation_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`cancellation_date`](Self::cancellation_date) (FHIR `_cancellationDate`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// The patient or group associated with the appointment
    pub subject: Option<types::Reference>,

    /// Participants involved in appointment
    pub participant: ::vec1::Vec1<AppointmentParticipant>,

    /// The sequence number in the recurrence
    pub recurrence_id: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`recurrence_id`](Self::recurrence_id) (FHIR `_recurrenceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recurrenceId")]
    pub recurrence_id_ext: Option<types::Element>,

    /// Indicates that this appointment varies from a recurrence pattern
    pub occurrence_changed: Option<types::Boolean>,
    /// Primitive extension sibling for [`occurrence_changed`](Self::occurrence_changed) (FHIR `_occurrenceChanged`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_occurrenceChanged")]
    pub occurrence_changed_ext: Option<types::Element>,

    /// Details of the recurrence pattern/template used to generate occurrences
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurrence_template: Vec<AppointmentRecurrenceTemplate>,
}

/// List of participants involved in the appointment.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::appointment::AppointmentParticipant;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// The individual, device, location, or service participating in the
    /// appointment
    pub actor: Option<types::Reference>,

    /// The participant is required to attend (optional when false)
    pub required: Option<types::Boolean>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// accepted | declined | tentative | needs-action
    pub status: crate::coded::Coded<crate::r6::codes::Participationstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,
}

/// The details of the recurrence pattern or template that is used to generate
/// recurring appointments.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::appointment::AppointmentRecurrenceTemplate;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`last_occurrence_date`](Self::last_occurrence_date) (FHIR `_lastOccurrenceDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastOccurrenceDate")]
    pub last_occurrence_date_ext: Option<types::Element>,

    /// The number of planned occurrences
    pub occurrence_count: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`occurrence_count`](Self::occurrence_count) (FHIR `_occurrenceCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_occurrenceCount")]
    pub occurrence_count_ext: Option<types::Element>,

    /// Specific dates for a recurring set of appointments (no template)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub occurrence_date: Vec<types::Date>,
    /// Primitive extension sibling for [`occurrence_date`](Self::occurrence_date) (FHIR `_occurrenceDate`):
    /// carries `id` and/or `extension` for the primitive value.
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluding_date: Vec<types::Date>,
    /// Primitive extension sibling for [`excluding_date`](Self::excluding_date) (FHIR `_excludingDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_excludingDate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluding_date_ext: Vec<Option<types::Element>>,

    /// Any recurrence IDs that should be excluded from the recurrence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluding_recurrence_id: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`excluding_recurrence_id`](Self::excluding_recurrence_id) (FHIR `_excludingRecurrenceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_excludingRecurrenceId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluding_recurrence_id_ext: Vec<Option<types::Element>>,
}

/// Information about monthly recurring appointments.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::appointment::AppointmentRecurrenceTemplateMonthlyTemplate;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`day_of_month`](Self::day_of_month) (FHIR `_dayOfMonth`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dayOfMonth")]
    pub day_of_month_ext: Option<types::Element>,

    /// Indicates which week of the month the appointment should occur
    pub nth_week_of_month: Option<types::Coding>,

    /// Indicates which day of the week the appointment should occur
    pub day_of_week: Option<types::Coding>,

    /// Recurs every nth month
    pub month_interval: types::PositiveInt,
    /// Primitive extension sibling for [`month_interval`](Self::month_interval) (FHIR `_monthInterval`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_monthInterval")]
    pub month_interval_ext: Option<types::Element>,
}

/// Information about weekly recurring appointments.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::appointment::AppointmentRecurrenceTemplateWeeklyTemplate;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`monday`](Self::monday) (FHIR `_monday`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_monday")]
    pub monday_ext: Option<types::Element>,

    /// Recurs on Tuesday
    pub tuesday: Option<types::Boolean>,
    /// Primitive extension sibling for [`tuesday`](Self::tuesday) (FHIR `_tuesday`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_tuesday")]
    pub tuesday_ext: Option<types::Element>,

    /// Recurs on Wednesday
    pub wednesday: Option<types::Boolean>,
    /// Primitive extension sibling for [`wednesday`](Self::wednesday) (FHIR `_wednesday`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_wednesday")]
    pub wednesday_ext: Option<types::Element>,

    /// Recurs on Thursday
    pub thursday: Option<types::Boolean>,
    /// Primitive extension sibling for [`thursday`](Self::thursday) (FHIR `_thursday`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_thursday")]
    pub thursday_ext: Option<types::Element>,

    /// Recurs on Friday
    pub friday: Option<types::Boolean>,
    /// Primitive extension sibling for [`friday`](Self::friday) (FHIR `_friday`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_friday")]
    pub friday_ext: Option<types::Element>,

    /// Recurs on Saturday
    pub saturday: Option<types::Boolean>,
    /// Primitive extension sibling for [`saturday`](Self::saturday) (FHIR `_saturday`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_saturday")]
    pub saturday_ext: Option<types::Element>,

    /// Recurs on Sunday
    pub sunday: Option<types::Boolean>,
    /// Primitive extension sibling for [`sunday`](Self::sunday) (FHIR `_sunday`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sunday")]
    pub sunday_ext: Option<types::Element>,

    /// Recurs every nth week
    pub week_interval: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`week_interval`](Self::week_interval) (FHIR `_weekInterval`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_weekInterval")]
    pub week_interval_ext: Option<types::Element>,
}

/// Information about yearly recurring appointments.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::appointment::AppointmentRecurrenceTemplateYearlyTemplate;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`year_interval`](Self::year_interval) (FHIR `_yearInterval`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_yearInterval")]
    pub year_interval_ext: Option<types::Element>,
}
