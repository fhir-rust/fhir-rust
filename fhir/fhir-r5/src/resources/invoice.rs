//! Invoice
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Invoice
//!
//! Version: 5.0.0
//!
//! Invoice Resource: Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
///
/// The Invoice resource represents a formal statement of financial charges
/// issued to a subject or organization. It aggregates individual ChargeItem
/// entries drawn from an [`Account`](crate::r5::resources::account::Account)
/// into discrete line items, each carrying its own price components, and rolls
/// them up into calculated net and gross totals. Invoices are produced by an
/// issuing organization once the services or goods recorded against an account
/// are ready to be billed, and they carry the status of the billing process
/// (draft, issued, balanced, cancelled, or entered-in-error) as it progresses.
///
/// Within the FHIR R5 financial and billing workflows, an Invoice sits
/// downstream of clinical and administrative activity: charges captured as
/// they occur are collected onto an account, and the Invoice then presents
/// those charges in a structured, human- and machine-readable form suitable
/// for submission, payment, and reconciliation. Payment terms, participants
/// involved in its creation, and free-text notes may accompany the charges.
///
/// See also: [`Account`](crate::r5::resources::account::Account) for the
/// running balance being invoiced, [`Reference`](crate::r5::types::Reference)
/// for links to the subject, recipient, and issuer, and
/// [`Money`](crate::r5::types::Money) and
/// [`MonetaryComponent`](crate::r5::types::MonetaryComponent) for how amounts
/// are expressed. The recipient and issuer are commonly a `Patient`,
/// `RelatedPerson`, or `Organization`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::invoice::Invoice;
/// use fhir::r5::types;
///
/// let value = Invoice {
///     cancelled_reason: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `cancelledReason` is the name this serializes to on the wire.
/// assert_eq!(json["cancelledReason"], ::serde_json::json!("abc"));
///
/// let back: Invoice = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
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

    /// Business Identifier for item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Current state of the invoice in the billing process: draft, issued, balanced, cancelled, or entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::InvoiceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for cancellation of this Invoice
    pub cancelled_reason: Option<types::String>,
    /// Primitive extension sibling for [`cancelled_reason`](Self::cancelled_reason) (FHIR `_cancelledReason`).
    #[serde(rename = "_cancelledReason")]
    pub cancelled_reason_ext: Option<types::Element>,

    /// Type of Invoice
    pub r#type: Option<types::CodeableConcept>,

    /// Recipient(s) of goods and services
    pub subject: Option<types::Reference>,

    /// Recipient of this invoice
    pub recipient: Option<types::Reference>,

    /// DEPRICATED
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// When posted
    pub creation: Option<types::DateTime>,
    /// Primitive extension sibling for [`creation`](Self::creation) (FHIR `_creation`).
    #[serde(rename = "_creation")]
    pub creation_ext: Option<types::Element>,

    /// The `Invoice.period[x]` choice element (0..1); see [`InvoicePeriod`].
    #[serde(flatten)]
    pub period: Option<InvoicePeriod>,

    /// Participant in creation of this Invoice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<InvoiceParticipant>,

    /// Issuing Organization of Invoice
    pub issuer: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Reference to the Account whose collected charges are being balanced by this invoice.
    pub account: Option<types::Reference<crate::r5::resources::Account>>,

    /// Individual charge lines that make up this invoice, each with its own price components.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_item: Vec<InvoiceLineItem>,

    /// Components of Invoice total
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub total_price_component: Vec<types::MonetaryComponent>,

    /// Net total of this invoice, excluding taxes and surcharges captured as components.
    pub total_net: Option<types::Money>,

    /// Gross total of this invoice, including all price components such as taxes.
    pub total_gross: Option<types::Money>,

    /// Payment details
    pub payment_terms: Option<types::Markdown>,
    /// Primitive extension sibling for [`payment_terms`](Self::payment_terms) (FHIR `_paymentTerms`).
    #[serde(rename = "_paymentTerms")]
    pub payment_terms_ext: Option<types::Element>,

    /// Comments made about the invoice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Participant in creation of this Invoice.
///
/// Indicates who or what was involved in the creation of the Invoice and in
/// what role.
/// # Examples
///
/// ```
/// use fhir::r5::resources::invoice::InvoiceParticipant;
/// use fhir::r5::types;
///
/// let value = InvoiceParticipant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InvoiceParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of involvement in creation of this Invoice
    pub role: Option<types::CodeableConcept>,

    /// Individual who was involved
    pub actor: types::Reference,
}

/// Line items of this Invoice.
///
/// Each line item captures a specific charge, referencing a ChargeItem or an
/// inline billing code, together with its price components.
/// # Examples
///
/// ```
/// use fhir::r5::resources::invoice::InvoiceLineItem;
/// use fhir::r5::types;
///
/// let value = InvoiceLineItem {
///     sequence: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sequence` is the name this serializes to on the wire.
/// assert_eq!(json["sequence"], ::serde_json::json!(1));
///
/// let back: InvoiceLineItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLineItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Sequence number of line item
    pub sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// The `Invoice.lineItem.serviced[x]` choice element (0..1); see [`InvoiceLineItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<InvoiceLineItemServiced>,

    /// The `Invoice.lineItem.chargeItem[x]` choice element (0..1); see [`InvoiceLineItemChargeItem`].
    #[serde(flatten)]
    pub charge_item: Option<InvoiceLineItemChargeItem>,

    /// Components of total line item price
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_component: Vec<types::MonetaryComponent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Invoice;

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
/// The `Invoice.lineItem.chargeItem[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum InvoiceLineItemChargeItem {
    /// `chargeItemReference` variant.
    #[fhir("chargeItemReference")]
    Reference(Box<types::Reference>),
    /// `chargeItemCodeableConcept` variant.
    #[fhir("chargeItemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `Invoice.lineItem.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum InvoiceLineItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `Invoice.period[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum InvoicePeriod {
    /// `periodDate` variant.
    #[fhir("periodDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `periodPeriod` variant.
    #[fhir("periodPeriod")]
    Period(Box<types::Period>),
}
