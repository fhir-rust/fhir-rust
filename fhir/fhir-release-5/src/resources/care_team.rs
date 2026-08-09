//! CareTeam
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CareTeam
//!
//! Version: 5.0.0
//!
//! CareTeam Resource: The Care Team includes all the people and organizations who plan to participate in the coordination and delivery of care.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Care Team includes all the people and organizations who plan to
/// participate in the coordination and delivery of care.
///
/// A CareTeam resource identifies the group of practitioners, patients,
/// caregivers, and organizations that work together to coordinate and deliver
/// care for a particular subject, condition, or episode. Each participant may
/// have a specified role, coverage period, and the organization on whose behalf
/// they act. In FHIR R5 it is frequently referenced by CarePlan and other
/// workflow resources to describe who is responsible for a patient's care.
/// A CareTeam is commonly used to answer "who is on the team looking after
/// this patient" for a given condition, episode of care, or period of time,
/// and it supports coordination across clinicians, family members, and
/// community or social service organizations.
///
/// See also: [`CodeableConcept`](crate::r5::types::CodeableConcept) for the
/// `category` and `status`-adjacent coded values, and `Patient` or `Group`
/// as the typical `subject` of a care team. Related workflow resources
/// include `CarePlan` and `Encounter`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::care_team::CareTeam;
/// use fhir::r5::types;
///
/// let value = CareTeam {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: CareTeam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CareTeam {
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

    /// External Ids for this team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The lifecycle status of the team: proposed | active | suspended | inactive | entered-in-error.
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::CareTeamStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type or focus of team, such as encounter-focused or condition-focused.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Name of the team, such as crisis assessment team
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The patient or group that this care team is organized to provide care for.
    pub subject: Option<types::Reference>,

    /// Time period team covers
    pub period: Option<types::Period>,

    /// The individual members of the care team, each with an optional role and coverage period.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<CareTeamParticipant>,

    /// Why the care team exists
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Organization responsible for the care team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub managing_organization: Vec<types::Reference<crate::r5::resources::Organization>>,

    /// A contact detail for the care team (that applies to all members)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Comments made about the CareTeam
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Members of the team.
///
/// Identifies all people and organizations who are expected to be involved in
/// the care team, along with their role, the coverage period during which they
/// are generally available, and the organization on whose behalf they act.
/// # Examples
///
/// ```
/// use fhir::r5::resources::care_team::CareTeamParticipant;
/// use fhir::r5::types;
///
/// let value = CareTeamParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CareTeamParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CareTeamParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of involvement
    pub role: Option<types::CodeableConcept>,

    /// Who is involved
    pub member: Option<types::Reference>,

    /// Organization of the practitioner
    pub on_behalf_of: Option<types::Reference<crate::r5::resources::Organization>>,

    /// The `CareTeam.participant.coverage[x]` choice element (0..1); see [`CareTeamParticipantCoverage`].
    #[serde(flatten)]
    pub coverage: Option<CareTeamParticipantCoverage>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CareTeam;

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
/// The `CareTeam.participant.coverage[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CareTeamParticipantCoverage {
    /// `coveragePeriod` variant.
    #[fhir("coveragePeriod")]
    Period(Box<types::Period>),
    /// `coverageTiming` variant.
    #[fhir("coverageTiming")]
    Timing(Box<types::Timing>),
}
