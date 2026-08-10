//! PractitionerRole
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
//!
//! Version: 4.0.1
//!
//! Roles/organizations the practitioner is associated with
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A specific set of Roles/Locations/specialties/services that a practitioner
/// may perform at an organization for a period of time.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::practitioner_role::PractitionerRole;
/// use fhir::r4::types;
///
/// let value = PractitionerRole {
///     availability_exceptions: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `availabilityExceptions` is the name this serializes to on the wire.
/// assert_eq!(json["availabilityExceptions"], ::serde_json::json!("abc"));
///
/// let back: PractitionerRole = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct PractitionerRole {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifiers that are specific to a role/location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this practitioner role record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The period during which the practitioner is authorized to perform in
    /// these role(s)
    pub period: Option<types::Period>,

    /// Practitioner that is able to provide the defined services for the
    /// organization
    pub practitioner: Option<types::Reference<crate::r4::resources::Practitioner>>,

    /// Organization where the roles are available
    pub organization: Option<types::Reference<crate::r4::resources::Organization>>,

    /// Roles which this practitioner may perform
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Specific specialty of the practitioner
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// The location(s) at which this practitioner provides care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference<crate::r4::resources::Location>>,

    /// The list of healthcare services that this worker provides for this
    /// role's Organization/Location(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub healthcare_service: Vec<types::Reference<crate::r4::resources::HealthcareService>>,

    /// Contact details that are specific to the role/location/service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Times the Service Site is available
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_time: Vec<PractitionerRoleAvailableTime>,

    /// Not available during this time due to provided reason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_available: Vec<PractitionerRoleNotAvailable>,

    /// Description of availability exceptions
    pub availability_exceptions: Option<types::String>,
    /// Primitive extension sibling for [`availability_exceptions`](Self::availability_exceptions) (FHIR `_availabilityExceptions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availabilityExceptions")]
    pub availability_exceptions_ext: Option<types::Element>,

    /// Technical endpoints providing access to services operated for the
    /// practitioner with this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r4::resources::Endpoint>>,
}

/// A collection of times the practitioner is available or performing this role
/// at the location and/or healthcareservice.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::practitioner_role::PractitionerRoleAvailableTime;
/// use fhir::r4::types;
///
/// let value = PractitionerRoleAvailableTime {
///     all_day: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `allDay` is the name this serializes to on the wire.
/// assert_eq!(json["allDay"], ::serde_json::json!(true));
///
/// let back: PractitionerRoleAvailableTime = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct PractitionerRoleAvailableTime {
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
    pub days_of_week: ::fhir_core::PrimVec<crate::coded::Coded<crate::r4::codes::DaysOfWeek>>,
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

/// The practitioner is not available or performing this role during this
/// period of time due to the provided reason.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::practitioner_role::PractitionerRoleNotAvailable;
/// use fhir::r4::types;
///
/// let value = PractitionerRoleNotAvailable {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PractitionerRoleNotAvailable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct PractitionerRoleNotAvailable {
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
    type T = PractitionerRole;

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
