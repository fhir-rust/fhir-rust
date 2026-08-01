//! Person
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Person
//!
//! Version: 5.0.0
//!
//! Person Resource: Demographics and administrative information about a person independent of a specific health-related context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Demographics and administrative information about a person independent of a
/// specific health-related context.
///
/// A Person resource captures the identity and demographic details of an
/// individual who may play one or more roles in the healthcare system, such as
/// a patient, practitioner, related person, or user of a system. Unlike those
/// role-specific resources, a Person is not tied to any single health-related
/// context; instead it models the underlying human being and provides a stable
/// point of identity that can be reused across contexts. Its most distinctive
/// purpose is index and cross-reference management: through its links a Person
/// record can associate the several records that concern the same actual human
/// being, supporting patient-matching, master person indexes, and record
/// linkage across organizations and systems. In FHIR R5 a Person typically
/// carries identifiers, human names, contact details, address, gender, birth
/// date, marital status, preferred communication languages, and the managing
/// organization that acts as custodian of the record.
///
/// # Related resources
///
/// A Person is frequently linked to a [`Patient`](crate::r5::resources::patient::Patient),
/// as well as to `Practitioner` and `RelatedPerson` records that represent the
/// same individual in different roles. Coded elements such as marital status and
/// communication language use [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// and links to other resources use [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::person::Person;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Person {
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

    /// Business identifiers by which this person is known, such as a national or organizational person identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this person's record is in active use; a false value indicates the record should no longer be relied upon
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// One or more names associated with the person, each expressed as a structured human name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<types::HumanName>,

    /// A contact detail for the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// male | female | other | unknown
    pub gender: Option<crate::r5::coded::Coded<crate::r5::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`).
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// The date on which the person was born
    pub birth_date: Option<types::Date>,
    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR `_birthDate`).
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

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

    /// A language which may be used to communicate with the person about his or her health
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PersonCommunication>,

    /// The organization that is the custodian of the person record
    pub managing_organization: Option<types::Reference>,

    /// Links to Patient, Practitioner, RelatedPerson, or other Person records that concern the same actual person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<PersonLink>,
}

/// A language which may be used to communicate with the person about his or her
/// health.
/// # Examples
///
/// ```
/// use fhir::r5::resources::person::PersonCommunication;
/// use fhir::r5::types;
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
pub struct PersonCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language which can be used to communicate with the person about his or her health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`).
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,
}

/// Link to a resource that concerns the same actual person.
/// # Examples
///
/// ```
/// use fhir::r5::resources::person::PersonLink;
/// use fhir::r5::types;
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
    pub assurance: Option<crate::r5::coded::Coded<crate::r5::codes::IdentityAssuranceLevel>>,
    /// Primitive extension sibling for [`assurance`](Self::assurance) (FHIR `_assurance`).
    #[serde(rename = "_assurance")]
    pub assurance_ext: Option<types::Element>,
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
/// The `Person.deceased[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PersonDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `deceasedDateTime` variant.
    #[fhir("deceasedDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
