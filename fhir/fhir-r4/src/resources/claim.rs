//! Claim
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Claim
//!
//! Version: 4.0.1
//!
//! Claim, Pre-determination or Pre-authorization
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A provider issued list of professional services and products which have
/// been provided, or are to be provided, to a patient which is sent to an
/// insurer for reimbursement.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::claim::Claim;
///
/// let value = Claim::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Claim = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Claim {
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

    /// Business Identifier for claim
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::FmStatus>,
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
    pub patient: types::Reference<crate::r4::resources::Patient>,

    /// Relevant time frame for the claim
    pub billable_period: Option<types::Period>,

    /// Resource creation date
    pub created: types::DateTime,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Author of the claim
    pub enterer: Option<types::Reference>,

    /// Target
    pub insurer: Option<types::Reference<crate::r4::resources::Organization>>,

    /// Party responsible for the claim
    pub provider: types::Reference,

    /// Desired processing ugency
    pub priority: types::CodeableConcept,

    /// For whom to reserve funds
    pub funds_reserve: Option<types::CodeableConcept>,

    /// Prior or corollary claims
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<ClaimRelated>,

    /// Prescription authorizing services and products
    pub prescription: Option<types::Reference>,

    /// Original prescription if superseded by fulfiller
    pub original_prescription: Option<types::Reference>,

    /// Recipient of benefits payable
    pub payee: Option<ClaimPayee>,

    /// Treatment referral
    pub referral: Option<types::Reference<crate::r4::resources::ServiceRequest>>,

    /// Servicing facility
    pub facility: Option<types::Reference<crate::r4::resources::Location>>,

    /// Members of the care team
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub care_team: Vec<ClaimCareTeam>,

    /// Supporting information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<ClaimSupportingInfo>,

    /// Pertinent diagnosis information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<ClaimDiagnosis>,

    /// Clinical procedures performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<ClaimProcedure>,

    /// Patient insurance information
    pub insurance: ::vec1::Vec1<ClaimInsurance>,

    /// Details of the event
    pub accident: Option<ClaimAccident>,

    /// Product or service provided
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ClaimItem>,

    /// Total claim cost
    pub total: Option<types::Money>,
}

/// Details of an accident which resulted in injuries which required the
/// products and services listed in the claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimAccident;
/// use fhir::r4::types;
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
#[serde(from = "ClaimAccidentDe")]
#[fhir_version("r4")]
pub struct ClaimAccident {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// When the incident occurred
    pub date: types::Date,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The nature of the accident
    pub r#type: Option<types::CodeableConcept>,

    /// Where the event occurred
    /// The `Claim.accident.location[x]` choice element (0..1); see [`ClaimAccidentLocation`].
    #[serde(flatten)]
    pub location: Option<ClaimAccidentLocation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimAccidentDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    date: types::Date,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    r#type: Option<types::CodeableConcept>,
    #[serde(flatten)]
    location: crate::r4::choice::Slot<ClaimAccidentLocation>,
}

impl ::core::convert::From<ClaimAccidentDe> for ClaimAccident {
    fn from(v: ClaimAccidentDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            date: v.date,
            date_ext: v.date_ext,
            r#type: v.r#type,
            location: v.location.0,
        }
    }
}

/// The members of the team who provided the products and services.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimCareTeam;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct ClaimCareTeam {
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
/// use fhir::r4::resources::claim::ClaimDiagnosis;
/// use fhir::r4::types;
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
#[serde(from = "ClaimDiagnosisDe")]
#[fhir_version("r4")]
pub struct ClaimDiagnosis {
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
    /// The `Claim.diagnosis.diagnosis[x]` choice element (1..1); see [`ClaimDiagnosisDiagnosis`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub diagnosis: Option<ClaimDiagnosisDiagnosis>,

    /// Timing or nature of the diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Present on admission
    pub on_admission: Option<types::CodeableConcept>,

    /// Package billing code
    pub package_code: Option<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimDiagnosisDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    sequence: types::PositiveInt,
    #[serde(rename = "_sequence")]
    sequence_ext: Option<types::Element>,
    #[serde(flatten)]
    diagnosis: crate::r4::choice::Slot<ClaimDiagnosisDiagnosis>,
    #[serde(default)]
    r#type: Vec<types::CodeableConcept>,
    on_admission: Option<types::CodeableConcept>,
    package_code: Option<types::CodeableConcept>,
}

impl ::core::convert::From<ClaimDiagnosisDe> for ClaimDiagnosis {
    fn from(v: ClaimDiagnosisDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            sequence: v.sequence,
            sequence_ext: v.sequence_ext,
            diagnosis: v.diagnosis.0,
            r#type: v.r#type,
            on_admission: v.on_admission,
            package_code: v.package_code,
        }
    }
}

/// Financial instruments for reimbursement for the health care products and
/// services specified on the claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimInsurance;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct ClaimInsurance {
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

    /// Pre-assigned Claim number
    pub identifier: Option<types::Identifier>,

    /// Insurance information
    pub coverage: types::Reference<crate::r4::resources::Coverage>,

    /// Additional provider contract number
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,

    /// Prior authorization reference number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref: Vec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,

    /// Adjudication results
    pub claim_response: Option<types::Reference<crate::r4::resources::ClaimResponse>>,
}

/// A claim line. Either a simple product or service or a 'group' of details
/// which can each be a simple items or groups of sub-details.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimItem;
/// use fhir::r4::types;
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
#[serde(from = "ClaimItemDe")]
#[fhir_version("r4")]
pub struct ClaimItem {
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

    /// Applicable careTeam members
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
    /// The `Claim.item.serviced[x]` choice element (0..1); see [`ClaimItemServiced`].
    #[serde(flatten)]
    pub serviced: Option<ClaimItemServiced>,

    /// Place of service or where product was supplied
    /// The `Claim.item.location[x]` choice element (0..1); see [`ClaimItemLocation`].
    #[serde(flatten)]
    pub location: Option<ClaimItemLocation>,

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
    pub udi: Vec<types::Reference<crate::r4::resources::Device>>,

    /// Anatomical location
    pub body_site: Option<types::CodeableConcept>,

    /// Anatomical sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::CodeableConcept>,

    /// Encounters related to this billed item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub encounter: Vec<types::Reference<crate::r4::resources::Encounter>>,

    /// Product or service provided
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimItemDetail>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    sequence: types::PositiveInt,
    #[serde(rename = "_sequence")]
    sequence_ext: Option<types::Element>,
    #[serde(default)]
    care_team_sequence: Vec<types::PositiveInt>,
    #[serde(rename = "_careTeamSequence")]
    #[serde(default)]
    care_team_sequence_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    diagnosis_sequence: Vec<types::PositiveInt>,
    #[serde(rename = "_diagnosisSequence")]
    #[serde(default)]
    diagnosis_sequence_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    procedure_sequence: Vec<types::PositiveInt>,
    #[serde(rename = "_procedureSequence")]
    #[serde(default)]
    procedure_sequence_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    information_sequence: Vec<types::PositiveInt>,
    #[serde(rename = "_informationSequence")]
    #[serde(default)]
    information_sequence_ext: Vec<Option<types::Element>>,
    revenue: Option<types::CodeableConcept>,
    category: Option<types::CodeableConcept>,
    product_or_service: types::CodeableConcept,
    #[serde(default)]
    modifier: Vec<types::CodeableConcept>,
    #[serde(default)]
    program_code: Vec<types::CodeableConcept>,
    #[serde(flatten)]
    serviced: crate::r4::choice::Slot<ClaimItemServiced>,
    #[serde(flatten)]
    location: crate::r4::choice::Slot<ClaimItemLocation>,
    quantity: Option<types::Quantity>,
    unit_price: Option<types::Money>,
    factor: Option<types::Decimal>,
    #[serde(rename = "_factor")]
    factor_ext: Option<types::Element>,
    net: Option<types::Money>,
    #[serde(default)]
    udi: Vec<types::Reference<crate::r4::resources::Device>>,
    body_site: Option<types::CodeableConcept>,
    #[serde(default)]
    sub_site: Vec<types::CodeableConcept>,
    #[serde(default)]
    encounter: Vec<types::Reference<crate::r4::resources::Encounter>>,
    #[serde(default)]
    detail: Vec<ClaimItemDetail>,
}

impl ::core::convert::From<ClaimItemDe> for ClaimItem {
    fn from(v: ClaimItemDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            sequence: v.sequence,
            sequence_ext: v.sequence_ext,
            care_team_sequence: v.care_team_sequence,
            care_team_sequence_ext: v.care_team_sequence_ext,
            diagnosis_sequence: v.diagnosis_sequence,
            diagnosis_sequence_ext: v.diagnosis_sequence_ext,
            procedure_sequence: v.procedure_sequence,
            procedure_sequence_ext: v.procedure_sequence_ext,
            information_sequence: v.information_sequence,
            information_sequence_ext: v.information_sequence_ext,
            revenue: v.revenue,
            category: v.category,
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
            udi: v.udi,
            body_site: v.body_site,
            sub_site: v.sub_site,
            encounter: v.encounter,
            detail: v.detail,
        }
    }
}

/// A claim detail line. Either a simple (a product or service) or a 'group' of
/// sub-details which are simple items.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimItemDetail;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct ClaimItemDetail {
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
    pub udi: Vec<types::Reference<crate::r4::resources::Device>>,

    /// Product or service provided
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimItemDetailSubDetail>,
}

/// A claim detail line. Either a simple (a product or service) or a 'group' of
/// sub-details which are simple items.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimItemDetailSubDetail;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct ClaimItemDetailSubDetail {
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
    pub udi: Vec<types::Reference<crate::r4::resources::Device>>,
}

/// The party to be reimbursed for cost of the products and services according
/// to the terms of the policy.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimPayee;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct ClaimPayee {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Category of recipient
    pub r#type: types::CodeableConcept,

    /// Recipient reference
    pub party: Option<types::Reference>,
}

/// Procedures performed on the patient relevant to the billing items with the
/// claim.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimProcedure;
/// use fhir::r4::types;
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
#[serde(from = "ClaimProcedureDe")]
#[fhir_version("r4")]
pub struct ClaimProcedure {
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
    /// The `Claim.procedure.procedure[x]` choice element (1..1); see [`ClaimProcedureProcedure`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub procedure: Option<ClaimProcedureProcedure>,

    /// Unique device identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub udi: Vec<types::Reference<crate::r4::resources::Device>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimProcedureDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    sequence: types::PositiveInt,
    #[serde(rename = "_sequence")]
    sequence_ext: Option<types::Element>,
    #[serde(default)]
    r#type: Vec<types::CodeableConcept>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    #[serde(flatten)]
    procedure: crate::r4::choice::Slot<ClaimProcedureProcedure>,
    #[serde(default)]
    udi: Vec<types::Reference<crate::r4::resources::Device>>,
}

impl ::core::convert::From<ClaimProcedureDe> for ClaimProcedure {
    fn from(v: ClaimProcedureDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            sequence: v.sequence,
            sequence_ext: v.sequence_ext,
            r#type: v.r#type,
            date: v.date,
            date_ext: v.date_ext,
            procedure: v.procedure.0,
            udi: v.udi,
        }
    }
}

/// Other claims which are related to this claim such as prior submissions or
/// claims for related services or for the same event.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::claim::ClaimRelated;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
pub struct ClaimRelated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference to the related claim
    pub claim: Option<types::Reference<crate::r4::resources::Claim>>,

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
/// use fhir::r4::resources::claim::ClaimSupportingInfo;
/// use fhir::r4::types;
///
/// let value = ClaimSupportingInfo {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ClaimSupportingInfo = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ClaimSupportingInfoDe")]
#[fhir_version("r4")]
pub struct ClaimSupportingInfo {
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
    /// The `Claim.supportingInfo.timing[x]` choice element (0..1); see [`ClaimSupportingInfoTiming`].
    #[serde(flatten)]
    pub timing: Option<ClaimSupportingInfoTiming>,

    /// Data to be provided
    /// The `Claim.supportingInfo.value[x]` choice element (0..1); see [`ClaimSupportingInfoValue`].
    #[serde(flatten)]
    pub value: Option<ClaimSupportingInfoValue>,

    /// Explanation for the information
    pub reason: Option<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClaimSupportingInfoDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    sequence: types::PositiveInt,
    #[serde(rename = "_sequence")]
    sequence_ext: Option<types::Element>,
    category: types::CodeableConcept,
    code: Option<types::CodeableConcept>,
    #[serde(flatten)]
    timing: crate::r4::choice::Slot<ClaimSupportingInfoTiming>,
    #[serde(flatten)]
    value: crate::r4::choice::Slot<ClaimSupportingInfoValue>,
    reason: Option<types::CodeableConcept>,
}

impl ::core::convert::From<ClaimSupportingInfoDe> for ClaimSupportingInfo {
    fn from(v: ClaimSupportingInfoDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            sequence: v.sequence,
            sequence_ext: v.sequence_ext,
            category: v.category,
            code: v.code,
            timing: v.timing.0,
            value: v.value.0,
            reason: v.reason,
        }
    }
}

/// The `Claim.accident.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimDiagnosisDiagnosis {
    /// `diagnosisCodeableConcept` variant.
    #[fhir("diagnosisCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `diagnosisReference` variant.
    #[fhir("diagnosisReference")]
    Reference(Box<types::Reference>),
}

/// The `Claim.item.serviced[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimItemServiced {
    /// `servicedDate` variant.
    #[fhir("servicedDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `servicedPeriod` variant.
    #[fhir("servicedPeriod")]
    Period(Box<types::Period>),
}

/// The `Claim.item.location[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimProcedureProcedure {
    /// `procedureCodeableConcept` variant.
    #[fhir("procedureCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `procedureReference` variant.
    #[fhir("procedureReference")]
    Reference(Box<types::Reference>),
}

/// The `Claim.supportingInfo.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimSupportingInfoTiming {
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r4::choice::Primitive<types::Date>),
    /// `timingPeriod` variant.
    #[fhir("timingPeriod")]
    Period(Box<types::Period>),
}

/// The `Claim.supportingInfo.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ClaimSupportingInfoValue {
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
