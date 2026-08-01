//! AppointmentResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/AppointmentResponse
//!
//! Version: 4.0.1
//!
//! A reply to an appointment request for a patient and/or practitioner(s),
//! such as a confirmation or rejection
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A reply to an appointment request for a patient and/or practitioner(s),
/// such as a confirmation or rejection.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::appointment_response::AppointmentResponse;
/// use fhir::r4::types;
///
/// let value = AppointmentResponse {
///     start: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `start` is the name this serializes to on the wire.
/// assert_eq!(json["start"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: AppointmentResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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

    /// Person, Location, HealthcareService, or Device
    pub actor: Option<types::Reference>,

    /// accepted | declined | tentative | needs-action
    pub participant_status: crate::coded::Coded<crate::r4::codes::Participationstatus>,
    /// Primitive extension sibling for [`participant_status`](Self::participant_status) (FHIR `_participantStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_participantStatus")]
    pub participant_status_ext: Option<types::Element>,

    /// Additional comments
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
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
