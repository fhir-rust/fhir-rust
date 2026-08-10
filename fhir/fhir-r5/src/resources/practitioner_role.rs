//! PractitionerRole
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
//!
//! Version: 5.0.0
//!
//! PractitionerRole Resource: A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A specific set of roles, locations, specialties, and services that a
/// practitioner may perform, or has performed, at an organization during a
/// period of time. In FHIR R5 this resource is administrative rather than
/// clinical: it does not describe a person, but instead the relationship that
/// binds a practitioner to an organization and the context in which that
/// practitioner acts. A single practitioner can have many PractitionerRole
/// records, one for each distinct combination of organization, role, specialty,
/// location, or engagement period. The resource is central to provider
/// directories, scheduling and referral workflows, and access-control decisions,
/// because it answers who can perform what, on whose behalf, where, and when.
/// It also carries contact details, spoken languages, availability, and endpoints
/// used to reach the practitioner while acting in this particular role.
///
/// # See also
///
/// A PractitionerRole references a `Practitioner` (the person) and an
/// [`Organization`](crate::r5::resources::organization::Organization), and may
/// point to `Location` and `HealthcareService` resources. Roles and specialties
/// are expressed with [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// and links to other systems use [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::practitioner_role::PractitionerRole;
/// use fhir::r5::types;
///
/// let value = PractitionerRole {
///     active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `active` is the name this serializes to on the wire.
/// assert_eq!(json["active"], ::serde_json::json!(true));
///
/// let back: PractitionerRole = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerRole {
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

    /// Identifiers for a role/location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this practitioner role record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The period during which the practitioner is authorized to perform in these role(s)
    pub period: Option<types::Period>,

    /// Reference to the Practitioner (the person) who acts in this role
    pub practitioner: Option<types::Reference<crate::r5::resources::Practitioner>>,

    /// Reference to the Organization on whose behalf the practitioner performs this role
    pub organization: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Coded roles this practitioner may perform, such as doctor, nurse, or pharmacist
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Coded clinical specialties exercised by the practitioner in this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// Location(s) where the practitioner provides care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference<crate::r5::resources::Location>>,

    /// Healthcare services provided for this role's Organization/Location(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub healthcare_service: Vec<types::Reference<crate::r5::resources::HealthcareService>>,

    /// Official contact details relating to this PractitionerRole
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Collection of characteristics (attributes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<types::CodeableConcept>,

    /// A language the practitioner (in this role) can use in patient communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<types::CodeableConcept>,

    /// Times the Practitioner is available at this location and/or healthcare service (including exceptions)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availability: Vec<types::Availability>,

    /// Endpoints for interacting with the practitioner in this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r5::resources::Endpoint>>,
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
