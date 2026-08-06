//! HealthcareService
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
//!
//!
//!
//! The details of a healthcare service available at a location
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for HealthcareService Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::healthcare_service::HealthcareService;
/// use fhir::r2::types;
///
/// let value = HealthcareService {
///     service_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `serviceName` is the name this serializes to on the wire.
/// assert_eq!(json["serviceName"], ::serde_json::json!("abc"));
///
/// let back: HealthcareService = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct HealthcareService {
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

    /// External identifiers for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Organization that provides this service
    pub provided_by: Option<types::Reference>,

    /// Broad category of service being performed or delivered
    pub service_category: Option<types::CodeableConcept>,

    /// Specific service delivered or performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_type: Vec<HealthcareServiceServiceType>,

    /// Location where service may be provided
    pub location: types::Reference,

    /// Description of service as presented to a consumer while searching
    pub service_name: Option<types::String>,
    /// Primitive extension sibling for [`service_name`](Self::service_name) (FHIR `_serviceName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_serviceName")]
    pub service_name_ext: Option<types::Element>,

    /// Additional description and/or any specific issues not covered elsewhere
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Extra details about the service that can't be placed in the other
    /// fields
    pub extra_details: Option<types::String>,
    /// Primitive extension sibling for [`extra_details`](Self::extra_details) (FHIR `_extraDetails`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_extraDetails")]
    pub extra_details_ext: Option<types::Element>,

    /// Facilitates quick identification of the service
    pub photo: Option<types::Attachment>,

    /// Contacts related to the healthcare service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Location(s) service is inteded for/available to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference>,

    /// Conditions under which service is available/offered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_provision_code: Vec<types::CodeableConcept>,

    /// Specific eligibility requirements required to use the service
    pub eligibility: Option<types::CodeableConcept>,

    /// Describes the eligibility conditions for the service
    pub eligibility_note: Option<types::String>,
    /// Primitive extension sibling for [`eligibility_note`](Self::eligibility_note) (FHIR `_eligibilityNote`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_eligibilityNote")]
    pub eligibility_note_ext: Option<types::Element>,

    /// Program Names that categorize the service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_name: Vec<types::String>,
    /// Primitive extension sibling for [`program_name`](Self::program_name) (FHIR `_programName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_programName")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_name_ext: Vec<Option<types::Element>>,

    /// Collection of characteristics (attributes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<types::CodeableConcept>,

    /// Ways that the service accepts referrals
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referral_method: Vec<types::CodeableConcept>,

    /// PKI Public keys to support secure communications
    pub public_key: Option<types::String>,
    /// Primitive extension sibling for [`public_key`](Self::public_key) (FHIR `_publicKey`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicKey")]
    pub public_key_ext: Option<types::Element>,

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
}

/// A collection of times that the Service Site is available.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::healthcare_service::HealthcareServiceAvailableTime;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct HealthcareServiceAvailableTime {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_week: Vec<crate::coded::Coded<crate::r2::codes::DaysOfWeek>>,
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

/// The HealthcareService is not available during this period of time due to
/// the provided reason.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::healthcare_service::HealthcareServiceNotAvailable;
/// use fhir::r2::types;
///
/// let value = HealthcareServiceNotAvailable {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: HealthcareServiceNotAvailable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct HealthcareServiceNotAvailable {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reason presented to the user explaining why time not available
    pub description: types::String,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Service not availablefrom this date
    pub during: Option<types::Period>,
}

/// A specific type of service that may be delivered or performed.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::healthcare_service::HealthcareServiceServiceType;
/// use fhir::r2::types;
///
/// let value = HealthcareServiceServiceType {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: HealthcareServiceServiceType = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct HealthcareServiceServiceType {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of service delivered or performed
    pub r#type: types::CodeableConcept,

    /// Specialties handled by the Service Site
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,
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
