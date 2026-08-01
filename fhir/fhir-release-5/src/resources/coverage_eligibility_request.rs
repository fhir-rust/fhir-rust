//! CoverageEligibilityRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityRequest
//!
//! Version: 5.0.0
//!
//! CoverageEligibilityRequest Resource: Provides patient and insurance coverage information to an insurer to determine coverage validity and benefits.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of a
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance
/// details of the policy. It is used to inquire about auth requirements,
/// benefits, coverage discovery, or validation prior to delivering products
/// and services.
///
/// This resource is typically sent by a provider, biller, or patient to a
/// payor system as a pre-check before rendering care, so that the requesting
/// party can confirm eligibility, understand what is covered, and avoid
/// claim denials. A single request may bundle multiple `purpose` values
/// (for example checking both benefits and prior authorization
/// requirements) and may reference one or more items or services, along
/// with any diagnoses, supporting information, or applicable insurance
/// policies needed by the payor to evaluate the inquiry.
///
/// # Related resources
///
/// The `patient` field references [`Patient`](crate::r5::resources::patient::Patient),
/// the `insurer` and `provider` fields reference organizations or
/// practitioners, and coded elements such as `status`, `purpose`, and
/// `priority` use [`CodeableConcept`](crate::r5::types::CodeableConcept) or
/// coded types. A payor typically answers this request with a
/// `CoverageEligibilityResponse` resource.
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::coverage_eligibility_request::CoverageEligibilityRequest;
///
/// let value = CoverageEligibilityRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CoverageEligibilityRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequest {
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

    /// The status of this request; one of active | cancelled | draft | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Desired processing priority, such as normal or stat
    pub priority: Option<types::CodeableConcept>,

    /// The reason(s) for the request, one or more of auth-requirements | benefits | discovery | validation
    pub purpose: vec1::Vec1<crate::r5::coded::Coded<crate::r5::codes::EligibilityrequestPurpose>>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose_ext: Vec<Option<types::Element>>,

    /// Reference to the [`Patient`](crate::r5::resources::patient::Patient) who is the subject of the eligibility check
    pub patient: types::Reference,

    /// Event information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CoverageEligibilityRequestEvent>,

    /// The `CoverageEligibilityRequest.serviced[x]` choice element (0..1); see [`CoverageEligibilityRequestServiced`].
    #[serde(flatten)]
    pub serviced: Option<CoverageEligibilityRequestServiced>,

    /// Creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Author
    pub enterer: Option<types::Reference>,

    /// Party responsible for the request
    pub provider: Option<types::Reference>,

    /// Reference to the insurer organization that will process this request
    pub insurer: types::Reference,

    /// Servicing facility
    pub facility: Option<types::Reference>,

    /// Supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<CoverageEligibilityRequestSupportingInfo>,

    /// Patient insurance information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<CoverageEligibilityRequestInsurance>,

    /// Item to be evaluated for eligibiity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<CoverageEligibilityRequestItem>,
}

/// Additional information codes regarding exceptions, special considerations,
/// the condition, situation, prior or concurrent issues.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_request::CoverageEligibilityRequestEvent;
/// use fhir::r5::types;
///
/// let value = CoverageEligibilityRequestEvent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageEligibilityRequestEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestEvent {
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

    /// The `CoverageEligibilityRequest.event.when[x]` choice element (0..1); see [`CoverageEligibilityRequestEventWhen`].
    #[serde(flatten)]
    pub when: Option<CoverageEligibilityRequestEventWhen>,
}

/// Additional information codes regarding exceptions, special considerations,
/// the condition, situation, prior or concurrent issues supplied in support of
/// the coverage eligibility request.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_request::CoverageEligibilityRequestSupportingInfo;
/// use fhir::r5::types;
///
/// let value = CoverageEligibilityRequestSupportingInfo {
///     applies_to_all: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `appliesToAll` is the name this serializes to on the wire.
/// assert_eq!(json["appliesToAll"], ::serde_json::json!(true));
///
/// let back: CoverageEligibilityRequestSupportingInfo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Information instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Data to be provided
    pub information: types::Reference,

    /// Applies to all items
    pub applies_to_all: Option<types::Boolean>,
    /// Primitive extension sibling for [`applies_to_all`](Self::applies_to_all) (FHIR `_appliesToAll`).
    #[serde(rename = "_appliesToAll")]
    pub applies_to_all_ext: Option<types::Element>,
}

/// Financial instruments for reimbursement for the health care products and
/// services identified in the request.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_request::CoverageEligibilityRequestInsurance;
/// use fhir::r5::types;
///
/// let value = CoverageEligibilityRequestInsurance {
///     business_arrangement: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `businessArrangement` is the name this serializes to on the wire.
/// assert_eq!(json["businessArrangement"], ::serde_json::json!("abc"));
///
/// let back: CoverageEligibilityRequestInsurance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Applicable coverage
    pub focal: Option<types::Boolean>,
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`).
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`).
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,
}

/// Service categories or billable services for which benefit details and/or an
/// authorization prior to service delivery may be required by the payor.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_request::CoverageEligibilityRequestItem;
/// use fhir::r5::types;
///
/// let value = CoverageEligibilityRequestItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageEligibilityRequestItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Applicable exception or supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`supporting_info_sequence`](Self::supporting_info_sequence) (FHIR `_supportingInfoSequence`).
    #[serde(rename = "_supportingInfoSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info_sequence_ext: Vec<Option<types::Element>>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// Product or service billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Perfoming practitioner
    pub provider: Option<types::Reference>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Servicing facility
    pub facility: Option<types::Reference>,

    /// Applicable diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<CoverageEligibilityRequestItemDiagnosis>,

    /// Product or service details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<types::Reference>,
}

/// Patient diagnosis for which care is sought, expressed as a code or a
/// reference to a Condition resource.
/// # Examples
///
/// ```
/// use fhir::r5::resources::coverage_eligibility_request::CoverageEligibilityRequestItemDiagnosis;
/// use fhir::r5::types;
///
/// let value = CoverageEligibilityRequestItemDiagnosis {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CoverageEligibilityRequestItemDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestItemDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `CoverageEligibilityRequest.item.diagnosis.diagnosis[x]` choice element (0..1); see [`CoverageEligibilityRequestItemDiagnosisDiagnosis`].
    #[serde(flatten)]
    pub diagnosis: Option<CoverageEligibilityRequestItemDiagnosisDiagnosis>,
}

/// The `CoverageEligibilityRequest.event.when[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityRequestEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
}

/// The `CoverageEligibilityRequest.item.diagnosis.diagnosis[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityRequestItemDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `CoverageEligibilityRequest.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityRequestServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}
