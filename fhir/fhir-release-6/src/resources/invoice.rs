//! Invoice
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Invoice
//!
//! Version: 6.0.0-ballot3
//!
//! Invoice containing ChargeItems from an Account
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::invoice::Invoice;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub contained: Vec<::serde_json::Value>,

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
    pub status: crate::coded::Coded<crate::r6::codes::InvoiceStatus>,
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

    /// DEPRICATED
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// When posted
    pub creation: Option<types::DateTime>,
    /// Primitive extension sibling for [`creation`](Self::creation) (FHIR `_creation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_creation")]
    pub creation_ext: Option<types::Element>,

    /// Billing date or period
    /// The `Invoice.period[x]` choice element (0..1); see [`InvoicePeriod`].
    #[serde(flatten)]
    pub period: Option<InvoicePeriod>,

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
    pub total_price_component: Vec<types::MonetaryComponent>,

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
/// Details such.ofType(date), code and amount are found in the referenced
/// ChargeItem resource.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::invoice::InvoiceLineItem;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Service data or period
    /// The `Invoice.lineItem.serviced[x]` choice element (0..1); see [`InvoiceLineItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<InvoiceLineItemServiced>,

    /// Reference to ChargeItem containing details of this line item or an
    /// inline billing code
    /// The `Invoice.lineItem.chargeItem[x]` choice element (1..1); see [`InvoiceLineItemChargeItem`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub charge_item: Option<InvoiceLineItemChargeItem>,

    /// Components of total line item price
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub price_component: Vec<types::MonetaryComponent>,
}

/// Indicates who or what performed or participated in the charged service.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::invoice::InvoiceParticipant;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// The `Invoice.period[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum InvoicePeriod {
    /// `periodDate` variant.
    #[fhir("periodDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `periodPeriod` variant.
    #[fhir("periodPeriod")]
    Period(Box<types::Period>),
}

/// The `Invoice.lineItem.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum InvoiceLineItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `Invoice.lineItem.chargeItem[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
