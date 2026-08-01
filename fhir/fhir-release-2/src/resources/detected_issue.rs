//! DetectedIssue
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DetectedIssue
//!
//!
//!
//! Clinical issue with action
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DetectedIssue Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::detected_issue::DetectedIssue;
/// use fhir::r2::types;
///
/// let value = DetectedIssue {
///     detail: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `detail` is the name this serializes to on the wire.
/// assert_eq!(json["detail"], ::serde_json::json!("abc"));
///
/// let back: DetectedIssue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DetectedIssue {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Associated patient
    pub patient: Option<types::Reference>,

    /// Issue Category, e.g. drug-drug, duplicate therapy, etc.
    pub category: Option<types::CodeableConcept>,

    /// high | moderate | low
    pub severity: Option<crate::coded::Coded<crate::r2::codes::DetectedissueSeverity>>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// Problem resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub implicated: Vec<types::Reference>,

    /// Description and context
    pub detail: Option<types::String>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,

    /// When identified
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The provider or device that identified the issue
    pub author: Option<types::Reference>,

    /// Unique id for the detected issue
    pub identifier: Option<types::Identifier>,

    /// Authority for issue
    pub reference: Option<types::Uri>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Step taken to address
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mitigation: Vec<DetectedIssueMitigation>,
}

/// Indicates an action that has been taken or is committed to to reduce or
/// eliminate the likelihood of the risk identified by the detected issue from
/// manifesting. Can also reflect an observation of known mitigating factors
/// that may reduce/eliminate the need for any action.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::detected_issue::DetectedIssueMitigation;
/// use fhir::r2::types;
///
/// let value = DetectedIssueMitigation {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DetectedIssueMitigation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DetectedIssueMitigation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What mitigation?
    pub action: types::CodeableConcept,

    /// Date committed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is committing?
    pub author: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DetectedIssue;

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
