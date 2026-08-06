//! RelatedPerson
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RelatedPerson
//!
//! Version: 5.0.0
//!
//! RelatedPerson Resource: Information about a person that is involved in a patient's health or the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Information about a person that is involved in a patient's health or the care
/// for a patient, but who is not the target of healthcare, nor has a formal
/// responsibility in the care process.
///
/// A RelatedPerson resource captures a person such as a parent, spouse, guardian,
/// neighbour, friend, or informal caregiver who has a personal or non-professional
/// relationship to a patient. Unlike a practitioner, this person is not a member of
/// the care team and has no formal clinical or legal responsibility for the care
/// process, yet is often the party who accompanies the patient, provides history,
/// acts as an emergency contact, or serves as an interpreter. The resource records
/// the person's identity, demographics, contact details, addresses, and the nature
/// and validity period of their relationship to the patient, so that a system can
/// represent and reference them without conflating them with a patient or a
/// clinician.
///
/// In FHIR R5 a RelatedPerson is anchored to a single patient through its required
/// `patient` reference, and is commonly referenced from workflow resources such as
/// Encounter, Appointment, CarePlan, and Communication wherever a non-clinician
/// party participates in or is contacted about the patient's care. The relationship
/// type and any preferred languages are conveyed with coded concepts.
///
/// # Related resources
///
/// See also [`Patient`](crate::r5::resources::patient::Patient) for the individual
/// this person is related to, and [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// for how the relationship and communication language are coded. For members of the
/// care team who do have a formal clinical role, use the `Practitioner` resource
/// instead.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::related_person::RelatedPerson;
/// use fhir::r5::types;
///
/// let value = RelatedPerson {
///     birth_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `birthDate` is the name this serializes to on the wire.
/// assert_eq!(json["birthDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: RelatedPerson = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPerson {
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

    /// Business identifier(s) assigned to this related person, distinct from any identifiers held for the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this related person's record is currently in active use for care coordination
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Required reference to the patient this person is related to, anchoring the record to a single patient
    pub patient: types::Reference,

    /// Coded nature of the relationship to the patient, such as parent, spouse, guardian, or emergency contact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<types::CodeableConcept>,

    /// Name(s) by which this related person is known, using the same `HumanName` structure as on `Patient`
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

    /// The date on which the related person was born
    pub birth_date: Option<types::Date>,
    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR `_birthDate`).
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

    /// Address where the related person can be contacted or visited
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<types::Address>,

    /// Image of the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<types::Attachment>,

    /// Period of time during which this relationship to the patient is considered valid and in effect
    pub period: Option<types::Period>,

    /// A language which may be used to communicate with the related person about the patient's health
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<RelatedPersonCommunication>,
}

/// A language which may be used to communicate with the related person about the
/// patient's health.
/// # Examples
///
/// ```
/// use fhir::r5::resources::related_person::RelatedPersonCommunication;
/// use fhir::r5::types;
///
/// let value = RelatedPersonCommunication {
///     preferred: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preferred` is the name this serializes to on the wire.
/// assert_eq!(json["preferred"], ::serde_json::json!(true));
///
/// let back: RelatedPersonCommunication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPersonCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language which can be used to communicate with the related person about the patient's health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`).
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RelatedPerson;

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
