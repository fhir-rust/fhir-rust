//! Claim
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Claim
//!
//!
//!
//! Claim, Pre-determination or Pre-authorization
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Claim Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::Claim;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// institutional | oral | pharmacy | professional | vision
    pub r#type: crate::coded::Coded<crate::r2::codes::ClaimTypeLink>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Claim number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Current specification followed
    pub ruleset: Option<types::Coding>,

    /// Original specification followed
    pub original_ruleset: Option<types::Coding>,

    /// Creation date
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Insurer
    pub target: Option<types::Reference<crate::r2::resources::Organization>>,

    /// Responsible provider
    pub provider: Option<types::Reference<crate::r2::resources::Practitioner>>,

    /// Responsible organization
    pub organization: Option<types::Reference<crate::r2::resources::Organization>>,

    /// complete | proposed | exploratory | other
    pub r#use: Option<crate::coded::Coded<crate::r2::codes::ClaimUseLink>>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Desired processing priority
    pub priority: Option<types::Coding>,

    /// Funds requested to be reserved
    pub funds_reserve: Option<types::Coding>,

    /// Author
    pub enterer: Option<types::Reference<crate::r2::resources::Practitioner>>,

    /// Servicing Facility
    pub facility: Option<types::Reference<crate::r2::resources::Location>>,

    /// Prescription
    pub prescription: Option<types::Reference>,

    /// Original Prescription
    pub original_prescription: Option<types::Reference<crate::r2::resources::MedicationOrder>>,

    /// Payee
    pub payee: Option<ClaimPayee>,

    /// Treatment Referral
    pub referral: Option<types::Reference<crate::r2::resources::ReferralRequest>>,

    /// Diagnosis
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis: Vec<ClaimDiagnosis>,

    /// List of presenting Conditions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<types::Coding>,

    /// The subject of the Products and Services
    pub patient: types::Reference<crate::r2::resources::Patient>,

    /// Insurance or medical plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverage: Vec<ClaimCoverage>,

    /// Eligibility exceptions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exception: Vec<types::Coding>,

    /// Name of School
    pub school: Option<types::String>,
    /// Primitive extension sibling for [`school`](Self::school) (FHIR `_school`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_school")]
    pub school_ext: Option<types::Element>,

    /// Accident Date
    pub accident: Option<types::Date>,
    /// Primitive extension sibling for [`accident`](Self::accident) (FHIR `_accident`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_accident")]
    pub accident_ext: Option<types::Element>,

    /// Accident Type
    pub accident_type: Option<types::Coding>,

    /// Intervention and exception code (Pharma)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intervention_exception: Vec<types::Coding>,

    /// Goods and Services
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ClaimItem>,

    /// Additional materials, documents, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_materials: Vec<types::Coding>,

    /// Only if type = oral
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_teeth: Vec<ClaimMissingTeeth>,
}

/// Financial instrument by which payment information for health care.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimCoverage;
/// use fhir::r2::types;
///
/// let value = ClaimCoverage {
///     business_arrangement: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `businessArrangement` is the name this serializes to on the wire.
/// assert_eq!(json["businessArrangement"], ::serde_json::json!("abc"));
///
/// let back: ClaimCoverage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimCoverage {
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

    /// The focal Coverage
    pub focal: types::Boolean,
    /// Primitive extension sibling for [`focal`](Self::focal) (FHIR `_focal`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_focal")]
    pub focal_ext: Option<types::Element>,

    /// Insurance information
    pub coverage: types::Reference<crate::r2::resources::Coverage>,

    /// Business agreement
    pub business_arrangement: Option<types::String>,
    /// Primitive extension sibling for [`business_arrangement`](Self::business_arrangement) (FHIR `_businessArrangement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_businessArrangement")]
    pub business_arrangement_ext: Option<types::Element>,

    /// Patient relationship to subscriber
    pub relationship: types::Coding,

    /// Pre-Authorization/Determination Reference
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub pre_auth_ref: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`pre_auth_ref`](Self::pre_auth_ref) (FHIR `_preAuthRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preAuthRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pre_auth_ref_ext: Vec<Option<types::Element>>,

    /// Adjudication results
    pub claim_response: Option<types::Reference<crate::r2::resources::ClaimResponse>>,

    /// Original version
    pub original_ruleset: Option<types::Coding>,
}

/// Ordered list of patient diagnosis for which care is sought.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimDiagnosis;
/// use fhir::r2::types;
///
/// let value = ClaimDiagnosis {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimDiagnosis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimDiagnosis {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Sequence of diagnosis
    pub sequence: types::PositiveInt,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Patient's list of diagnosis
    pub diagnosis: types::Coding,
}

/// First tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimItem;
/// use fhir::r2::types;
///
/// let value = ClaimItem {
///     service_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `serviceDate` is the name this serializes to on the wire.
/// assert_eq!(json["serviceDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: ClaimItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub r#type: types::Coding,

    /// Responsible practitioner
    pub provider: Option<types::Reference<crate::r2::resources::Practitioner>>,

    /// Diagnosis Link
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub diagnosis_link_id: ::fhir_core::PrimVec<types::PositiveInt>,
    /// Primitive extension sibling for [`diagnosis_link_id`](Self::diagnosis_link_id) (FHIR `_diagnosisLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_diagnosisLinkId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnosis_link_id_ext: Vec<Option<types::Element>>,

    /// Item Code
    pub service: types::Coding,

    /// Date of Service
    pub service_date: Option<types::Date>,
    /// Primitive extension sibling for [`service_date`](Self::service_date) (FHIR `_serviceDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_serviceDate")]
    pub service_date_ext: Option<types::Element>,

    /// Count of Products or Services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per point
    pub unit_price: Option<types::Quantity>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Difficulty scaling factor
    pub points: Option<types::Decimal>,
    /// Primitive extension sibling for [`points`](Self::points) (FHIR `_points`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_points")]
    pub points_ext: Option<types::Element>,

    /// Total item cost
    pub net: Option<types::Quantity>,

    /// Unique Device Identifier
    pub udi: Option<types::Coding>,

    /// Service Location
    pub body_site: Option<types::Coding>,

    /// Service Sub-location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_site: Vec<types::Coding>,

    /// Service/Product billing modifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier: Vec<types::Coding>,

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detail: Vec<ClaimItemDetail>,

    /// Prosthetic details
    pub prosthesis: Option<ClaimItemProsthesis>,
}

/// Second tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimItemDetail;
/// use fhir::r2::types;
///
/// let value = ClaimItemDetail {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimItemDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimItemDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub r#type: types::Coding,

    /// Additional item codes
    pub service: types::Coding,

    /// Count of Products or Services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per point
    pub unit_price: Option<types::Quantity>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Difficulty scaling factor
    pub points: Option<types::Decimal>,
    /// Primitive extension sibling for [`points`](Self::points) (FHIR `_points`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_points")]
    pub points_ext: Option<types::Element>,

    /// Total additional item cost
    pub net: Option<types::Quantity>,

    /// Unique Device Identifier
    pub udi: Option<types::Coding>,

    /// Additional items
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_detail: Vec<ClaimItemDetailSubDetail>,
}

/// Third tier of goods and services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimItemDetailSubDetail;
/// use fhir::r2::types;
///
/// let value = ClaimItemDetailSubDetail {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimItemDetailSubDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimItemDetailSubDetail {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub r#type: types::Coding,

    /// Additional item codes
    pub service: types::Coding,

    /// Count of Products or Services
    pub quantity: Option<types::Quantity>,

    /// Fee, charge or cost per point
    pub unit_price: Option<types::Quantity>,

    /// Price scaling factor
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Difficulty scaling factor
    pub points: Option<types::Decimal>,
    /// Primitive extension sibling for [`points`](Self::points) (FHIR `_points`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_points")]
    pub points_ext: Option<types::Element>,

    /// Net additional item cost
    pub net: Option<types::Quantity>,

    /// Unique Device Identifier
    pub udi: Option<types::Coding>,
}

/// The materials and placement date of prior fixed prosthesis.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimItemProsthesis;
/// use fhir::r2::types;
///
/// let value = ClaimItemProsthesis {
///     prior_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `priorDate` is the name this serializes to on the wire.
/// assert_eq!(json["priorDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: ClaimItemProsthesis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimItemProsthesis {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Is this the initial service
    pub initial: Option<types::Boolean>,
    /// Primitive extension sibling for [`initial`](Self::initial) (FHIR `_initial`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_initial")]
    pub initial_ext: Option<types::Element>,

    /// Initial service Date
    pub prior_date: Option<types::Date>,
    /// Primitive extension sibling for [`prior_date`](Self::prior_date) (FHIR `_priorDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priorDate")]
    pub prior_date_ext: Option<types::Element>,

    /// Prosthetic Material
    pub prior_material: Option<types::Coding>,
}

/// A list of teeth which would be expected but are not found due to having
/// been previously extracted or for other reasons.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimMissingTeeth;
/// use fhir::r2::types;
///
/// let value = ClaimMissingTeeth {
///     extraction_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `extractionDate` is the name this serializes to on the wire.
/// assert_eq!(json["extractionDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: ClaimMissingTeeth = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimMissingTeeth {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Tooth Code
    pub tooth: types::Coding,

    /// Reason for missing
    pub reason: Option<types::Coding>,

    /// Date of Extraction
    pub extraction_date: Option<types::Date>,
    /// Primitive extension sibling for [`extraction_date`](Self::extraction_date) (FHIR `_extractionDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_extractionDate")]
    pub extraction_date_ext: Option<types::Element>,
}

/// The party to be reimbursed for the services.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::claim::ClaimPayee;
/// use fhir::r2::types;
///
/// let value = ClaimPayee {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ClaimPayee = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ClaimPayee {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Party to be paid any benefits payable
    pub r#type: Option<types::Coding>,

    /// Provider who is the payee
    pub provider: Option<types::Reference<crate::r2::resources::Practitioner>>,

    /// Organization who is the payee
    pub organization: Option<types::Reference<crate::r2::resources::Organization>>,

    /// Other person who is the payee
    pub person: Option<types::Reference<crate::r2::resources::Patient>>,
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
