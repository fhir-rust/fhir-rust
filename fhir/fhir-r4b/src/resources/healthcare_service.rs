//! HealthcareService
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
//!
//! Version: 4.3.0
//!
//! The details of a healthcare service available at a location
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The details of a healthcare service available at a location.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::healthcare_service::HealthcareService;
/// use fhir::r4b::types;
///
/// let value = HealthcareService {
///     extra_details: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `extraDetails` is the name this serializes to on the wire.
/// assert_eq!(json["extraDetails"], ::serde_json::json!("# Heading"));
///
/// let back: HealthcareService = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct HealthcareService {
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

    /// External identifiers for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this HealthcareService record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Organization that provides this service
    pub provided_by: Option<types::Reference<crate::r4b::resources::Organization>>,

    /// Broad category of service being performed or delivered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Type of service that may be delivered or performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Specialties handled by the HealthcareService
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// Location(s) where service may be provided
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference<crate::r4b::resources::Location>>,

    /// Description of service as presented to a consumer while searching
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Additional description and/or any specific issues not covered elsewhere
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Extra details about the service that can't be placed in the other
    /// fields
    pub extra_details: Option<types::Markdown>,
    /// Primitive extension sibling for [`extra_details`](Self::extra_details) (FHIR `_extraDetails`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_extraDetails")]
    pub extra_details_ext: Option<types::Element>,

    /// Facilitates quick identification of the service
    pub photo: Option<types::Attachment>,

    /// Contacts related to the healthcare service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Location(s) service is intended for/available to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference<crate::r4b::resources::Location>>,

    /// Conditions under which service is available/offered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_provision_code: Vec<types::CodeableConcept>,

    /// Specific eligibility requirements required to use the service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub eligibility: Vec<HealthcareServiceEligibility>,

    /// Programs that this service is applicable to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program: Vec<types::CodeableConcept>,

    /// Collection of characteristics (attributes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<types::CodeableConcept>,

    /// The language that this service is offered in
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<types::CodeableConcept>,

    /// Ways that the service accepts referrals
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referral_method: Vec<types::CodeableConcept>,

    /// If an appointment is required for access to this service
    pub appointment_required: Option<types::Boolean>,
    /// Primitive extension sibling for [`appointment_required`](Self::appointment_required) (FHIR `_appointmentRequired`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_appointmentRequired")]
    pub appointment_required_ext: Option<types::Element>,

    /// Times the Service Site is available
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_time: Vec<HealthcareServiceAvailableTime>,

    /// Not available during this time due to provided reason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_available: Vec<HealthcareServiceNotAvailable>,

    /// Description of availability exceptions
    pub availability_exceptions: Option<types::String>,
    /// Primitive extension sibling for [`availability_exceptions`](Self::availability_exceptions) (FHIR `_availabilityExceptions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availabilityExceptions")]
    pub availability_exceptions_ext: Option<types::Element>,

    /// Technical endpoints providing access to electronic services operated
    /// for the healthcare service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r4b::resources::Endpoint>>,
}

/// A collection of times that the Service Site is available.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::healthcare_service::HealthcareServiceAvailableTime;
/// use fhir::r4b::types;
///
/// let value = HealthcareServiceAvailableTime {
///     all_day: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `allDay` is the name this serializes to on the wire.
/// assert_eq!(json["allDay"], ::serde_json::json!(true));
///
/// let back: HealthcareServiceAvailableTime = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct HealthcareServiceAvailableTime {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub days_of_week: ::fhir_core::PrimVec<crate::coded::Coded<crate::r4b::codes::DaysOfWeek>>,
    /// Primitive extension sibling for [`days_of_week`](Self::days_of_week) (FHIR `_daysOfWeek`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_daysOfWeek")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_week_ext: Vec<Option<types::Element>>,

    /// Always available? e.g. 24 hour service
    pub all_day: Option<types::Boolean>,
    /// Primitive extension sibling for [`all_day`](Self::all_day) (FHIR `_allDay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allDay")]
    pub all_day_ext: Option<types::Element>,

    /// Opening time of day (ignored if allDay = true)
    pub available_start_time: Option<types::Time>,
    /// Primitive extension sibling for [`available_start_time`](Self::available_start_time) (FHIR `_availableStartTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availableStartTime")]
    pub available_start_time_ext: Option<types::Element>,

    /// Closing time of day (ignored if allDay = true)
    pub available_end_time: Option<types::Time>,
    /// Primitive extension sibling for [`available_end_time`](Self::available_end_time) (FHIR `_availableEndTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availableEndTime")]
    pub available_end_time_ext: Option<types::Element>,
}

/// Does this service have specific eligibility requirements that need to be
/// met in order to use the service?
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::healthcare_service::HealthcareServiceEligibility;
/// use fhir::r4b::types;
///
/// let value = HealthcareServiceEligibility {
///     comment: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comment` is the name this serializes to on the wire.
/// assert_eq!(json["comment"], ::serde_json::json!("# Heading"));
///
/// let back: HealthcareServiceEligibility = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct HealthcareServiceEligibility {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coded value for the eligibility
    pub code: Option<types::CodeableConcept>,

    /// Describes the eligibility conditions for the service
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// The HealthcareService is not available during this period of time due to
/// the provided reason.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::healthcare_service::HealthcareServiceNotAvailable;
/// use fhir::r4b::types;
///
/// let value = HealthcareServiceNotAvailable {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: HealthcareServiceNotAvailable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct HealthcareServiceNotAvailable {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reason presented to the user explaining why time not available
    pub description: types::String,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Service not available from this date
    pub during: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = HealthcareService;

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
