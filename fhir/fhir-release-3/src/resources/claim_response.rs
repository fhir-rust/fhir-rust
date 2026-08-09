//! ClaimResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
//!
//!
//!
//! Remittance resource
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ClaimResponse Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponse;
/// use fhir::r3::types;
///
/// let value = ClaimResponse {
///     created: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `created` is the name this serializes to on the wire.
/// assert_eq!(json["created"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ClaimResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Response number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r3::codes::FmStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The subject of the Products and Services
    pub patient: Option<types::Reference<crate::r3::resources::Patient>>,

    /// Creation date
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Insurance issuing organization
    pub insurer: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Responsible practitioner
    pub request_provider: Option<types::Reference<crate::r3::resources::Practitioner>>,

    /// Responsible organization
    pub request_organization: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Id of resource triggering adjudication
    pub request: Option<types::Reference<crate::r3::resources::Claim>>,

    /// complete | error | partial
    pub outcome: Option<types::CodeableConcept>,

    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// Party to be paid any benefits payable
    pub payee_type: Option<types::CodeableConcept>,

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
    pub total_cost: Option<types::Money>,

    /// Unallocated deductible
    pub unalloc_deductable: Option<types::Money>,

    /// Total benefit payable for the Claim
    pub total_benefit: Option<types::Money>,

    /// Payment details, if paid
    pub payment: Option<ClaimResponsePayment>,

    /// Funds reserved status
    pub reserved: Option<types::Coding>,

    /// Printed Form Identifier
    pub form: Option<types::CodeableConcept>,

    /// Processing notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_note: Vec<ClaimResponseProcessNote>,

    /// Request for additional information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication_request: Vec<types::Reference<crate::r3::resources::CommunicationRequest>>,

    /// Insurance or medical plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<ClaimResponseInsurance>,
}

/// The first tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseAddItem;
/// use fhir::r3::types;
///
/// let value = ClaimResponseAddItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseAddItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseAddItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Type of service or product
    pub category: Option<types::CodeableConcept>,

    /// Group, Service or Product
    pub service: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Professional fee or Product charge
    pub fee: Option<types::Money>,

    /// List of note numbers which apply
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Added items details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimResponseAddItemDetail>,
}

/// The second tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseAddItemDetail;
/// use fhir::r3::types;
///
/// let value = ClaimResponseAddItemDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseAddItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseAddItemDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Type of service or product
    pub category: Option<types::CodeableConcept>,

    /// Service or Product
    pub service: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Professional fee or Product charge
    pub fee: Option<types::Money>,

    /// List of note numbers which apply
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Added items detail adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,
}

/// Mutually exclusive with Services Provided (Item).
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseError;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ClaimResponseError {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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
    pub code: types::CodeableConcept,
}

/// Financial instrument by which payment information for health care.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseInsurance;
/// use fhir::r3::types;
///
/// let value = ClaimResponseInsurance {
///     business_arrangement: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `businessArrangement` is the name this serializes to on the wire.
/// assert_eq!(json["businessArrangement"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseInsurance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseInsurance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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
    pub coverage: types::Reference<crate::r3::resources::Coverage>,

    /// Business agreement
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,

    /// Pre-Authorization/Determination Reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,

    /// Adjudication results
    pub claim_response: Option<types::Reference<crate::r3::resources::ClaimResponse>>,
}

/// The first tier service adjudications for submitted services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseItem;
/// use fhir::r3::types;
///
/// let value = ClaimResponseItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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

/// The adjudication results.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseItemAdjudication;
/// use fhir::r3::types;
///
/// let value = ClaimResponseItemAdjudication {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseItemAdjudication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseItemAdjudication {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Adjudication category such as co-pay, eligible, benefit, etc.
    pub category: types::CodeableConcept,

    /// Explanation of Adjudication outcome
    pub reason: Option<types::CodeableConcept>,

    /// Monetary amount
    pub amount: Option<types::Money>,

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
/// use fhir::r3::resources::claim_response::ClaimResponseItemDetail;
/// use fhir::r3::types;
///
/// let value = ClaimResponseItemDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseItemDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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

    /// Detail level adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Subdetail line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}

/// The third tier service adjudications for submitted services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseItemDetailSubDetail;
/// use fhir::r3::types;
///
/// let value = ClaimResponseItemDetailSubDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseItemDetailSubDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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

    /// Subdetail level adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,
}

/// Payment details for the claim if the claim has been paid.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponsePayment;
/// use fhir::r3::types;
///
/// let value = ClaimResponsePayment {
///     date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01"));
///
/// let back: ClaimResponsePayment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponsePayment {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Partial or Complete
    pub r#type: Option<types::CodeableConcept>,

    /// Payment adjustment for non-Claim issues
    pub adjustment: Option<types::Money>,

    /// Explanation for the non-claim adjustment
    pub adjustment_reason: Option<types::CodeableConcept>,

    /// Expected data of Payment
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Payable amount after adjustment
    pub amount: Option<types::Money>,

    /// Identifier of the payment instrument
    pub identifier: Option<types::Identifier>,
}

/// Note text.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim_response::ClaimResponseProcessNote;
/// use fhir::r3::types;
///
/// let value = ClaimResponseProcessNote {
///     number: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `number` is the name this serializes to on the wire.
/// assert_eq!(json["number"], ::serde_json::json!(1));
///
/// let back: ClaimResponseProcessNote = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimResponseProcessNote {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Sequence Number for this note
    pub number: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// display | print | printoper
    pub r#type: Option<types::CodeableConcept>,

    /// Note explanatory text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Language if different from the resource
    pub language: Option<types::CodeableConcept>,
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
