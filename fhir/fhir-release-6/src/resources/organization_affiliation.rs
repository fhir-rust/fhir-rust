//! OrganizationAffiliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OrganizationAffiliation
//!
//! Version: 6.0.0-ballot3
//!
//! Defines an affiliation/association/relationship between 2 distinct
//! organizations, that is not a part-of relationship/sub-division relationship
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Defines an affiliation/assotiation/relationship between 2 distinct
/// organizations, that is not a part-of relationship/sub-division
/// relationship.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::organization_affiliation::OrganizationAffiliation;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct OrganizationAffiliation {
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

    /// Business identifiers that are specific to this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this organization affiliation record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The period during which the participatingOrganization is affiliated
    /// with the primary organization
    pub period: Option<types::Period>,

    /// Organization where the role is available
    pub organization: Option<types::Reference>,

    /// Organization that provides/performs the role (e.g. providing services
    /// or is a member of)
    pub participating_organization: Option<types::Reference>,

    /// The network in which the participatingOrganization provides the role's
    /// services (if defined) at the indicated locations (if defined)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference>,

    /// Definition of the role the participatingOrganization plays
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Specific specialty of the participatingOrganization in the context of
    /// the role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specialty: Vec<types::CodeableConcept>,

    /// The location(s) at which the role occurs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference>,

    /// Healthcare services provided through the role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub healthcare_service: Vec<types::Reference>,

    /// Official contact details at the participatingOrganization relevant to
    /// this Affiliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Technical endpoints providing access to services operated for this role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,
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
