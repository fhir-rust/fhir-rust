//! ImmunizationRecommendation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImmunizationRecommendation
//!
//!
//!
//! Guidance or advice relating to an immunization
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ImmunizationRecommendation Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::immunization_recommendation::ImmunizationRecommendation;
///
/// let value = ImmunizationRecommendation::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImmunizationRecommendation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImmunizationRecommendation {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who this profile is for
    pub patient: types::Reference<crate::r3::resources::Patient>,

    /// Vaccine administration recommendations
    pub recommendation: ::vec1::Vec1<ImmunizationRecommendationRecommendation>,
}

/// Vaccine administration recommendations.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::immunization_recommendation::ImmunizationRecommendationRecommendation;
/// use fhir::r3::types;
///
/// let value = ImmunizationRecommendationRecommendation {
///     dose_number: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doseNumber` is the name this serializes to on the wire.
/// assert_eq!(json["doseNumber"], ::serde_json::json!(1));
///
/// let back: ImmunizationRecommendationRecommendation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImmunizationRecommendationRecommendation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Date recommendation created
    pub date: types::DateTime,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Vaccine recommendation applies to
    pub vaccine_code: Option<types::CodeableConcept>,

    /// Disease to be immunized against
    pub target_disease: Option<types::CodeableConcept>,

    /// Recommended dose number
    pub dose_number: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`dose_number`](Self::dose_number) (FHIR `_doseNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doseNumber")]
    pub dose_number_ext: Option<types::Element>,

    /// Vaccine administration status
    pub forecast_status: types::CodeableConcept,

    /// Dates governing proposed immunization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub date_criterion: Vec<ImmunizationRecommendationRecommendationDateCriterion>,

    /// Protocol used by recommendation
    pub protocol: Option<ImmunizationRecommendationRecommendationProtocol>,

    /// Past immunizations supporting recommendation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_immunization: Vec<types::Reference<crate::r3::resources::Immunization>>,

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
/// use fhir::r3::resources::immunization_recommendation::ImmunizationRecommendationRecommendationDateCriterion;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
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

/// Contains information about the protocol under which the vaccine was
/// administered.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::immunization_recommendation::ImmunizationRecommendationRecommendationProtocol;
/// use fhir::r3::types;
///
/// let value = ImmunizationRecommendationRecommendationProtocol {
///     dose_sequence: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doseSequence` is the name this serializes to on the wire.
/// assert_eq!(json["doseSequence"], ::serde_json::json!(1));
///
/// let back: ImmunizationRecommendationRecommendationProtocol = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImmunizationRecommendationRecommendationProtocol {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Dose number within sequence
    pub dose_sequence: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`dose_sequence`](Self::dose_sequence) (FHIR `_doseSequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doseSequence")]
    pub dose_sequence_ext: Option<types::Element>,

    /// Protocol details
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Who is responsible for protocol
    pub authority: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Name of vaccination series
    pub series: Option<types::String>,
    /// Primitive extension sibling for [`series`](Self::series) (FHIR `_series`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_series")]
    pub series_ext: Option<types::Element>,
}
