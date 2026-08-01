//! HealthcareService
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HealthcareService
//!
//! Version: 5.0.0
//!
//! HealthcareService Resource: The details of a healthcare service available at a location or in a catalog.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The details of a healthcare service available at a location or in a catalog.
///
/// `HealthcareService` is used to describe a service, or grouping of services,
/// that a healthcare organization makes available for delivery of care. It
/// captures administrative and clinical metadata such as the type and
/// specialty of service, the organization and location(s) that offer it,
/// contact details, eligibility criteria, referral methods, and the
/// availability schedule under which the service can be accessed. It is
/// commonly published in provider directories and service catalogs so that
/// systems and practitioners can discover and refer patients to appropriate
/// care.
///
/// In the case where there is a hierarchy of services (for example, Lab ->
/// Pathology -> Wound Cultures), this can be represented using a set of linked
/// HealthcareServices. It describes the type of service, where it is offered,
/// who provides it, and the conditions and availability under which the service
/// can be accessed.
///
/// # Related resources
///
/// - [`Organization`](crate::r5::resources::organization::Organization) — the
///   entity that typically provides the service via `provided_by`.
/// - [`Location`](crate::r5::resources::location::Location) — the place(s)
///   where the service is delivered.
/// - [`Endpoint`](crate::r5::resources::endpoint::Endpoint) — technical
///   endpoints for electronic access to the service.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used throughout
///   for coded categories, types, and specialties.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::healthcare_service::HealthcareService;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareService {
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

    /// External identifiers for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this HealthcareService record is currently in active use versus retired or superseded
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Reference to the [`Organization`](crate::r5::resources::organization::Organization) that provides this service
    pub provided_by: Option<types::Reference>,

    /// The service within which this service is offered, supporting hierarchical service catalogs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub offered_in: Vec<types::Reference>,

    /// Broad category of service being performed or delivered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Type of service that may be delivered or performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Specialties handled by the HealthcareService
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// References to the [`Location`](crate::r5::resources::location::Location) resource(s) where the service may be provided
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference>,

    /// Description of service as presented to a consumer while searching
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Additional description and/or any specific issues not covered elsewhere
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Extra details about the service that can't be placed in the other fields
    pub extra_details: Option<types::Markdown>,
    /// Primitive extension sibling for [`extra_details`](Self::extra_details) (FHIR `_extraDetails`).
    #[serde(rename = "_extraDetails")]
    pub extra_details_ext: Option<types::Element>,

    /// Facilitates quick identification of the service
    pub photo: Option<types::Attachment>,

    /// Official contact details for the HealthcareService
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Location(s) service is intended for/available to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference>,

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
    /// Primitive extension sibling for [`appointment_required`](Self::appointment_required) (FHIR `_appointmentRequired`).
    #[serde(rename = "_appointmentRequired")]
    pub appointment_required_ext: Option<types::Element>,

    /// Times the healthcare service is available (including exceptions)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availability: Vec<types::Availability>,

    /// Technical endpoints providing access to electronic services operated for the healthcare service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,
}

/// Specific eligibility requirements required to use the service.
///
/// Does this service have specific eligibility requirements that need to be met
/// in order to use the service?
/// # Examples
///
/// ```
/// use fhir::r5::resources::healthcare_service::HealthcareServiceEligibility;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
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
