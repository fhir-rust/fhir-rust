//! Practitioner
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Practitioner
//!
//! Version: 6.0.0-ballot3
//!
//! A person with a formal responsibility in the provisioning of healthcare or
//! related services
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A person who is directly or indirectly involved in the provisioning of
/// healthcare or related services.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::practitioner::Practitioner;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct Practitioner {
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

    /// An identifier for the person as this agent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this practitioner's record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// The name(s) associated with the practitioner
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<types::HumanName>,

    /// A contact detail for the practitioner (that apply to all roles)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// male | female | other | unknown
    pub gender: Option<crate::coded::Coded<crate::r6::codes::AdministrativeGender>>,
    /// Primitive extension sibling for [`gender`](Self::gender) (FHIR `_gender`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gender")]
    pub gender_ext: Option<types::Element>,

    /// The date on which the practitioner was born
    pub birth_date: Option<types::Date>,
    /// Primitive extension sibling for [`birth_date`](Self::birth_date) (FHIR `_birthDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_birthDate")]
    pub birth_date_ext: Option<types::Element>,

    /// Indicates if the practitioner is deceased or not
    /// The `Practitioner.deceased[x]` choice element (0..1); see [`PractitionerDeceased`].
    #[serde(flatten)]
    pub deceased: Option<PractitionerDeceased>,

    /// Address(es) of the practitioner that are not role specific (typically
    /// home address)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub address: Vec<types::Address>,

    /// Image of the person
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<types::Attachment>,

    /// Qualifications, certifications, accreditations, licenses, training,
    /// etc. pertaining to the provision of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualification: Vec<PractitionerQualification>,

    /// A language which may be used to communicate with the practitioner
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication: Vec<PractitionerCommunication>,
}

/// A language which may be used to communicate with the practitioner, often
/// for correspondence/administrative purposes. The
/// `PractitionerRole.communication` property should be used for publishing the
/// languages that a practitioner is able to communicate with patients (on a
/// per Organization/Role basis).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::practitioner::PractitionerCommunication;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`preferred`](Self::preferred) (FHIR `_preferred`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preferred")]
    pub preferred_ext: Option<types::Element>,
}

/// The official qualifications, certifications, accreditations, training,
/// licenses (and other types of educations/skills/capabilities) that authorize
/// or otherwise pertain to the provision of care by the practitioner. For
/// example, a medical license issued by a medical board of licensure
/// authorizing the practitioner to practice medicine within a certain
/// locality.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::practitioner::PractitionerQualification;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Status/progress of the qualification
    pub status: Option<types::CodeableConcept>,

    /// Period during which the qualification is valid
    pub period: Option<types::Period>,

    /// Organization that regulates and issues the qualification
    pub issuer: Option<types::Reference>,
}

/// The `Practitioner.deceased[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PractitionerDeceased {
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
