//! AppointmentResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AppointmentResponse
//!
//! Version: 6.0.0-ballot3
//!
//! A reply to an appointment request for a patient and/or practitioner(s),
//! such as a confirmation or rejection
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A reply to an appointment request for a patient and/or practitioner(s),
/// such as a confirmation or rejection.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::appointment_response::AppointmentResponse;
/// use fhir::r6::types;
///
/// let value = AppointmentResponse {
///     proposed_new_time: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `proposedNewTime` is the name this serializes to on the wire.
/// assert_eq!(json["proposedNewTime"], ::serde_json::json!(true));
///
/// let back: AppointmentResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct AppointmentResponse {
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

    /// Appointment this response relates to
    pub appointment: types::Reference,

    /// Indicator for a counter proposal
    pub proposed_new_time: Option<types::Boolean>,
    /// Primitive extension sibling for [`proposed_new_time`](Self::proposed_new_time) (FHIR `_proposedNewTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_proposedNewTime")]
    pub proposed_new_time_ext: Option<types::Element>,

    /// Time from appointment, or requested new start time
    pub start: Option<types::Instant>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// Time from appointment, or requested new end time
    pub end: Option<types::Instant>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Role of participant in the appointment
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant_type: Vec<types::CodeableConcept>,

    /// Person(s), Location, HealthcareService, or Device
    pub actor: Option<types::Reference>,

    /// accepted | declined | tentative | needs-action | entered-in-error
    pub participant_status: types::Code,
    /// Primitive extension sibling for [`participant_status`](Self::participant_status) (FHIR `_participantStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_participantStatus")]
    pub participant_status_ext: Option<types::Element>,

    /// Additional comments
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// This response is for all occurrences in a recurring request
    pub recurring: Option<types::Boolean>,
    /// Primitive extension sibling for [`recurring`](Self::recurring) (FHIR `_recurring`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recurring")]
    pub recurring_ext: Option<types::Element>,

    /// Original date within a recurring request
    pub occurrence_date: Option<types::Date>,
    /// Primitive extension sibling for [`occurrence_date`](Self::occurrence_date) (FHIR `_occurrenceDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_occurrenceDate")]
    pub occurrence_date_ext: Option<types::Element>,

    /// The recurrence ID of the specific recurring request
    pub recurrence_id: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`recurrence_id`](Self::recurrence_id) (FHIR `_recurrenceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recurrenceId")]
    pub recurrence_id_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = AppointmentResponse;

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
