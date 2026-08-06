//! PaymentReconciliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
//!
//! Version: 6.0.0-ballot3
//!
//! PaymentReconciliation resource
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource provides the details including amount of a payment and
/// allocates the payment items being paid.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::payment_reconciliation::PaymentReconciliation;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PaymentReconciliation {
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

    /// Business Identifier for a payment reconciliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Category of payment
    pub r#type: types::CodeableConcept,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Workflow originating payment
    pub kind: Option<types::CodeableConcept>,

    /// Period covered
    pub period: Option<types::Period>,

    /// Creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
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
    pub outcome: Option<crate::coded::Coded<crate::r6::codes::PaymentOutcome>>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// Disposition message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// When payment issued
    pub date: types::Date,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Where payment collected
    pub location: Option<types::Reference>,

    /// Payment instrument
    pub method: Option<types::CodeableConcept>,

    /// Type of card
    pub card_brand: Option<types::String>,
    /// Primitive extension sibling for [`card_brand`](Self::card_brand) (FHIR `_cardBrand`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cardBrand")]
    pub card_brand_ext: Option<types::Element>,

    /// Digits for verification
    pub account_number: Option<types::String>,
    /// Primitive extension sibling for [`account_number`](Self::account_number) (FHIR `_accountNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_accountNumber")]
    pub account_number_ext: Option<types::Element>,

    /// Expiration year-month
    pub expiration_date: Option<types::Date>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// Processor name
    pub processor: Option<types::String>,
    /// Primitive extension sibling for [`processor`](Self::processor) (FHIR `_processor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_processor")]
    pub processor_ext: Option<types::Element>,

    /// Check number or payment reference
    pub reference_number: Option<types::String>,
    /// Primitive extension sibling for [`reference_number`](Self::reference_number) (FHIR `_referenceNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_referenceNumber")]
    pub reference_number_ext: Option<types::Element>,

    /// Authorization number
    pub authorization: Option<types::String>,
    /// Primitive extension sibling for [`authorization`](Self::authorization) (FHIR `_authorization`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authorization")]
    pub authorization_ext: Option<types::Element>,

    /// Amount offered by the issuer
    pub tendered_amount: Option<types::Money>,

    /// Amount returned by the receiver
    pub returned_amount: Option<types::Money>,

    /// Total amount of Payment
    pub amount: types::Money,

    /// Business identifier for the payment
    pub payment_identifier: Option<types::Identifier>,

    /// Settlement particulars
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocation: Vec<PaymentReconciliationAllocation>,

    /// Printed form identifier
    pub form_code: Option<types::CodeableConcept>,

    /// Note concerning processing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_note: Vec<PaymentReconciliationProcessNote>,
}

/// Distribution of the payment amount for a previously acknowledged payable.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::payment_reconciliation::PaymentReconciliationAllocation;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Sub-element of the subject
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Contact for the response
    pub responsible: Option<types::Reference>,

    /// Recipient of the payment
    pub payee: Option<types::Reference>,

    /// Amount allocated to this payable
    pub amount: Option<types::Money>,
}

/// A note that describes or explains the processing in a human readable form.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::payment_reconciliation::PaymentReconciliationProcessNote;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::NoteType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Note explanatory text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// The `PaymentReconciliation.allocation.targetItem[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PaymentReconciliationAllocationTargetItem {
    /// `targetItemString` variant.
    #[fhir("targetItemString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `targetItemIdentifier` variant.
    #[fhir("targetItemIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `targetItemPositiveInt` variant.
    #[fhir("targetItemPositiveInt")]
    PositiveInt(crate::r6::choice::Primitive<types::PositiveInt>),
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
