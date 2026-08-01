//! ExplanationOfBenefit
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
//!
//!
//!
//! Explanation of Benefit resource
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ExplanationOfBenefit Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefit;
/// use fhir::r3::types;
///
/// let value = ExplanationOfBenefit {
///     created: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `created` is the name this serializes to on the wire.
/// assert_eq!(json["created"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ExplanationOfBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExplanationOfBenefit {
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
    pub status: Option<crate::coded::Coded<crate::r3::codes::ExplanationofbenefitStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type or discipline
    pub r#type: Option<types::CodeableConcept>,

    /// Finer grained claim type information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_type: Vec<types::CodeableConcept>,

    /// The subject of the Products and Services
    pub patient: Option<types::Reference>,

    /// Period for charge submission
    pub billable_period: Option<types::Period>,

    /// Creation date
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Author
    pub enterer: Option<types::Reference>,

    /// Insurer responsible for the EOB
    pub insurer: Option<types::Reference>,

    /// Responsible provider for the claim
    pub provider: Option<types::Reference>,

    /// Responsible organization for the claim
    pub organization: Option<types::Reference>,

    /// Treatment Referral
    pub referral: Option<types::Reference>,

    /// Servicing Facility
    pub facility: Option<types::Reference>,

    /// Claim reference
    pub claim: Option<types::Reference>,

    /// Claim response reference
    pub claim_response: Option<types::Reference>,

    /// complete | error | partial
    pub outcome: Option<types::CodeableConcept>,

    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,

    /// Related Claims which may be revelant to processing this claim
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<ExplanationOfBenefitRelated>,

    /// Prescription authorizing services or products
    pub prescription: Option<types::Reference>,

    /// Original prescription if superceded by fulfiller
    pub original_prescription: Option<types::Reference>,

    /// Party to be paid any benefits payable
    pub payee: Option<ExplanationOfBenefitPayee>,

    /// Exceptions, special considerations, the condition, situation, prior or
    /// concurrent issues
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information: Vec<ExplanationOfBenefitInformation>,

    /// Care Team members
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<ExplanationOfBenefitCareTeam>,

    /// List of Diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<ExplanationOfBenefitDiagnosis>,

    /// Procedures performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<ExplanationOfBenefitProcedure>,

    /// Precedence (primary, secondary, etc.)
    pub precedence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`precedence`](Self::precedence) (FHIR `_precedence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precedence")]
    pub precedence_ext: Option<types::Element>,

    /// Insurance or medical plan
    pub insurance: Option<ExplanationOfBenefitInsurance>,

    /// Details of an accident
    pub accident: Option<ExplanationOfBenefitAccident>,

    /// Period unable to work
    pub employment_impacted: Option<types::Period>,

    /// Period in hospital
    pub hospitalization: Option<types::Period>,

    /// Goods and Services
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ExplanationOfBenefitItem>,

    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add_item: Vec<ExplanationOfBenefitAddItem>,

    /// Total Cost of service from the Claim
    pub total_cost: Option<types::Money>,

    /// Unallocated deductable
    pub unalloc_deductable: Option<types::Money>,

    /// Total benefit payable for the Claim
    pub total_benefit: Option<types::Money>,

    /// Payment (if paid)
    pub payment: Option<ExplanationOfBenefitPayment>,

    /// Printed Form Identifier
    pub form: Option<types::CodeableConcept>,

    /// Processing notes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process_note: Vec<ExplanationOfBenefitProcessNote>,

    /// Balance by Benefit Category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub benefit_balance: Vec<ExplanationOfBenefitBenefitBalance>,
}

/// An accident which resulted in the need for healthcare services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitAccident;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitAccident {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// When the accident occurred
    pub date: Option<types::Date>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,

    /// Accident Place
    /// The `ExplanationOfBenefit.accident.location[x]` choice element (0..1); see [`ExplanationOfBenefitAccidentLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitAccidentLocation>,
}

/// The first tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitAddItem;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitAddItem {
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

    /// Billing Code
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
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Added items details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ExplanationOfBenefitAddItemDetail>,
}

/// The second tier service adjudications for payor added services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitAddItemDetail;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitAddItemDetail {
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

    /// Billing Code
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
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}

/// Balance by Benefit Category.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitBenefitBalance;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitBenefitBalance {
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
    pub financial: Vec<ExplanationOfBenefitBenefitBalanceFinancial>,
}

/// Benefits Used to date.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitBenefitBalanceFinancial;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
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
    /// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialAllowed`].
    #[serde(flatten)]
    pub allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,

    /// Benefits used
    /// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialUsed`].
    #[serde(flatten)]
    pub used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
}

/// The members of the team who provided the overall service as well as their
/// role and whether responsible and qualifications.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitCareTeam;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitCareTeam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Number to covey order of careteam
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Member of the Care Team
    pub provider: types::Reference,

    /// Billing practitioner
    pub responsible: Option<types::Boolean>,
    /// Primitive extension sibling for [`responsible`](Self::responsible) (FHIR `_responsible`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_responsible")]
    pub responsible_ext: Option<types::Element>,

    /// Role on the team
    pub role: Option<types::CodeableConcept>,

    /// Type, classification or Specialization
    pub qualification: Option<types::CodeableConcept>,
}

/// Ordered list of patient diagnosis for which care is sought.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitDiagnosis;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitDiagnosis {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Number to covey order of diagnosis
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Patient's diagnosis
    /// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (1..1); see [`ExplanationOfBenefitDiagnosisDiagnosis`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub diagnosis: Option<ExplanationOfBenefitDiagnosisDiagnosis>,

    /// Timing or nature of the diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Package billing code
    pub package_code: Option<types::CodeableConcept>,
}

/// Additional information codes regarding exceptions, special considerations,
/// the condition, situation, prior or concurrent issues. Often there are
/// mutiple jurisdiction specific valuesets which are required.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitInformation;
/// use fhir::r3::types;
///
/// let value = ExplanationOfBenefitInformation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitInformation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ExplanationOfBenefitInformation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Information instance identifier
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// General class of information
    pub category: types::CodeableConcept,

    /// Type of information
    pub code: Option<types::CodeableConcept>,

    /// When it occurred
    /// The `ExplanationOfBenefit.information.timing[x]` choice element (0..1); see [`ExplanationOfBenefitInformationTiming`].
    #[serde(flatten)]
    pub timing: Option<ExplanationOfBenefitInformationTiming>,

    /// Additional Data or supporting information
    /// The `ExplanationOfBenefit.information.value[x]` choice element (0..1); see [`ExplanationOfBenefitInformationValue`].
    #[serde(flatten)]
    pub value: Option<ExplanationOfBenefitInformationValue>,

    /// Reason associated with the information
    pub reason: Option<types::Coding>,
}

/// Financial instrument by which payment information for health care.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitInsurance;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitInsurance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Insurance information
    pub coverage: Option<types::Reference>,

    /// Pre-Authorization/Determination Reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,
}

/// First tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitItem;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Applicable careteam members
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team_link_id: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`care_team_link_id`](Self::care_team_link_id) (FHIR `_careTeamLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_careTeamLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team_link_id_ext: Vec<Option<types::Element>>,

    /// Applicable diagnoses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis_link_id: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`diagnosis_link_id`](Self::diagnosis_link_id) (FHIR `_diagnosisLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_diagnosisLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis_link_id_ext: Vec<Option<types::Element>>,

    /// Applicable procedures
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_link_id: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`procedure_link_id`](Self::procedure_link_id) (FHIR `_procedureLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_procedureLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_link_id_ext: Vec<Option<types::Element>>,

    /// Applicable exception and supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_link_id: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`information_link_id`](Self::information_link_id) (FHIR `_informationLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_informationLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_link_id_ext: Vec<Option<types::Element>>,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Type of service or product
    pub category: Option<types::CodeableConcept>,

    /// Billing Code
    pub service: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Program specific reason for item inclusion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,

    /// Date or dates of Service
    /// The `ExplanationOfBenefit.item.serviced[x]` choice element (0..1); see [`ExplanationOfBenefitItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ExplanationOfBenefitItemServiced>,

    /// Place of service
    /// The `ExplanationOfBenefit.item.location[x]` choice element (0..1); see [`ExplanationOfBenefitItemLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitItemLocation>,

    /// Count of Products or Services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per point
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Total item cost
    pub net: Option<types::Money>,

    /// Unique Device Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,

    /// Service Location
    pub body_site: Option<types::CodeableConcept>,

    /// Service Sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::CodeableConcept>,

    /// Encounters related to this billed item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encounter: Vec<types::Reference>,

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
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ExplanationOfBenefitItemDetail>,
}

/// The adjudications results.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitItemAdjudication;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitItemAdjudication {
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

    /// Non-monitory value
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Second tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitItemDetail;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitItemDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Group or type of product or service
    pub r#type: types::CodeableConcept,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Type of service or product
    pub category: Option<types::CodeableConcept>,

    /// Billing Code
    pub service: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Program specific reason for item inclusion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,

    /// Count of Products or Services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per point
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Total additional item cost
    pub net: Option<types::Money>,

    /// Unique Device Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,

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
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
}

/// Third tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitItemDetailSubDetail;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Type of product or service
    pub r#type: types::CodeableConcept,

    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,

    /// Type of service or product
    pub category: Option<types::CodeableConcept>,

    /// Billing Code
    pub service: Option<types::CodeableConcept>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,

    /// Program specific reason for item inclusion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,

    /// Count of Products or Services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per point
    pub unit_price: Option<types::Money>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Net additional item cost
    pub net: Option<types::Money>,

    /// Unique Device Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,

    /// List of note numbers which apply
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,

    /// Language if different from the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}

/// The party to be reimbursed for the services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitPayee;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitPayee {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of party: Subscriber, Provider, other
    pub r#type: Option<types::CodeableConcept>,

    /// organization | patient | practitioner | relatedperson
    pub resource_type: Option<types::CodeableConcept>,

    /// Party to receive the payable
    pub party: Option<types::Reference>,
}

/// Payment details for the claim if the claim has been paid.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitPayment;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitPayment {
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

    /// Expected date of Payment
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

/// Ordered list of patient procedures performed to support the adjudication.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitProcedure;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitProcedure {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Procedure sequence for reference
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// When the procedure was performed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Patient's list of procedures performed
    /// The `ExplanationOfBenefit.procedure.procedure[x]` choice element (1..1); see [`ExplanationOfBenefitProcedureProcedure`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub procedure: Option<ExplanationOfBenefitProcedureProcedure>,
}

/// Note text.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitProcessNote;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitProcessNote {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Sequence number for this note
    pub number: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// display | print | printoper
    pub r#type: Option<types::CodeableConcept>,

    /// Note explanitory text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Language if different from the resource
    pub language: Option<types::CodeableConcept>,
}

/// Other claims which are related to this claim such as prior claim versions
/// or for related services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::explanation_of_benefit::ExplanationOfBenefitRelated;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ExplanationOfBenefitRelated {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to the related claim
    pub claim: Option<types::Reference>,

    /// How the reference claim is related
    pub relationship: Option<types::CodeableConcept>,

    /// Related file or case reference
    pub reference: Option<types::Identifier>,
}

/// The `ExplanationOfBenefit.accident.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAccidentLocation {
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
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

/// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    /// `usedUnsignedInt` variant.
    #[fhir("usedUnsignedInt")]
    UnsignedInt(crate::r3::choice::Primitive<types::UnsignedInt>),
    /// `usedMoney` variant.
    #[fhir("usedMoney")]
    Money(Box<types::Money>),
}

/// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.information.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitInformationTiming {
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.information.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitInformationValue {
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r3::choice::Primitive<types::String>),
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

/// The `ExplanationOfBenefit.item.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.item.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
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
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitProcedureProcedure {
    /// `procedureCodeableConcept` variant.
    #[fhir("procedureCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `procedureReference` variant.
    #[fhir("procedureReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ExplanationOfBenefit;

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
