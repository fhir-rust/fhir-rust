//! Permission
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Permission
//!
//! Version: 6.0.0-ballot3
//!
//! Access Rules
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Permission resource holds access rules for a given data and access request
/// context.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::Permission;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Permission {
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

    /// Business Identifier for permission
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | entered-in-error | draft | rejected
    pub status: crate::coded::Coded<crate::r6::codes::PermissionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The person or entity that asserts the permission
    pub asserter: Option<types::Reference>,

    /// The date that permission was asserted
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub date: ::fhir_core::PrimVec<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_ext: Vec<Option<types::Element>>,

    /// The period in which the permission is active
    pub validity: Option<types::Period>,

    /// The asserted justification for using the data
    pub justification: Option<PermissionJustification>,

    /// deny-overrides | permit-overrides | ordered-deny-overrides |
    /// ordered-permit-overrides | deny-unless-permit | permit-unless-deny
    pub combining: crate::coded::Coded<crate::r6::codes::PermissionRuleCombining>,
    /// Primitive extension sibling for [`combining`](Self::combining) (FHIR `_combining`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_combining")]
    pub combining_ext: Option<types::Element>,

    /// Constraints to the Permission
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<PermissionRule>,
}

/// The asserted justification for using the data.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::PermissionJustification;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// A set of rules.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::PermissionRule;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct PermissionRule {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to a Permission
    pub import: Option<types::Reference<crate::r6::resources::Permission>>,

    /// deny | permit
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::ConsentProvisionType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The selection criteria to identify data that is within scope of this
    /// provision
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<PermissionRuleData>,

    /// A description or definition of which activities are allowed to be done
    /// on the data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activity: Vec<PermissionRuleActivity>,

    /// What limits apply to the use of the data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limit: Vec<PermissionRuleLimit>,
}

/// A description or definition of which activities are allowed to be done on
/// the data.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::PermissionRuleActivity;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct PermissionRuleActivity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who|what is controlled by this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<PermissionRuleActivityActor>,

    /// Actions controlled by this rule
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<types::CodeableConcept>,

    /// The purpose for which the permission is given
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose: Vec<types::CodeableConcept>,
}

/// Who or what is controlled by this rule. Use group to identify a set of
/// actors by some property they share (e.g. 'admitting officers').
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::PermissionRuleActivityActor;
/// use fhir::r6::types;
///
/// let value = PermissionRuleActivityActor {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PermissionRuleActivityActor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PermissionRuleActivityActor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// How the actor is involved
    pub role: Option<types::CodeableConcept>,

    /// Authorized actor(s)
    pub reference: Option<types::Reference>,
}

/// A description or definition of which activities are allowed to be done on
/// the data.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::PermissionRuleData;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub period: Option<types::Period>,

    /// Expression identifying the data
    pub expression: Option<types::Expression>,
}

/// Explicit FHIR Resource references.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::PermissionRuleDataResource;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub meaning: crate::coded::Coded<crate::r6::codes::ConsentDataMeaning>,
    /// Primitive extension sibling for [`meaning`](Self::meaning) (FHIR `_meaning`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_meaning")]
    pub meaning_ext: Option<types::Element>,

    /// The actual data reference
    pub reference: types::Reference,
}

/// What restrictions must be applied to the use of the data by the actor.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::permission::PermissionRuleLimit;
/// use fhir::r6::types;
///
/// let value = PermissionRuleLimit {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PermissionRuleLimit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PermissionRuleLimit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What coded limits apply to the use of the data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub control: Vec<types::CodeableConcept>,

    /// The sensitivity codes that must be removed from the data
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<types::Coding>,

    /// What data elements that must be removed from the data
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub element: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`element`](Self::element) (FHIR `_element`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_element")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub element_ext: Vec<Option<types::Element>>,
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
