//! ClaimResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
//!
//! Version: 6.0.0-ballot3
//!
//! Response to a claim predetermination or preauthorization
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource provides the adjudication details from the processing of a
/// Claim resource.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponse;
/// use fhir::r6::types;
///
/// let value = ClaimResponse {
///     pre_auth_ref: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preAuthRef` is the name this serializes to on the wire.
/// assert_eq!(json["preAuthRef"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClaimResponse {
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

    /// Business Identifier for a claim response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// More granular claim type
    pub r#type: types::CodeableConcept,

    /// More granular claim type
    pub sub_type: Option<types::CodeableConcept>,

    /// claim | preauthorization | predetermination
    pub r#use: crate::coded::Coded<crate::r6::codes::ClaimUse>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// The recipient of the products and services
    pub patient: types::Reference,

    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Party responsible for reimbursement
    pub insurer: Option<types::Reference>,

    /// Party responsible for the claim
    pub requestor: Option<types::Reference>,

    /// Id of resource triggering adjudication
    pub request: Option<types::Reference>,

    /// queued | complete | error | partial
    pub outcome: crate::coded::Coded<crate::r6::codes::ClaimOutcome>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,

    /// Result of the adjudication
    pub decision: Option<types::CodeableConcept>,

    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<types::Element>,

    /// Preauthorization reference effective period
    pub pre_auth_period: Option<types::Period>,

    /// Event information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<ClaimResponseEvent>,

    /// Party to be paid any benefits payable
    pub payee_type: Option<types::CodeableConcept>,

    /// Encounters associated with the listed treatments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encounter: Vec<types::Reference>,

    /// Package billing code
    pub diagnosis_related_group: Option<types::CodeableConcept>,

    /// Adjudication for claim line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ClaimResponseItem>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_item: Vec<ClaimResponseAddItem>,

    /// Header-level adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Adjudication totals
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub total: Vec<ClaimResponseTotal>,

    /// Payment Details
    pub payment: Option<ClaimResponsePayment>,

    /// Funds reserved status
    pub funds_reserve: Option<types::CodeableConcept>,

    /// Printed form identifier
    pub form_code: Option<types::CodeableConcept>,

    /// Printed reference or actual form
    pub form: Option<types::Attachment>,

    /// Note concerning adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_note: Vec<ClaimResponseProcessNote>,

    /// Request for additional information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub communication_request: Vec<types::Reference>,

    /// Patient insurance information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<ClaimResponseInsurance>,

    /// Processing errors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub error: Vec<ClaimResponseError>,
}

/// The first-tier service adjudications for payor added product or service
/// lines.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseAddItem;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseAddItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Item sequence number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`item_sequence`](Self::item_sequence) (FHIR `_itemSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_itemSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_sequence_ext: Vec<Option<types::Element>>,

    /// Detail sequence number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`detail_sequence`](Self::detail_sequence) (FHIR `_detailSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detailSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail_sequence_ext: Vec<Option<types::Element>>,

    /// Subdetail sequence number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subdetail_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`subdetail_sequence`](Self::subdetail_sequence) (FHIR `_subdetailSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subdetailSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subdetail_sequence_ext: Vec<Option<types::Element>>,

    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,

    /// Authorized providers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<types::Reference>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,

    /// Request or Referral for Service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<types::Reference>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Program the product or service is provided under
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,

    /// Date or dates of service or product delivery
    /// The `ClaimResponse.addItem.serviced[x]` choice element (0..1); see [`ClaimResponseAddItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ClaimResponseAddItemServiced>,

    /// Place of service or where product was supplied
    /// The `ClaimResponse.addItem.location[x]` choice element (0..1); see [`ClaimResponseAddItemLocation`].
    #[serde(flatten)]
    pub location: Option<ClaimResponseAddItemLocation>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Total tax
    pub tax: Option<types::Money>,

    /// Total item cost
    pub net: Option<types::Money>,

    /// Anatomical location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<ClaimResponseAddItemBodySite>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Added items adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Insurer added line details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimResponseAddItemDetail>,
}

/// Physical location where the service is performed or applies.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::claim_response::ClaimResponseAddItemBodySite;
///
/// let value = ClaimResponseAddItemBodySite::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClaimResponseAddItemBodySite = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClaimResponseAddItemBodySite {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location
    pub site: ::vec1::Vec1<types::CodeableReference>,

    /// Sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::CodeableConcept>,
}

/// The second-tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseAddItemDetail;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseAddItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Total tax
    pub tax: Option<types::Money>,

    /// Total item cost
    pub net: Option<types::Money>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Added items detail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items detail adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimResponseAddItemDetailSubDetail>,
}

/// The third-tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseAddItemDetailSubDetail;
/// use fhir::r6::types;
///
/// let value = ClaimResponseAddItemDetailSubDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseAddItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClaimResponseAddItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,

    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Count of products or services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Total tax
    pub tax: Option<types::Money>,

    /// Total item cost
    pub net: Option<types::Money>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Added items subdetail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items subdetail adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,
}

/// Errors encountered during the processing of the adjudication.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseError;
/// use fhir::r6::types;
///
/// let value = ClaimResponseError {
///     item_sequence: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `itemSequence` is the name this serializes to on the wire.
/// assert_eq!(json["itemSequence"], ::serde_json::json!(1));
///
/// let back: ClaimResponseError = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClaimResponseError {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Item sequence number
    pub item_sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`item_sequence`](Self::item_sequence) (FHIR `_itemSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_itemSequence")]
    pub item_sequence_ext: Option<types::Element>,

    /// Detail sequence number
    pub detail_sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`detail_sequence`](Self::detail_sequence) (FHIR `_detailSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detailSequence")]
    pub detail_sequence_ext: Option<types::Element>,

    /// Subdetail sequence number
    pub sub_detail_sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`sub_detail_sequence`](Self::sub_detail_sequence) (FHIR `_subDetailSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subDetailSequence")]
    pub sub_detail_sequence_ext: Option<types::Element>,

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
/// use fhir::r6::resources::claim_response::ClaimResponseEvent;
/// use fhir::r6::types;
///
/// let value = ClaimResponseEvent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClaimResponseEvent {
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
    /// The `ClaimResponse.event.when[x]` choice element (1..1); see [`ClaimResponseEventWhen`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub when: Option<ClaimResponseEventWhen>,
}

/// Financial instruments for reimbursement for the health care products and
/// services specified on the claim.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseInsurance;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Insurance instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Coverage to be used for adjudication
    pub focal: types::Boolean,
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,

    /// Adjudication results
    pub claim_response: Option<types::Reference>,
}

/// A claim line. Either a simple (a product or service) or a 'group' of
/// details which can also be a simple items or groups of sub-details.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseItem;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Claim item instance identifier
    pub item_sequence: types::PositiveInt,
    /// Primitive extension sibling for [`item_sequence`](Self::item_sequence) (FHIR `_itemSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_itemSequence")]
    pub item_sequence_ext: Option<types::Element>,

    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Adjudication for claim details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimResponseItemDetail>,
}

/// If this item is a group then the values here are a summary of the
/// adjudication of the detail items. If this item is a simple product or
/// service then this is the result of the adjudication of this item.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseItemAdjudication;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseItemAdjudication {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of adjudication information
    pub category: types::CodeableConcept,

    /// Explanation of adjudication outcome
    pub reason: Option<types::CodeableConcept>,

    /// Monetary amount
    pub amount: Option<types::Money>,

    /// Non-monetary value
    pub quantity: Option<types::Quantity>,
}

/// A claim detail. Either a simple (a product or service) or a 'group' of
/// sub-details which are simple items.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseItemDetail;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Claim detail instance identifier
    pub detail_sequence: types::PositiveInt,
    /// Primitive extension sibling for [`detail_sequence`](Self::detail_sequence) (FHIR `_detailSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detailSequence")]
    pub detail_sequence_ext: Option<types::Element>,

    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Detail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Detail level adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Adjudication for claim sub-details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}

/// A sub-detail adjudication of a simple product or service.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseItemDetailSubDetail;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Claim sub-detail instance identifier
    pub sub_detail_sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sub_detail_sequence`](Self::sub_detail_sequence) (FHIR `_subDetailSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subDetailSequence")]
    pub sub_detail_sequence_ext: Option<types::Element>,

    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Subdetail level adjudication results
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Subdetail level adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ClaimResponseItemAdjudication>,
}

/// The high-level results of the adjudication if adjudication has been
/// performed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseItemReviewOutcome;
/// use fhir::r6::types;
///
/// let value = ClaimResponseItemReviewOutcome {
///     pre_auth_ref: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preAuthRef` is the name this serializes to on the wire.
/// assert_eq!(json["preAuthRef"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseItemReviewOutcome = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClaimResponseItemReviewOutcome {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Result of the adjudication
    pub decision: Option<types::CodeableConcept>,

    /// Reason for result of the adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<types::Element>,

    /// Preauthorization reference effective period
    pub pre_auth_period: Option<types::Period>,
}

/// Payment details for the adjudication of the claim.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponsePayment;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponsePayment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Partial or complete payment
    pub r#type: types::CodeableConcept,

    /// Payment adjustment for non-claim issues
    pub adjustment: Option<types::Money>,

    /// Explanation for the adjustment
    pub adjustment_reason: Option<types::CodeableConcept>,

    /// Expected date of payment
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Payable amount after adjustment
    pub amount: types::Money,

    /// Business identifier for the payment
    pub identifier: Option<types::Identifier>,
}

/// A note that describes or explains adjudication results in a human readable
/// form.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseProcessNote;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ClaimResponseProcessNote {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Note instance identifier
    pub number: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// Note purpose
    pub r#type: Option<types::CodeableConcept>,

    /// Note explanatory text
    pub text: types::String,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Language of the text
    pub language: Option<types::CodeableConcept>,
}

/// Categorized monetary totals for the adjudication.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::claim_response::ClaimResponseTotal;
/// use fhir::r6::types;
///
/// let value = ClaimResponseTotal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimResponseTotal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ClaimResponseTotal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of adjudication information
    pub category: types::CodeableConcept,

    /// Financial total for the category
    pub amount: types::Money,
}

/// The `ClaimResponse.addItem.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimResponseAddItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ClaimResponse.addItem.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimResponseAddItemLocation {
    /// `locationCodeableConcept` variant.
    #[fhir("locationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `ClaimResponse.event.when[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimResponseEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
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
