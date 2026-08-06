//! Permission
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Permission
//!
//! Version: 5.0.0
//!
//! Permission Resource: Permission resource holds access rules for a given data and context.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The Permission resource holds access rules for a given data and context.
///
/// In FHIR R5 the Permission resource captures a machine-processable authorization
/// policy: it expresses the set of constraints under which specific data may be
/// accessed or acted upon. Each Permission carries one or more rules that either
/// permit or deny particular activities, scoped by the actors involved, the
/// purposes of use, the actions performed, and the data selected by explicit
/// references, security labels, time periods, or FHIRPath expressions. A Permission
/// is asserted by a person or organization, may be constrained to a validity
/// period, and specifies a combining algorithm (for example deny-overrides or
/// permit-overrides) that determines how its rules are reconciled when more than
/// one applies. This makes it well suited to modeling fine-grained access-control
/// decisions, security policies, and the enforceable representation of a patient's
/// or organization's data-sharing directives.
///
/// # Related resources
///
/// A Permission frequently complements a broader
/// [`Consent`](crate::r5::resources::consent::Consent), which records a subject's
/// wishes, while the Permission expresses the enforceable rules derived from them.
/// Rules commonly reference actors and data such as
/// [`Patient`](crate::r5::resources::patient::Patient) records and audit trails
/// like [`Provenance`](crate::r5::resources::provenance::Provenance), and they
/// classify activities and limits using
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::permission::Permission;
/// use fhir::r5::types;
///
/// let value = Permission {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Permission = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
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

    /// Lifecycle state of the permission: active, entered-in-error, draft, or rejected.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PermissionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reference to the person or entity that asserts this permission and its rules.
    pub asserter: Option<types::Reference>,

    /// The date(s) on which the permission was asserted by the asserter.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date: Vec<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_ext: Vec<Option<types::Element>>,

    /// The period during which this permission's rules are in effect.
    pub validity: Option<types::Period>,

    /// The legal or regulatory basis and supporting evidence justifying the use of the data.
    pub justification: Option<PermissionJustification>,

    /// Combining algorithm that reconciles conflicting rules: deny-overrides, permit-overrides, ordered-deny-overrides, ordered-permit-overrides, deny-unless-permit, or permit-unless-deny.
    pub combining: crate::r5::coded::Coded<crate::r5::codes::PermissionRuleCombining>,
    /// Primitive extension sibling for [`combining`](Self::combining) (FHIR `_combining`).
    #[serde(rename = "_combining")]
    pub combining_ext: Option<types::Element>,

    /// The ordered set of rules that constrain access under this permission.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<PermissionRule>,
}

/// The asserted justification for using the data.
/// # Examples
///
/// ```
/// use fhir::r5::resources::permission::PermissionJustification;
/// use fhir::r5::types;
///
/// let value = PermissionJustification {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PermissionJustification = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionJustification {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The regulatory grounds upon which this Permission builds
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basis: Vec<types::CodeableConcept>,

    /// Justifing rational
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<types::Reference>,
}

/// Constraints to the Permission.
/// # Examples
///
/// ```
/// use fhir::r5::resources::permission::PermissionRule;
/// use fhir::r5::types;
///
/// let value = PermissionRule {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PermissionRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// deny | permit
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::ConsentProvisionType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The selection criteria to identify data that is within scope of this provision
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<PermissionRuleData>,

    /// A description or definition of which activities are allowed to be done on the data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<PermissionRuleActivity>,

    /// What limits apply to the use of the data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limit: Vec<types::CodeableConcept>,
}

/// The selection criteria to identify data that is within scope of this provision.
/// # Examples
///
/// ```
/// use fhir::r5::resources::permission::PermissionRuleData;
/// use fhir::r5::types;
///
/// let value = PermissionRuleData {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PermissionRuleData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Explicit FHIR Resource references
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<PermissionRuleDataResource>,

    /// Security tag code on .meta.security
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<types::Coding>,

    /// Timeframe encompasing data create/update
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub period: Vec<types::Period>,

    /// Expression identifying the data
    pub expression: Option<types::Expression>,
}

/// Explicit FHIR Resource references.
/// # Examples
///
/// ```
/// use fhir::r5::resources::permission::PermissionRuleDataResource;
/// use fhir::r5::types;
///
/// let value = PermissionRuleDataResource {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PermissionRuleDataResource = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleDataResource {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// instance | related | dependents | authoredby
    pub meaning: crate::r5::coded::Coded<crate::r5::codes::ConsentDataMeaning>,
    /// Primitive extension sibling for [`meaning`](Self::meaning) (FHIR `_meaning`).
    #[serde(rename = "_meaning")]
    pub meaning_ext: Option<types::Element>,

    /// The actual data reference
    pub reference: types::Reference,
}

/// A description or definition of which activities are allowed to be done on the data.
/// # Examples
///
/// ```
/// use fhir::r5::resources::permission::PermissionRuleActivity;
/// use fhir::r5::types;
///
/// let value = PermissionRuleActivity {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PermissionRuleActivity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleActivity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Authorized actor(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<types::Reference>,

    /// Actions controlled by this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// The purpose for which the permission is given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Permission;

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
