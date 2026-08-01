//! CoverageEligibilityResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse
//!
//! Version: 6.0.0-ballot3
//!
//! CoverageEligibilityResponse resource
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource provides eligibility and plan details from the processing of
/// an CoverageEligibilityRequest resource.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::coverage_eligibility_response::CoverageEligibilityResponse;
///
/// let value = CoverageEligibilityResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CoverageEligibilityResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageEligibilityResponse {
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

    /// Business Identifier for coverage eligiblity request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// auth-requirements | benefits | discovery | validation
    pub purpose: ::vec1::Vec1<crate::coded::Coded<crate::r6::codes::EligibilityresponsePurpose>>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose_ext: Vec<Option<types::Element>>,

    /// Intended recipient of products and services
    pub patient: types::Reference,

    /// Event information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CoverageEligibilityResponseEvent>,

    /// Estimated date or dates of service
    /// The `CoverageEligibilityResponse.serviced[x]` choice element (0..1); see [`CoverageEligibilityResponseServiced`].
    #[serde(flatten)]
    pub serviced: Option<CoverageEligibilityResponseServiced>,

    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Party responsible for the request
    pub requestor: Option<types::Reference>,

    /// Eligibility request reference
    pub request: types::Reference,

    /// queued | complete | error | partial
    pub outcome: crate::coded::Coded<crate::r6::codes::EligibilityOutcome>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// Coverage issuer
    pub insurer: types::Reference,

    /// Patient insurance information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<CoverageEligibilityResponseInsurance>,

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<types::Element>,

    /// Printed form identifier
    pub form: Option<types::CodeableConcept>,

    /// Processing errors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<CoverageEligibilityResponseError>,
}

/// Errors encountered during the processing of the request.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_response::CoverageEligibilityResponseError;
/// use fhir::r6::types;
///
/// let value = CoverageEligibilityResponseError {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageEligibilityResponseError = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageEligibilityResponseError {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Error code detailing processing issues
    pub code: types::CodeableConcept,

    /// FHIRPath of element(s) related to issue
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression: Vec<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression_ext: Vec<Option<types::Element>>,
}

/// Information code for an event with a corresponding date or period.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_response::CoverageEligibilityResponseEvent;
/// use fhir::r6::types;
///
/// let value = CoverageEligibilityResponseEvent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageEligibilityResponseEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageEligibilityResponseEvent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Specific event
    pub r#type: types::CodeableConcept,

    /// Occurance date or period
    /// The `CoverageEligibilityResponse.event.when[x]` choice element (1..1); see [`CoverageEligibilityResponseEventWhen`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub when: Option<CoverageEligibilityResponseEventWhen>,
}

/// Financial instruments for reimbursement for the health care products and
/// services.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_response::CoverageEligibilityResponseInsurance;
/// use fhir::r6::types;
///
/// let value = CoverageEligibilityResponseInsurance {
///     inforce: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `inforce` is the name this serializes to on the wire.
/// assert_eq!(json["inforce"], ::serde_json::json!(true));
///
/// let back: CoverageEligibilityResponseInsurance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageEligibilityResponseInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Coverage inforce indicator
    pub inforce: Option<types::Boolean>,
    /// Primitive extension sibling for [`inforce`](Self::inforce) (FHIR `_inforce`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_inforce")]
    pub inforce_ext: Option<types::Element>,

    /// When the benefits are applicable
    pub benefit_period: Option<types::Period>,

    /// Benefits and authorization details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<CoverageEligibilityResponseInsuranceItem>,
}

/// Benefits and optionally current balances, and authorization details by
/// category or service.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_response::CoverageEligibilityResponseInsuranceItem;
/// use fhir::r6::types;
///
/// let value = CoverageEligibilityResponseInsuranceItem {
///     authorization_required: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authorizationRequired` is the name this serializes to on the wire.
/// assert_eq!(json["authorizationRequired"], ::serde_json::json!(true));
///
/// let back: CoverageEligibilityResponseInsuranceItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageEligibilityResponseInsuranceItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// Product or service billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Performing practitioner
    pub provider: Option<types::Reference>,

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
    pub benefit: Vec<CoverageEligibilityResponseInsuranceItemBenefit>,

    /// Authorization required flag
    pub authorization_required: Option<types::Boolean>,
    /// Primitive extension sibling for [`authorization_required`](Self::authorization_required) (FHIR `_authorizationRequired`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authorizationRequired")]
    pub authorization_required_ext: Option<types::Element>,

    /// Type of required supporting materials
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_supporting: Vec<types::CodeableConcept>,

    /// Preauthorization requirements endpoint
    pub authorization_url: Option<types::Uri>,
    /// Primitive extension sibling for [`authorization_url`](Self::authorization_url) (FHIR `_authorizationUrl`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authorizationUrl")]
    pub authorization_url_ext: Option<types::Element>,
}

/// Benefits used to date.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_response::CoverageEligibilityResponseInsuranceItemBenefit;
/// use fhir::r6::types;
///
/// let value = CoverageEligibilityResponseInsuranceItemBenefit {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageEligibilityResponseInsuranceItemBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Benefit classification
    pub r#type: types::CodeableConcept,

    /// Benefits allowed
    /// The `CoverageEligibilityResponse.insurance.item.benefit.allowed[x]` choice element (0..1); see [`CoverageEligibilityResponseInsuranceItemBenefitAllowed`].
    #[serde(flatten)]
    pub allowed: Option<CoverageEligibilityResponseInsuranceItemBenefitAllowed>,

    /// Benefits used
    /// The `CoverageEligibilityResponse.insurance.item.benefit.used[x]` choice element (0..1); see [`CoverageEligibilityResponseInsuranceItemBenefitUsed`].
    #[serde(flatten)]
    pub used: Option<CoverageEligibilityResponseInsuranceItemBenefitUsed>,
}

/// The `CoverageEligibilityResponse.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `CoverageEligibilityResponse.event.when[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
}

/// The `CoverageEligibilityResponse.insurance.item.benefit.allowed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitAllowed {
    /// `allowedUnsignedInt` variant.
    #[fhir("allowedUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `allowedString` variant.
    #[fhir("allowedString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `allowedMoney` variant.
    #[fhir("allowedMoney")]
    Money(Box<types::Money>),
}

/// The `CoverageEligibilityResponse.insurance.item.benefit.used[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitUsed {
    /// `usedUnsignedInt` variant.
    #[fhir("usedUnsignedInt")]
    UnsignedInt(crate::r6::choice::Primitive<types::UnsignedInt>),
    /// `usedString` variant.
    #[fhir("usedString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `usedMoney` variant.
    #[fhir("usedMoney")]
    Money(Box<types::Money>),
}
