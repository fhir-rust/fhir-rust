//! ExplanationOfBenefit
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ExplanationOfBenefit
//!
//! Version: 5.0.0
//!
//! ExplanationOfBenefit Resource: the claim details, adjudication details from the processing of a Claim, and optionally account balance information.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for
/// informing the subscriber of the benefits provided.
///
/// An ExplanationOfBenefit is the response to a Claim and combines information
/// from the original claim request with the payer's adjudication results,
/// making it central to the FHIR R5 financial and billing workflow. It is
/// typically produced by an insurer, health plan, or other adjudicating
/// system after processing a Claim, and it is returned to the subscriber,
/// patient, or provider so they can understand what services or products
/// were billed, how each line item was evaluated, and what amounts (if
/// any) are payable, denied, or the patient's responsibility. Unlike a
/// Claim, which represents a request for payment or authorization, the
/// ExplanationOfBenefit represents the outcome of that request, including
/// per-item and per-category adjudication, totals, payment information,
/// and optional account balance data such as benefit limits already used.
///
/// The resource mirrors much of the structure of the Claim it responds to
/// (items, details, sub-details, diagnoses, procedures, and insurance),
/// while adding adjudication-specific elements such as `adjudication`,
/// `total`, `payment`, and `benefit_balance` that capture the payer's
/// determination for each element of the original request.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — the subject
///   referenced by the `patient` field.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used
///   throughout for coded categories such as status, type, and adjudication
///   categories.
/// - `Claim` and `ClaimResponse` — related resources representing the
///   original request and the underlying adjudication response.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefit;
/// use fhir::r5::types;
///
/// let value = ExplanationOfBenefit {
///     disposition: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `disposition` is the name this serializes to on the wire.
/// assert_eq!(json["disposition"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefit {
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
    pub contained: Vec<crate::r5::resources::Resource>,
    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,
    /// Business Identifier for the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,
    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,
    /// The status of the resource instance: active | cancelled | draft | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::ExplanationofbenefitStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,
    /// The category or discipline of the claim, e.g. institutional, oral, pharmacy, professional, or vision
    pub r#type: types::CodeableConcept,
    /// More granular claim type
    pub sub_type: Option<types::CodeableConcept>,
    /// Whether this represents a claim, a preauthorization request, or a predetermination
    pub r#use: crate::r5::coded::Coded<crate::r5::codes::ClaimUse>,
    /// Primitive extension sibling for [`use`](Self::r#use) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,
    /// Reference to the patient who is the recipient of the products and services
    pub patient: types::Reference,
    /// Relevant time frame for the claim
    pub billable_period: Option<types::Period>,
    /// Response creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,
    /// Author of the claim
    pub enterer: Option<types::Reference>,
    /// Party responsible for reimbursement
    pub insurer: Option<types::Reference>,
    /// Party responsible for the claim
    pub provider: Option<types::Reference>,
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
    /// Event information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<ExplanationOfBenefitEvent>,
    /// Recipient of benefits payable
    pub payee: Option<ExplanationOfBenefitPayee>,
    /// Treatment Referral
    pub referral: Option<types::Reference>,
    /// Encounters associated with the listed treatments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encounter: Vec<types::Reference>,
    /// Servicing Facility
    pub facility: Option<types::Reference>,
    /// Reference to the Claim resource that this ExplanationOfBenefit is the outcome of
    pub claim: Option<types::Reference>,
    /// Claim response reference
    pub claim_response: Option<types::Reference>,
    /// The processing outcome of the adjudication: queued | complete | error | partial
    pub outcome: crate::r5::coded::Coded<crate::r5::codes::ClaimOutcome>,
    /// Primitive extension sibling for [`outcome`](Self::outcome) (FHIR `_outcome`).
    #[serde(rename = "_outcome")]
    pub outcome_ext: Option<types::Element>,
    /// Result of the adjudication
    pub decision: Option<types::CodeableConcept>,
    /// Disposition Message
    pub disposition: Option<types::String>,
    /// Primitive extension sibling for [`disposition`](Self::disposition) (FHIR `_disposition`).
    #[serde(rename = "_disposition")]
    pub disposition_ext: Option<types::Element>,
    /// Preauthorization reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,
    /// Preauthorization in-effect period
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_period: Vec<types::Period>,
    /// Package billing code
    pub diagnosis_related_group: Option<types::CodeableConcept>,
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
    /// Primitive extension sibling for [`precedence`](Self::precedence) (FHIR `_precedence`).
    #[serde(rename = "_precedence")]
    pub precedence_ext: Option<types::Element>,
    /// Patient insurance information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<ExplanationOfBenefitInsurance>,
    /// Details of the event
    pub accident: Option<ExplanationOfBenefitAccident>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
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

/// Prior or corollary claims related to this ExplanationOfBenefit.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitRelated;
/// use fhir::r5::types;
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

/// Event information associated with the ExplanationOfBenefit.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitEvent;
/// use fhir::r5::types;
///
/// let value = ExplanationOfBenefitEvent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitEvent {
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
    /// The `ExplanationOfBenefit.event.when[x]` choice element (0..1); see [`ExplanationOfBenefitEventWhen`].
    #[serde(flatten)]
    pub when: Option<ExplanationOfBenefitEventWhen>,
}

/// Recipient of the benefits payable.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitPayee;
/// use fhir::r5::types;
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

/// Members of the care team who provided the products and services.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitCareTeam;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Practitioner or organization
    pub provider: types::Reference,
    /// Indicator of the lead practitioner
    pub responsible: Option<types::Boolean>,
    /// Primitive extension sibling for [`responsible`](Self::responsible) (FHIR `_responsible`).
    #[serde(rename = "_responsible")]
    pub responsible_ext: Option<types::Element>,
    /// Function within the team
    pub role: Option<types::CodeableConcept>,
    /// Practitioner or provider specialization
    pub specialty: Option<types::CodeableConcept>,
}

/// Supporting information regarding the claim.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitSupportingInfo;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Classification of the supplied information
    pub category: types::CodeableConcept,
    /// Type of information
    pub code: Option<types::CodeableConcept>,
    /// The `ExplanationOfBenefit.supportingInfo.timing[x]` choice element (0..1); see [`ExplanationOfBenefitSupportingInfoTiming`].
    #[serde(flatten)]
    pub timing: Option<ExplanationOfBenefitSupportingInfoTiming>,
    /// The `ExplanationOfBenefit.supportingInfo.value[x]` choice element (0..1); see [`ExplanationOfBenefitSupportingInfoValue`].
    #[serde(flatten)]
    pub value: Option<ExplanationOfBenefitSupportingInfoValue>,
    /// Explanation for the information
    pub reason: Option<types::Coding>,
}

/// Pertinent diagnosis information.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitDiagnosis;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (0..1); see [`ExplanationOfBenefitDiagnosisDiagnosis`].
    #[serde(flatten)]
    pub diagnosis: Option<ExplanationOfBenefitDiagnosisDiagnosis>,
    /// Timing or nature of the diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,
    /// Present on admission
    pub on_admission: Option<types::CodeableConcept>,
}

/// Clinical procedures performed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitProcedure;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Category of Procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,
    /// When the procedure was performed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
    /// The `ExplanationOfBenefit.procedure.procedure[x]` choice element (0..1); see [`ExplanationOfBenefitProcedureProcedure`].
    #[serde(flatten)]
    pub procedure: Option<ExplanationOfBenefitProcedureProcedure>,
    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,
}

/// Patient insurance information for adjudication.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitInsurance;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`).
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,
    /// Insurance information
    pub coverage: types::Reference,
    /// Prior authorization reference number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,
}

/// Details of an accident event that triggered the claim.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitAccident;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,
    /// The `ExplanationOfBenefit.accident.location[x]` choice element (0..1); see [`ExplanationOfBenefitAccidentLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitAccidentLocation>,
}

/// Product or service line item provided.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitItem;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Applicable care team members
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`care_team_sequence`](Self::care_team_sequence) (FHIR `_careTeamSequence`).
    #[serde(rename = "_careTeamSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team_sequence_ext: Vec<Option<types::Element>>,
    /// Applicable diagnoses
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`diagnosis_sequence`](Self::diagnosis_sequence) (FHIR `_diagnosisSequence`).
    #[serde(rename = "_diagnosisSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis_sequence_ext: Vec<Option<types::Element>>,
    /// Applicable procedures
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`procedure_sequence`](Self::procedure_sequence) (FHIR `_procedureSequence`).
    #[serde(rename = "_procedureSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_sequence_ext: Vec<Option<types::Element>>,
    /// Applicable exception and supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`information_sequence`](Self::information_sequence) (FHIR `_informationSequence`).
    #[serde(rename = "_informationSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_sequence_ext: Vec<Option<types::Element>>,
    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,
    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,
    /// Benefit classification
    pub category: Option<types::CodeableConcept>,
    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,
    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,
    /// Request or Referral for Service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<types::Reference>,
    /// Product or service billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,
    /// Program the product or service is provided under
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,
    /// The `ExplanationOfBenefit.item.serviced[x]` choice element (0..1); see [`ExplanationOfBenefitItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ExplanationOfBenefitItemServiced>,
    /// The `ExplanationOfBenefit.item.location[x]` choice element (0..1); see [`ExplanationOfBenefitItemLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitItemLocation>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,
    /// Anatomical location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<ExplanationOfBenefitItemBodySite>,
    /// Encounters associated with the listed treatments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encounter: Vec<types::Reference>,
    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,
    /// Adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ExplanationOfBenefitItemDetail>,
}

/// Anatomical location for an item.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitItemBodySite;
///
/// let value = ExplanationOfBenefitItemBodySite::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ExplanationOfBenefitItemBodySite = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemBodySite {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,
    /// Location
    pub site: vec1::Vec1<types::CodeableReference>,
    /// Sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::CodeableConcept>,
}

/// Adjudication review outcome for an item.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitItemReviewOutcome;
/// use fhir::r5::types;
///
/// let value = ExplanationOfBenefitItemReviewOutcome {
///     pre_auth_ref: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `preAuthRef` is the name this serializes to on the wire.
/// assert_eq!(json["preAuthRef"], ::serde_json::json!("abc"));
///
/// let back: ExplanationOfBenefitItemReviewOutcome = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemReviewOutcome {
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
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`).
    #[serde(rename = "_preAuthRef")]
    pub pre_auth_ref_ext: Option<types::Element>,
    /// Preauthorization reference effective period
    pub pre_auth_period: Option<types::Period>,
}

/// Adjudication details for an item.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitItemAdjudication;
/// use fhir::r5::types;
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
    pub quantity: Option<types::Quantity>,
}

/// Additional detail items under an item.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitItemDetail;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,
    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,
    /// Benefit classification
    pub category: Option<types::CodeableConcept>,
    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,
    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,
    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,
    /// Program the product or service is provided under
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,
    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,
    /// Detail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Detail level adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ExplanationOfBenefitItemDetailSubDetail>,
}

/// Additional sub-detail items under a detail.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitItemDetailSubDetail;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,
    /// Number for tracking
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_number: Vec<types::Identifier>,
    /// Revenue or cost center code
    pub revenue: Option<types::CodeableConcept>,
    /// Benefit classification
    pub category: Option<types::CodeableConcept>,
    /// Billing, service, product, or drug code
    pub product_or_service: Option<types::CodeableConcept>,
    /// End of a range of codes
    pub product_or_service_end: Option<types::CodeableConcept>,
    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::CodeableConcept>,
    /// Program the product or service is provided under
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub program_code: Vec<types::CodeableConcept>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference>,
    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,
    /// Subdetail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Subdetail level adjudication details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}

/// Insurer added line items.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitAddItem;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`item_sequence`](Self::item_sequence) (FHIR `_itemSequence`).
    #[serde(rename = "_itemSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item_sequence_ext: Vec<Option<types::Element>>,
    /// Detail sequence number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`detail_sequence`](Self::detail_sequence) (FHIR `_detailSequence`).
    #[serde(rename = "_detailSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail_sequence_ext: Vec<Option<types::Element>>,
    /// Subdetail sequence number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail_sequence: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`sub_detail_sequence`](Self::sub_detail_sequence) (FHIR `_subDetailSequence`).
    #[serde(rename = "_subDetailSequence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail_sequence_ext: Vec<Option<types::Element>>,
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
    /// The `ExplanationOfBenefit.addItem.serviced[x]` choice element (0..1); see [`ExplanationOfBenefitAddItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ExplanationOfBenefitAddItemServiced>,
    /// The `ExplanationOfBenefit.addItem.location[x]` choice element (0..1); see [`ExplanationOfBenefitAddItemLocation`].
    #[serde(flatten)]
    pub location: Option<ExplanationOfBenefitAddItemLocation>,
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Anatomical location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<ExplanationOfBenefitAddItemBodySite>,
    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,
    /// Additem level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ExplanationOfBenefitAddItemDetail>,
}

/// Anatomical location for an insurer-added item.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitAddItemBodySite;
///
/// let value = ExplanationOfBenefitAddItemBodySite::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ExplanationOfBenefitAddItemBodySite = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItemBodySite {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,
    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,
    /// Location
    pub site: vec1::Vec1<types::CodeableReference>,
    /// Sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::CodeableConcept>,
}

/// Insurer added detail items under an add item.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitAddItemDetail;
/// use fhir::r5::types;
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
pub struct ExplanationOfBenefitAddItemDetail {
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
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,
    /// Additem detail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
    /// Insurer added line items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ExplanationOfBenefitAddItemDetailSubDetail>,
}

/// Insurer added sub-detail items under an add item detail.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitAddItemDetailSubDetail;
/// use fhir::r5::types;
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
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
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
    /// Paid by the patient
    pub patient_paid: Option<types::Money>,
    /// Count of products or services
    pub quantity: Option<types::Quantity>,
    /// Fee, charge or cost per item
    pub unit_price: Option<types::Money>,
    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// Total tax
    pub tax: Option<types::Money>,
    /// Total item cost
    pub net: Option<types::Money>,
    /// Applicable note numbers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number: Vec<types::PositiveInt>,
    /// Primitive extension sibling for [`note_number`](Self::note_number) (FHIR `_noteNumber`).
    #[serde(rename = "_noteNumber")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note_number_ext: Vec<Option<types::Element>>,
    /// Additem subdetail level adjudication results
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,
    /// Added items adjudication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjudication: Vec<ExplanationOfBenefitItemAdjudication>,
}

/// Adjudication totals per category.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitTotal;
/// use fhir::r5::types;
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

/// Payment details for the adjudicated claim.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitPayment;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,
    /// Payable amount after adjustment
    pub amount: Option<types::Money>,
    /// Business identifier for the payment
    pub identifier: Option<types::Identifier>,
}

/// Note concerning adjudication.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitProcessNote;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`).
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,
    /// Note purpose
    pub r#type: Option<types::CodeableConcept>,
    /// Note explanatory text
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
    /// Language of the text
    pub language: Option<types::CodeableConcept>,
}

/// Balance by benefit category.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitBenefitBalance;
/// use fhir::r5::types;
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
    pub financial: Vec<ExplanationOfBenefitBenefitBalanceFinancial>,
}

/// Benefit summary detail for a benefit balance.
/// # Examples
///
/// ```
/// use fhir::r5::resources::explanation_of_benefit::ExplanationOfBenefitBenefitBalanceFinancial;
/// use fhir::r5::types;
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
    /// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialAllowed`].
    #[serde(flatten)]
    pub allowed: Option<ExplanationOfBenefitBenefitBalanceFinancialAllowed>,
    /// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (0..1); see [`ExplanationOfBenefitBenefitBalanceFinancialUsed`].
    #[serde(flatten)]
    pub used: Option<ExplanationOfBenefitBenefitBalanceFinancialUsed>,
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
/// The `ExplanationOfBenefit.accident.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAccidentLocation {
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.addItem.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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

/// The `ExplanationOfBenefit.addItem.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitAddItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.benefitBalance.financial.allowed[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialAllowed {
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

/// The `ExplanationOfBenefit.benefitBalance.financial.used[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitBenefitBalanceFinancialUsed {
    /// `usedUnsignedInt` variant.
    #[fhir("usedUnsignedInt")]
    UnsignedInt(crate::r5::choice::Primitive<types::UnsignedInt>),
    /// `usedMoney` variant.
    #[fhir("usedMoney")]
    Money(Box<types::Money>),
}

/// The `ExplanationOfBenefit.diagnosis.diagnosis[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.event.when[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitEventWhen {
    /// `whenDateTime` variant.
    #[fhir("whenDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `whenPeriod` variant.
    #[fhir("whenPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.item.location[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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

/// The `ExplanationOfBenefit.item.serviced[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.procedure.procedure[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitProcedureProcedure {
    /// `procedureCodeableConcept` variant.
    #[fhir("procedureCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `procedureReference` variant.
    #[fhir("procedureReference")]
    Reference(Box<types::Reference>),
}

/// The `ExplanationOfBenefit.supportingInfo.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitSupportingInfoTiming {
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
}

/// The `ExplanationOfBenefit.supportingInfo.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ExplanationOfBenefitSupportingInfoValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
    /// `valueIdentifier` variant.
    #[fhir("valueIdentifier")]
    Identifier(Box<types::Identifier>),
}
