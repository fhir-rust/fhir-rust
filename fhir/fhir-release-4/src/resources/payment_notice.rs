//! PaymentNotice
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentNotice
//!
//! Version: 4.0.1
//!
//! PaymentNotice request
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource provides the status of the payment for goods and services
/// rendered, and the request and response resource references.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::payment_notice::PaymentNotice;
/// use fhir::r4::types;
///
/// let value = PaymentNotice {
///     payment_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `paymentDate` is the name this serializes to on the wire.
/// assert_eq!(json["paymentDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: PaymentNotice = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct PaymentNotice {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for the payment noctice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Request reference
    pub request: Option<types::Reference>,

    /// Response reference
    pub response: Option<types::Reference>,

    /// Creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Responsible practitioner
    pub provider: Option<types::Reference>,

    /// Payment reference
    pub payment: types::Reference,

    /// Payment or clearing date
    pub payment_date: Option<types::Date>,
    /// Primitive extension sibling for [`payment_date`](Self::payment_date) (FHIR `_paymentDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paymentDate")]
    pub payment_date_ext: Option<types::Element>,

    /// Party being paid
    pub payee: Option<types::Reference>,

    /// Party being notified
    pub recipient: types::Reference,

    /// Monetary amount of the payment
    pub amount: types::Money,

    /// Issued or cleared Status of the payment
    pub payment_status: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PaymentNotice;

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
