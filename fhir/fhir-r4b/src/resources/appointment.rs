//! Appointment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Appointment
//!
//! Version: 4.3.0
//!
//! A booking of a healthcare event among patient(s), practitioner(s), related
//! person(s) and/or device(s) for a specific date/time. This may result in one
//! or more Encounter(s)
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one
/// or more Encounter(s).
///
/// # Examples
///
/// ```ignore
/// use fhir::r4b::resources::appointment::Appointment;
///
/// let value = Appointment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Appointment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
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
    pub contained: Vec<crate::r4b::resources::Resource>,

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
    pub status: crate::coded::Coded<crate::r4b::codes::Appointmentstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The coded reason for the appointment being cancelled
    pub cancelation_reason: Option<types::CodeableConcept>,

    /// A broad categorization of the service that is to be performed during
    /// this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_category: Vec<types::CodeableConcept>,

    /// The specific service that is to be performed during this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<types::CodeableConcept>,

    /// The specialty of a practitioner that would be required to perform the
    /// service requested in this appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// The style of appointment or patient that has been booked in the slot
    /// (not service type)
    pub appointment_type: Option<types::CodeableConcept>,

    /// Coded reason this appointment is scheduled
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Reason the appointment is to take place (resource)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Used to make informed decisions if needing to re-prioritize
    pub priority: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Additional information to support the appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

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

    /// The slots that this appointment is filling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slot: Vec<types::Reference<crate::r4b::resources::Slot>>,

    /// The date that this appointment was initially created
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Additional comments
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Detailed information and instructions for the patient
    pub patient_instruction: Option<types::String>,
    /// Primitive extension sibling for [`patient_instruction`](Self::patient_instruction) (FHIR `_patientInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_patientInstruction")]
    pub patient_instruction_ext: Option<types::Element>,

    /// The service request this appointment is allocated to assess
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference<crate::r4b::resources::ServiceRequest>>,

    /// Participants involved in appointment
    pub participant: ::vec1::Vec1<AppointmentParticipant>,

    /// Potential date/time interval(s) requested to allocate the appointment
    /// within
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_period: Vec<types::Period>,
}

/// List of participants involved in the appointment.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::appointment::AppointmentParticipant;
/// use fhir::r4b::types;
///
/// let value = AppointmentParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: AppointmentParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
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

    /// Person, Location/HealthcareService or Device
    pub actor: Option<types::Reference>,

    /// required | optional | information-only
    pub required: Option<crate::coded::Coded<crate::r4b::codes::Participantrequired>>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// accepted | declined | tentative | needs-action
    pub status: crate::coded::Coded<crate::r4b::codes::Participationstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Participation period of the actor
    pub period: Option<types::Period>,
}
