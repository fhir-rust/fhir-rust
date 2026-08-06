//! MeasureReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
//!
//! Version: 4.0.1
//!
//! Results of a measure evaluation
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The MeasureReport resource contains the results of the calculation of a
/// measure; and optionally a reference to the resources involved in that
/// calculation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::measure_report::MeasureReport;
/// use fhir::r4::types;
///
/// let value = MeasureReport {
///     date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `date` is the name this serializes to on the wire.
/// assert_eq!(json["date"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MeasureReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MeasureReport {
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

    /// Additional identifier for the MeasureReport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// complete | pending | error
    pub status: crate::coded::Coded<crate::r4::codes::MeasureReportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// individual | subject-list | summary | data-collection
    pub r#type: crate::coded::Coded<crate::r4::codes::MeasureReportType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// What measure was calculated
    pub measure: types::Canonical,
    /// Primitive extension sibling for [`measure`](Self::measure) (FHIR `_measure`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measure")]
    pub measure_ext: Option<types::Element>,

    /// What individual(s) the report is for
    pub subject: Option<types::Reference>,

    /// When the report was generated
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is reporting the data
    pub reporter: Option<types::Reference>,

    /// What period the report covers
    pub period: types::Period,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Measure results for each group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<MeasureReportGroup>,

    /// What data was used to calculate the measure score
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evaluated_resource: Vec<types::Reference>,
}

/// The results of the calculation, one for each population group in the
/// measure.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::measure_report::MeasureReportGroup;
/// use fhir::r4::types;
///
/// let value = MeasureReportGroup {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MeasureReportGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Meaning of the group
    pub code: Option<types::CodeableConcept>,

    /// The populations in the group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureReportGroupPopulation>,

    /// What score this group achieved
    pub measure_score: Option<types::Quantity>,

    /// Stratification results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stratifier: Vec<MeasureReportGroupStratifier>,
}

/// The populations that make up the population group, one for each type of
/// population appropriate for the measure.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::measure_report::MeasureReportGroupPopulation;
/// use fhir::r4::types;
///
/// let value = MeasureReportGroupPopulation {
///     count: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `count` is the name this serializes to on the wire.
/// assert_eq!(json["count"], ::serde_json::json!(42));
///
/// let back: MeasureReportGroupPopulation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MeasureReportGroupPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// initial-population | numerator | numerator-exclusion | denominator |
    /// denominator-exclusion | denominator-exception | measure-population |
    /// measure-population-exclusion | measure-observation
    pub code: Option<types::CodeableConcept>,

    /// Size of the population
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// For subject-list reports, the subject results in this population
    pub subject_results: Option<types::Reference>,
}

/// When a measure includes multiple stratifiers, there will be a stratifier
/// group for each stratifier defined by the measure.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::measure_report::MeasureReportGroupStratifier;
/// use fhir::r4::types;
///
/// let value = MeasureReportGroupStratifier {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroupStratifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MeasureReportGroupStratifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What stratifier of the group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// Stratum results, one for each unique value, or set of values, in the
    /// stratifier, or stratifier components
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stratum: Vec<MeasureReportGroupStratifierStratum>,
}

/// This element contains the results for a single stratum within the
/// stratifier. For example, when stratifying on administrative gender, there
/// will be four strata, one for each possible gender value.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::measure_report::MeasureReportGroupStratifierStratum;
/// use fhir::r4::types;
///
/// let value = MeasureReportGroupStratifierStratum {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroupStratifierStratum = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MeasureReportGroupStratifierStratum {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The stratum value, e.g. male
    pub value: Option<types::CodeableConcept>,

    /// Stratifier component values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<MeasureReportGroupStratifierStratumComponent>,

    /// Population results in this stratum
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureReportGroupStratifierStratumPopulation>,

    /// What score this stratum achieved
    pub measure_score: Option<types::Quantity>,
}

/// A stratifier component value.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::measure_report::MeasureReportGroupStratifierStratumComponent;
/// use fhir::r4::types;
///
/// let value = MeasureReportGroupStratifierStratumComponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroupStratifierStratumComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MeasureReportGroupStratifierStratumComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What stratifier component of the group
    pub code: types::CodeableConcept,

    /// The stratum component value, e.g. male
    pub value: types::CodeableConcept,
}

/// The populations that make up the stratum, one for each type of population
/// appropriate to the measure.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::measure_report::MeasureReportGroupStratifierStratumPopulation;
/// use fhir::r4::types;
///
/// let value = MeasureReportGroupStratifierStratumPopulation {
///     count: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `count` is the name this serializes to on the wire.
/// assert_eq!(json["count"], ::serde_json::json!(42));
///
/// let back: MeasureReportGroupStratifierStratumPopulation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MeasureReportGroupStratifierStratumPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// initial-population | numerator | numerator-exclusion | denominator |
    /// denominator-exclusion | denominator-exception | measure-population |
    /// measure-population-exclusion | measure-observation
    pub code: Option<types::CodeableConcept>,

    /// Size of the population
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// For subject-list reports, the subject results in this population
    pub subject_results: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MeasureReport;

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
