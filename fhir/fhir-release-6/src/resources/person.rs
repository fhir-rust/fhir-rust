//! Person
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Person
//!
//! Version: 6.0.0-ballot3
//!
//! A generic person record
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Demographics and administrative information about a person independent of a
/// specific health-related context.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::person::Person;
/// use fhir::r6::types;
///
/// let value = Person {
///     birth_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `birthDate` is the name this serializes to on the wire.
/// assert_eq!(json["birthDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: Person = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Person {
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

    /// A human identifier for this person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// This person's record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// A name associated with the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<types::HumanName>,

    /// A contact detail for the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// male | female | other | unknown
    pub gender: Option<crate::coded::Coded<crate::r6::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// The date on which the person was born
    pub birth_date: Option<types::Date>,
    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR `_birthDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

    /// Indicates if the individual is deceased or not
    /// The `Person.deceased[x]` choice element (0..1); see [`PersonDeceased`].
    #[serde(flatten)]
    pub deceased: Option<PersonDeceased>,

    /// One or more addresses for the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<types::Address>,

    /// Marital (civil) status of a person
    pub marital_status: Option<types::CodeableConcept>,

    /// Image of the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<types::Attachment>,

    /// A language which may be used to communicate with the person about his
    /// or her health
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PersonCommunication>,

    /// The organization that is the custodian of the person record
    pub managing_organization: Option<types::Reference>,

    /// Link to a resource that concerns the same actual person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<PersonLink>,
}

/// A language which may be used to communicate with the person about his or
/// her health.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::person::PersonCommunication;
/// use fhir::r6::types;
///
/// let value = PersonCommunication {
///     preferred: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preferred` is the name this serializes to on the wire.
/// assert_eq!(json["preferred"], ::serde_json::json!(true));
///
/// let back: PersonCommunication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PersonCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language which can be used to communicate with the person about his
    /// or her health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,
}

/// Link to a resource that concerns the same actual person.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::person::PersonLink;
/// use fhir::r6::types;
///
/// let value = PersonLink {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PersonLink = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PersonLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The resource to which this actual person is associated
    pub target: types::Reference,

    /// level1 | level2 | level3 | level4
    pub assurance: Option<crate::coded::Coded<crate::r6::codes::IdentityAssuranceLevel>>,
    /// Primitive extension sibling for [`assurance`](Self::assurance) (FHIR `_assurance`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_assurance")]
    pub assurance_ext: Option<types::Element>,
}

/// The `Person.deceased[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PersonDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `deceasedDateTime` variant.
    #[fhir("deceasedDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Person;

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
