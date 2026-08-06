//! Appointment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Appointment
//!
//!
//!
//! A booking of a healthcare event among patient(s), practitioner(s), related
//! person(s) and/or device(s) for a specific date/time. This may result in one
//! or more Encounter(s)
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Appointment Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::appointment::Appointment;
///
/// let value = Appointment::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Appointment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Appointment {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// proposed | pending | booked | arrived | fulfilled | cancelled | noshow
    pub status: crate::coded::Coded<crate::r2::codes::Appointmentstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The type of appointment that is being booked
    pub r#type: Option<types::CodeableConcept>,

    /// Reason this appointment is scheduled
    pub reason: Option<types::CodeableConcept>,

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

    /// If provided, then no schedule and start/end values MUST match slot
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slot: Vec<types::Reference>,

    /// Additional comments
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Participants involved in appointment
    pub participant: ::vec1::Vec1<AppointmentParticipant>,
}

/// List of participants involved in the appointment.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::appointment::AppointmentParticipant;
/// use fhir::r2::types;
///
/// let value = AppointmentParticipant {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: AppointmentParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct AppointmentParticipant {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Role of participant in the appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Person, Location/HealthcareService or Device
    pub actor: Option<types::Reference>,

    /// required | optional | information-only
    pub required: Option<crate::coded::Coded<crate::r2::codes::Participantrequired>>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// accepted | declined | tentative | needs-action
    pub status: crate::coded::Coded<crate::r2::codes::Participationstatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,
}
