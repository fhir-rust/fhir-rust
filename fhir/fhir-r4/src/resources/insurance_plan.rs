//! InsurancePlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InsurancePlan
//!
//! Version: 4.0.1
//!
//! Details of a Health Insurance product/plan provided by an organization
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Details of a Health Insurance product/plan provided by an organization.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlan;
/// use fhir::r4::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for Product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | active | retired | unknown
    pub status: Option<crate::coded::Coded<crate::r4::codes::PublicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Official name
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Alternate names
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub alias: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// When the product is available
    pub period: Option<types::Period>,

    /// Plan issuer
    pub owned_by: Option<types::Reference<crate::r4::resources::Organization>>,

    /// Product administrator
    pub administered_by: Option<types::Reference<crate::r4::resources::Organization>>,

    /// Where product applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference<crate::r4::resources::Location>>,

    /// Contact for the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<InsurancePlanContact>,

    /// Technical endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r4::resources::Endpoint>>,

    /// What networks are Included
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference<crate::r4::resources::Organization>>,

    /// Coverage details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<InsurancePlanCoverage>,

    /// Plan details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plan: Vec<InsurancePlanPlan>,
}

/// The contact for the health insurance product for a certain purpose.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanContact;
/// use fhir::r4::types;
///
/// let value = InsurancePlanContact {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsurancePlanContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct InsurancePlanContact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of contact
    pub purpose: Option<types::CodeableConcept>,

    /// A name associated with the contact
    pub name: Option<types::HumanName>,

    /// Contact details (telephone, email, etc.) for a contact
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Visiting or postal addresses for the contact
    pub address: Option<types::Address>,
}

/// Details about the coverage offered by the insurance product.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::insurance_plan::InsurancePlanCoverage;
///
/// let value = InsurancePlanCoverage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: InsurancePlanCoverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub network: Vec<types::Reference<crate::r4::resources::Organization>>,

    /// List of benefits
    pub benefit: ::vec1::Vec1<InsurancePlanCoverageBenefit>,
}

/// Specific benefits under this type of coverage.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanCoverageBenefit;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// Benefit limits
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limit: Vec<InsurancePlanCoverageBenefitLimit>,
}

/// The specific limits on the benefit.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanCoverageBenefitLimit;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// Details about an insurance plan.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanPlan;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub coverage_area: Vec<types::Reference<crate::r4::resources::Location>>,

    /// What networks provide coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference<crate::r4::resources::Organization>>,

    /// Overall costs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub general_cost: Vec<InsurancePlanPlanGeneralCost>,

    /// Specific costs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specific_cost: Vec<InsurancePlanPlanSpecificCost>,
}

/// Overall costs associated with the plan.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanPlanGeneralCost;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// Costs associated with the coverage provided by the product.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanPlanSpecificCost;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// List of the specific benefits under this category of benefit.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanPlanSpecificCostBenefit;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// List of the costs associated with a specific benefit.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::insurance_plan::InsurancePlanPlanSpecificCostBenefitCost;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
