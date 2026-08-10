//! Goal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Goal
//!
//! Version: 5.0.0
//!
//! Goal Resource: Describes the intended objective(s) for a patient, group or organization care.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Describes the intended objective(s) for a patient, group or organization
/// care, for example, weight loss, restoring an activity of daily living,
/// obtaining herd immunity via immunization, meeting a process improvement
/// objective, etc. A Goal captures the desired outcome and, optionally, the
/// specific measurable targets that indicate progress toward that outcome.
///
/// Goals are used throughout clinical and administrative workflows to record
/// what a care team, patient, or organization is trying to achieve, and to
/// track progress over time. A Goal is often referenced from a care plan and
/// may be associated with one or more conditions, observations, or other
/// clinical findings that motivated it. Each Goal has a lifecycle status
/// (such as `proposed`, `active`, or `completed`) and, separately, an
/// optional achievement status describing how well the goal is being met.
/// The `target` element allows one or more measurable outcomes to be
/// specified, each expressed using a quantity, range, ratio, or other value
/// type, optionally with a due date or duration by which it should be met.
///
/// # Related resources
///
/// A Goal's `subject` is commonly a [`Patient`](crate::r5::resources::patient::Patient),
/// though it may also reference a group or organization. Goals are typically
/// linked from a care plan and may reference conditions or observations as
/// the basis for the goal; codes describing categories, priority, and
/// achievement status use [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::goal::Goal;
/// use fhir::r5::types;
///
/// let value = Goal {
///     status_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusDate` is the name this serializes to on the wire.
/// assert_eq!(json["statusDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: Goal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "GoalDe")]
pub struct Goal {
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

    /// External Ids for this goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The current lifecycle status of this goal: proposed | planned | accepted | active | on-hold | completed | cancelled | entered-in-error | rejected
    pub lifecycle_status: crate::r5::coded::Coded<crate::r5::codes::GoalStatus>,
    /// Primitive extension sibling for [`lifecycle_status`](Self::lifecycle_status) (FHIR `_lifecycleStatus`).
    #[serde(rename = "_lifecycleStatus")]
    pub lifecycle_status_ext: Option<types::Element>,

    /// Describes progress toward meeting the goal: in-progress | improving | worsening | no-change | achieved | sustaining | not-achieved | no-progress | not-attainable
    pub achievement_status: Option<types::CodeableConcept>,

    /// E.g. Treatment, dietary, behavioral, etc
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// After meeting the goal, ongoing activity is needed to sustain the goal objective
    pub continuous: Option<types::Boolean>,
    /// Primitive extension sibling for [`continuous`](Self::continuous) (FHIR `_continuous`).
    #[serde(rename = "_continuous")]
    pub continuous_ext: Option<types::Element>,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<types::CodeableConcept>,

    /// Human-readable code or text describing the goal to be achieved
    pub description: types::CodeableConcept,

    /// The patient, group, or organization for whom this goal is intended
    pub subject: types::Reference,

    /// The `Goal.start[x]` choice element (0..1); see [`GoalStart`].
    #[serde(flatten)]
    pub start: Option<GoalStart>,

    /// One or more measurable target outcomes that define what should be achieved
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<GoalTarget>,

    /// When goal status took effect
    pub status_date: Option<types::Date>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`).
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::String>,
    /// Primitive extension sibling for [`status_reason`](Self::status_reason) (FHIR `_statusReason`).
    #[serde(rename = "_statusReason")]
    pub status_reason_ext: Option<types::Element>,

    /// Who's responsible for creating Goal?
    pub source: Option<types::Reference>,

    /// Issues addressed by this goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::Reference>,

    /// Comments about the goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// What result was achieved regarding the goal?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome: Vec<types::CodeableReference>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoalDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    lifecycle_status: crate::r5::coded::Coded<crate::r5::codes::GoalStatus>,
    #[serde(rename = "_lifecycleStatus")]
    lifecycle_status_ext: Option<types::Element>,
    achievement_status: Option<types::CodeableConcept>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    continuous: Option<types::Boolean>,
    #[serde(rename = "_continuous")]
    continuous_ext: Option<types::Element>,
    priority: Option<types::CodeableConcept>,
    description: types::CodeableConcept,
    subject: types::Reference,
    #[serde(flatten)]
    start: crate::r5::choice::Slot<GoalStart>,
    #[serde(default)]
    target: Vec<GoalTarget>,
    status_date: Option<types::Date>,
    #[serde(rename = "_statusDate")]
    status_date_ext: Option<types::Element>,
    status_reason: Option<types::String>,
    #[serde(rename = "_statusReason")]
    status_reason_ext: Option<types::Element>,
    source: Option<types::Reference>,
    #[serde(default)]
    addresses: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    outcome: Vec<types::CodeableReference>,
}

impl ::core::convert::From<GoalDe> for Goal {
    fn from(v: GoalDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            lifecycle_status: v.lifecycle_status,
            lifecycle_status_ext: v.lifecycle_status_ext,
            achievement_status: v.achievement_status,
            category: v.category,
            continuous: v.continuous,
            continuous_ext: v.continuous_ext,
            priority: v.priority,
            description: v.description,
            subject: v.subject,
            start: v.start.0,
            target: v.target,
            status_date: v.status_date,
            status_date_ext: v.status_date_ext,
            status_reason: v.status_reason,
            status_reason_ext: v.status_reason_ext,
            source: v.source,
            addresses: v.addresses,
            note: v.note,
            outcome: v.outcome,
        }
    }
}

/// Target outcome for the goal.
///
/// Indicates what should be done and by when to consider progress toward the
/// goal to have been made. A goal may have zero or more targets.
/// # Examples
///
/// ```
/// use fhir::r5::resources::goal::GoalTarget;
/// use fhir::r5::types;
///
/// let value = GoalTarget {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: GoalTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "GoalTargetDe")]
pub struct GoalTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The parameter whose value is being tracked
    pub measure: Option<types::CodeableConcept>,

    /// The `Goal.target.detail[x]` choice element (0..1); see [`GoalTargetDetail`].
    #[serde(flatten)]
    pub detail: Option<GoalTargetDetail>,

    /// The `Goal.target.due[x]` choice element (0..1); see [`GoalTargetDue`].
    #[serde(flatten)]
    pub due: Option<GoalTargetDue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoalTargetDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    measure: Option<types::CodeableConcept>,
    #[serde(flatten)]
    detail: crate::r5::choice::Slot<GoalTargetDetail>,
    #[serde(flatten)]
    due: crate::r5::choice::Slot<GoalTargetDue>,
}

impl ::core::convert::From<GoalTargetDe> for GoalTarget {
    fn from(v: GoalTargetDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            measure: v.measure,
            detail: v.detail.0,
            due: v.due.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Goal;

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
/// The `Goal.start[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum GoalStart {
    /// `startDate` variant.
    #[fhir("startDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `startCodeableConcept` variant.
    #[fhir("startCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `Goal.target.detail[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum GoalTargetDetail {
    /// `detailQuantity` variant.
    #[fhir("detailQuantity")]
    Quantity(Box<types::Quantity>),
    /// `detailRange` variant.
    #[fhir("detailRange")]
    Range(Box<types::Range>),
    /// `detailCodeableConcept` variant.
    #[fhir("detailCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `detailString` variant.
    #[fhir("detailString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `detailBoolean` variant.
    #[fhir("detailBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `detailInteger` variant.
    #[fhir("detailInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `detailRatio` variant.
    #[fhir("detailRatio")]
    Ratio(Box<types::Ratio>),
}

/// The `Goal.target.due[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum GoalTargetDue {
    /// `dueDate` variant.
    #[fhir("dueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `dueDuration` variant.
    #[fhir("dueDuration")]
    Duration(Box<types::Duration>),
}
