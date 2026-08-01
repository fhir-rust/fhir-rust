//! CarePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CarePlan
//!
//! Version: 6.0.0-ballot3
//!
//! Healthcare plan for patient or group
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes the intention of how one or more practitioners intend to deliver
/// care for a particular patient, group or community for a period of time,
/// possibly limited to care for a specific condition or set of conditions.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::care_plan::CarePlan;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CarePlan {
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

    /// External Ids for this plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Fulfills plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// CarePlan replaced by this CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference>,

    /// Part of referenced CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// draft | active | on-hold | entered-in-error | ended | completed |
    /// revoked | unknown
    pub status: crate::coded::Coded<crate::r6::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | order | option | directive
    pub intent: types::Code,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Type of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Human-friendly name for the care plan
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Summary of nature of plan
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who the care plan is for
    pub subject: types::Reference,

    /// The Encounter during which this CarePlan was created
    pub encounter: Option<types::Reference>,

    /// Time period plan covers
    pub period: Option<types::Period>,

    /// Date record was first recorded
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
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

    /// Action to occur or has occurred as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<CarePlanActivity>,

    /// Comments about the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Identifies an action that has occurred or is a planned action to occur as
/// part of the plan. For example, a medication to be used, lab tests to
/// perform, self-monitoring that has occurred, education etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::care_plan::CarePlanActivity;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct CarePlanActivity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Activities that are completed or in progress (concept, or Appointment,
    /// Encounter, Procedure, etc.)
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
