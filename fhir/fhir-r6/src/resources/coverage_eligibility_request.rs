//! CoverageEligibilityRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CoverageEligibilityRequest
//!
//! Version: 6.0.0-ballot3
//!
//! CoverageEligibilityRequest resource
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance
/// details of the policy.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::coverage_eligibility_request::CoverageEligibilityRequest;
///
/// let value = CoverageEligibilityRequest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: CoverageEligibilityRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "CoverageEligibilityRequestDe")]
#[fhir_version("r6")]
pub struct CoverageEligibilityRequest {
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

    /// Business Identifier for coverage eligiblity request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Desired processing priority
    pub priority: Option<types::CodeableConcept>,

    /// auth-requirements | benefits | discovery | validation
    pub purpose: ::vec1::Vec1<crate::coded::Coded<crate::r6::codes::EligibilityrequestPurpose>>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purpose_ext: Vec<Option<types::Element>>,

    /// Intended recipient of products and services
    pub patient: types::Reference<crate::r6::resources::Patient>,

    /// Event information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<CoverageEligibilityRequestEvent>,

    /// Estimated date or dates of service
    /// The `CoverageEligibilityRequest.serviced[x]` choice element (0..1); see [`CoverageEligibilityRequestServiced`].
    #[serde(flatten)]
    pub serviced: Option<CoverageEligibilityRequestServiced>,

    /// Creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Author
    pub enterer: Option<types::Reference>,

    /// Party responsible for the request
    pub provider: Option<types::Reference>,

    /// Coverage issuer
    pub insurer: types::Reference<crate::r6::resources::Organization>,

    /// Servicing facility
    pub facility: Option<types::Reference<crate::r6::resources::Location>>,

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoverageEligibilityRequestDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: crate::coded::Coded<crate::r6::codes::FmStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    priority: Option<types::CodeableConcept>,
    purpose: ::vec1::Vec1<crate::coded::Coded<crate::r6::codes::EligibilityrequestPurpose>>,
    #[serde(rename = "_purpose")]
    #[serde(default)]
    purpose_ext: Vec<Option<types::Element>>,
    patient: types::Reference<crate::r6::resources::Patient>,
    #[serde(default)]
    event: Vec<CoverageEligibilityRequestEvent>,
    #[serde(flatten)]
    serviced: crate::r6::choice::Slot<CoverageEligibilityRequestServiced>,
    created: types::DateTime,
    #[serde(rename = "_created")]
    created_ext: Option<types::Element>,
    enterer: Option<types::Reference>,
    provider: Option<types::Reference>,
    insurer: types::Reference<crate::r6::resources::Organization>,
    facility: Option<types::Reference<crate::r6::resources::Location>>,
    #[serde(default)]
    supporting_info: Vec<CoverageEligibilityRequestSupportingInfo>,
    #[serde(default)]
    insurance: Vec<CoverageEligibilityRequestInsurance>,
    #[serde(default)]
    item: Vec<CoverageEligibilityRequestItem>,
}

impl ::core::convert::From<CoverageEligibilityRequestDe> for CoverageEligibilityRequest {
    fn from(v: CoverageEligibilityRequestDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            status: v.status,
            status_ext: v.status_ext,
            priority: v.priority,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            patient: v.patient,
            event: v.event,
            serviced: v.serviced.0,
            created: v.created,
            created_ext: v.created_ext,
            enterer: v.enterer,
            provider: v.provider,
            insurer: v.insurer,
            facility: v.facility,
            supporting_info: v.supporting_info,
            insurance: v.insurance,
            item: v.item,
        }
    }
}

/// Information code for an event with a corresponding date or period.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_request::CoverageEligibilityRequestEvent;
/// use fhir::r6::types;
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
#[serde(from = "CoverageEligibilityRequestEventDe")]
#[fhir_version("r6")]
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

    /// Occurance date or period
    /// The `CoverageEligibilityRequest.event.when[x]` choice element (1..1); see [`CoverageEligibilityRequestEventWhen`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub when: Option<CoverageEligibilityRequestEventWhen>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoverageEligibilityRequestEventDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::CodeableConcept,
    #[serde(flatten)]
    when: crate::r6::choice::Slot<CoverageEligibilityRequestEventWhen>,
}

impl ::core::convert::From<CoverageEligibilityRequestEventDe> for CoverageEligibilityRequestEvent {
    fn from(v: CoverageEligibilityRequestEventDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            when: v.when.0,
        }
    }
}

/// Financial instruments for reimbursement for the health care products and
/// services.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_request::CoverageEligibilityRequestInsurance;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,

    /// Insurance information
    pub coverage: types::Reference<crate::r6::resources::Coverage>,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,
}

/// Service categories or billable services for which benefit details and/or an
/// authorization prior to service delivery may be required by the payor.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_request::CoverageEligibilityRequestItem;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`supporting_info_sequence`](Self::supporting_info_sequence) (FHIR `_supportingInfoSequence`):
    /// carries `id` and/or `extension` for the primitive value.
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

/// Patient diagnosis for which care is sought.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_request::CoverageEligibilityRequestItemDiagnosis;
/// use fhir::r6::types;
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
#[serde(from = "CoverageEligibilityRequestItemDiagnosisDe")]
#[fhir_version("r6")]
pub struct CoverageEligibilityRequestItemDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Nature of illness or problem
    /// The `CoverageEligibilityRequest.item.diagnosis.diagnosis[x]` choice element (0..1); see [`CoverageEligibilityRequestItemDiagnosisDiagnosis`].
    #[serde(flatten)]
    pub diagnosis: Option<CoverageEligibilityRequestItemDiagnosisDiagnosis>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoverageEligibilityRequestItemDiagnosisDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    diagnosis: crate::r6::choice::Slot<CoverageEligibilityRequestItemDiagnosisDiagnosis>,
}

impl ::core::convert::From<CoverageEligibilityRequestItemDiagnosisDe>
    for CoverageEligibilityRequestItemDiagnosis
{
    fn from(v: CoverageEligibilityRequestItemDiagnosisDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            diagnosis: v.diagnosis.0,
        }
    }
}

/// Additional information codes regarding exceptions, special considerations,
/// the condition, situation, prior or concurrent issues.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::coverage_eligibility_request::CoverageEligibilityRequestSupportingInfo;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Data to be provided
    pub information: types::Reference,

    /// Applies to all items
    pub applies_to_all: Option<types::Boolean>,
    /// Primitive extension sibling for [`applies_to_all`](Self::applies_to_all) (FHIR `_appliesToAll`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_appliesToAll")]
    pub applies_to_all_ext: Option<types::Element>,
}

/// The `CoverageEligibilityRequest.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityRequestServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `CoverageEligibilityRequest.event.when[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityRequestEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
}

/// The `CoverageEligibilityRequest.item.diagnosis.diagnosis[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum CoverageEligibilityRequestItemDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}
