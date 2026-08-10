//! Coverage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coverage
//!
//! Version: 6.0.0-ballot3
//!
//! Insurance or medical plan or a payment agreement
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage::Coverage;
/// use fhir::r6::types;
///
/// let value = Coverage {
///     dependent: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `dependent` is the name this serializes to on the wire.
/// assert_eq!(json["dependent"], ::serde_json::json!("abc"));
///
/// let back: Coverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Coverage {
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

    /// Business identifier(s) for this coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// insurance | self-pay | other
    pub kind: crate::coded::Coded<crate::r6::codes::CoverageKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Self-pay parties and responsibility
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_by: Vec<CoveragePaymentBy>,

    /// Coverage category such as medical or accident
    pub r#type: Option<types::CodeableConcept>,

    /// Owner of the policy
    pub policy_holder: Option<types::Reference>,

    /// Subscriber to the policy
    pub subscriber: Option<types::Reference>,

    /// ID assigned to the subscriber
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriber_id: Vec<types::Identifier>,

    /// Plan beneficiary
    pub beneficiary: types::Reference<crate::r6::resources::Patient>,

    /// Dependent number
    pub dependent: Option<types::String>,
    /// Primitive extension sibling for [`dependent`](Self::dependent) (FHIR `_dependent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dependent")]
    pub dependent_ext: Option<types::Element>,

    /// Beneficiary relationship to the subscriber
    pub relationship: Option<types::CodeableConcept>,

    /// Coverage start and end dates
    pub period: Option<types::Period>,

    /// Issuer of the policy
    pub insurer: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Additional coverage classifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<CoverageClass>,

    /// Relative order of the coverage
    pub order: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`order`](Self::order) (FHIR `_order`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_order")]
    pub order_ext: Option<types::Element>,

    /// Insurer network
    pub network: Option<types::String>,
    /// Primitive extension sibling for [`network`](Self::network) (FHIR `_network`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_network")]
    pub network_ext: Option<types::Element>,

    /// Patient payments for services/products
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost_to_beneficiary: Vec<CoverageCostToBeneficiary>,

    /// Reimbursement to insurer
    pub subrogation: Option<types::Boolean>,
    /// Primitive extension sibling for [`subrogation`](Self::subrogation) (FHIR `_subrogation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subrogation")]
    pub subrogation_ext: Option<types::Element>,

    /// Contract details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<types::Reference<crate::r6::resources::Contract>>,

    /// Insurance plan details
    pub insurance_plan: Option<types::Reference<crate::r6::resources::InsurancePlan>>,
}

/// A suite of underwriter specific classifiers.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage::CoverageClass;
/// use fhir::r6::types;
///
/// let value = CoverageClass {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: CoverageClass = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageClass {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of class such as 'group' or 'plan'
    pub r#type: types::CodeableConcept,

    /// Value associated with the type
    pub value: types::Identifier,

    /// Human readable description of the type and value
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
}

/// A suite of codes indicating the cost category and associated amount which
/// have been detailed in the policy and may have been included on the health
/// card.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage::CoverageCostToBeneficiary;
/// use fhir::r6::types;
///
/// let value = CoverageCostToBeneficiary {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageCostToBeneficiary = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "CoverageCostToBeneficiaryDe")]
#[fhir_version("r6")]
pub struct CoverageCostToBeneficiary {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Cost category
    pub r#type: Option<types::CodeableConcept>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// In or out of network
    pub network: Option<types::CodeableConcept>,

    /// Individual or family
    pub unit: Option<types::CodeableConcept>,

    /// Annual or lifetime
    pub term: Option<types::CodeableConcept>,

    /// The amount or percentage due from the beneficiary
    /// The `Coverage.costToBeneficiary.value[x]` choice element (0..1); see [`CoverageCostToBeneficiaryValue`].
    #[serde(flatten)]
    pub value: Option<CoverageCostToBeneficiaryValue>,

    /// Exceptions for patient payments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exception: Vec<CoverageCostToBeneficiaryException>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoverageCostToBeneficiaryDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: Option<types::CodeableConcept>,
    category: Option<types::CodeableConcept>,
    network: Option<types::CodeableConcept>,
    unit: Option<types::CodeableConcept>,
    term: Option<types::CodeableConcept>,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<CoverageCostToBeneficiaryValue>,
    #[serde(default)]
    exception: Vec<CoverageCostToBeneficiaryException>,
}

impl ::core::convert::From<CoverageCostToBeneficiaryDe> for CoverageCostToBeneficiary {
    fn from(v: CoverageCostToBeneficiaryDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            category: v.category,
            network: v.network,
            unit: v.unit,
            term: v.term,
            value: v.value.0,
            exception: v.exception,
        }
    }
}

/// A suite of codes indicating exceptions or reductions to patient costs and
/// their effective periods.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage::CoverageCostToBeneficiaryException;
/// use fhir::r6::types;
///
/// let value = CoverageCostToBeneficiaryException {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageCostToBeneficiaryException = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageCostToBeneficiaryException {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Exception category
    pub r#type: types::CodeableConcept,

    /// The effective period of the exception
    pub period: Option<types::Period>,
}

/// Link to the paying party and optionally what specifically they will be
/// responsible to pay.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage::CoveragePaymentBy;
/// use fhir::r6::types;
///
/// let value = CoveragePaymentBy {
///     responsibility: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `responsibility` is the name this serializes to on the wire.
/// assert_eq!(json["responsibility"], ::serde_json::json!("abc"));
///
/// let back: CoveragePaymentBy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoveragePaymentBy {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Parties performing self-payment
    pub party: types::Reference,

    /// Party's responsibility
    pub responsibility: Option<types::String>,
    /// Primitive extension sibling for [`responsibility`](Self::responsibility) (FHIR `_responsibility`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_responsibility")]
    pub responsibility_ext: Option<types::Element>,
}

/// The `Coverage.costToBeneficiary.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageCostToBeneficiaryValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueMoney` variant.
    #[fhir("valueMoney")]
    Money(Box<types::Money>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coverage;

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
