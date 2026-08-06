//! InsurancePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
//!
//! Version: 6.0.0-ballot3
//!
//! Plan details
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Details of a Health Insurance plan provided by an organization under an
/// InsuranceProduct.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_plan::InsurancePlan;
/// use fhir::r6::types;
///
/// let value = InsurancePlan {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsurancePlan {
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

    /// Business Identifier for Plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Classification of Plan
    pub r#type: Option<types::CodeableConcept>,

    /// The product that this plan is available under
    pub product: Option<types::Reference>,

    /// Where product-plan applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference>,

    /// What networks provide coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference>,

    /// Overall costs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub general_cost: Vec<InsurancePlanGeneralCost>,

    /// Individual cost elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specific_cost: Vec<InsurancePlanSpecificCost>,
}

/// Overall costs associated with the plan.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_plan::InsurancePlanGeneralCost;
/// use fhir::r6::types;
///
/// let value = InsurancePlanGeneralCost {
///     group_size: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `groupSize` is the name this serializes to on the wire.
/// assert_eq!(json["groupSize"], ::serde_json::json!(1));
///
/// let back: InsurancePlanGeneralCost = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsurancePlanGeneralCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of specific cost
    pub r#type: Option<types::CodeableConcept>,

    /// Number of enrollees
    pub group_size: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`group_size`](Self::group_size) (FHIR `_groupSize`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_groupSize")]
    pub group_size_ext: Option<types::Element>,

    /// Cost value
    pub cost: Option<types::Money>,

    /// Additional cost information
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// Costs associated with the coverage provided by the product-plan.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_plan::InsurancePlanSpecificCost;
/// use fhir::r6::types;
///
/// let value = InsurancePlanSpecificCost {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanSpecificCost = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsurancePlanSpecificCost {
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
    pub benefit: Vec<InsurancePlanSpecificCostBenefit>,
}

/// List of the specific benefits under this category of benefit.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_plan::InsurancePlanSpecificCostBenefit;
/// use fhir::r6::types;
///
/// let value = InsurancePlanSpecificCostBenefit {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanSpecificCostBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsurancePlanSpecificCostBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of benefit provided
    pub r#type: types::CodeableConcept,

    /// List of the costs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cost: Vec<InsurancePlanSpecificCostBenefitCost>,
}

/// List of the costs associated with a specific benefit.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_plan::InsurancePlanSpecificCostBenefitCost;
/// use fhir::r6::types;
///
/// let value = InsurancePlanSpecificCostBenefitCost {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanSpecificCostBenefitCost = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsurancePlanSpecificCostBenefitCost {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of specific cost
    pub r#type: types::CodeableConcept,

    /// in-network | out-of-network | other
    pub applicability: Option<types::CodeableConcept>,

    /// Additional information about the cost
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualifier: Vec<types::CodeableConcept>,

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
