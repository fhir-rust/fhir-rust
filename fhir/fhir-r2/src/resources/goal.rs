//! Goal
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Goal
//!
//!
//!
//! Describes the intended objective(s) for a patient, group or organization
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Goal Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::goal::Goal;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who this goal is intended for
    pub subject: Option<types::Reference>,

    /// When goal pursuit begins
    /// The `Goal.start[x]` choice element (0..1); see [`GoalStart`].
    #[serde(flatten)]
    pub start: Option<GoalStart>,

    /// Reach goal on or before
    /// The `Goal.target[x]` choice element (0..1); see [`GoalTarget`].
    #[serde(flatten)]
    pub target: Option<GoalTarget>,

    /// E.g. Treatment, dietary, behavioral, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// What's the desired outcome?
    pub description: types::String,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// proposed | planned | accepted | rejected | in-progress | achieved |
    /// sustaining | on-hold | cancelled
    pub status: crate::coded::Coded<crate::r2::codes::GoalStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// When goal status took effect
    pub status_date: Option<types::Date>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Who's responsible for creating Goal?
    pub author: Option<types::Reference>,

    /// high | medium |low
    pub priority: Option<types::CodeableConcept>,

    /// Issues addressed by this goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<types::Reference>,

    /// Comments about the goal
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// What was end result of goal?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome: Vec<GoalOutcome>,
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
    contained: Vec<crate::r2::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    subject: Option<types::Reference>,
    #[serde(flatten)]
    start: crate::r2::choice::Slot<GoalStart>,
    #[serde(flatten)]
    target: crate::r2::choice::Slot<GoalTarget>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    description: types::String,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    status: crate::coded::Coded<crate::r2::codes::GoalStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    status_date: Option<types::Date>,
    #[serde(rename = "_statusDate")]
    status_date_ext: Option<types::Element>,
    status_reason: Option<types::CodeableConcept>,
    author: Option<types::Reference>,
    priority: Option<types::CodeableConcept>,
    #[serde(default)]
    addresses: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    outcome: Vec<GoalOutcome>,
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
            subject: v.subject,
            start: v.start.0,
            target: v.target.0,
            category: v.category,
            description: v.description,
            description_ext: v.description_ext,
            status: v.status,
            status_ext: v.status_ext,
            status_date: v.status_date,
            status_date_ext: v.status_date_ext,
            status_reason: v.status_reason,
            author: v.author,
            priority: v.priority,
            addresses: v.addresses,
            note: v.note,
            outcome: v.outcome,
        }
    }
}

/// Identifies the change (or lack of change) at the point where the goal was
/// deepmed to be cancelled or achieved.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::goal::GoalOutcome;
/// use fhir::r2::types;
///
/// let value = GoalOutcome {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: GoalOutcome = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "GoalOutcomeDe")]
#[fhir_version("r2")]
pub struct GoalOutcome {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code or observation that resulted from goal
    /// The `Goal.outcome.result[x]` choice element (0..1); see [`GoalOutcomeResult`].
    #[serde(flatten)]
    pub result: Option<GoalOutcomeResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoalOutcomeDe {
    id: Option<types::Id>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    result: crate::r2::choice::Slot<GoalOutcomeResult>,
}

impl ::core::convert::From<GoalOutcomeDe> for GoalOutcome {
    fn from(v: GoalOutcomeDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            result: v.result.0,
        }
    }
}

/// The `Goal.start[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum GoalStart {
    /// `startDate` variant.
    #[fhir("startDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `startCodeableConcept` variant.
    #[fhir("startCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `Goal.target[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum GoalTarget {
    /// `targetDate` variant.
    #[fhir("targetDate")]
    Date(crate::r2::choice::Primitive<types::Date>),
    /// `targetQuantity` variant.
    #[fhir("targetQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `Goal.outcome.result[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum GoalOutcomeResult {
    /// `resultCodeableConcept` variant.
    #[fhir("resultCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `resultReference` variant.
    #[fhir("resultReference")]
    Reference(Box<types::Reference>),
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
