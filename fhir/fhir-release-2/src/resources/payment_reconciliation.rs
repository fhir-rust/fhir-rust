//! PaymentReconciliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
//!
//!
//!
//! PaymentReconciliation resource
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for PaymentReconciliation Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::payment_reconciliation::PaymentReconciliation;
/// use fhir::r2::types;
///
/// let value = PaymentReconciliation {
///     disposition: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `disposition` is the name this serializes to on the wire.
/// assert_eq!(json["disposition"], ::serde_json::json!("abc"));
///
/// let back: PaymentReconciliation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct PaymentReconciliation {
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

    /// Business Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Claim reference
    pub request: Option<types::Reference>,

    /// complete | error
    pub outcome: Option<crate::coded::Coded<crate::r2::codes::RemittanceOutcome>>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// Resource version
    pub ruleset: Option<types::Coding>,

    /// Original version
    pub original_ruleset: Option<types::Coding>,

    /// Creation date
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Period covered
    pub period: Option<types::Period>,

    /// Insurer
    pub organization: Option<types::Reference>,

    /// Responsible practitioner
    pub request_provider: Option<types::Reference>,

    /// Responsible organization
    pub request_organization: Option<types::Reference>,

    /// Details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<PaymentReconciliationDetail>,

    /// Printed Form Identifier
    pub form: Option<types::Coding>,

    /// Total amount of Payment
    pub total: types::Quantity,

    /// Note text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<PaymentReconciliationNote>,
}

/// List of individual settlement amounts and the corresponding transaction.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::payment_reconciliation::PaymentReconciliationDetail;
/// use fhir::r2::types;
///
/// let value = PaymentReconciliationDetail {
///     date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01"));
///
/// let back: PaymentReconciliationDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct PaymentReconciliationDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type code
    pub r#type: types::Coding,

    /// Claim
    pub request: Option<types::Reference>,

    /// Claim Response
    pub responce: Option<types::Reference>,

    /// Submitter
    pub submitter: Option<types::Reference>,

    /// Payee
    pub payee: Option<types::Reference>,

    /// Invoice date
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Detail amount
    pub amount: Option<types::Quantity>,
}

/// Suite of notes.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::payment_reconciliation::PaymentReconciliationNote;
/// use fhir::r2::types;
///
/// let value = PaymentReconciliationNote {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: PaymentReconciliationNote = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct PaymentReconciliationNote {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// display | print | printoper
    pub r#type: Option<types::Coding>,

    /// Notes text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PaymentReconciliation;

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
