//! InsurancePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
//!
//! Version: 5.0.0
//!
//! InsurancePlan Resource: Details of a Health Insurance product/plan provided by an organization.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Details of a Health Insurance product/plan provided by an organization. An
/// InsurancePlan describes a health insurance offering comprised of a list of
/// covered benefits (i.e. the product), costs associated with those benefits
/// (i.e. the plan), and additional information about the offering, such as who
/// it is owned and administered by, a coverage area, contact information, etc.
/// It is distinct from a member's actual Coverage of a given product.
///
/// InsurancePlan is used by payers, health plan directories, and provider
/// network publications to describe the products and plans an insurer
/// offers, including the networks that participate in them, the geographic
/// areas they serve, and the general and specific costs (premiums,
/// copayments, deductibles, and similar) associated with each plan. It
/// supports use cases such as plan comparison shopping, network adequacy
/// reporting, and payer-to-payer or payer-to-provider directory exchange.
///
/// # Related resources
///
/// - [`Organization`](crate::r5::resources::organization::Organization) is
///   typically referenced as the product issuer (`owned_by`) and
///   administrator (`administered_by`).
/// - `Coverage` represents a member's actual enrollment in a product
///   described by an InsurancePlan.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used
///   throughout to code plan types, coverage types, and benefit categories.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlan;
/// use fhir::r5::types;
///
/// let value = InsurancePlan {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlan {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for Product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The lifecycle status of this plan: draft | active | retired | unknown.
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of product, such as medical or dental, coded as a
    /// [`CodeableConcept`](crate::r5::types::CodeableConcept).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Official name of the health insurance product or plan.
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Alternate names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`).
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// When the product is available
    pub period: Option<types::Period>,

    /// Reference to the [`Organization`](crate::r5::resources::organization::Organization)
    /// that is the product issuer.
    pub owned_by: Option<types::Reference>,

    /// Reference to the [`Organization`](crate::r5::resources::organization::Organization)
    /// that administers the product.
    pub administered_by: Option<types::Reference>,

    /// Geographic area(s), typically Location resources, where the product applies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference>,

    /// Official contact details relevant to the health insurance plan/product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Technical endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,

    /// What networks are Included
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference>,

    /// Coverage details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<InsurancePlanCoverage>,

    /// Plan details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plan: Vec<InsurancePlanPlan>,
}

/// Coverage details: the details of a coverage offered by the insurance product.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::insurance_plan::InsurancePlanCoverage;
///
/// let value = InsurancePlanCoverage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: InsurancePlanCoverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of coverage
    pub r#type: types::CodeableConcept,

    /// What networks provide coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference>,

    /// List of benefits
    pub benefit: vec1::Vec1<InsurancePlanCoverageBenefit>,
}

/// List of benefits: specific benefits under this type of coverage.
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlanCoverageBenefit;
/// use fhir::r5::types;
///
/// let value = InsurancePlanCoverageBenefit {
///     requirement: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `requirement` is the name this serializes to on the wire.
/// assert_eq!(json["requirement"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanCoverageBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverageBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of benefit
    pub r#type: types::CodeableConcept,

    /// Referral requirements
    pub requirement: Option<types::String>,
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`).
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// Benefit limits
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limit: Vec<InsurancePlanCoverageBenefitLimit>,
}

/// Benefit limits: the specific limits on the benefit.
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlanCoverageBenefitLimit;
/// use fhir::r5::types;
///
/// let value = InsurancePlanCoverageBenefitLimit {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanCoverageBenefitLimit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverageBenefitLimit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Maximum value allowed
    pub value: Option<types::Quantity>,

    /// Benefit limit details
    pub code: Option<types::CodeableConcept>,
}

/// Plan details: details about an insurance plan.
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlanPlan;
/// use fhir::r5::types;
///
/// let value = InsurancePlanPlan {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanPlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlan {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for Product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Type of plan
    pub r#type: Option<types::CodeableConcept>,

    /// Where product applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference>,

    /// What networks provide coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference>,

    /// Overall costs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub general_cost: Vec<InsurancePlanPlanGeneralCost>,

    /// Specific costs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specific_cost: Vec<InsurancePlanPlanSpecificCost>,
}

/// Overall costs: overall costs associated with the plan.
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlanPlanGeneralCost;
/// use fhir::r5::types;
///
/// let value = InsurancePlanPlanGeneralCost {
///     group_size: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `groupSize` is the name this serializes to on the wire.
/// assert_eq!(json["groupSize"], ::serde_json::json!(1));
///
/// let back: InsurancePlanPlanGeneralCost = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanGeneralCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of cost
    pub r#type: Option<types::CodeableConcept>,

    /// Number of enrollees
    pub group_size: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`group_size`](Self::group_size) (FHIR `_groupSize`).
    #[serde(rename = "_groupSize")]
    pub group_size_ext: Option<types::Element>,

    /// Cost value
    pub cost: Option<types::Money>,

    /// Additional cost information
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// Specific costs: costs associated with the coverage provided by the product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlanPlanSpecificCost;
/// use fhir::r5::types;
///
/// let value = InsurancePlanPlanSpecificCost {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanPlanSpecificCost = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// General category of benefit
    pub category: types::CodeableConcept,

    /// Benefits list
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub benefit: Vec<InsurancePlanPlanSpecificCostBenefit>,
}

/// Benefits list: list of the specific benefits under this category of benefit.
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlanPlanSpecificCostBenefit;
/// use fhir::r5::types;
///
/// let value = InsurancePlanPlanSpecificCostBenefit {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanPlanSpecificCostBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCostBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of specific benefit
    pub r#type: types::CodeableConcept,

    /// List of the costs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost: Vec<InsurancePlanPlanSpecificCostBenefitCost>,
}

/// List of the costs: list of the costs associated with a specific benefit.
/// # Examples
///
/// ```
/// use fhir::r5::resources::insurance_plan::InsurancePlanPlanSpecificCostBenefitCost;
/// use fhir::r5::types;
///
/// let value = InsurancePlanPlanSpecificCostBenefitCost {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanPlanSpecificCostBenefitCost = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCostBenefitCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of cost
    pub r#type: types::CodeableConcept,

    /// in-network | out-of-network | other
    pub applicability: Option<types::CodeableConcept>,

    /// Additional information about the cost
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualifiers: Vec<types::CodeableConcept>,

    /// The actual cost value
    pub value: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = InsurancePlan;

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
