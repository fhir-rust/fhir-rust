//! Practitioner
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Practitioner
//!
//! Version: 5.0.0
//!
//! Practitioner Resource: A person who is directly or indirectly involved in the provisioning of healthcare or related services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A person who is directly or indirectly involved in the provisioning of
/// healthcare or related services. In FHIR R5 the Practitioner resource captures
/// the durable, identity-level facts about such a person, including their names,
/// contact details, gender, birth date, addresses, spoken languages, and formal
/// qualifications, certifications, and licenses. It is used to represent
/// clinicians such as physicians, nurses, midwives, and pharmacists, as well as
/// non-clinical staff such as receptionists, IT personnel, and other agents who
/// participate in care or its administration.
///
/// The resource deliberately models the person independent of any particular
/// role, employment, or organizational affiliation. The context in which a
/// practitioner acts, including their specialty, the organization they work for,
/// the location where they provide services, and the periods of that engagement,
/// is expressed separately via the `PractitionerRole` resource, allowing one
/// Practitioner to hold many roles over time without duplicating identity data.
///
/// Related resources: a Practitioner is commonly referenced from care-related
/// resources and contrasts with [`Patient`](crate::r5::resources::patient::Patient),
/// which represents the recipient of care. Coded values on this resource, such as
/// qualification codes and communication languages, use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and human-readable
/// names use [`HumanName`](crate::r5::types::HumanName). See also `PractitionerRole`
/// and `Organization`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::practitioner::Practitioner;
/// use fhir::r5::types;
///
/// let value = Practitioner {
///     birth_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `birthDate` is the name this serializes to on the wire.
/// assert_eq!(json["birthDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: Practitioner = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Practitioner {
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

    /// Business identifiers for the practitioner, such as a national provider or license number, that persist as the person acts across systems.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this practitioner's record is in active use, allowing records to be retired without being deleted.
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`).
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The name or names associated with the practitioner, supporting official, usual, and historical forms.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<types::HumanName>,

    /// A contact detail for the practitioner (that apply to all roles)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// male | female | other | unknown
    pub gender: Option<crate::r5::coded::Coded<crate::r5::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`).
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// The date  on which the practitioner was born
    pub birth_date: Option<types::Date>,
    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR `_birthDate`).
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

    /// The `Practitioner.deceased[x]` choice element (0..1); see [`PractitionerDeceased`].
    #[serde(flatten)]
    pub deceased: Option<PractitionerDeceased>,

    /// Address(es) of the practitioner that are not role specific (typically home address)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<types::Address>,

    /// Image of the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<types::Attachment>,

    /// Qualifications, certifications, accreditations, licenses, and training pertaining to the provision of care held by this practitioner.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualification: Vec<PractitionerQualification>,

    /// A language which may be used to communicate with the practitioner
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PractitionerCommunication>,
}

/// Qualifications, certifications, accreditations, licenses, training, etc.
/// pertaining to the provision of care held by the practitioner.
/// # Examples
///
/// ```
/// use fhir::r5::resources::practitioner::PractitionerQualification;
/// use fhir::r5::types;
///
/// let value = PractitionerQualification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PractitionerQualification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerQualification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// An identifier for this qualification for the practitioner
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Coded representation of the qualification
    pub code: types::CodeableConcept,

    /// Period during which the qualification is valid
    pub period: Option<types::Period>,

    /// Organization that regulates and issues the qualification
    pub issuer: Option<types::Reference>,
}

/// A language which may be used to communicate with the practitioner about their
/// care, along with an indication of preference.
/// # Examples
///
/// ```
/// use fhir::r5::resources::practitioner::PractitionerCommunication;
/// use fhir::r5::types;
///
/// let value = PractitionerCommunication {
///     preferred: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preferred` is the name this serializes to on the wire.
/// assert_eq!(json["preferred"], ::serde_json::json!(true));
///
/// let back: PractitionerCommunication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language code used to communicate with the practitioner
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
    type T = Practitioner;

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
/// The `Practitioner.deceased[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PractitionerDeceased {
    /// `deceasedBoolean` variant.
    #[fhir("deceasedBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `deceasedDateTime` variant.
    #[fhir("deceasedDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
