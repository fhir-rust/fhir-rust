//! Patient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Patient
//!
//! Version: 6.0.0-ballot3
//!
//! Information about an individual or animal receiving health care services
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Demographics and other administrative information about an individual or
/// animal that is the subject of potential, past, current, or future
/// health-related care, services, or processes.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::patient::Patient;
/// use fhir::r6::types;
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
#[serde(from = "PatientDe")]
#[fhir_version("r6")]
pub struct Patient {
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
    pub gender: Option<crate::coded::Coded<crate::r6::codes::AdministrativeGender>>,
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

    /// Indicates if/when the individual is deceased
    /// The `Patient.deceased[x]` choice element (0..1); see [`PatientDeceased`].
    #[serde(flatten)]
    pub deceased: Option<PatientDeceased>,

    /// An address for the individual
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

    /// A language which may be used to communicate with the patient about his
    /// or her health
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PatientCommunication>,

    /// Patient's nominated primary care provider
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub general_practitioner: Vec<types::Reference>,

    /// Organization that is the custodian of the patient record
    pub managing_organization: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Link to a Patient or RelatedPerson resource that concerns the same
    /// actual individual
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<PatientLink>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PatientDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    active: Option<types::Boolean>,
    #[serde(rename = "_active")]
    active_ext: Option<types::Element>,
    #[serde(default)]
    name: Vec<types::HumanName>,
    #[serde(default)]
    telecom: Vec<types::ContactPoint>,
    gender: Option<crate::coded::Coded<crate::r6::codes::AdministrativeGender>>,
    #[serde(rename = "_gender")]
    gender_ext: Option<types::Element>,
    birth_date: Option<types::Date>,
    #[serde(rename = "_birthDate")]
    birth_date_ext: Option<types::Element>,
    #[serde(flatten)]
    deceased: crate::r6::choice::Slot<PatientDeceased>,
    #[serde(default)]
    address: Vec<types::Address>,
    marital_status: Option<types::CodeableConcept>,
    #[serde(flatten)]
    multiple_birth: crate::r6::choice::Slot<PatientMultipleBirth>,
    #[serde(default)]
    photo: Vec<types::Attachment>,
    #[serde(default)]
    contact: Vec<PatientContact>,
    #[serde(default)]
    communication: Vec<PatientCommunication>,
    #[serde(default)]
    general_practitioner: Vec<types::Reference>,
    managing_organization: Option<types::Reference<crate::r6::resources::Organization>>,
    #[serde(default)]
    link: Vec<PatientLink>,
}

impl ::core::convert::From<PatientDe> for Patient {
    fn from(v: PatientDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            active: v.active,
            active_ext: v.active_ext,
            name: v.name,
            telecom: v.telecom,
            gender: v.gender,
            gender_ext: v.gender_ext,
            birth_date: v.birth_date,
            birth_date_ext: v.birth_date_ext,
            deceased: v.deceased.0,
            address: v.address,
            marital_status: v.marital_status,
            multiple_birth: v.multiple_birth.0,
            photo: v.photo,
            contact: v.contact,
            communication: v.communication,
            general_practitioner: v.general_practitioner,
            managing_organization: v.managing_organization,
            link: v.link,
        }
    }
}

/// A language which may be used to communicate with the patient about his or
/// her health.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::patient::PatientCommunication;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct PatientCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
/// use fhir::r6::resources::patient::PatientContact;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct PatientContact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The kind of personal relationship
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationship: Vec<types::CodeableConcept>,

    /// The kind of functional role
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,

    /// A name associated with the contact person
    pub name: Option<types::HumanName>,

    /// Additional names for the contact person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_name: Vec<types::HumanName>,

    /// A contact detail for the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Address for the contact person
    pub address: Option<types::Address>,

    /// Additional addresses for the contact person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_address: Vec<types::Address>,

    /// male | female | other | unknown
    pub gender: Option<crate::coded::Coded<crate::r6::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// Organization that is associated with the contact
    pub organization: Option<types::Reference<crate::r6::resources::Organization>>,

    /// The period during which this contact person or organization is valid to
    /// be contacted relating to this patient
    pub period: Option<types::Period>,
}

/// Link to a Patient or RelatedPerson resource that concerns the same actual
/// individual.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::patient::PatientLink;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct PatientLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The other patient or related person resource that the link refers to
    pub other: types::Reference,

    /// replaced-by | replaces | refer | seealso
    pub r#type: crate::coded::Coded<crate::r6::codes::LinkType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
}

/// The `Patient.deceased[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PatientDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `deceasedDateTime` variant.
    #[fhir("deceasedDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
}

/// The `Patient.multipleBirth[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PatientMultipleBirth {
    /// `multipleBirthBoolean` variant.
    #[fhir("multipleBirthBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `multipleBirthInteger` variant.
    #[fhir("multipleBirthInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
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
