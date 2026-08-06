//! ImmunizationEvaluation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImmunizationEvaluation
//!
//! Version: 4.0.1
//!
//! Immunization evaluation information
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes a comparison of an immunization event against published
/// recommendations to determine if the administration is "valid" in relation
/// to those recommendations.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::immunization_evaluation::ImmunizationEvaluation;
/// use fhir::r4::types;
///
/// let value = ImmunizationEvaluation {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ImmunizationEvaluation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct ImmunizationEvaluation {
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

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// completed | entered-in-error
    pub status: types::Code,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Who this evaluation is for
    pub patient: types::Reference,

    /// Date evaluation was performed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is responsible for publishing the recommendations
    pub authority: Option<types::Reference>,

    /// Evaluation target disease
    pub target_disease: types::CodeableConcept,

    /// Immunization being evaluated
    pub immunization_event: types::Reference,

    /// Status of the dose relative to published recommendations
    pub dose_status: types::CodeableConcept,

    /// Reason for the dose status
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dose_status_reason: Vec<types::CodeableConcept>,

    /// Evaluation notes
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Name of vaccine series
    pub series: Option<types::String>,
    /// Primitive extension sibling for [`series`](Self::series) (FHIR `_series`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_series")]
    pub series_ext: Option<types::Element>,

    /// Dose number within series
    /// The `ImmunizationEvaluation.doseNumber[x]` choice element (0..1); see [`ImmunizationEvaluationDoseNumber`].
    #[serde(flatten)]
    pub dose_number: Option<ImmunizationEvaluationDoseNumber>,

    /// Recommended number of doses for immunity
    /// The `ImmunizationEvaluation.seriesDoses[x]` choice element (0..1); see [`ImmunizationEvaluationSeriesDoses`].
    #[serde(flatten)]
    pub series_doses: Option<ImmunizationEvaluationSeriesDoses>,
}

/// The `ImmunizationEvaluation.doseNumber[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ImmunizationEvaluationDoseNumber {
    /// `doseNumberPositiveInt` variant.
    #[fhir("doseNumberPositiveInt")]
    PositiveInt(crate::r4::choice::Primitive<types::PositiveInt>),
    /// `doseNumberString` variant.
    #[fhir("doseNumberString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `ImmunizationEvaluation.seriesDoses[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ImmunizationEvaluationSeriesDoses {
    /// `seriesDosesPositiveInt` variant.
    #[fhir("seriesDosesPositiveInt")]
    PositiveInt(crate::r4::choice::Primitive<types::PositiveInt>),
    /// `seriesDosesString` variant.
    #[fhir("seriesDosesString")]
    String(crate::r4::choice::Primitive<types::String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImmunizationEvaluation;

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
