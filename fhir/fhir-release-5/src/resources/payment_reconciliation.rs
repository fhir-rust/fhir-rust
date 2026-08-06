//! PaymentReconciliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
//!
//! Version: 5.0.0
//!
//! PaymentReconciliation Resource: This resource provides the details including amount of a payment and allocates the payment items being paid.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource provides the details including amount of a payment and
/// allocates the payment items being paid. A PaymentReconciliation is issued by
/// a payer, such as an insurer or other financially responsible party, to convey
/// the outcome of processing one or more payment requests. It records the total
/// payment made and describes how that single payment is distributed, or
/// allocated, across the individual claims, invoices, encounters, accounts, or
/// other payables that the payment settles. In FHIR R5 it is the primary
/// electronic remittance advice artifact of the financial module, typically
/// generated in a business-to-business exchange between a payer and a provider or
/// clearinghouse. Each allocation line ties a portion of the payment back to the
/// resource it satisfies, while process notes carry human-readable explanations
/// of the adjudication, enabling automated posting of receivables and audit of
/// how funds were applied.
///
/// Related resources: the payment often responds to a claim-adjudication or
/// funds-reservation request referenced from the `request` field, and each
/// allocation may point at an `Encounter`, `Account`, `Invoice`, or `ChargeItem`
/// that it settles. Monetary amounts use [`Money`](crate::r5::types::Money),
/// classifications use [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// and parties are captured as a [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::payment_reconciliation::PaymentReconciliation;
/// use fhir::r5::types;
///
/// let value = PaymentReconciliation {
///     card_brand: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `cardBrand` is the name this serializes to on the wire.
/// assert_eq!(json["cardBrand"], ::serde_json::json!("abc"));
///
/// let back: PaymentReconciliation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliation {
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

    /// Business Identifier for a payment reconciliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Coded category of payment, such as a payment for a claim or a batch remittance.
    pub r#type: types::CodeableConcept,

    /// Lifecycle status of the reconciliation: active, cancelled, draft, or entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Workflow originating payment
    pub kind: Option<types::CodeableConcept>,

    /// Period covered
    pub period: Option<types::Period>,

    /// Creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Who entered the payment
    pub enterer: Option<types::Reference>,

    /// Nature of the source
    pub issuer_type: Option<types::CodeableConcept>,

    /// Party generating payment
    pub payment_issuer: Option<types::Reference>,

    /// Reference to requesting resource
    pub request: Option<types::Reference>,

    /// Responsible practitioner
    pub requestor: Option<types::Reference>,

    /// queued | complete | error | partial
    pub outcome: Option<crate::r5::coded::Coded<crate::r5::codes::PaymentOutcome>>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`).
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// Disposition message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`).
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// When payment issued
    pub date: types::Date,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Where payment collected
    pub location: Option<types::Reference>,

    /// Payment instrument
    pub method: Option<types::CodeableConcept>,

    /// Type of card
    pub card_brand: Option<types::String>,
    /// Primitive extension sibling for [`card_brand`](Self::card_brand) (FHIR `_cardBrand`).
    #[serde(rename = "_cardBrand")]
    pub card_brand_ext: Option<types::Element>,

    /// Digits for verification
    pub account_number: Option<types::String>,
    /// Primitive extension sibling for [`account_number`](Self::account_number) (FHIR `_accountNumber`).
    #[serde(rename = "_accountNumber")]
    pub account_number_ext: Option<types::Element>,

    /// Expiration year-month
    pub expiration_date: Option<types::Date>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`).
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// Processor name
    pub processor: Option<types::String>,
    /// Primitive extension sibling for [`processor`](Self::processor) (FHIR `_processor`).
    #[serde(rename = "_processor")]
    pub processor_ext: Option<types::Element>,

    /// Check number or payment reference
    pub reference_number: Option<types::String>,
    /// Primitive extension sibling for [`reference_number`](Self::reference_number) (FHIR `_referenceNumber`).
    #[serde(rename = "_referenceNumber")]
    pub reference_number_ext: Option<types::Element>,

    /// Authorization number
    pub authorization: Option<types::String>,
    /// Primitive extension sibling for [`authorization`](Self::authorization) (FHIR `_authorization`).
    #[serde(rename = "_authorization")]
    pub authorization_ext: Option<types::Element>,

    /// Amount offered by the issuer
    pub tendered_amount: Option<types::Money>,

    /// Amount returned by the receiver
    pub returned_amount: Option<types::Money>,

    /// Total monetary amount of the payment being reconciled, prior to any allocation.
    pub amount: types::Money,

    /// Business identifier for the payment
    pub payment_identifier: Option<types::Identifier>,

    /// Settlement particulars: how the total payment is distributed across the individual payables.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocation: Vec<PaymentReconciliationAllocation>,

    /// Printed form identifier
    pub form_code: Option<types::CodeableConcept>,

    /// Note concerning processing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_note: Vec<PaymentReconciliationProcessNote>,
}

/// Settlement particulars: distribution of the total payment across the
/// individual payables being reconciled.
/// # Examples
///
/// ```
/// use fhir::r5::resources::payment_reconciliation::PaymentReconciliationAllocation;
/// use fhir::r5::types;
///
/// let value = PaymentReconciliationAllocation {
///     date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01"));
///
/// let back: PaymentReconciliationAllocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliationAllocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier of the payment detail
    pub identifier: Option<types::Identifier>,

    /// Business identifier of the prior payment detail
    pub predecessor: Option<types::Identifier>,

    /// Subject of the payment
    pub target: Option<types::Reference>,

    /// The `PaymentReconciliation.allocation.targetItem[x]` choice element (0..1); see [`PaymentReconciliationAllocationTargetItem`].
    #[serde(flatten)]
    pub target_item: Option<PaymentReconciliationAllocationTargetItem>,

    /// Applied-to encounter
    pub encounter: Option<types::Reference>,

    /// Applied-to account
    pub account: Option<types::Reference>,

    /// Category of payment
    pub r#type: Option<types::CodeableConcept>,

    /// Submitter of the request
    pub submitter: Option<types::Reference>,

    /// Response committing to a payment
    pub response: Option<types::Reference>,

    /// Date of commitment to pay
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Contact for the response
    pub responsible: Option<types::Reference>,

    /// Recipient of the payment
    pub payee: Option<types::Reference>,

    /// Amount allocated to this payable
    pub amount: Option<types::Money>,
}

/// Note concerning processing: suggested notes to convey to or explain the
/// processing outcome to the requestor.
/// # Examples
///
/// ```
/// use fhir::r5::resources::payment_reconciliation::PaymentReconciliationProcessNote;
/// use fhir::r5::types;
///
/// let value = PaymentReconciliationProcessNote {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: PaymentReconciliationProcessNote = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliationProcessNote {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// display | print | printoper
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::NoteType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Note explanatory text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
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
/// The `PaymentReconciliation.allocation.targetItem[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum PaymentReconciliationAllocationTargetItem {
    /// `targetItemString` variant.
    #[fhir("targetItemString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `targetItemIdentifier` variant.
    #[fhir("targetItemIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `targetItemPositiveInt` variant.
    #[fhir("targetItemPositiveInt")]
    PositiveInt(crate::r5::choice::Primitive<types::PositiveInt>),
}
