//! ImmunizationRecommendation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
//!
//! Version: 6.0.0-ballot3
//!
//! Guidance or advice relating to an immunization
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A patient's point-in-time set of recommendations (i.e. forecasting)
/// according to a published schedule with optional supporting justification.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::immunization_recommendation::ImmunizationRecommendation;
///
/// let value = ImmunizationRecommendation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImmunizationRecommendation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImmunizationRecommendation {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who this profile is for
    pub patient: types::Reference<crate::r6::resources::Patient>,

    /// Date recommendation(s) created
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is responsible for schedule
    pub authority: Option<types::Reference<crate::r6::resources::Organization>>,

    /// Vaccine administration recommendations
    pub recommendation: ::vec1::Vec1<ImmunizationRecommendationRecommendation>,
}

/// Vaccine administration recommendations.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::immunization_recommendation::ImmunizationRecommendationRecommendation;
/// use fhir::r6::types;
///
/// let value = ImmunizationRecommendationRecommendation {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: ImmunizationRecommendationRecommendation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImmunizationRecommendationRecommendation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Vaccine or vaccine group recommendation applies to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vaccine_code: Vec<types::CodeableConcept>,

    /// Disease to be immunized against
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_disease: Vec<types::CodeableConcept>,

    /// Vaccine which is contraindicated to fulfill the recommendation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contraindicated_vaccine_code: Vec<types::CodeableConcept>,

    /// Vaccine recommendation status
    pub forecast_status: types::CodeableConcept,

    /// Vaccine administration status reason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forecast_reason: Vec<types::CodeableConcept>,

    /// Dates governing proposed immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_criterion: Vec<ImmunizationRecommendationRecommendationDateCriterion>,

    /// Protocol details
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Name of vaccination series
    pub series: Option<types::String>,
    /// Primitive extension sibling for [`series`](Self::series) (FHIR `_series`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_series")]
    pub series_ext: Option<types::Element>,

    /// Recommended dose number within series
    pub dose_number: Option<types::CodeableConcept>,

    /// Recommended number of doses for immunity
    pub series_doses: Option<types::CodeableConcept>,

    /// Past immunizations supporting recommendation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_immunization: Vec<types::Reference>,

    /// Patient observations supporting recommendation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_patient_information: Vec<types::Reference>,
}

/// Vaccine date recommendations. For example, earliest date to administer,
/// latest date to administer, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::immunization_recommendation::ImmunizationRecommendationRecommendationDateCriterion;
/// use fhir::r6::types;
///
/// let value = ImmunizationRecommendationRecommendationDateCriterion {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImmunizationRecommendationRecommendationDateCriterion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of date
    pub code: types::CodeableConcept,

    /// Recommended date
    pub value: types::DateTime,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}
