//! ClaimResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
//!
//!
//!
//! Remittance resource
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ClaimResponse Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponse;
/// use fhir::r2::types;
///
/// let value = ClaimResponse {
///     payment_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `paymentDate` is the name this serializes to on the wire.
/// assert_eq!(json["paymentDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: ClaimResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponse {
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

    /// Response number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Id of resource triggering adjudication
    pub request: Option<types::Reference>,

    /// Resource version
    pub ruleset: Option<types::Coding>,

    /// Original version
    pub original_ruleset: Option<types::Coding>,

    /// Creation date
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Insurer
    pub organization: Option<types::Reference>,

    /// Responsible practitioner
    pub request_provider: Option<types::Reference>,

    /// Responsible organization
    pub request_organization: Option<types::Reference>,

    /// complete | error
    pub outcome: Option<crate::coded::Coded<crate::r2::codes::RemittanceOutcome>>,
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

    /// Party to be paid any benefits payable
    pub payee_type: Option<types::Coding>,

    /// Line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ClaimResponseItem>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_item: Vec<ClaimResponseAddItem>,

    /// Processing errors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<ClaimResponseError>,

    /// Total Cost of service from the Claim
    pub total_cost: Option<types::Quantity>,

    /// Unallocated deductible
    pub unalloc_deductable: Option<types::Quantity>,

    /// Total benefit payable for the Claim
    pub total_benefit: Option<types::Quantity>,

    /// Payment adjustment for non-Claim issues
    pub payment_adjustment: Option<types::Quantity>,

    /// Reason for Payment adjustment
    pub payment_adjustment_reason: Option<types::Coding>,

    /// Expected data of Payment
    pub payment_date: Option<types::Date>,
    /// Primitive extension sibling for [`payment_date`](Self::payment_date) (FHIR `_paymentDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_paymentDate")]
    pub payment_date_ext: Option<types::Element>,

    /// Payment amount
    pub payment_amount: Option<types::Quantity>,

    /// Payment identifier
    pub payment_ref: Option<types::Identifier>,

    /// Funds reserved status
    pub reserved: Option<types::Coding>,

    /// Printed Form Identifier
    pub form: Option<types::Coding>,

    /// Processing notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ClaimResponseNote>,

    /// Insurance or medical plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<ClaimResponseCoverage>,
}

/// The first tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseAddItem;
/// use fhir::r2::types;
///
/// let value = ClaimResponseAddItem {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseAddItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseAddItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sequence_link_id: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`sequence_link_id`](Self::sequence_link_id) (FHIR `_sequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequenceLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sequence_link_id_ext: Vec<Option<types::Element>>,

    /// Group, Service or Product
    pub service: types::Coding,

    /// Professional fee or Product charge
    pub fee: Option<types::Quantity>,

    /// List of note numbers which apply
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_link_id: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number_link_id`](Self::note_number_link_id) (FHIR `_noteNumberLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumberLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_link_id_ext: Vec<Option<types::Element>>,

    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseAddItemAdjudication>,

    /// Added items details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimResponseAddItemDetail>,
}

/// The adjudications results.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseAddItemAdjudication;
/// use fhir::r2::types;
///
/// let value = ClaimResponseAddItemAdjudication {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseAddItemAdjudication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseAddItemAdjudication {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adjudication category such as co-pay, eligible, benefit, etc.
    pub code: types::Coding,

    /// Monetary amount
    pub amount: Option<types::Quantity>,

    /// Non-monetary value
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The second tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseAddItemDetail;
/// use fhir::r2::types;
///
/// let value = ClaimResponseAddItemDetail {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseAddItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseAddItemDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service or Product
    pub service: types::Coding,

    /// Professional fee or Product charge
    pub fee: Option<types::Quantity>,

    /// Added items detail adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseAddItemDetailAdjudication>,
}

/// The adjudications results.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseAddItemDetailAdjudication;
/// use fhir::r2::types;
///
/// let value = ClaimResponseAddItemDetailAdjudication {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseAddItemDetailAdjudication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseAddItemDetailAdjudication {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adjudication category such as co-pay, eligible, benefit, etc.
    pub code: types::Coding,

    /// Monetary amount
    pub amount: Option<types::Quantity>,

    /// Non-monetary value
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Financial instrument by which payment information for health care.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseCoverage;
/// use fhir::r2::types;
///
/// let value = ClaimResponseCoverage {
///     business_arrangement: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `businessArrangement` is the name this serializes to on the wire.
/// assert_eq!(json["businessArrangement"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseCoverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseCoverage {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Is the focal Coverage
    pub focal: types::Boolean,
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Business agreement
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,

    /// Patient relationship to subscriber
    pub relationship: types::Coding,

    /// Pre-Authorization/Determination Reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,

    /// Adjudication results
    pub claim_response: Option<types::Reference>,

    /// Original version
    pub original_ruleset: Option<types::Coding>,
}

/// Mutually exclusive with Services Provided (Item).
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseError;
/// use fhir::r2::types;
///
/// let value = ClaimResponseError {
///     sequence_link_id: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sequenceLinkId` is the name this serializes to on the wire.
/// assert_eq!(json["sequenceLinkId"], ::serde_json::json!(1));
///
/// let back: ClaimResponseError = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseError {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Item sequence number
    pub sequence_link_id: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`sequence_link_id`](Self::sequence_link_id) (FHIR `_sequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequenceLinkId")]
    pub sequence_link_id_ext: Option<types::Element>,

    /// Detail sequence number
    pub detail_sequence_link_id: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`detail_sequence_link_id`](Self::detail_sequence_link_id) (FHIR `_detailSequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detailSequenceLinkId")]
    pub detail_sequence_link_id_ext: Option<types::Element>,

    /// Subdetail sequence number
    pub subdetail_sequence_link_id: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`subdetail_sequence_link_id`](Self::subdetail_sequence_link_id) (FHIR `_subdetailSequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subdetailSequenceLinkId")]
    pub subdetail_sequence_link_id_ext: Option<types::Element>,

    /// Error code detailing processing issues
    pub code: types::Coding,
}

/// The first tier service adjudications for submitted services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseItem;
/// use fhir::r2::types;
///
/// let value = ClaimResponseItem {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance
    pub sequence_link_id: types::PositiveInt,
    /// Primitive extension sibling for [`sequence_link_id`](Self::sequence_link_id) (FHIR `_sequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequenceLinkId")]
    pub sequence_link_id_ext: Option<types::Element>,

    /// List of note numbers which apply
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Detail line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimResponseItemDetail>,
}

/// The adjudications results.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseItemAdjudication;
/// use fhir::r2::types;
///
/// let value = ClaimResponseItemAdjudication {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseItemAdjudication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseItemAdjudication {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adjudication category such as co-pay, eligible, benefit, etc.
    pub code: types::Coding,

    /// Monetary amount
    pub amount: Option<types::Quantity>,

    /// Non-monetary value
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The second tier service adjudications for submitted services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseItemDetail;
/// use fhir::r2::types;
///
/// let value = ClaimResponseItemDetail {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseItemDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance
    pub sequence_link_id: types::PositiveInt,
    /// Primitive extension sibling for [`sequence_link_id`](Self::sequence_link_id) (FHIR `_sequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequenceLinkId")]
    pub sequence_link_id_ext: Option<types::Element>,

    /// Detail adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemDetailAdjudication>,

    /// Subdetail line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}

/// The adjudications results.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseItemDetailAdjudication;
/// use fhir::r2::types;
///
/// let value = ClaimResponseItemDetailAdjudication {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseItemDetailAdjudication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseItemDetailAdjudication {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adjudication category such as co-pay, eligible, benefit, etc.
    pub code: types::Coding,

    /// Monetary amount
    pub amount: Option<types::Quantity>,

    /// Non-monetary value
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The third tier service adjudications for submitted services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseItemDetailSubDetail;
/// use fhir::r2::types;
///
/// let value = ClaimResponseItemDetailSubDetail {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseItemDetailSubDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance
    pub sequence_link_id: types::PositiveInt,
    /// Primitive extension sibling for [`sequence_link_id`](Self::sequence_link_id) (FHIR `_sequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequenceLinkId")]
    pub sequence_link_id_ext: Option<types::Element>,

    /// Subdetail adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemDetailSubDetailAdjudication>,
}

/// The adjudications results.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseItemDetailSubDetailAdjudication;
/// use fhir::r2::types;
///
/// let value = ClaimResponseItemDetailSubDetailAdjudication {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimResponseItemDetailSubDetailAdjudication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseItemDetailSubDetailAdjudication {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adjudication category such as co-pay, eligible, benefit, etc.
    pub code: types::Coding,

    /// Monetary amount
    pub amount: Option<types::Quantity>,

    /// Non-monetary value
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Note text.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim_response::ClaimResponseNote;
/// use fhir::r2::types;
///
/// let value = ClaimResponseNote {
///     number: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `number` is the name this serializes to on the wire.
/// assert_eq!(json["number"], ::serde_json::json!(1));
///
/// let back: ClaimResponseNote = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimResponseNote {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Note Number for this note
    pub number: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// display | print | printoper
    pub r#type: Option<types::Coding>,

    /// Note explanatory text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ClaimResponse;

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
