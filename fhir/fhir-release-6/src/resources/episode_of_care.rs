//! EpisodeOfCare
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
//!
//! Version: 6.0.0-ballot3
//!
//! An association of a Patient with an Organization and Healthcare Provider(s)
//! for a period of time that the Organization assumes some level of
//! responsibility
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An association between a patient and an organization / healthcare
/// provider(s) during which time encounters may occur. The managing
/// organization assumes a level of responsibility for the patient during this
/// time.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::episode_of_care::EpisodeOfCare;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct EpisodeOfCare {
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

    /// Business Identifier(s) relevant for this EpisodeOfCare
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// planned | waitlist | active | onhold | finished | cancelled |
    /// entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::EpisodeOfCareStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Past list of status codes (the current status may be included to cover
    /// the start date of the status)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_history: Vec<EpisodeOfCareStatusHistory>,

    /// Type/class - e.g. specialist referral, disease management
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The list of medical reasons that are expected to be addressed during
    /// the episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<EpisodeOfCareReason>,

    /// The list of medical conditions that were addressed during the episode
    /// of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<EpisodeOfCareDiagnosis>,

    /// The patient/group who is the focus of this episode of care
    pub subject: types::Reference,

    /// Organization that assumes responsibility for care coordination
    pub managing_organization: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Interval during responsibility is assumed
    pub period: Option<types::Period>,

    /// Originating Referral Request(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referral_request: Vec<types::Reference<crate::r6::resources::ServiceRequest>>,

    /// Care manager/care coordinator for the patient
    pub care_manager: Option<types::Reference>,

    /// Other practitioners facilitating this episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<types::Reference<crate::r6::resources::CareTeam>>,

    /// The set of accounts that may be used for billing for this EpisodeOfCare
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub account: Vec<types::Reference<crate::r6::resources::Account>>,
}

/// The list of medical conditions that were addressed during the episode of
/// care.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::episode_of_care::EpisodeOfCareDiagnosis;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Role that this diagnosis has within the episode of care (e.g.
    /// admission, billing, discharge …)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#use: Vec<types::CodeableConcept>,
}

/// The list of medical reasons that are expected to be addressed during the
/// episode of care.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::episode_of_care::EpisodeOfCareReason;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#use: Vec<types::CodeableConcept>,

    /// Medical reason to be addressed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<types::CodeableReference>,
}

/// The history of statuses that the EpisodeOfCare has been through (without
/// requiring processing the history of the resource).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::episode_of_care::EpisodeOfCareStatusHistory;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct EpisodeOfCareStatusHistory {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// planned | waitlist | active | onhold | finished | cancelled |
    /// entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::EpisodeOfCareStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Duration the EpisodeOfCare was in the specified status
    pub period: types::Period,
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
