//! PractitionerRole
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PractitionerRole
//!
//! Version: 6.0.0-ballot3
//!
//! Roles/organizations the practitioner is associated with
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A specific set of Roles/Locations/specialties/services that a practitioner
/// may perform, or has performed at an organization during a period of time.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::practitioner_role::PractitionerRole;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

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
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The period during which the practitioner is authorized to perform in
    /// these role(s)
    pub period: Option<types::Period>,

    /// Practitioner that provides services for the organization
    pub practitioner: Option<types::Reference<crate::r6::resources::Practitioner>>,

    /// Organization where the role is available
    pub organization: Option<types::Reference<crate::r6::resources::Organization>>,

    /// The network in which the PractitionerRole provides the role's services
    /// (if defined) at the indicated locations (if defined)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference<crate::r6::resources::Organization>>,

    /// Roles which this practitioner may perform
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Denormalized practitioner name, role, organization and location
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Specific specialty of the practitioner
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// Location(s) where the practitioner provides care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference<crate::r6::resources::Location>>,

    /// Healthcare services provided for this role's Organization/Location(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub healthcare_service: Vec<types::Reference<crate::r6::resources::HealthcareService>>,

    /// Official contact details relating to this PractitionerRole
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Collection of characteristics (attributes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<types::CodeableConcept>,

    /// A language the practitioner (in this role) can use in patient
    /// communication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<types::CodeableConcept>,

    /// Times the Practitioner is available at this location and/or healthcare
    /// service (including exceptions)
    pub availability: Option<types::Availability>,

    /// Endpoints for interacting with the practitioner in this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r6::resources::Endpoint>>,
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
