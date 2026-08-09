//! OrganizationAffiliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OrganizationAffiliation
//!
//! Version: 5.0.0
//!
//! OrganizationAffiliation Resource: Defines an affiliation/assotiation/relationship between 2 distinct organizations, that is not a part-of relationship/sub-division relationship.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// OrganizationAffiliation describes an affiliation, association, or
/// relationship between two distinct organizations that is not a part-of or
/// sub-division relationship. Rather than expressing organizational hierarchy,
/// it records the functional role that a participating organization plays with
/// respect to a primary organization, together with the period, networks,
/// locations, specialties, and healthcare services that scope that role.
///
/// In FHIR R5 this resource is primarily an administrative building block for
/// provider directories and healthcare interoperability. It answers questions
/// such as which organizations deliver services on behalf of another
/// organization, which provider or payer networks an organization participates
/// in, at which locations an affiliation applies, and which technical endpoints
/// and contact details serve that affiliation. Because affiliations are often
/// time-bound and network-specific, the resource carries an active flag and an
/// effective period so directories can reflect current versus historical
/// relationships.
///
/// Related resources: the affiliation links a primary
/// [`Organization`](crate::r5::resources::organization::Organization) with a
/// participating one, may reference
/// [`Location`](crate::r5::resources::location::Location) and
/// [`HealthcareService`](crate::r5::resources::healthcare_service::HealthcareService)
/// entries that the role covers, and exposes technical
/// [`Endpoint`](crate::r5::resources::endpoint::Endpoint) resources. Role,
/// specialty, and similar coded values are expressed as
/// [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::organization_affiliation::OrganizationAffiliation;
/// use fhir::r5::types;
///
/// let value = OrganizationAffiliation {
///     active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `active` is the name this serializes to on the wire.
/// assert_eq!(json["active"], ::serde_json::json!(true));
///
/// let back: OrganizationAffiliation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAffiliation {
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

    /// Business identifiers that are specific to this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this organization affiliation record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The period during which the participatingOrganization is affiliated with the primary organization
    pub period: Option<types::Period>,

    /// Reference to the primary Organization on whose behalf, or for which, the role is made available
    pub organization: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Reference to the Organization that provides or performs the role, such as delivering services or being a network member
    pub participating_organization: Option<types::Reference<crate::r5::resources::Organization>>,

    /// The network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference<crate::r5::resources::Organization>>,

    /// Coded definition of the role the participatingOrganization plays with respect to the primary organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Specific specialty of the participatingOrganization in the context of the role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// The location(s) at which the role occurs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference<crate::r5::resources::Location>>,

    /// Healthcare services provided through the role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub healthcare_service: Vec<types::Reference<crate::r5::resources::HealthcareService>>,

    /// Official contact details at the participatingOrganization relevant to this Affiliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Technical endpoints providing access to services operated for this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r5::resources::Endpoint>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = OrganizationAffiliation;

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
