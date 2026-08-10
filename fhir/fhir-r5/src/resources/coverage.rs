//! Coverage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coverage
//!
//! Version: 5.0.0
//!
//! Coverage Resource: Financial instrument which may be used to reimburse or pay for health care products and services.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. This includes both insurance and self-payment. In
/// FHIR R5 the Coverage resource records the identifiers and relationships of a
/// payment source (such as an insurance policy, a government program, or a
/// self-pay arrangement) and links a beneficiary to the party responsible for
/// payment. It is referenced by billing and adjudication resources such as
/// Claim, ExplanationOfBenefit, and CoverageEligibilityRequest.
///
/// Coverage captures the administrative and financial details needed to
/// determine who will pay for a beneficiary's health care: the subscriber and
/// beneficiary, the insurer or other responsible party, the coverage period,
/// classification information such as group or plan identifiers, and any
/// patient cost-sharing rules (for example copays, deductibles, or
/// exceptions). Systems typically use it to check eligibility, to populate
/// claims and pre-authorization requests, and to reconcile payments during
/// adjudication.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) is commonly the
///   `beneficiary` or `subscriber` referenced by this resource.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used for coded
///   fields such as `type`, `relationship`, and cost category classifications.
/// - `Claim`, `ExplanationOfBenefit`, and `CoverageEligibilityRequest` are
///   related billing and adjudication resources that reference a Coverage.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage::Coverage;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
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

    /// Business identifier(s) for this coverage, such as a member or policy number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The status of the resource instance: active | cancelled | draft | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The nature of the coverage: insurance | self-pay | other
    pub kind: crate::r5::coded::Coded<crate::r5::codes::CoverageKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
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

    /// Plan beneficiary, typically a reference to a Patient
    pub beneficiary: types::Reference<crate::r5::resources::Patient>,

    /// Dependent number
    pub dependent: Option<types::String>,
    /// Primitive extension sibling for [`dependent`](Self::dependent) (FHIR `_dependent`).
    #[serde(rename = "_dependent")]
    pub dependent_ext: Option<types::Element>,

    /// Beneficiary relationship to the subscriber
    pub relationship: Option<types::CodeableConcept>,

    /// Coverage start and end dates
    pub period: Option<types::Period>,

    /// Issuer of the policy, such as the insurance organization or program
    pub insurer: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Additional coverage classifications
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<CoverageClass>,

    /// Relative order of the coverage
    pub order: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`order`](Self::order) (FHIR `_order`).
    #[serde(rename = "_order")]
    pub order_ext: Option<types::Element>,

    /// Insurer network
    pub network: Option<types::String>,
    /// Primitive extension sibling for [`network`](Self::network) (FHIR `_network`).
    #[serde(rename = "_network")]
    pub network_ext: Option<types::Element>,

    /// Patient payments for services/products
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost_to_beneficiary: Vec<CoverageCostToBeneficiary>,

    /// Reimbursement to insurer
    pub subrogation: Option<types::Boolean>,
    /// Primitive extension sibling for [`subrogation`](Self::subrogation) (FHIR `_subrogation`).
    #[serde(rename = "_subrogation")]
    pub subrogation_ext: Option<types::Element>,

    /// Contract details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<types::Reference<crate::r5::resources::Contract>>,

    /// Insurance plan details
    pub insurance_plan: Option<types::Reference<crate::r5::resources::InsurancePlan>>,
}

/// Self-pay parties and responsibility. Identifies parties that are responsible
/// for self-payment of a portion of the beneficiary's costs and the extent of
/// their responsibility.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage::CoveragePaymentBy;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`responsibility`](Self::responsibility) (FHIR `_responsibility`).
    #[serde(rename = "_responsibility")]
    pub responsibility_ext: Option<types::Element>,
}

/// Additional coverage classifications. A suite of underwriter-specific
/// classifiers, such as group, plan, or subgroup, used to categorize the
/// coverage.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage::CoverageClass;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
}

/// Patient payments for services/products. A suite of codes indicating the cost
/// category and associated amount which have been detailed in the policy and
/// may have been included on the health card.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage::CoverageCostToBeneficiary;
/// use fhir::r5::types;
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
    value: crate::r5::choice::Slot<CoverageCostToBeneficiaryValue>,
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

/// Exceptions for patient payments. A suite of codes indicating exceptions or
/// reductions to patient costs and their effective periods.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage::CoverageCostToBeneficiaryException;
/// use fhir::r5::types;
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
/// The `Coverage.costToBeneficiary.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageCostToBeneficiaryValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueMoney` variant.
    #[fhir("valueMoney")]
    Money(Box<types::Money>),
}
