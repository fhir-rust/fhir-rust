//! ImmunizationRecommendation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
//!
//! Version: 5.0.0
//!
//! ImmunizationRecommendation Resource: A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ImmunizationRecommendation resource.
///
/// A patient's point-in-time set of recommendations (i.e. forecasting)
/// according to a published schedule, with optional supporting justification.
/// The resource captures which vaccines or vaccine groups a patient should
/// receive, the forecast status of each recommendation (for example due,
/// overdue, complete, or contraindicated), the dose number within a series,
/// and the calendar dates that govern when a dose becomes due. Each
/// recommendation may also cite the past immunizations and patient
/// observations that support the reasoning, so the forecast can be audited
/// and explained.
///
/// In FHIR R5 this resource is typically produced by a clinical decision
/// support system or immunization forecasting engine and consumed by
/// electronic health records, patient portals, and immunization information
/// systems to remind clinicians and patients of vaccinations that are due.
/// Because it is a point-in-time snapshot, a new instance is generally
/// generated whenever the forecast is recalculated rather than updating a
/// prior one.
///
/// # Related resources
///
/// The recommendation is always about a single subject, referenced from the
/// `patient` field and typically resolving to a [`Patient`](crate::r5::resources::patient::Patient).
/// Supporting evidence commonly references past [`Immunization`](crate::r5::resources::immunization::Immunization)
/// records and [`Observation`](crate::r5::resources::observation::Observation) results. Coded
/// concepts such as the vaccine code and forecast status use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), while links to other
/// resources use [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::immunization_recommendation::ImmunizationRecommendation;
///
/// let value = ImmunizationRecommendation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImmunizationRecommendation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendation {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Reference to the patient for whom this set of recommendations is forecast.
    pub patient: types::Reference<crate::r5::resources::Patient>,

    /// Point in time at which this set of recommendations was generated.
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Organization responsible for the immunization protocol used to forecast.
    pub authority: Option<types::Reference<crate::r5::resources::Organization>>,

    /// One entry per vaccine or vaccine group being recommended for the patient.
    pub recommendation: vec1::Vec1<ImmunizationRecommendationRecommendation>,
}

/// Vaccine administration recommendations.
/// # Examples
///
/// ```
/// use fhir::r5::resources::immunization_recommendation::ImmunizationRecommendationRecommendation;
/// use fhir::r5::types;
///
/// let value = ImmunizationRecommendationRecommendation {
///     dose_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doseNumber` is the name this serializes to on the wire.
/// assert_eq!(json["doseNumber"], ::serde_json::json!("abc"));
///
/// let back: ImmunizationRecommendationRecommendation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendationRecommendation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Coded vaccine or vaccine group that this recommendation applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vaccine_code: Vec<types::CodeableConcept>,

    /// Coded disease or diseases the recommended vaccine protects against.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_disease: Vec<types::CodeableConcept>,

    /// Vaccine which is contraindicated to fulfill the recommendation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contraindicated_vaccine_code: Vec<types::CodeableConcept>,

    /// Coded forecast status such as due, overdue, immune, complete, or contraindicated.
    pub forecast_status: types::CodeableConcept,

    /// Vaccine administration status reason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forecast_reason: Vec<types::CodeableConcept>,

    /// Dates governing proposed immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_criterion: Vec<ImmunizationRecommendationRecommendationDateCriterion>,

    /// Protocol details
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Name of vaccination series
    pub series: Option<types::String>,
    /// Primitive extension sibling for [`series`](Self::series) (FHIR `_series`).
    #[serde(rename = "_series")]
    pub series_ext: Option<types::Element>,

    /// Recommended dose number within series
    pub dose_number: Option<types::String>,
    /// Primitive extension sibling for [`dose_number`](Self::dose_number) (FHIR `_doseNumber`).
    #[serde(rename = "_doseNumber")]
    pub dose_number_ext: Option<types::Element>,

    /// Recommended number of doses for immunity
    pub series_doses: Option<types::String>,
    /// Primitive extension sibling for [`series_doses`](Self::series_doses) (FHIR `_seriesDoses`).
    #[serde(rename = "_seriesDoses")]
    pub series_doses_ext: Option<types::Element>,

    /// Past immunizations supporting recommendation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_immunization: Vec<types::Reference>,

    /// Patient observations supporting recommendation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_patient_information: Vec<types::Reference>,
}

/// Dates governing proposed immunization.
/// # Examples
///
/// ```
/// use fhir::r5::resources::immunization_recommendation::ImmunizationRecommendationRecommendationDateCriterion;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}
