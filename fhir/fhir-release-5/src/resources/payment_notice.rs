//! PaymentNotice
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentNotice
//!
//! Version: 5.0.0
//!
//! PaymentNotice Resource: This resource provides the status of the payment for goods and services rendered, and the request and response resource references.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource provides the status of the payment for goods and services
/// rendered.
///
/// PaymentNotice is a financial administrative resource used to notify a payer,
/// provider, or other interested party of the status of a payment relating to a
/// claim, invoice, or other financial transaction. In FHIR R5 it typically
/// travels between providers, payers, and clearinghouses as part of the revenue
/// cycle, confirming that a payment has been issued, cleared, or is otherwise
/// expected. The resource references the originating request and its response,
/// records the creation date and optional payment or clearing date, and reports
/// the monetary amount, the party being paid (the payee), and the party being
/// notified (the recipient). Downstream systems commonly use it to reconcile
/// accounts and to trigger or confirm follow-up financial processing.
///
/// # Related resources
///
/// The referenced payment detail is usually carried by a
/// [`PaymentReconciliation`](crate::r5::resources::payment_reconciliation::PaymentReconciliation)
/// resource, and the notified or paying party is frequently an
/// [`Organization`](crate::r5::resources::organization::Organization). The
/// monetary amount is expressed as a [`Money`](crate::r5::types::Money) value
/// and the payment status uses a
/// [`CodeableConcept`](crate::r5::types::CodeableConcept). The originating
/// `request` and `response` typically reference a `Claim` and `ClaimResponse`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::payment_notice::PaymentNotice;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentNotice {
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

    /// Business Identifier for the payment notice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Lifecycle state of this notice: active, cancelled, draft, or entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Request reference
    pub request: Option<types::Reference>,

    /// Response reference
    pub response: Option<types::Reference>,

    /// Creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Responsible practitioner
    pub reporter: Option<types::Reference>,

    /// Payment reference
    pub payment: Option<types::Reference<crate::r5::resources::PaymentReconciliation>>,

    /// Payment or clearing date
    pub payment_date: Option<types::Date>,
    /// Primitive extension sibling for [`payment_date`](Self::payment_date) (FHIR `_paymentDate`).
    #[serde(rename = "_paymentDate")]
    pub payment_date_ext: Option<types::Element>,

    /// Party being paid
    pub payee: Option<types::Reference>,

    /// Reference to the party being notified of the payment, such as the payer or provider.
    pub recipient: types::Reference<crate::r5::resources::Organization>,

    /// Monetary amount of the payment expressed as a Money value with currency.
    pub amount: types::Money,

    /// Coded status of the payment, such as whether it has been issued or cleared.
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
