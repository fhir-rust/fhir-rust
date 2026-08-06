//! Organization
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Organization
//!
//! Version: 6.0.0-ballot3
//!
//! A grouping of people or organizations with a common purpose
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A formally or informally recognized grouping of people or organizations
/// formed for the purpose of achieving some form of collective action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::organization::Organization;
/// use fhir::r6::types;
///
/// let value = Organization {
///     active: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `active` is the name this serializes to on the wire.
/// assert_eq!(json["active"], ::serde_json::json!(true));
///
/// let back: Organization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Organization {
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

    /// Identifies this organization across multiple systems
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether the organization's record is still in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Kind of organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Name used for the organization
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// A list of alternate names that the organization is known as, or was
    /// known as in the past
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// Additional details about the Organization that could be displayed as
    /// further information to identify the Organization beyond its name
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Official contact details for the Organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// The organization of which this organization forms a part
    pub part_of: Option<types::Reference>,

    /// Technical endpoints providing access to services operated for the
    /// organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,

    /// Qualifications, certifications, accreditations, licenses, training,
    /// etc. pertaining to the provision of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualification: Vec<OrganizationQualification>,
}

/// The official certifications, accreditations, training, designations and
/// licenses that authorize and/or otherwise endorse the provision of care by
/// the organization. For example, an approval to provide a type of services
/// issued by a certifying body (such as the US Joint Commission) to an
/// organization.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::organization::OrganizationQualification;
/// use fhir::r6::types;
///
/// let value = OrganizationQualification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: OrganizationQualification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct OrganizationQualification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// An identifier for this qualification for the organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Coded representation of the qualification
    pub code: types::CodeableConcept,

    /// Status/progress of the qualification
    pub status: Option<types::CodeableConcept>,

    /// Period during which the qualification is valid
    pub period: Option<types::Period>,

    /// Organization that regulates and issues the qualification
    pub issuer: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Organization;

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
