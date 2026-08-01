//! Patient
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Patient
//!
//! Version: 5.0.0
//!
//! Patient Resource: Demographics and other administrative information about an individual or animal receiving care or other health-related services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Demographics and other administrative information about an individual or
/// animal receiving care or other health-related services.
///
/// The Patient resource is the primary administrative record of a person or
/// animal receiving care. It captures the demographic and contact details that
/// identify the subject of care and that support scheduling, billing, matching,
/// and communication, rather than any single clinical finding. Typical content
/// includes one or more names, identifiers issued by organizations such as a
/// medical record number or national health number, gender and birth date,
/// addresses and telecom contact points, marital status, deceased status, and
/// preferred languages of communication. Because care spans many settings, the
/// resource covers a wide range of health-related activities, including curative
/// care, psychiatric care, social services, pregnancy care, nursing and assisted
/// living, dietary services, and tracking of personal health and exercise data.
///
/// In FHIR R5 the Patient resource is one of the most frequently referenced
/// resources and serves as the subject or focus for most clinical and
/// administrative resources, so that observations, encounters, conditions,
/// medications, and similar records point back to a single stable identity. A
/// Patient may also be linked to other Patient or RelatedPerson records that
/// concern the same real-world individual, which supports record merging,
/// de-duplication, and cross-organization matching.
///
/// # Related resources
///
/// See also the nested backbone types [`PatientContact`],
/// [`PatientCommunication`],
/// and [`PatientLink`]. Common
/// building-block data types include [`HumanName`](crate::r5::types::HumanName),
/// [`Identifier`](crate::r5::types::Identifier),
/// [`ContactPoint`](crate::r5::types::ContactPoint),
/// [`Address`](crate::r5::types::Address), and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept). Related resources such
/// as `Person`, `RelatedPerson`, `Practitioner`, and `Group` describe other kinds
/// of parties and are referenced from or alongside Patient.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::patient::Patient;
/// use fhir::r5::types;
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
pub struct Patient {
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

    /// Business identifiers assigned by an organization, such as a medical record number, that uniquely reference this patient.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this patient's record is in active use; a false value typically marks a record that is retired, merged, or entered in error.
    pub active: Option<types::Boolean>,

    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value. See the
    /// [primitive extensions spec](https://hl7.org/fhir/R5/json.html#primitive).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// One or more names associated with the patient, allowing for official, usual, maiden, and other name uses.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<types::HumanName>,

    /// A contact detail for the individual
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// The administrative gender used for record keeping, coded as male, female, other, or unknown, which may differ from clinical or biological sex.
    pub gender: Option<crate::r5::coded::Coded<crate::r5::codes::AdministrativeGender>>,

    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`).
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// The date of birth for the individual, used for identity matching and age-based clinical decisions.
    pub birth_date: Option<types::Date>,

    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR
    /// `_birthDate`). Commonly carries the `patient-birthTime` extension.
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

    /// The `Patient.deceased[x]` choice element (0..1); see [`PatientDeceased`].
    #[serde(flatten)]
    pub deceased: Option<PatientDeceased>,

    /// An address for the individual
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<types::Address>,

    /// Marital (civil) status of a patient
    pub marital_status: Option<types::CodeableConcept>,

    /// The `Patient.multipleBirth[x]` choice element (0..1); see [`PatientMultipleBirth`].
    #[serde(flatten)]
    pub multiple_birth: Option<PatientMultipleBirth>,

    /// Image of the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<types::Attachment>,

    /// A contact party (e.g. guardian, partner, friend) for the patient
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<PatientContact>,

    /// A language which may be used to communicate with the patient about his or her health
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PatientCommunication>,

    /// Patient's nominated primary care provider
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub general_practitioner: Vec<types::Reference>,

    /// Organization that is the custodian of the patient record
    pub managing_organization: Option<types::Reference>,

    /// Link to a Patient or RelatedPerson resource that concerns the same actual individual
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<PatientLink>,
}

/// A contact party (e.g. guardian, partner, friend) for the patient.
/// # Examples
///
/// ```
/// use fhir::r5::resources::patient::PatientContact;
/// use fhir::r5::types;
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
pub struct PatientContact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    pub gender: Option<crate::r5::coded::Coded<crate::r5::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`).
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// Organization that is associated with the contact
    pub organization: Option<types::Reference>,

    /// The period during which this contact person or organization is valid to be contacted relating to this patient
    pub period: Option<types::Period>,
}

/// A language which may be used to communicate with the patient about his or
/// her health.
/// # Examples
///
/// ```
/// use fhir::r5::resources::patient::PatientCommunication;
/// use fhir::r5::types;
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
pub struct PatientCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language which can be used to communicate with the patient about his or her health
    pub language: types::CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<types::Boolean>,
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`).
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,
}

/// Link to a Patient or RelatedPerson resource that concerns the same actual
/// individual.
/// # Examples
///
/// ```
/// use fhir::r5::resources::patient::PatientLink;
/// use fhir::r5::types;
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
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::LinkType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,
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
/// The `Patient.deceased[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PatientDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `deceasedDateTime` variant.
    #[fhir("deceasedDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}

/// The `Patient.multipleBirth[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PatientMultipleBirth {
    /// `multipleBirthBoolean` variant.
    #[fhir("multipleBirthBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `multipleBirthInteger` variant.
    #[fhir("multipleBirthInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
}
