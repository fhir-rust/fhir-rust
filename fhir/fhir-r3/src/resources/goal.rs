//! Goal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Goal
//!
//!
//!
//! Describes the intended objective(s) for a patient, group or organization
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Goal Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::goal::Goal;
/// use fhir::r3::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "GoalDe")]
#[fhir_version("r3")]
pub struct Goal {
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

    /// External Ids for this goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// proposed | accepted | planned | in-progress | on-target |
    /// ahead-of-target | behind-target | sustaining | achieved | on-hold |
    /// cancelled | entered-in-error | rejected
    pub status: crate::coded::Coded<crate::r3::codes::GoalStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// E.g. Treatment, dietary, behavioral, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<types::CodeableConcept>,

    /// Code or text describing goal
    pub description: types::CodeableConcept,

    /// Who this goal is intended for
    pub subject: Option<types::Reference>,

    /// When goal pursuit begins
    /// The `Goal.start[x]` choice element (0..1); see [`GoalStart`].
    #[serde(flatten)]
    pub start: Option<GoalStart>,

    /// Target outcome for the goal
    pub target: Option<GoalTarget>,

    /// When goal status took effect
    pub status_date: Option<types::Date>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::String>,
    /// Primitive extension sibling for [`status_reason`](Self::status_reason) (FHIR `_statusReason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusReason")]
    pub status_reason_ext: Option<types::Element>,

    /// Who's responsible for creating Goal?
    pub expressed_by: Option<types::Reference>,

    /// Issues addressed by this goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::Reference>,

    /// Comments about the goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// What result was achieved regarding the goal?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_code: Vec<types::CodeableConcept>,

    /// Observation that resulted from goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome_reference: Vec<types::Reference<crate::r3::resources::Observation>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoalDe {
    id: Option<types::Id>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r3::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: crate::coded::Coded<crate::r3::codes::GoalStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    priority: Option<types::CodeableConcept>,
    description: types::CodeableConcept,
    subject: Option<types::Reference>,
    #[serde(flatten)]
    start: crate::r3::choice::Slot<GoalStart>,
    target: Option<GoalTarget>,
    status_date: Option<types::Date>,
    #[serde(rename = "_statusDate")]
    status_date_ext: Option<types::Element>,
    status_reason: Option<types::String>,
    #[serde(rename = "_statusReason")]
    status_reason_ext: Option<types::Element>,
    expressed_by: Option<types::Reference>,
    #[serde(default)]
    addresses: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    outcome_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    outcome_reference: Vec<types::Reference<crate::r3::resources::Observation>>,
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
            status: v.status,
            status_ext: v.status_ext,
            category: v.category,
            priority: v.priority,
            description: v.description,
            subject: v.subject,
            start: v.start.0,
            target: v.target,
            status_date: v.status_date,
            status_date_ext: v.status_date_ext,
            status_reason: v.status_reason,
            status_reason_ext: v.status_reason_ext,
            expressed_by: v.expressed_by,
            addresses: v.addresses,
            note: v.note,
            outcome_code: v.outcome_code,
            outcome_reference: v.outcome_reference,
        }
    }
}

/// Indicates what should be done by when.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::goal::GoalTarget;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct GoalTarget {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The parameter whose value is being tracked
    pub measure: Option<types::CodeableConcept>,

    /// The target value to be achieved
    /// The `Goal.target.detail[x]` choice element (0..1); see [`GoalTargetDetail`].
    #[serde(flatten)]
    pub detail: Option<GoalTargetDetail>,

    /// Reach goal on or before
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
    detail: crate::r3::choice::Slot<GoalTargetDetail>,
    #[serde(flatten)]
    due: crate::r3::choice::Slot<GoalTargetDue>,
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

/// The `Goal.start[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum GoalStart {
    /// `startDate` variant.
    #[fhir("startDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `startCodeableConcept` variant.
    #[fhir("startCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `Goal.target.detail[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
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
}

/// The `Goal.target.due[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum GoalTargetDue {
    /// `dueDate` variant.
    #[fhir("dueDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `dueDuration` variant.
    #[fhir("dueDuration")]
    Duration(Box<types::Duration>),
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
