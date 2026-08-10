//! ClaimResponse
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ClaimResponse
//!
//! Version: 4.0.1
//!
//! Response to a claim predetermination or preauthorization
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource provides the adjudication details from the processing of a
/// Claim resource.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim_response::ClaimResponse;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business Identifier for a claim response
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::FmStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// More granular claim type
    pub r#type: types::CodeableConcept,

    /// More granular claim type
    pub sub_type: Option<types::CodeableConcept>,

    /// claim | preauthorization | predetermination
    pub r#use: crate::coded::Coded<crate::r4::codes::ClaimUse>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// The recipient of the products and services
    pub patient: types::Reference<crate::r4::resources::Patient>,

    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Party responsible for reimbursement
    pub insurer: types::Reference<crate::r4::resources::Organization>,

    /// Party responsible for the claim
    pub requestor: Option<types::Reference>,

    /// Id of resource triggering adjudication
    pub request: Option<types::Reference<crate::r4::resources::Claim>>,

    /// queued | complete | error | partial
    pub outcome: crate::coded::Coded<crate::r4::codes::RemittanceOutcome>,
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

    /// Preauthorization reference
    pub pre_auth_ref: Option<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<types::Element>,

    /// Preauthorization reference effective period
    pub pre_auth_period: Option<types::Period>,

    /// Party to be paid any benefits payable
    pub payee_type: Option<types::CodeableConcept>,

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
    pub communication_request: Vec<types::Reference<crate::r4::resources::CommunicationRequest>>,

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
/// ```ignore
/// use fhir::r4::resources::claim_response::ClaimResponseAddItem;
///
/// let value = ClaimResponseAddItem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClaimResponseAddItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ClaimResponseAddItemDe")]
#[fhir_version("r4")]
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

    /// Authorized providers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provider: Vec<types::Reference>,

    /// Billing, service, product, or drug code
    pub product_or_service: types::CodeableConcept,

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

    /// Total item cost
    pub net: Option<types::Money>,

    /// Anatomical location
    pub body_site: Option<types::CodeableConcept>,

    /// Anatomical sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::CodeableConcept>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Added items adjudication
    pub adjudication: ::vec1::Vec1<ClaimResponseItemAdjudication>,

    /// Insurer added line details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimResponseAddItemDetail>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimResponseAddItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    item_sequence: Vec<types::PositiveInt>,
    #[serde(rename = "_itemSequence")]
    #[serde(default)]
    item_sequence_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    detail_sequence: Vec<types::PositiveInt>,
    #[serde(rename = "_detailSequence")]
    #[serde(default)]
    detail_sequence_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    subdetail_sequence: Vec<types::PositiveInt>,
    #[serde(rename = "_subdetailSequence")]
    #[serde(default)]
    subdetail_sequence_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    provider: Vec<types::Reference>,
    product_or_service: types::CodeableConcept,
    #[serde(default)]
    modifier: Vec<types::CodeableConcept>,
    #[serde(default)]
    program_code: Vec<types::CodeableConcept>,
    #[serde(flatten)]
    serviced: crate::r4::choice::Slot<ClaimResponseAddItemServiced>,
    #[serde(flatten)]
    location: crate::r4::choice::Slot<ClaimResponseAddItemLocation>,
    quantity: Option<types::Quantity>,
    unit_price: Option<types::Money>,
    factor: Option<types::Decimal>,
    #[serde(rename = "_factor")]
    factor_ext: Option<types::Element>,
    net: Option<types::Money>,
    body_site: Option<types::CodeableConcept>,
    #[serde(default)]
    sub_site: Vec<types::CodeableConcept>,
    #[serde(default)]
    note_number: Vec<types::PositiveInt>,
    #[serde(rename = "_noteNumber")]
    #[serde(default)]
    note_number_ext: Vec<Option<types::Element>>,
    adjudication: ::vec1::Vec1<ClaimResponseItemAdjudication>,
    #[serde(default)]
    detail: Vec<ClaimResponseAddItemDetail>,
}

impl ::core::convert::From<ClaimResponseAddItemDe> for ClaimResponseAddItem {
    fn from(v: ClaimResponseAddItemDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            item_sequence: v.item_sequence,
            item_sequence_ext: v.item_sequence_ext,
            detail_sequence: v.detail_sequence,
            detail_sequence_ext: v.detail_sequence_ext,
            subdetail_sequence: v.subdetail_sequence,
            subdetail_sequence_ext: v.subdetail_sequence_ext,
            provider: v.provider,
            product_or_service: v.product_or_service,
            modifier: v.modifier,
            program_code: v.program_code,
            serviced: v.serviced.0,
            location: v.location.0,
            quantity: v.quantity,
            unit_price: v.unit_price,
            factor: v.factor,
            factor_ext: v.factor_ext,
            net: v.net,
            body_site: v.body_site,
            sub_site: v.sub_site,
            note_number: v.note_number,
            note_number_ext: v.note_number_ext,
            adjudication: v.adjudication,
            detail: v.detail,
        }
    }
}

/// The second-tier service adjudications for payor added services.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::claim_response::ClaimResponseAddItemDetail;
///
/// let value = ClaimResponseAddItemDetail::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClaimResponseAddItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ClaimResponseAddItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Billing, service, product, or drug code
    pub product_or_service: types::CodeableConcept,

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

    /// Added items detail adjudication
    pub adjudication: ::vec1::Vec1<ClaimResponseItemAdjudication>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimResponseAddItemDetailSubDetail>,
}

/// The third-tier service adjudications for payor added services.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::claim_response::ClaimResponseAddItemDetailSubDetail;
///
/// let value = ClaimResponseAddItemDetailSubDetail::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClaimResponseAddItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ClaimResponseAddItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Billing, service, product, or drug code
    pub product_or_service: types::CodeableConcept,

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

    /// Added items detail adjudication
    pub adjudication: ::vec1::Vec1<ClaimResponseItemAdjudication>,
}

/// Errors encountered during the processing of the adjudication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim_response::ClaimResponseError;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
}

/// Financial instruments for reimbursement for the health care products and
/// services specified on the claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim_response::ClaimResponseInsurance;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub coverage: types::Reference<crate::r4::resources::Coverage>,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,

    /// Adjudication results
    pub claim_response: Option<types::Reference<crate::r4::resources::ClaimResponse>>,
}

/// A claim line. Either a simple (a product or service) or a 'group' of
/// details which can also be a simple items or groups of sub-details.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::claim_response::ClaimResponseItem;
///
/// let value = ClaimResponseItem::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClaimResponseItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Adjudication details
    pub adjudication: ::vec1::Vec1<ClaimResponseItemAdjudication>,

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
/// use fhir::r4::resources::claim_response::ClaimResponseItemAdjudication;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A claim detail. Either a simple (a product or service) or a 'group' of
/// sub-details which are simple items.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::claim_response::ClaimResponseItemDetail;
///
/// let value = ClaimResponseItemDetail::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ClaimResponseItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Detail level adjudication details
    pub adjudication: ::vec1::Vec1<ClaimResponseItemAdjudication>,

    /// Adjudication for claim sub-details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}

/// A sub-detail adjudication of a simple product or service.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim_response::ClaimResponseItemDetailSubDetail;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

    /// Applicable note numbers
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

/// Payment details for the adjudication of the claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim_response::ClaimResponsePayment;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
/// use fhir::r4::resources::claim_response::ClaimResponseProcessNote;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

    /// display | print | printoper
    pub r#type: Option<crate::coded::Coded<crate::r4::codes::NoteType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

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
/// use fhir::r4::resources::claim_response::ClaimResponseTotal;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimResponseAddItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ClaimResponse.addItem.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
