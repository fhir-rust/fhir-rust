//! PaymentReconciliation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PaymentReconciliation
//!
//! Version: 4.0.1
//!
//! PaymentReconciliation resource
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource provides the details including amount of a payment and
/// allocates the payment items being paid.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::payment_reconciliation::PaymentReconciliation;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for a payment reconciliation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Period covered
    pub period: Option<types::Period>,

    /// Creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Party generating payment
    pub payment_issuer: Option<types::Reference>,

    /// Reference to requesting resource
    pub request: Option<types::Reference>,

    /// Responsible practitioner
    pub requestor: Option<types::Reference>,

    /// queued | complete | error | partial
    pub outcome: Option<crate::coded::Coded<crate::r4::codes::RemittanceOutcome>>,
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
    pub payment_date: types::Date,
    /// Primitive extension sibling for [`payment_date`](Self::payment_date) (FHIR `_paymentDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paymentDate")]
    pub payment_date_ext: Option<types::Element>,

    /// Total amount of Payment
    pub payment_amount: types::Money,

    /// Business identifier for the payment
    pub payment_identifier: Option<types::Identifier>,

    /// Settlement particulars
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<PaymentReconciliationDetail>,

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
/// use fhir::r4::resources::payment_reconciliation::PaymentReconciliationDetail;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct PaymentReconciliationDetail {
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

    /// Category of payment
    pub r#type: types::CodeableConcept,

    /// Request giving rise to the payment
    pub request: Option<types::Reference>,

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
/// use fhir::r4::resources::payment_reconciliation::PaymentReconciliationProcessNote;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub r#type: Option<crate::coded::Coded<crate::r4::codes::NoteType>>,
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
