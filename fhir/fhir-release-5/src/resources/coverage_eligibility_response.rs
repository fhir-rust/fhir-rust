//! CoverageEligibilityResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityResponse
//!
//! Version: 5.0.0
//!
//! CoverageEligibilityResponse Resource: This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource provides eligibility and plan details from the processing of a
/// CoverageEligibilityRequest resource. It conveys the insurer's response about
/// whether coverage is in force for a patient, along with the benefits, limits,
/// authorization requirements, and any processing errors. A
/// CoverageEligibilityResponse is typically returned in reply to a
/// CoverageEligibilityRequest.
///
/// Administratively, this resource lets a payer (insurer) communicate, for a
/// given patient and coverage, whether the requested benefits are currently in
/// force, what cost-sharing or limits apply, and whether prior authorization is
/// required before services are rendered. It supports several use cases
/// distinguished by the `purpose` field: general eligibility discovery,
/// validation that a coverage is active, benefit detail lookup, and
/// authorization requirement checks. Because eligibility and benefit data can be
/// extensive, the response is organized into nested groups covering the events
/// that occurred during adjudication, the insurance coverages evaluated, the
/// benefit items and their categories, and any errors encountered while
/// processing the originating request.
///
/// # Related resources
///
/// A `CoverageEligibilityResponse` is produced in reply to a
/// `CoverageEligibilityRequest` and typically references a
/// [`Patient`](crate::r5::resources::patient::Patient), an insurer and
/// requestor represented as `Reference` values, and coverage details described
/// using [`CodeableConcept`](crate::r5::types::CodeableConcept) values for
/// categories, product or service codes, and benefit types.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::coverage_eligibility_response::CoverageEligibilityResponse;
///
/// let value = CoverageEligibilityResponse::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CoverageEligibilityResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponse {
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

    /// Business Identifier for coverage eligiblity request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The status of the resource instance itself: active | cancelled | draft | entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The reason(s) this eligibility check was performed: auth-requirements | benefits | discovery | validation.
    pub purpose: vec1::Vec1<crate::r5::coded::Coded<crate::r5::codes::EligibilityresponsePurpose>>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose_ext: Vec<Option<types::Element>>,

    /// Reference to the [`Patient`](crate::r5::resources::patient::Patient) whose coverage is being described.
    pub patient: types::Reference,

    /// Event information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CoverageEligibilityResponseEvent>,

    /// The `CoverageEligibilityResponse.serviced[x]` choice element (0..1); see [`CoverageEligibilityResponseServiced`].
    #[serde(flatten)]
    pub serviced: Option<CoverageEligibilityResponseServiced>,

    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Party responsible for the request
    pub requestor: Option<types::Reference>,

    /// Eligibility request reference
    pub request: types::Reference,

    /// The outcome of the processing: queued | complete | error | partial.
    pub outcome: crate::r5::coded::Coded<crate::r5::codes::EligibilityOutcome>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`).
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`).
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// Coverage issuer
    pub insurer: types::Reference,

    /// Patient insurance information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<CoverageEligibilityResponseInsurance>,

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<types::Element>,

    /// Printed form identifier
    pub form: Option<types::CodeableConcept>,

    /// Processing errors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<CoverageEligibilityResponseError>,
}

/// Event information for a CoverageEligibilityResponse.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_response::CoverageEligibilityResponseEvent;
/// use fhir::r5::types;
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

    /// The `CoverageEligibilityResponse.event.when[x]` choice element (0..1); see [`CoverageEligibilityResponseEventWhen`].
    #[serde(flatten)]
    pub when: Option<CoverageEligibilityResponseEventWhen>,
}

/// Patient insurance information within a CoverageEligibilityResponse.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_response::CoverageEligibilityResponseInsurance;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`inforce`](Self::inforce) (FHIR `_inforce`).
    #[serde(rename = "_inforce")]
    pub inforce_ext: Option<types::Element>,

    /// When the benefits are applicable
    pub benefit_period: Option<types::Period>,

    /// Benefits and authorization details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<CoverageEligibilityResponseInsuranceItem>,
}

/// Benefits and authorization details for an insurance entry.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_response::CoverageEligibilityResponseInsuranceItem;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`excluded`](Self::excluded) (FHIR `_excluded`).
    #[serde(rename = "_excluded")]
    pub excluded_ext: Option<types::Element>,

    /// Short name for the benefit
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Description of the benefit or services covered
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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
    /// Primitive extension sibling for [`authorization_required`](Self::authorization_required) (FHIR `_authorizationRequired`).
    #[serde(rename = "_authorizationRequired")]
    pub authorization_required_ext: Option<types::Element>,

    /// Type of required supporting materials
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_supporting: Vec<types::CodeableConcept>,

    /// Preauthorization requirements endpoint
    pub authorization_url: Option<types::Uri>,
    /// Primitive extension sibling for [`authorization_url`](Self::authorization_url) (FHIR `_authorizationUrl`).
    #[serde(rename = "_authorizationUrl")]
    pub authorization_url_ext: Option<types::Element>,
}

/// Benefit Summary detailing allowed and used amounts.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_response::CoverageEligibilityResponseInsuranceItemBenefit;
/// use fhir::r5::types;
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

    /// The `CoverageEligibilityResponse.insurance.item.benefit.allowed[x]` choice element (0..1); see [`CoverageEligibilityResponseInsuranceItemBenefitAllowed`].
    #[serde(flatten)]
    pub allowed: Option<CoverageEligibilityResponseInsuranceItemBenefitAllowed>,

    /// The `CoverageEligibilityResponse.insurance.item.benefit.used[x]` choice element (0..1); see [`CoverageEligibilityResponseInsuranceItemBenefitUsed`].
    #[serde(flatten)]
    pub used: Option<CoverageEligibilityResponseInsuranceItemBenefitUsed>,
}

/// Processing errors reported in a CoverageEligibilityResponse.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_response::CoverageEligibilityResponseError;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`).
    #[serde(rename = "_expression")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression_ext: Vec<Option<types::Element>>,
}

/// The `CoverageEligibilityResponse.event.when[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
}

/// The `CoverageEligibilityResponse.insurance.item.benefit.allowed[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitAllowed {
    /// `allowedUnsignedInt` variant.
    #[fhir("allowedUnsignedInt")]
    UnsignedInt(crate::r5::choice::Primitive<types::UnsignedInt>),
    /// `allowedString` variant.
    #[fhir("allowedString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `allowedMoney` variant.
    #[fhir("allowedMoney")]
    Money(Box<types::Money>),
}

/// The `CoverageEligibilityResponse.insurance.item.benefit.used[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitUsed {
    /// `usedUnsignedInt` variant.
    #[fhir("usedUnsignedInt")]
    UnsignedInt(crate::r5::choice::Primitive<types::UnsignedInt>),
    /// `usedString` variant.
    #[fhir("usedString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `usedMoney` variant.
    #[fhir("usedMoney")]
    Money(Box<types::Money>),
}

/// The `CoverageEligibilityResponse.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityResponseServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}
