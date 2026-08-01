//! EligibilityResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/EligibilityResponse
//!
//!
//!
//! EligibilityResponse resource
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for EligibilityResponse Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::eligibility_response::EligibilityResponse;
/// use fhir::r3::types;
///
/// let value = EligibilityResponse {
///     created: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `created` is the name this serializes to on the wire.
/// assert_eq!(json["created"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: EligibilityResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EligibilityResponse {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r3::codes::FmStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Creation date
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Responsible practitioner
    pub request_provider: Option<types::Reference>,

    /// Responsible organization
    pub request_organization: Option<types::Reference>,

    /// Eligibility reference
    pub request: Option<types::Reference>,

    /// complete | error | partial
    pub outcome: Option<types::CodeableConcept>,

    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// Insurer issuing the coverage
    pub insurer: Option<types::Reference>,

    /// Coverage inforce indicator
    pub inforce: Option<types::Boolean>,
    /// Primitive extension sibling for [`inforce`](Self::inforce) (FHIR `_inforce`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_inforce")]
    pub inforce_ext: Option<types::Element>,

    /// Details by insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<EligibilityResponseInsurance>,

    /// Printed Form Identifier
    pub form: Option<types::CodeableConcept>,

    /// Processing errors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<EligibilityResponseError>,
}

/// Mutually exclusive with Services Provided (Item).
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::eligibility_response::EligibilityResponseError;
/// use fhir::r3::types;
///
/// let value = EligibilityResponseError {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EligibilityResponseError = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EligibilityResponseError {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Error code detailing processing issues
    pub code: types::CodeableConcept,
}

/// The insurer may provide both the details for the requested coverage as well
/// as details for additional coverages known to the insurer.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::eligibility_response::EligibilityResponseInsurance;
/// use fhir::r3::types;
///
/// let value = EligibilityResponseInsurance {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EligibilityResponseInsurance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EligibilityResponseInsurance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Updated Coverage details
    pub coverage: Option<types::Reference>,

    /// Contract details
    pub contract: Option<types::Reference>,

    /// Benefits by Category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub benefit_balance: Vec<EligibilityResponseInsuranceBenefitBalance>,
}

/// Benefits and optionally current balances by Category.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::eligibility_response::EligibilityResponseInsuranceBenefitBalance;
/// use fhir::r3::types;
///
/// let value = EligibilityResponseInsuranceBenefitBalance {
///     excluded: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `excluded` is the name this serializes to on the wire.
/// assert_eq!(json["excluded"], ::serde_json::json!(true));
///
/// let back: EligibilityResponseInsuranceBenefitBalance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EligibilityResponseInsuranceBenefitBalance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of services covered
    pub category: types::CodeableConcept,

    /// Detailed services covered within the type
    pub sub_category: Option<types::CodeableConcept>,

    /// Excluded from the plan
    pub excluded: Option<types::Boolean>,
    /// Primitive extension sibling for [`excluded`](Self::excluded) (FHIR `_excluded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_excluded")]
    pub excluded_ext: Option<types::Element>,

    /// Short name for the benefit
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Description of the benefit or services covered
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// In or out of network
    pub network: Option<types::CodeableConcept>,

    /// Individual or family
    pub unit: Option<types::CodeableConcept>,

    /// Annual or lifetime
    pub term: Option<types::CodeableConcept>,

    /// Benefit Summary
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub financial: Vec<EligibilityResponseInsuranceBenefitBalanceFinancial>,
}

/// Benefits Used to date.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::eligibility_response::EligibilityResponseInsuranceBenefitBalanceFinancial;
/// use fhir::r3::types;
///
/// let value = EligibilityResponseInsuranceBenefitBalanceFinancial {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: EligibilityResponseInsuranceBenefitBalanceFinancial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct EligibilityResponseInsuranceBenefitBalanceFinancial {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Deductable, visits, benefit amount
    pub r#type: types::CodeableConcept,

    /// Benefits allowed
    /// The `EligibilityResponse.insurance.benefitBalance.financial.allowed[x]` choice element (0..1); see [`EligibilityResponseInsuranceBenefitBalanceFinancialAllowed`].
    #[serde(flatten)]
    pub allowed: Option<EligibilityResponseInsuranceBenefitBalanceFinancialAllowed>,

    /// Benefits used
    /// The `EligibilityResponse.insurance.benefitBalance.financial.used[x]` choice element (0..1); see [`EligibilityResponseInsuranceBenefitBalanceFinancialUsed`].
    #[serde(flatten)]
    pub used: Option<EligibilityResponseInsuranceBenefitBalanceFinancialUsed>,
}

/// The `EligibilityResponse.insurance.benefitBalance.financial.allowed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum EligibilityResponseInsuranceBenefitBalanceFinancialAllowed {
    /// `allowedUnsignedInt` variant.
    #[fhir("allowedUnsignedInt")]
    UnsignedInt(crate::r3::choice::Primitive<types::UnsignedInt>),
    /// `allowedString` variant.
    #[fhir("allowedString")]
    String(crate::r3::choice::Primitive<types::String>),
    /// `allowedMoney` variant.
    #[fhir("allowedMoney")]
    Money(Box<types::Money>),
}

/// The `EligibilityResponse.insurance.benefitBalance.financial.used[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum EligibilityResponseInsuranceBenefitBalanceFinancialUsed {
    /// `usedUnsignedInt` variant.
    #[fhir("usedUnsignedInt")]
    UnsignedInt(crate::r3::choice::Primitive<types::UnsignedInt>),
    /// `usedMoney` variant.
    #[fhir("usedMoney")]
    Money(Box<types::Money>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = EligibilityResponse;

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
