//! CarePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CarePlan
//!
//! Version: 5.0.0
//!
//! CarePlan Resource: Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Describes the intention of how one or more practitioners intend to deliver
/// care for a particular patient, group or community for a period of time,
/// possibly limited to care for a specific condition or set of conditions.
///
/// The CarePlan resource is used across many care settings to coordinate the
/// planned and ongoing activities directed at addressing a patient's health
/// concerns. It links the subject of care to the goals, addressed conditions,
/// care team, and the set of intended or performed activities that make up the
/// plan. A CarePlan may be created by a single practitioner for straightforward
/// needs, or it may be built collaboratively by a multi-disciplinary care team
/// spanning several encounters and organizations, and it can be nested or
/// chained via `part_of` and `replaces` so that plans can be superseded or
/// composed of sub-plans over time. Because it is intentional in nature, a
/// CarePlan records what is planned or has occurred, not the clinical findings
/// themselves, and it is often driven by, or drives, order and workflow
/// resources through its `based_on` and `activity` elements.
///
/// In FHIR R5 it commonly references `Goal`, `CareTeam`, and `Condition`
/// resources to build a comprehensive, longitudinal view of a patient's care.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) - the typical subject of a care plan.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) - used for the plan's category.
/// - `Goal`, `CareTeam`, `Condition`, and `Encounter` - frequently associated resources.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::care_plan::CarePlan;
/// use fhir::r5::types;
///
/// let value = CarePlan {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: CarePlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CarePlan {
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

    /// External Ids for this plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// Fulfills plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// CarePlan replaced by this CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Part of referenced CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Indicates whether the plan is currently in effect: draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Indicates the level of authority/intentionality behind the plan: proposal | plan | order | option | directive
    pub intent: types::Code,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Type of plan, such as a disease-specific or discharge-oriented plan of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Human-friendly name for the care plan
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Summary of nature of plan
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who the care plan is for, typically a reference to a [`Patient`](crate::r5::resources::patient::Patient) or group
    pub subject: types::Reference,

    /// The Encounter during which this CarePlan was created
    pub encounter: Option<types::Reference>,

    /// Time period plan covers
    pub period: Option<types::Period>,

    /// Date record was first recorded
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Who is the designated responsible party
    pub custodian: Option<types::Reference>,

    /// Who provided the content of the care plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributor: Vec<types::Reference>,

    /// Who's involved in plan?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<types::Reference>,

    /// Health issues this plan addresses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::CodeableReference>,

    /// Information considered as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Desired outcome of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference>,

    /// The individual planned or performed activities that make up the plan of care
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<CarePlanActivity>,

    /// Comments about the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Action to occur or has occurred as part of plan.
///
/// Identifies an action that has occurred or is a planned action to occur as
/// part of the plan. For example, a medication to be used, lab tests to perform,
/// self-monitoring, or education.
/// # Examples
///
/// ```
/// use fhir::r5::resources::care_plan::CarePlanActivity;
/// use fhir::r5::types;
///
/// let value = CarePlanActivity {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CarePlanActivity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CarePlanActivity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Results of the activity (concept, or Appointment, Encounter, Procedure, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performed_activity: Vec<types::CodeableReference>,

    /// Comments about the activity status/progress
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub progress: Vec<types::Annotation>,

    /// Activity that is intended to be part of the care plan
    pub planned_activity_reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CarePlan;

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
