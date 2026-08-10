//! CarePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CarePlan
//!
//!
//!
//! Healthcare plan for patient or group
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for CarePlan Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::care_plan::CarePlan;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// Fulfills care plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference<crate::r3::resources::CarePlan>>,

    /// CarePlan replaced by this CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference<crate::r3::resources::CarePlan>>,

    /// Part of referenced CarePlan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference<crate::r3::resources::CarePlan>>,

    /// draft | active | suspended | completed | entered-in-error | cancelled |
    /// unknown
    pub status: crate::coded::Coded<crate::r3::codes::CarePlanStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | order | option
    pub intent: crate::coded::Coded<crate::r3::codes::CarePlanIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Type of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Human-friendly name for the CarePlan
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

    /// Who care plan is for
    pub subject: types::Reference,

    /// Created in context of
    pub context: Option<types::Reference>,

    /// Time period plan covers
    pub period: Option<types::Period>,

    /// Who is responsible for contents of the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub author: Vec<types::Reference>,

    /// Who's involved in plan?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<types::Reference<crate::r3::resources::CareTeam>>,

    /// Health issues this plan addresses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::Reference<crate::r3::resources::Condition>>,

    /// Information considered as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Desired outcome of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference<crate::r3::resources::Goal>>,

    /// Action to occur as part of plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<CarePlanActivity>,

    /// Comments about the plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Identifies a planned action to occur as part of the plan. For example, a
/// medication to be used, lab tests to perform, self-monitoring, education,
/// etc.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::care_plan::CarePlanActivity;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct CarePlanActivity {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Results of the activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_codeable_concept: Vec<types::CodeableConcept>,

    /// Appointment, Encounter, Procedure, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_reference: Vec<types::Reference>,

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
/// use fhir::r3::resources::care_plan::CarePlanActivityDetail;
/// use fhir::r3::types;
///
/// let value = CarePlanActivityDetail {
///     status_reason: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusReason` is the name this serializes to on the wire.
/// assert_eq!(json["statusReason"], ::serde_json::json!("abc"));
///
/// let back: CarePlanActivityDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "CarePlanActivityDetailDe")]
#[fhir_version("r3")]
pub struct CarePlanActivityDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// diet | drug | encounter | observation | procedure | supply | other
    pub category: Option<types::CodeableConcept>,

    /// Protocol or definition
    pub definition: Option<types::Reference>,

    /// Detail type of activity
    pub code: Option<types::CodeableConcept>,

    /// Why activity should be done or why activity was prohibited
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition triggering need for activity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference<crate::r3::resources::Condition>>,

    /// Goals this activity relates to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub goal: Vec<types::Reference<crate::r3::resources::Goal>>,

    /// not-started | scheduled | in-progress | on-hold | completed | cancelled
    /// | unknown
    pub status: crate::coded::Coded<crate::r3::codes::CarePlanActivityStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::String>,
    /// Primitive extension sibling for [`status_reason`](Self::status_reason) (FHIR `_statusReason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusReason")]
    pub status_reason_ext: Option<types::Element>,

    /// Do NOT do
    pub prohibited: Option<types::Boolean>,
    /// Primitive extension sibling for [`prohibited`](Self::prohibited) (FHIR `_prohibited`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prohibited")]
    pub prohibited_ext: Option<types::Element>,

    /// When activity is to occur
    /// The `CarePlan.activity.detail.scheduled[x]` choice element (0..1); see [`CarePlanActivityDetailScheduled`].
    #[serde(flatten)]
    pub scheduled: Option<CarePlanActivityDetailScheduled>,

    /// Where it should happen
    pub location: Option<types::Reference<crate::r3::resources::Location>>,

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CarePlanActivityDetailDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    category: Option<types::CodeableConcept>,
    definition: Option<types::Reference>,
    code: Option<types::CodeableConcept>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference<crate::r3::resources::Condition>>,
    #[serde(default)]
    goal: Vec<types::Reference<crate::r3::resources::Goal>>,
    status: crate::coded::Coded<crate::r3::codes::CarePlanActivityStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    status_reason: Option<types::String>,
    #[serde(rename = "_statusReason")]
    status_reason_ext: Option<types::Element>,
    prohibited: Option<types::Boolean>,
    #[serde(rename = "_prohibited")]
    prohibited_ext: Option<types::Element>,
    #[serde(flatten)]
    scheduled: crate::r3::choice::Slot<CarePlanActivityDetailScheduled>,
    location: Option<types::Reference<crate::r3::resources::Location>>,
    #[serde(default)]
    performer: Vec<types::Reference>,
    #[serde(flatten)]
    product: crate::r3::choice::Slot<CarePlanActivityDetailProduct>,
    daily_amount: Option<types::Quantity>,
    quantity: Option<types::Quantity>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
}

impl ::core::convert::From<CarePlanActivityDetailDe> for CarePlanActivityDetail {
    fn from(v: CarePlanActivityDetailDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            category: v.category,
            definition: v.definition,
            code: v.code,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            goal: v.goal,
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            status_reason_ext: v.status_reason_ext,
            prohibited: v.prohibited,
            prohibited_ext: v.prohibited_ext,
            scheduled: v.scheduled.0,
            location: v.location,
            performer: v.performer,
            product: v.product.0,
            daily_amount: v.daily_amount,
            quantity: v.quantity,
            description: v.description,
            description_ext: v.description_ext,
        }
    }
}

/// The `CarePlan.activity.detail.scheduled[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
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
    String(crate::r3::choice::Primitive<types::String>),
}

/// The `CarePlan.activity.detail.product[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
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
