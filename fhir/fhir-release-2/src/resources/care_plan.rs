//! CarePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CarePlan
//!
//!
//!
//! Healthcare plan for patient or group
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for CarePlan Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::care_plan::CarePlan;
/// use fhir::r2::types;
///
/// let value = CarePlan {
///     modified: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `modified` is the name this serializes to on the wire.
/// assert_eq!(json["modified"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CarePlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct CarePlan {
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

    /// External Ids for this plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who care plan is for
    pub subject: Option<types::Reference>,

    /// proposed | draft | active | completed | cancelled
    pub status: crate::coded::Coded<crate::r2::codes::CarePlanStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Created in context of
    pub context: Option<types::Reference>,

    /// Time period plan covers
    pub period: Option<types::Period>,

    /// Who is responsible for contents of the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// When last updated
    pub modified: Option<types::DateTime>,
    /// Primitive extension sibling for [`modified`](Self::modified) (FHIR `_modified`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modified")]
    pub modified_ext: Option<types::Element>,

    /// Type of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Summary of nature of plan
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Health issues this plan addresses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::Reference>,

    /// Information considered as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub support: Vec<types::Reference>,

    /// Plans related to this one
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_plan: Vec<CarePlanRelatedPlan>,

    /// Who's involved in plan?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<CarePlanParticipant>,

    /// Desired outcome of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference>,

    /// Action to occur as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<CarePlanActivity>,

    /// Comments about the plan
    pub note: Option<types::Annotation>,
}

/// Identifies a planned action to occur as part of the plan. For example, a
/// medication to be used, lab tests to perform, self-monitoring, education,
/// etc.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::care_plan::CarePlanActivity;
/// use fhir::r2::types;
///
/// let value = CarePlanActivity {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: CarePlanActivity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct CarePlanActivity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Appointments, orders, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action_resulting: Vec<types::Reference>,

    /// Comments about the activity status/progress
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub progress: Vec<types::Annotation>,

    /// Activity details defined in specific resource
    pub reference: Option<types::Reference>,

    /// In-line definition of activity
    pub detail: Option<CarePlanActivityDetail>,
}

/// A simple summary of a planned activity suitable for a general care plan
/// system (e.g. form driven) that doesn't know about specific resources such
/// as procedure etc.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::care_plan::CarePlanActivityDetail;
/// use fhir::r2::types;
///
/// let value = CarePlanActivityDetail {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: CarePlanActivityDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct CarePlanActivityDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// diet | drug | encounter | observation | procedure | supply | other
    pub category: Option<types::CodeableConcept>,

    /// Detail type of activity
    pub code: Option<types::CodeableConcept>,

    /// Why activity should be done
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition triggering need for activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Goals this activity relates to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference>,

    /// not-started | scheduled | in-progress | on-hold | completed | cancelled
    pub status: Option<crate::coded::Coded<crate::r2::codes::CarePlanActivityStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Do NOT do
    pub prohibited: types::Boolean,
    /// Primitive extension sibling for [`prohibited`](Self::prohibited) (FHIR `_prohibited`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prohibited")]
    pub prohibited_ext: Option<types::Element>,

    /// When activity is to occur
    /// The `CarePlan.activity.detail.scheduled[x]` choice element (0..1); see [`CarePlanActivityDetailScheduled`].
    #[serde(flatten)]
    pub scheduled: Option<CarePlanActivityDetailScheduled>,

    /// Where it should happen
    pub location: Option<types::Reference>,

    /// Who will be responsible?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// What is to be administered/supplied
    /// The `CarePlan.activity.detail.product[x]` choice element (0..1); see [`CarePlanActivityDetailProduct`].
    #[serde(flatten)]
    pub product: Option<CarePlanActivityDetailProduct>,

    /// How to consume/day?
    pub daily_amount: Option<types::Quantity>,

    /// How much to administer/supply/consume
    pub quantity: Option<types::Quantity>,

    /// Extra info describing activity to perform
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Identifies all people and organizations who are expected to be involved in
/// the care envisioned by this plan.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::care_plan::CarePlanParticipant;
/// use fhir::r2::types;
///
/// let value = CarePlanParticipant {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: CarePlanParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct CarePlanParticipant {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of involvement
    pub role: Option<types::CodeableConcept>,

    /// Who is involved
    pub member: Option<types::Reference>,
}

/// Identifies CarePlans with some sort of formal relationship to the current
/// plan.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::care_plan::CarePlanRelatedPlan;
/// use fhir::r2::types;
///
/// let value = CarePlanRelatedPlan {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: CarePlanRelatedPlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct CarePlanRelatedPlan {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// includes | replaces | fulfills
    pub code: Option<crate::coded::Coded<crate::r2::codes::CarePlanRelationship>>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Plan relationship exists with
    pub plan: types::Reference,
}

/// The `CarePlan.activity.detail.scheduled[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum CarePlanActivityDetailScheduled {
    /// `scheduledTiming` variant.
    #[fhir("scheduledTiming")]
    Timing(Box<types::Timing>),
    /// `scheduledPeriod` variant.
    #[fhir("scheduledPeriod")]
    Period(Box<types::Period>),
    /// `scheduledString` variant.
    #[fhir("scheduledString")]
    String(crate::r2::choice::Primitive<types::String>),
}

/// The `CarePlan.activity.detail.product[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum CarePlanActivityDetailProduct {
    /// `productCodeableConcept` variant.
    #[fhir("productCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `productReference` variant.
    #[fhir("productReference")]
    Reference(Box<types::Reference>),
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
