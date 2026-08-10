//! CareTeam
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CareTeam
//!
//! Version: 6.0.0-ballot3
//!
//! Planned participants in the coordination and delivery of care
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The Care Team includes all the people, organizations, and care teams who
/// participate or plan to participate in the coordination and delivery of
/// care.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::care_team::CareTeam;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CareTeam {
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

    /// External Ids for this team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// proposed | active | suspended | inactive | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r6::codes::CareTeamStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type of team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Name of the team, such as crisis assessment team
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Who care team is for
    pub subject: Option<types::Reference>,

    /// Time period team covers
    pub period: Option<types::Period>,

    /// Members of the team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<CareTeamParticipant>,

    /// Why the care team exists
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Organization responsible for the care team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub managing_organization: Vec<types::Reference<crate::r6::resources::Organization>>,

    /// A contact detail for the care team (that applies to all members)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Comments made about the CareTeam
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Identifies all people and organizations who are expected to be involved in
/// the care team.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::care_team::CareTeamParticipant;
/// use fhir::r6::types;
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
#[serde(from = "CareTeamParticipantDe")]
#[fhir_version("r6")]
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

    /// Entity that the participant is acting as a proxy of, or an agent of, or
    /// in the interest of, or as a representative of
    pub on_behalf_of: Option<types::Reference>,

    /// When the member is generally available within this care team
    /// The `CareTeam.participant.effective[x]` choice element (0..1); see [`CareTeamParticipantEffective`].
    #[serde(flatten)]
    pub effective: Option<CareTeamParticipantEffective>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CareTeamParticipantDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    role: Option<types::CodeableConcept>,
    member: Option<types::Reference>,
    on_behalf_of: Option<types::Reference>,
    #[serde(flatten)]
    effective: crate::r6::choice::Slot<CareTeamParticipantEffective>,
}

impl ::core::convert::From<CareTeamParticipantDe> for CareTeamParticipant {
    fn from(v: CareTeamParticipantDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            role: v.role,
            member: v.member,
            on_behalf_of: v.on_behalf_of,
            effective: v.effective.0,
        }
    }
}

/// The `CareTeam.participant.effective[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CareTeamParticipantEffective {
    /// `effectivePeriod` variant.
    #[fhir("effectivePeriod")]
    Period(Box<types::Period>),
    /// `effectiveTiming` variant.
    #[fhir("effectiveTiming")]
    Timing(Box<types::Timing>),
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
