//! ExplanationOfBenefit
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
//!
//! Version: 4.0.1
//!
//! Explanation of Benefit resource
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for
/// informing the subscriber of the benefits provided.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefit;
///
/// let value = ExplanationOfBenefit::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ExplanationOfBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefit {
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

    /// Business Identifier for the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::ExplanationofbenefitStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Category or discipline
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
    pub patient: types::Reference,

    /// Relevant time frame for the claim
    pub billable_period: Option<types::Period>,

    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Author of the claim
    pub enterer: Option<types::Reference>,

    /// Party responsible for reimbursement
    pub insurer: types::Reference,

    /// Party responsible for the claim
    pub provider: types::Reference,

    /// Desired processing urgency
    pub priority: Option<types::CodeableConcept>,

    /// For whom to reserve funds
    pub funds_reserve_requested: Option<types::CodeableConcept>,

    /// Funds reserved status
    pub funds_reserve: Option<types::CodeableConcept>,

    /// Prior or corollary claims
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<ExplanationOfBenefitRelated>,

    /// Prescription authorizing services or products
    pub prescription: Option<types::Reference>,

    /// Original prescription if superceded by fulfiller
    pub original_prescription: Option<types::Reference>,

    /// Recipient of benefits payable
    pub payee: Option<ExplanationOfBenefitPayee>,

    /// Treatment Referral
    pub referral: Option<types::Reference>,

    /// Servicing Facility
    pub facility: Option<types::Reference>,

    /// Claim reference
    pub claim: Option<types::Reference>,

    /// Claim response reference
    pub claim_response: Option<types::Reference>,

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,

    /// Preauthorization in-effect period
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_period: Vec<types::Period>,

    /// Care Team members
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<ExplanationOfBenefitCareTeam>,

    /// Supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<ExplanationOfBenefitSupportingInfo>,

    /// Pertinent diagnosis information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<ExplanationOfBenefitDiagnosis>,

    /// Clinical procedures performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<ExplanationOfBenefitProcedure>,

    /// Precedence (primary, secondary, etc.)
    pub precedence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`precedence`](Self::precedence) (FHIR `_precedence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precedence")]
    pub precedence_ext: Option<types::Element>,

    /// Patient insurance information
    pub insurance: ::vec1::Vec1<ExplanationOfBenefitInsurance>,

    /// Details of the event
    pub accident: Option<ExplanationOfBenefitAccident>,

    /// Product or service provided
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ExplanationOfBenefitItem>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_item: Vec<ExplanationOfBenefitAddItem>,

    /// Header-level adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Adjudication totals
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub total: Vec<ExplanationOfBenefitTotal>,

    /// Payment Details
    pub payment: Option<ExplanationOfBenefitPayment>,

    /// Printed form identifier
    pub form_code: Option<types::CodeableConcept>,

    /// Printed reference or actual form
    pub form: Option<types::Attachment>,

    /// Note concerning adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_note: Vec<ExplanationOfBenefitProcessNote>,

    /// When the benefits are applicable
    pub benefit_period: Option<types::Period>,

    /// Balance by Benefit Category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub benefit_balance: Vec<ExplanationOfBenefitBenefitBalance>,
}

/// Details of a accident which resulted in injuries which required the
/// products and services listed in the claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitAccident;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitAccident {
///     date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01"));
///
/// let back: ExplanationOfBenefitAccident = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitAccident {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// When the incident occurred
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,

    /// Where the event occurred
    /// The `ExplanationOfBenefit.accident.location[x]` choice element (0..1); see [`ExplanationOfBenefitAccidentLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitAccidentLocation>,
}

/// The first-tier service adjudications for payor added product or service
/// lines.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitAddItem;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitAddItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitAddItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitAddItem {
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
    pub sub_detail_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`sub_detail_sequence`](Self::sub_detail_sequence) (FHIR `_subDetailSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subDetailSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail_sequence_ext: Vec<Option<types::Element>>,

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
    /// The `ExplanationOfBenefit.addItem.serviced[x]` choice element (0..1); see [`ExplanationOfBenefitAddItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ExplanationOfBenefitAddItemServiced>,

    /// Place of service or where product was supplied
    /// The `ExplanationOfBenefit.addItem.location[x]` choice element (0..1); see [`ExplanationOfBenefitAddItemLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitAddItemLocation>,

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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ExplanationOfBenefitAddItemDetail>,
}

/// The second-tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitAddItemDetail;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitAddItemDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitAddItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitAddItemDetail {
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

    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ExplanationOfBenefitAddItemDetailSubDetail>,
}

/// The third-tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitAddItemDetailSubDetail;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitAddItemDetailSubDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitAddItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
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

    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}

/// Balance by Benefit Category.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitBenefitBalance;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitBenefitBalance {
///     excluded: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `excluded` is the name this serializes to on the wire.
/// assert_eq!(json["excluded"], ::serde_json::json!(true));
///
/// let back: ExplanationOfBenefitBenefitBalance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitBenefitBalance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Benefit classification
    pub category: types::CodeableConcept,

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
    pub financial: Vec<ExplanationOfBenefitBenefitBalanceFinancial>,
}

/// Benefits Used to date.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitBenefitBalanceFinancial;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitBenefitBalanceFinancial {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitBenefitBalanceFinancial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
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
    /// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialAllowed`].
    #[serde(flatten)]
    pub allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,

    /// Benefits used
    /// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialUsed`].
    #[serde(flatten)]
    pub used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
}

/// The members of the team who provided the products and services.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitCareTeam;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitCareTeam {
///     responsible: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `responsible` is the name this serializes to on the wire.
/// assert_eq!(json["responsible"], ::serde_json::json!(true));
///
/// let back: ExplanationOfBenefitCareTeam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitCareTeam {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Order of care team
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Practitioner or organization
    pub provider: types::Reference,

    /// Indicator of the lead practitioner
    pub responsible: Option<types::Boolean>,
    /// Primitive extension sibling for [`responsible`](Self::responsible) (FHIR `_responsible`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_responsible")]
    pub responsible_ext: Option<types::Element>,

    /// Function within the team
    pub role: Option<types::CodeableConcept>,

    /// Practitioner credential or specialization
    pub qualification: Option<types::CodeableConcept>,
}

/// Information about diagnoses relevant to the claim items.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitDiagnosis;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitDiagnosis {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Diagnosis instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Nature of illness or problem
    /// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (1..1); see [`ExplanationOfBenefitDiagnosisDiagnosis`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub diagnosis: Option<ExplanationOfBenefitDiagnosisDiagnosis>,

    /// Timing or nature of the diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Present on admission
    pub on_admission: Option<types::CodeableConcept>,

    /// Package billing code
    pub package_code: Option<types::CodeableConcept>,
}

/// Financial instruments for reimbursement for the health care products and
/// services specified on the claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitInsurance;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitInsurance {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitInsurance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coverage to be used for adjudication
    pub focal: types::Boolean,
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,

    /// Insurance information
    pub coverage: types::Reference,

    /// Prior authorization reference number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,
}

/// A claim line. Either a simple (a product or service) or a 'group' of
/// details which can also be a simple items or groups of sub-details.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitItem;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Item instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Applicable care team members
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`care_team_sequence`](Self::care_team_sequence) (FHIR `_careTeamSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_careTeamSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team_sequence_ext: Vec<Option<types::Element>>,

    /// Applicable diagnoses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`diagnosis_sequence`](Self::diagnosis_sequence) (FHIR `_diagnosisSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_diagnosisSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis_sequence_ext: Vec<Option<types::Element>>,

    /// Applicable procedures
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`procedure_sequence`](Self::procedure_sequence) (FHIR `_procedureSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_procedureSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_sequence_ext: Vec<Option<types::Element>>,

    /// Applicable exception and supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`information_sequence`](Self::information_sequence) (FHIR `_informationSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_informationSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_sequence_ext: Vec<Option<types::Element>>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: types::CodeableConcept,

    /// Product or service billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Program the product or service is provided under
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,

    /// Date or dates of service or product delivery
    /// The `ExplanationOfBenefit.item.serviced[x]` choice element (0..1); see [`ExplanationOfBenefitItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ExplanationOfBenefitItemServiced>,

    /// Place of service or where product was supplied
    /// The `ExplanationOfBenefit.item.location[x]` choice element (0..1); see [`ExplanationOfBenefitItemLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitItemLocation>,

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

    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,

    /// Anatomical location
    pub body_site: Option<types::CodeableConcept>,

    /// Anatomical sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::CodeableConcept>,

    /// Encounters related to this billed item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encounter: Vec<types::Reference>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ExplanationOfBenefitItemDetail>,
}

/// If this item is a group then the values here are a summary of the
/// adjudication of the detail items. If this item is a simple product or
/// service then this is the result of the adjudication of this item.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitItemAdjudication;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitItemAdjudication {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitItemAdjudication = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitItemAdjudication {
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

    /// Non-monitary value
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Second-tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitItemDetail;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitItemDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Product or service provided
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: types::CodeableConcept,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Program the product or service is provided under
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,

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

    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,

    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Detail level adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
}

/// Third-tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitItemDetailSubDetail;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitItemDetailSubDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Product or service provided
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Benefit classification
    pub category: Option<types::CodeableConcept>,

    /// Billing, service, product, or drug code
    pub product_or_service: types::CodeableConcept,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Program the product or service is provided under
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,

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

    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,

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
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}

/// The party to be reimbursed for cost of the products and services according
/// to the terms of the policy.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitPayee;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitPayee {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitPayee = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitPayee {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Category of recipient
    pub r#type: Option<types::CodeableConcept>,

    /// Recipient reference
    pub party: Option<types::Reference>,
}

/// Payment details for the adjudication of the claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitPayment;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitPayment {
///     date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01"));
///
/// let back: ExplanationOfBenefitPayment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitPayment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Partial or complete payment
    pub r#type: Option<types::CodeableConcept>,

    /// Payment adjustment for non-claim issues
    pub adjustment: Option<types::Money>,

    /// Explanation for the variance
    pub adjustment_reason: Option<types::CodeableConcept>,

    /// Expected date of payment
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Payable amount after adjustment
    pub amount: Option<types::Money>,

    /// Business identifier for the payment
    pub identifier: Option<types::Identifier>,
}

/// Procedures performed on the patient relevant to the billing items with the
/// claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitProcedure;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitProcedure {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ExplanationOfBenefitProcedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Procedure instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Category of Procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// When the procedure was performed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Specific clinical procedure
    /// The `ExplanationOfBenefit.procedure.procedure[x]` choice element (1..1); see [`ExplanationOfBenefitProcedureProcedure`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub procedure: Option<ExplanationOfBenefitProcedureProcedure>,

    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,
}

/// A note that describes or explains adjudication results in a human readable
/// form.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitProcessNote;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitProcessNote {
///     number: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `number` is the name this serializes to on the wire.
/// assert_eq!(json["number"], ::serde_json::json!(1));
///
/// let back: ExplanationOfBenefitProcessNote = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitProcessNote {
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
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Language of the text
    pub language: Option<types::CodeableConcept>,
}

/// Other claims which are related to this claim such as prior submissions or
/// claims for related services or for the same event.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitRelated;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitRelated {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitRelated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitRelated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to the related claim
    pub claim: Option<types::Reference>,

    /// How the reference claim is related
    pub relationship: Option<types::CodeableConcept>,

    /// File or case reference
    pub reference: Option<types::Identifier>,
}

/// Additional information codes regarding exceptions, special considerations,
/// the condition, situation, prior or concurrent issues.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitSupportingInfo;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitSupportingInfo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitSupportingInfo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitSupportingInfo {
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

    /// Classification of the supplied information
    pub category: types::CodeableConcept,

    /// Type of information
    pub code: Option<types::CodeableConcept>,

    /// When it occurred
    /// The `ExplanationOfBenefit.supportingInfo.timing[x]` choice element (0..1); see [`ExplanationOfBenefitSupportingInfoTiming`].
    #[serde(flatten)]
    pub timing: Option<ExplanationOfBenefitSupportingInfoTiming>,

    /// Data to be provided
    /// The `ExplanationOfBenefit.supportingInfo.value[x]` choice element (0..1); see [`ExplanationOfBenefitSupportingInfoValue`].
    #[serde(flatten)]
    pub value: Option<ExplanationOfBenefitSupportingInfoValue>,

    /// Explanation for the information
    pub reason: Option<types::Coding>,
}

/// Categorized monetary totals for the adjudication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::explanation_of_benefit::ExplanationOfBenefitTotal;
/// use fhir::r4::types;
///
/// let value = ExplanationOfBenefitTotal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitTotal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ExplanationOfBenefitTotal {
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

/// The `ExplanationOfBenefit.accident.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAccidentLocation {
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.addItem.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAddItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.addItem.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAddItemLocation {
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

/// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
    /// `allowedUnsignedInt` variant.
    #[fhir("allowedUnsignedInt")]
    UnsignedInt(crate::r4::choice::Primitive<types::UnsignedInt>),
    /// `allowedString` variant.
    #[fhir("allowedString")]
    String(crate::r4::choice::Primitive<types::String>),
    /// `allowedMoney` variant.
    #[fhir("allowedMoney")]
    Money(Box<types::Money>),
}

/// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    /// `usedUnsignedInt` variant.
    #[fhir("usedUnsignedInt")]
    UnsignedInt(crate::r4::choice::Primitive<types::UnsignedInt>),
    /// `usedMoney` variant.
    #[fhir("usedMoney")]
    Money(Box<types::Money>),
}

/// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.item.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.item.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitItemLocation {
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

/// The `ExplanationOfBenefit.procedure.procedure[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitProcedureProcedure {
    /// `procedureCodeableConcept` variant.
    #[fhir("procedureCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `procedureReference` variant.
    #[fhir("procedureReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.supportingInfo.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitSupportingInfoTiming {
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.supportingInfo.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitSupportingInfoValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r4::choice::Primitive<types::Boolean>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r4::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}
