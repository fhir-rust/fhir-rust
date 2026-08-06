//! MeasureReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
//!
//!
//!
//! Results of a measure evaluation
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MeasureReport Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure_report::MeasureReport;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MeasureReport {
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

    /// Additional identifier for the Report
    pub identifier: Option<types::Identifier>,

    /// complete | pending | error
    pub status: crate::coded::Coded<crate::r3::codes::MeasureReportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// individual | patient-list | summary
    pub r#type: crate::coded::Coded<crate::r3::codes::MeasureReportType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// What measure was evaluated
    pub measure: types::Reference,

    /// What patient the report is for
    pub patient: Option<types::Reference>,

    /// When the report was generated
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is reporting the data
    pub reporting_organization: Option<types::Reference>,

    /// What period the report covers
    pub period: types::Period,

    /// Measure results for each group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<MeasureReportGroup>,

    /// What data was evaluated to produce the measure score
    pub evaluated_resources: Option<types::Reference>,
}

/// The results of the calculation, one for each population group in the
/// measure.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure_report::MeasureReportGroup;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MeasureReportGroup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What group of the measure
    pub identifier: types::Identifier,

    /// The populations in the group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureReportGroupPopulation>,

    /// What score this group achieved
    pub measure_score: Option<types::Decimal>,
    /// Primitive extension sibling for [`measure_score`](Self::measure_score) (FHIR `_measureScore`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measureScore")]
    pub measure_score_ext: Option<types::Element>,

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
/// use fhir::r3::resources::measure_report::MeasureReportGroupPopulation;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MeasureReportGroupPopulation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Population identifier as defined in the measure
    pub identifier: Option<types::Identifier>,

    /// initial-population | numerator | numerator-exclusion | denominator |
    /// denominator-exclusion | denominator-exception | measure-population |
    /// measure-population-exclusion | measure-score
    pub code: Option<types::CodeableConcept>,

    /// Size of the population
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// For patient-list reports, the patients in this population
    pub patients: Option<types::Reference>,
}

/// When a measure includes multiple stratifiers, there will be a stratifier
/// group for each stratifier defined by the measure.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure_report::MeasureReportGroupStratifier;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MeasureReportGroupStratifier {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// What stratifier of the group
    pub identifier: Option<types::Identifier>,

    /// Stratum results, one for each unique value in the stratifier
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
/// use fhir::r3::resources::measure_report::MeasureReportGroupStratifierStratum;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MeasureReportGroupStratifierStratum {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The stratum value, e.g. male
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Population results in this stratum
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureReportGroupStratifierStratumPopulation>,

    /// What score this stratum achieved
    pub measure_score: Option<types::Decimal>,
    /// Primitive extension sibling for [`measure_score`](Self::measure_score) (FHIR `_measureScore`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measureScore")]
    pub measure_score_ext: Option<types::Element>,
}

/// The populations that make up the stratum, one for each type of population
/// appropriate to the measure.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::measure_report::MeasureReportGroupStratifierStratumPopulation;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MeasureReportGroupStratifierStratumPopulation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Population identifier as defined in the measure
    pub identifier: Option<types::Identifier>,

    /// initial-population | numerator | numerator-exclusion | denominator |
    /// denominator-exclusion | denominator-exception | measure-population |
    /// measure-population-exclusion | measure-score
    pub code: Option<types::CodeableConcept>,

    /// Size of the population
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// For patient-list reports, the patients in this population
    pub patients: Option<types::Reference>,
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
