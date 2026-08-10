//! Patient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Patient
//!
//!
//!
//! Information about an individual or animal receiving health care services
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Patient Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::patient::Patient;
/// use fhir::r3::types;
///
/// let value = Patient {
///     birth_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `birthDate` is the name this serializes to on the wire.
/// assert_eq!(json["birthDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: Patient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Patient {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// An identifier for this patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this patient's record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// A name associated with the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<types::HumanName>,

    /// A contact detail for the individual
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// male | female | other | unknown
    pub gender: Option<crate::coded::Coded<crate::r3::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// The date of birth for the individual
    pub birth_date: Option<types::Date>,
    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR `_birthDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

    /// Indicates if the individual is deceased or not
    /// The `Patient.deceased[x]` choice element (0..1); see [`PatientDeceased`].
    #[serde(flatten)]
    pub deceased: Option<PatientDeceased>,

    /// Addresses for the individual
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<types::Address>,

    /// Marital (civil) status of a patient
    pub marital_status: Option<types::CodeableConcept>,

    /// Whether patient is part of a multiple birth
    /// The `Patient.multipleBirth[x]` choice element (0..1); see [`PatientMultipleBirth`].
    #[serde(flatten)]
    pub multiple_birth: Option<PatientMultipleBirth>,

    /// Image of the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<types::Attachment>,

    /// A contact party (e.g. guardian, partner, friend) for the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<PatientContact>,

    /// This patient is known to be an animal (non-human)
    pub animal: Option<PatientAnimal>,

    /// A list of Languages which may be used to communicate with the patient
    /// about his or her health
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PatientCommunication>,

    /// Patient's nominated primary care provider
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub general_practitioner: Vec<types::Reference>,

    /// Organization that is the custodian of the patient record
    pub managing_organization: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Link to another patient resource that concerns the same actual person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<PatientLink>,
}

/// This patient is known to be an animal.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::patient::PatientAnimal;
/// use fhir::r3::types;
///
/// let value = PatientAnimal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PatientAnimal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PatientAnimal {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// E.g. Dog, Cow
    pub species: types::CodeableConcept,

    /// E.g. Poodle, Angus
    pub breed: Option<types::CodeableConcept>,

    /// E.g. Neutered, Intact
    pub gender_status: Option<types::CodeableConcept>,
}

/// Languages which may be used to communicate with the patient about his or
/// her health.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::patient::PatientCommunication;
/// use fhir::r3::types;
///
/// let value = PatientCommunication {
///     preferred: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preferred` is the name this serializes to on the wire.
/// assert_eq!(json["preferred"], ::serde_json::json!(true));
///
/// let back: PatientCommunication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PatientCommunication {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language which can be used to communicate with the patient about
    /// his or her health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,
}

/// A contact party (e.g. guardian, partner, friend) for the patient.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::patient::PatientContact;
/// use fhir::r3::types;
///
/// let value = PatientContact {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PatientContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PatientContact {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of relationship
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<types::CodeableConcept>,

    /// A name associated with the contact person
    pub name: Option<types::HumanName>,

    /// A contact detail for the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Address for the contact person
    pub address: Option<types::Address>,

    /// male | female | other | unknown
    pub gender: Option<crate::coded::Coded<crate::r3::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// Organization that is associated with the contact
    pub organization: Option<types::Reference<crate::r3::resources::Organization>>,

    /// The period during which this contact person or organization is valid to
    /// be contacted relating to this patient
    pub period: Option<types::Period>,
}

/// Link to another patient resource that concerns the same actual patient.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::patient::PatientLink;
/// use fhir::r3::types;
///
/// let value = PatientLink {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PatientLink = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct PatientLink {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The other patient or related person resource that the link refers to
    pub other: types::Reference,

    /// replaced-by | replaces | refer | seealso - type of link
    pub r#type: crate::coded::Coded<crate::r3::codes::LinkType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// The `Patient.deceased[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum PatientDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `deceasedDateTime` variant.
    #[fhir("deceasedDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
}

/// The `Patient.multipleBirth[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum PatientMultipleBirth {
    /// `multipleBirthBoolean` variant.
    #[fhir("multipleBirthBoolean")]
    Boolean(crate::r3::choice::Primitive<types::Boolean>),
    /// `multipleBirthInteger` variant.
    #[fhir("multipleBirthInteger")]
    Integer(crate::r3::choice::Primitive<types::Integer>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Patient;

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
