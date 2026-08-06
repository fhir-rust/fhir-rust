//! EpisodeOfCare
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EpisodeOfCare
//!
//!
//!
//! An association of a Patient with an Organization and Healthcare Provider(s)
//! for a period of time that the Organization assumes some level of
//! responsibility
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for EpisodeOfCare Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::episode_of_care::EpisodeOfCare;
/// use fhir::r2::types;
///
/// let value = EpisodeOfCare {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: EpisodeOfCare = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct EpisodeOfCare {
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier(s) for the EpisodeOfCare
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// planned | waitlist | active | onhold | finished | cancelled
    pub status: crate::coded::Coded<crate::r2::codes::EpisodeOfCareStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Past list of status codes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_history: Vec<EpisodeOfCareStatusHistory>,

    /// Type/class - e.g. specialist referral, disease management
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Conditions/problems/diagnoses this episode of care is for
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<types::Reference>,

    /// Patient for this episode of care
    pub patient: types::Reference,

    /// Organization that assumes care
    pub managing_organization: Option<types::Reference>,

    /// Interval during responsibility is assumed
    pub period: Option<types::Period>,

    /// Originating Referral Request(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referral_request: Vec<types::Reference>,

    /// Care manager/care co-ordinator for the patient
    pub care_manager: Option<types::Reference>,

    /// Other practitioners facilitating this episode of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<EpisodeOfCareCareTeam>,
}

/// The list of practitioners that may be facilitating this episode of care for
/// specific purposes.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::episode_of_care::EpisodeOfCareCareTeam;
/// use fhir::r2::types;
///
/// let value = EpisodeOfCareCareTeam {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: EpisodeOfCareCareTeam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct EpisodeOfCareCareTeam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Role taken by this team member
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<types::CodeableConcept>,

    /// Period of time for this role
    pub period: Option<types::Period>,

    /// The practitioner (or Organization) within the team
    pub member: Option<types::Reference>,
}

/// The history of statuses that the EpisodeOfCare has been through (without
/// requiring processing the history of the resource).
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::episode_of_care::EpisodeOfCareStatusHistory;
/// use fhir::r2::types;
///
/// let value = EpisodeOfCareStatusHistory {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: EpisodeOfCareStatusHistory = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct EpisodeOfCareStatusHistory {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// planned | waitlist | active | onhold | finished | cancelled
    pub status: crate::coded::Coded<crate::r2::codes::EpisodeOfCareStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Period for the status
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
