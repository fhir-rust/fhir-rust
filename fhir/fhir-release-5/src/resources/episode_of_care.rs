//! EpisodeOfCare
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
//!
//! Version: 5.0.0
//!
//! EpisodeOfCare Resource: An association between a patient and an organization / healthcare provider(s) during which time encounters may occur.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// An association between a patient and an organization / healthcare
/// provider(s) during which time encounters may occur.
///
/// EpisodeOfCare represents a period during which a managing organization
/// assumes a level of responsibility for a patient's care, potentially
/// spanning many encounters. It groups together the type of care, the medical
/// reasons and diagnoses being addressed, the care manager and care team, and
/// the accounts used for billing. In FHIR R5 it is commonly used to model
/// longitudinal programs such as disease management, specialist referrals, or
/// ongoing treatment relationships.
///
/// Clinically and administratively, an EpisodeOfCare acts as the umbrella
/// under which individual clinical events are grouped: each
/// [`Encounter`](crate::r5::resources::encounter::Encounter) that occurs
/// while the episode is active may reference this resource so that the
/// visits, orders, and results belonging to the same program of care can be
/// tracked and reported together. The `status` and `status_history` fields
/// track the episode's lifecycle from `planned` through `active` to
/// `finished` or `cancelled`, while `reason` and `diagnosis` capture the
/// clinical justification and the conditions being managed.
///
/// Related resources: the `patient` field references the
/// [`Patient`](crate::r5::resources::patient::Patient) who is the focus of
/// the episode, and the type/class of the episode, its reasons, and its
/// diagnoses are typically coded using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept). See also
/// `CareTeam` and `Organization` for related workflow resources.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::episode_of_care::EpisodeOfCare;
/// use fhir::r5::types;
///
/// let value = EpisodeOfCare {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EpisodeOfCare = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCare {
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

    /// Business Identifier(s) relevant for this EpisodeOfCare
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Current lifecycle state of the episode: planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::EpisodeOfCareStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Past list of status codes (the current status may be included to cover the start date of the status)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_history: Vec<EpisodeOfCareStatusHistory>,

    /// Type/class  - e.g. specialist referral, disease management
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The list of medical reasons that are expected to be addressed during the episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<EpisodeOfCareReason>,

    /// The list of medical conditions that were addressed during the episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<EpisodeOfCareDiagnosis>,

    /// Reference to the [`Patient`](crate::r5::resources::patient::Patient) who is the focus of this episode of care
    pub patient: types::Reference<crate::r5::resources::Patient>,

    /// Organization that assumes overall responsibility for care coordination during the episode
    pub managing_organization: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Interval during which the managing organization's responsibility is assumed
    pub period: Option<types::Period>,

    /// Originating Referral Request(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referral_request: Vec<types::Reference<crate::r5::resources::ServiceRequest>>,

    /// Care manager/care coordinator for the patient
    pub care_manager: Option<types::Reference>,

    /// Other practitioners facilitating this episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<types::Reference<crate::r5::resources::CareTeam>>,

    /// The set of accounts that may be used for billing for this EpisodeOfCare
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference<crate::r5::resources::Account>>,
}

/// Past list of status codes (the current status may be included to cover the
/// start date of the status).
/// # Examples
///
/// ```
/// use fhir::r5::resources::episode_of_care::EpisodeOfCareStatusHistory;
/// use fhir::r5::types;
///
/// let value = EpisodeOfCareStatusHistory {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EpisodeOfCareStatusHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareStatusHistory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::EpisodeOfCareStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Duration the EpisodeOfCare was in the specified status
    pub period: types::Period,
}

/// The list of medical reasons that are expected to be addressed during the
/// episode of care.
/// # Examples
///
/// ```
/// use fhir::r5::resources::episode_of_care::EpisodeOfCareReason;
/// use fhir::r5::types;
///
/// let value = EpisodeOfCareReason {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EpisodeOfCareReason = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareReason {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What the reason value should be used for/as
    pub r#use: Option<types::CodeableConcept>,

    /// Medical reason to be addressed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<types::CodeableReference>,
}

/// The list of medical conditions that were addressed during the episode of
/// care.
/// # Examples
///
/// ```
/// use fhir::r5::resources::episode_of_care::EpisodeOfCareDiagnosis;
/// use fhir::r5::types;
///
/// let value = EpisodeOfCareDiagnosis {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EpisodeOfCareDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The medical condition that was addressed during the episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<types::CodeableReference>,

    /// Role that this diagnosis has within the episode of care (e.g. admission, billing, discharge …)
    pub r#use: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EpisodeOfCare;

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
