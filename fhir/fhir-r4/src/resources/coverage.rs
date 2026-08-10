//! Coverage
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coverage
//!
//! Version: 4.0.1
//!
//! Insurance or medical plan or a payment agreement
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Financial instrument which may be used to reimburse or pay for health care
/// products and services. Includes both insurance and self-payment.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::coverage::Coverage;
///
/// let value = Coverage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Coverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for the coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Coverage category such as medical or accident
    pub r#type: Option<types::CodeableConcept>,

    /// Owner of the policy
    pub policy_holder: Option<types::Reference>,

    /// Subscriber to the policy
    pub subscriber: Option<types::Reference>,

    /// ID assigned to the subscriber
    pub subscriber_id: Option<types::String>,
    /// Primitive extension sibling for [`subscriber_id`](Self::subscriber_id) (FHIR `_subscriberId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subscriberId")]
    pub subscriber_id_ext: Option<types::Element>,

    /// Plan beneficiary
    pub beneficiary: types::Reference<crate::r4::resources::Patient>,

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
    pub payor: ::vec1::Vec1<types::Reference>,

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
    pub contract: Vec<types::Reference<crate::r4::resources::Contract>>,
}

/// A suite of underwriter specific classifiers.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::coverage::CoverageClass;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

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
/// use fhir::r4::resources::coverage::CoverageCostToBeneficiary;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

    /// The amount or percentage due from the beneficiary
    /// The `Coverage.costToBeneficiary.value[x]` choice element (1..1); see [`CoverageCostToBeneficiaryValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
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
    #[serde(flatten)]
    value: crate::r4::choice::Slot<CoverageCostToBeneficiaryValue>,
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
/// use fhir::r4::resources::coverage::CoverageCostToBeneficiaryException;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// The `Coverage.costToBeneficiary.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageCostToBeneficiaryValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueMoney` variant.
    #[fhir("valueMoney")]
    Money(Box<types::Money>),
}
