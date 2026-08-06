//! Invoice
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Invoice
//!
//! Version: 4.0.1
//!
//! Invoice containing ChargeItems from an Account
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::invoice::Invoice;
/// use fhir::r4::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Invoice {
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

    /// Business Identifier for item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | issued | balanced | cancelled | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::InvoiceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for cancellation of this Invoice
    pub cancelled_reason: Option<types::String>,
    /// Primitive extension sibling for [`cancelled_reason`](Self::cancelled_reason) (FHIR `_cancelledReason`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cancelledReason")]
    pub cancelled_reason_ext: Option<types::Element>,

    /// Type of Invoice
    pub r#type: Option<types::CodeableConcept>,

    /// Recipient(s) of goods and services
    pub subject: Option<types::Reference>,

    /// Recipient of this invoice
    pub recipient: Option<types::Reference>,

    /// Invoice date / posting date
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Participant in creation of this Invoice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<InvoiceParticipant>,

    /// Issuing Organization of Invoice
    pub issuer: Option<types::Reference>,

    /// Account that is being balanced
    pub account: Option<types::Reference>,

    /// Line items of this Invoice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_item: Vec<InvoiceLineItem>,

    /// Components of Invoice total
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub total_price_component: Vec<InvoiceLineItemPriceComponent>,

    /// Net total of this Invoice
    pub total_net: Option<types::Money>,

    /// Gross total of this Invoice
    pub total_gross: Option<types::Money>,

    /// Payment details
    pub payment_terms: Option<types::Markdown>,
    /// Primitive extension sibling for [`payment_terms`](Self::payment_terms) (FHIR `_paymentTerms`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paymentTerms")]
    pub payment_terms_ext: Option<types::Element>,

    /// Comments made about the invoice
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Each line item represents one charge for goods and services rendered.
/// Details such as date, code and amount are found in the referenced
/// ChargeItem resource.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::invoice::InvoiceLineItem;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Reference to ChargeItem containing details of this line item or an
    /// inline billing code
    /// The `Invoice.lineItem.chargeItem[x]` choice element (1..1); see [`InvoiceLineItemChargeItem`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub charge_item: Option<InvoiceLineItemChargeItem>,

    /// Components of total line item price
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_component: Vec<InvoiceLineItemPriceComponent>,
}

/// The price for a ChargeItem may be calculated as a base price with
/// surcharges/deductions that apply in certain conditions. A
/// ChargeItemDefinition resource that defines the prices, factors and
/// conditions that apply to a billing code is currently under development. The
/// priceComponent element can be used to offer transparency to the recipient
/// of the Invoice as to how the prices have been calculated.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::invoice::InvoiceLineItemPriceComponent;
/// use fhir::r4::types;
///
/// let value = InvoiceLineItemPriceComponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InvoiceLineItemPriceComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct InvoiceLineItemPriceComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// base | surcharge | deduction | discount | tax | informational
    pub r#type: crate::coded::Coded<crate::r4::codes::InvoicePriceComponentType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Code identifying the specific component
    pub code: Option<types::CodeableConcept>,

    /// Factor used for calculating this component
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Monetary amount associated with this component
    pub amount: Option<types::Money>,
}

/// Indicates who or what performed or participated in the charged service.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::invoice::InvoiceParticipant;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// The `Invoice.lineItem.chargeItem[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum InvoiceLineItemChargeItem {
    /// `chargeItemReference` variant.
    #[fhir("chargeItemReference")]
    Reference(Box<types::Reference>),
    /// `chargeItemCodeableConcept` variant.
    #[fhir("chargeItemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
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
