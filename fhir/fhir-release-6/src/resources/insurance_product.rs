//! InsuranceProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InsuranceProduct
//!
//! Version: 6.0.0-ballot3
//!
//! Details of a Health Insurance product provided by an organization
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Details of a Health Insurance product provided by an organization.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_product::InsuranceProduct;
/// use fhir::r6::types;
///
/// let value = InsuranceProduct {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: InsuranceProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsuranceProduct {
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

    /// Business Identifier for Product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | active | retired | unknown
    pub status: Option<crate::coded::Coded<crate::r6::codes::PublicationStatus>>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias: Vec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// When the product is available
    pub period: Option<types::Period>,

    /// Product issuer
    pub owned_by: Option<types::Reference>,

    /// Product administrator
    pub administered_by: Option<types::Reference>,

    /// Where product applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage_area: Vec<types::Reference>,

    /// Official contact details relevant to the health insurance product
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
    pub coverage: Vec<InsuranceProductCoverage>,

    /// Associated insurance product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<InsuranceProductRelated>,
}

/// Details about the coverage offered by the insurance product.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::insurance_product::InsuranceProductCoverage;
///
/// let value = InsuranceProductCoverage::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: InsuranceProductCoverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsuranceProductCoverage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Classification of Coverage
    pub r#type: types::CodeableConcept,

    /// What networks provide coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network: Vec<types::Reference>,

    /// List of benefits
    pub benefit: ::vec1::Vec1<InsuranceProductCoverageBenefit>,
}

/// Specific benefits under this type of coverage.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_product::InsuranceProductCoverageBenefit;
/// use fhir::r6::types;
///
/// let value = InsuranceProductCoverageBenefit {
///     requirement: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `requirement` is the name this serializes to on the wire.
/// assert_eq!(json["requirement"], ::serde_json::json!("abc"));
///
/// let back: InsuranceProductCoverageBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsuranceProductCoverageBenefit {
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

    /// Referral requirements
    pub requirement: Option<types::String>,
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// Limits on the provided benefits
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limit: Vec<InsuranceProductCoverageBenefitLimit>,
}

/// The specific limits on the benefit.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_product::InsuranceProductCoverageBenefitLimit;
/// use fhir::r6::types;
///
/// let value = InsuranceProductCoverageBenefitLimit {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsuranceProductCoverageBenefitLimit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsuranceProductCoverageBenefitLimit {
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

/// Another product that is related to this product. Often used to create
/// relationships to parents or families of products.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::insurance_product::InsuranceProductRelated;
/// use fhir::r6::types;
///
/// let value = InsuranceProductRelated {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InsuranceProductRelated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InsuranceProductRelated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Related Product reference
    pub product: Option<types::Reference>,

    /// Relationship of this product to the related product
    pub relationship: Option<types::CodeableConcept>,

    /// Period that this Relationship is valid
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = InsuranceProduct;

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
