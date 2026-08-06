//! Claim
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Claim
//!
//!
//!
//! Claim, Pre-determination or Pre-authorization
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Claim Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::Claim;
/// use fhir::r3::types;
///
/// let value = Claim {
///     created: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `created` is the name this serializes to on the wire.
/// assert_eq!(json["created"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Claim = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Claim {
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

    /// Claim number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r3::codes::FmStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Type or discipline
    pub r#type: Option<types::CodeableConcept>,

    /// Finer grained claim type information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_type: Vec<types::CodeableConcept>,

    /// complete | proposed | exploratory | other
    pub r#use: Option<crate::coded::Coded<crate::r3::codes::ClaimUse>>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

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

    /// Target
    pub insurer: Option<types::Reference>,

    /// Responsible provider
    pub provider: Option<types::Reference>,

    /// Responsible organization
    pub organization: Option<types::Reference>,

    /// Desired processing priority
    pub priority: Option<types::CodeableConcept>,

    /// Funds requested to be reserved
    pub funds_reserve: Option<types::CodeableConcept>,

    /// Related Claims which may be revelant to processing this claimn
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<ClaimRelated>,

    /// Prescription authorizing services or products
    pub prescription: Option<types::Reference>,

    /// Original prescription if superceded by fulfiller
    pub original_prescription: Option<types::Reference>,

    /// Party to be paid any benefits payable
    pub payee: Option<ClaimPayee>,

    /// Treatment Referral
    pub referral: Option<types::Reference>,

    /// Servicing Facility
    pub facility: Option<types::Reference>,

    /// Members of the care team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<ClaimCareTeam>,

    /// Exceptions, special considerations, the condition, situation, prior or
    /// concurrent issues
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information: Vec<ClaimInformation>,

    /// List of Diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<ClaimDiagnosis>,

    /// Procedures performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<ClaimProcedure>,

    /// Insurance or medical plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<ClaimInsurance>,

    /// Details about an accident
    pub accident: Option<ClaimAccident>,

    /// Period unable to work
    pub employment_impacted: Option<types::Period>,

    /// Period in hospital
    pub hospitalization: Option<types::Period>,

    /// Goods and Services
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ClaimItem>,

    /// Total claim cost
    pub total: Option<types::Money>,
}

/// An accident which resulted in the need for healthcare services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimAccident;
/// use fhir::r3::types;
///
/// let value = ClaimAccident {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimAccident = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimAccident {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// When the accident occurred see information codes see information codes
    pub date: types::Date,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,

    /// Accident Place
    /// The `Claim.accident.location[x]` choice element (0..1); see [`ClaimAccidentLocation`].
    #[serde(flatten)]
    pub location: Option<ClaimAccidentLocation>,
}

/// The members of the team who provided the overall service as well as their
/// role and whether responsible and qualifications.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimCareTeam;
/// use fhir::r3::types;
///
/// let value = ClaimCareTeam {
///     responsible: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `responsible` is the name this serializes to on the wire.
/// assert_eq!(json["responsible"], ::serde_json::json!(true));
///
/// let back: ClaimCareTeam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimCareTeam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Number to covey order of careTeam
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Provider individual or organization
    pub provider: types::Reference,

    /// Billing provider
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

/// List of patient diagnosis for which care is sought.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimDiagnosis;
/// use fhir::r3::types;
///
/// let value = ClaimDiagnosis {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimDiagnosis {
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
    /// The `Claim.diagnosis.diagnosis[x]` choice element (1..1); see [`ClaimDiagnosisDiagnosis`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub diagnosis: Option<ClaimDiagnosisDiagnosis>,

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
/// use fhir::r3::resources::claim::ClaimInformation;
/// use fhir::r3::types;
///
/// let value = ClaimInformation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimInformation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimInformation {
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
    /// The `Claim.information.timing[x]` choice element (0..1); see [`ClaimInformationTiming`].
    #[serde(flatten)]
    pub timing: Option<ClaimInformationTiming>,

    /// Additional Data or supporting information
    /// The `Claim.information.value[x]` choice element (0..1); see [`ClaimInformationValue`].
    #[serde(flatten)]
    pub value: Option<ClaimInformationValue>,

    /// Reason associated with the information
    pub reason: Option<types::CodeableConcept>,
}

/// Financial instrument by which payment information for health care.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimInsurance;
/// use fhir::r3::types;
///
/// let value = ClaimInsurance {
///     business_arrangement: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `businessArrangement` is the name this serializes to on the wire.
/// assert_eq!(json["businessArrangement"], ::serde_json::json!("abc"));
///
/// let back: ClaimInsurance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimInsurance {
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
    pub coverage: types::Reference,

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
    pub claim_response: Option<types::Reference>,
}

/// First tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimItem;
/// use fhir::r3::types;
///
/// let value = ClaimItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimItem {
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

    /// Applicable careTeam members
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
    /// The `Claim.item.serviced[x]` choice element (0..1); see [`ClaimItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ClaimItemServiced>,

    /// Place of service
    /// The `Claim.item.location[x]` choice element (0..1); see [`ClaimItemLocation`].
    #[serde(flatten)]
    pub location: Option<ClaimItemLocation>,

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

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimItemDetail>,
}

/// Second tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimItemDetail;
/// use fhir::r3::types;
///
/// let value = ClaimItemDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimItemDetail {
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

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimItemDetailSubDetail>,
}

/// Third tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimItemDetailSubDetail;
/// use fhir::r3::types;
///
/// let value = ClaimItemDetailSubDetail {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimItemDetailSubDetail {
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
}

/// The party to be reimbursed for the services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimPayee;
/// use fhir::r3::types;
///
/// let value = ClaimPayee {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimPayee = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimPayee {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of party: Subscriber, Provider, other
    pub r#type: types::CodeableConcept,

    /// organization | patient | practitioner | relatedperson
    pub resource_type: Option<types::Coding>,

    /// Party to receive the payable
    pub party: Option<types::Reference>,
}

/// Ordered list of patient procedures performed to support the adjudication.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimProcedure;
/// use fhir::r3::types;
///
/// let value = ClaimProcedure {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ClaimProcedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimProcedure {
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
    /// The `Claim.procedure.procedure[x]` choice element (1..1); see [`ClaimProcedureProcedure`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub procedure: Option<ClaimProcedureProcedure>,
}

/// Other claims which are related to this claim such as prior claim versions
/// or for related services.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::claim::ClaimRelated;
/// use fhir::r3::types;
///
/// let value = ClaimRelated {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimRelated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ClaimRelated {
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

/// The `Claim.accident.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimAccidentLocation {
    /// `locationAddress` variant.
    #[fhir("locationAddress")]
    Address(Box<types::Address>),
    /// `locationReference` variant.
    #[fhir("locationReference")]
    Reference(Box<types::Reference>),
}

/// The `Claim.diagnosis.diagnosis[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `Claim.information.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimInformationTiming {
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
}

/// The `Claim.information.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimInformationValue {
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

/// The `Claim.item.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `Claim.item.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimItemLocation {
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

/// The `Claim.procedure.procedure[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimProcedureProcedure {
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
    type T = Claim;

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
