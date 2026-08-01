//! MeasureReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MeasureReport
//!
//! Version: 6.0.0-ballot3
//!
//! Results of a measure evaluation
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The MeasureReport resource contains the results of the calculation of a
/// measure; and optionally a reference to the resources involved in that
/// calculation.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure_report::MeasureReport;
/// use fhir::r6::types;
///
/// let value = MeasureReport {
///     measure: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `measure` is the name this serializes to on the wire.
/// assert_eq!(json["measure"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: MeasureReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Additional identifier for the MeasureReport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The category of measure report instance this is (example codes include
    /// deqm, ra, vbp)
    pub category: Option<types::CodeableConcept>,

    /// Evaluation messages
    pub messages: Option<types::Reference>,

    /// complete | pending | error
    pub status: crate::coded::Coded<crate::r6::codes::MeasureReportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// individual | subject-list | summary | data-exchange
    pub r#type: crate::coded::Coded<crate::r6::codes::MeasureReportType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// incremental | snapshot
    pub data_update_type: Option<crate::coded::Coded<crate::r6::codes::SubmitDataUpdateType>>,
    /// Primitive extension sibling for [`data_update_type`](Self::data_update_type) (FHIR `_dataUpdateType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dataUpdateType")]
    pub data_update_type_ext: Option<types::Element>,

    /// What measure was calculated
    pub measure: Option<types::Canonical>,
    /// Primitive extension sibling for [`measure`](Self::measure) (FHIR `_measure`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_measure")]
    pub measure_ext: Option<types::Element>,

    /// What individual(s) the report is for
    pub subject: Option<types::Reference>,

    /// When the measure report was generated
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who is reporting the data
    pub reporter: Option<types::Reference>,

    /// What vendor prepared the data
    pub reporting_vendor: Option<types::Reference>,

    /// Where the reported data is from
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::Reference>,

    /// What period the report covers
    pub period: types::Period,

    /// What parameters were provided to the report
    pub input_parameters: Option<types::Reference>,

    /// What scoring method (e.g. proportion, ratio, continuous-variable)
    /// (deprecated, use group.scoring)
    pub scoring: Option<types::CodeableConcept>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Measure results for each group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<MeasureReportGroup>,

    /// Additional information collected for the report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplemental_data: Vec<types::Reference>,

    /// What data was used to calculate the measure score
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evaluated_resource: Vec<types::Reference>,
}

/// The results of the calculation, one for each population group in the
/// measure. A MeasureReport SHALL have a group element corresponding to each
/// group element defined in the Measure being reported.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure_report::MeasureReportGroup;
/// use fhir::r6::types;
///
/// let value = MeasureReportGroup {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MeasureReportGroup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pointer to specific group from Measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// The date the Measure Report was calculated
    pub calculated_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`calculated_date`](Self::calculated_date) (FHIR `_calculatedDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_calculatedDate")]
    pub calculated_date_ext: Option<types::Element>,

    /// Meaning of the group
    pub code: Option<types::CodeableConcept>,

    /// Summary description
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// What individual(s) the report is for
    pub subject: Option<types::Reference>,

    /// What scoring method (e.g. proportion, ratio, continuous-variable)
    pub scoring: Option<types::CodeableConcept>,

    /// increase | decrease
    pub improvement_notation: Option<types::CodeableConcept>,

    /// Explanation of improvement notation
    pub improvement_notation_guidance: Option<types::Markdown>,
    /// Primitive extension sibling for [`improvement_notation_guidance`](Self::improvement_notation_guidance) (FHIR `_improvementNotationGuidance`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_improvementNotationGuidance")]
    pub improvement_notation_guidance_ext: Option<types::Element>,

    /// The populations in the group
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureReportGroupPopulation>,

    /// What score this group achieved
    /// The `MeasureReport.group.measureScore[x]` choice element (0..1); see [`MeasureReportGroupMeasureScore`].
    #[serde(flatten)]
    pub measure_score: Option<MeasureReportGroupMeasureScore>,

    /// Stratification results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stratifier: Vec<MeasureReportGroupStratifier>,
}

/// The populations that make up the population group, one for each type of
/// population appropriate for the measure. Each group in the MeasureReport
/// SHALL have populations as defined in the corresponding group of the Measure
/// being reported.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure_report::MeasureReportGroupPopulation;
/// use fhir::r6::types;
///
/// let value = MeasureReportGroupPopulation {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroupPopulation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MeasureReportGroupPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pointer to specific population from Measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// initial-population | numerator | numerator-exclusion | denominator |
    /// denominator-exclusion | denominator-exception | measure-population |
    /// measure-population-exclusion | measure-observation
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this population criteria
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Size of the population
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// Size of the population as a quantity
    pub count_quantity: Option<types::Quantity>,

    /// For subject-list reports, the subject results in this population
    pub subject_results: Option<types::Reference>,

    /// For subject-list reports, references to the individual reports for
    /// subjects in this population
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_report: Vec<types::Reference>,

    /// What individual(s) in the population
    pub subjects: Option<types::Reference>,
}

/// The stratification results for this measure group, calculated as defined by
/// the stratifier element of the measure being reported. Each group in the
/// MeasureReport SHALL have stratifiers as defined in the corresponding group
/// of the Measure being reported.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure_report::MeasureReportGroupStratifier;
/// use fhir::r6::types;
///
/// let value = MeasureReportGroupStratifier {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroupStratifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MeasureReportGroupStratifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pointer to specific stratifier from Measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// What stratifier of the group
    pub code: Option<types::CodeableConcept>,

    /// The human readable description of this stratifier
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

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
/// use fhir::r6::resources::measure_report::MeasureReportGroupStratifierStratum;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// The `MeasureReport.group.stratifier.stratum.value[x]` choice element (0..1); see [`MeasureReportGroupStratifierStratumValue`].
    #[serde(flatten)]
    pub value: Option<MeasureReportGroupStratifierStratumValue>,

    /// Stratifier component values
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<MeasureReportGroupStratifierStratumComponent>,

    /// Population results in this stratum
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<MeasureReportGroupStratifierStratumPopulation>,

    /// What score this stratum achieved
    /// The `MeasureReport.group.stratifier.stratum.measureScore[x]` choice element (0..1); see [`MeasureReportGroupStratifierStratumMeasureScore`].
    #[serde(flatten)]
    pub measure_score: Option<MeasureReportGroupStratifierStratumMeasureScore>,
}

/// A stratifier component value.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure_report::MeasureReportGroupStratifierStratumComponent;
/// use fhir::r6::types;
///
/// let value = MeasureReportGroupStratifierStratumComponent {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroupStratifierStratumComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MeasureReportGroupStratifierStratumComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pointer to specific stratifier component from Measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

    /// What stratifier component of the group
    pub code: types::CodeableConcept,

    /// The human readable description of this stratifier component
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The stratum component value, e.g. male
    /// The `MeasureReport.group.stratifier.stratum.component.value[x]` choice element (1..1); see [`MeasureReportGroupStratifierStratumComponentValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<MeasureReportGroupStratifierStratumComponentValue>,
}

/// The populations that make up the stratum, one for each type of population
/// appropriate to the measure. For each stratifier, systems MAY provide
/// population breakdowns in addition to the stratified scores.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::measure_report::MeasureReportGroupStratifierStratumPopulation;
/// use fhir::r6::types;
///
/// let value = MeasureReportGroupStratifierStratumPopulation {
///     link_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `linkId` is the name this serializes to on the wire.
/// assert_eq!(json["linkId"], ::serde_json::json!("abc"));
///
/// let back: MeasureReportGroupStratifierStratumPopulation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MeasureReportGroupStratifierStratumPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Pointer to specific population from Measure
    pub link_id: Option<types::String>,
    /// Primitive extension sibling for [`link_id`](Self::link_id) (FHIR `_linkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_linkId")]
    pub link_id_ext: Option<types::Element>,

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

    /// Size of the population as a quantity
    pub count_quantity: Option<types::Quantity>,

    /// For subject-list reports, the subject results in this population
    pub subject_results: Option<types::Reference>,

    /// For subject-list reports, a subject result in this population
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject_report: Vec<types::Reference>,

    /// What individual(s) in the population
    pub subjects: Option<types::Reference>,
}

/// The `MeasureReport.group.measureScore[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MeasureReportGroupMeasureScore {
    /// `measureScoreQuantity` variant.
    #[fhir("measureScoreQuantity")]
    Quantity(Box<types::Quantity>),
    /// `measureScoreDateTime` variant.
    #[fhir("measureScoreDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `measureScoreCodeableConcept` variant.
    #[fhir("measureScoreCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `measureScorePeriod` variant.
    #[fhir("measureScorePeriod")]
    Period(Box<types::Period>),
    /// `measureScoreRange` variant.
    #[fhir("measureScoreRange")]
    Range(Box<types::Range>),
    /// `measureScoreDuration` variant.
    #[fhir("measureScoreDuration")]
    Duration(Box<types::Duration>),
    /// `measureScoreBoolean` variant.
    #[fhir("measureScoreBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
}

/// The `MeasureReport.group.stratifier.stratum.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MeasureReportGroupStratifierStratumValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}

/// The `MeasureReport.group.stratifier.stratum.measureScore[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MeasureReportGroupStratifierStratumMeasureScore {
    /// `measureScoreQuantity` variant.
    #[fhir("measureScoreQuantity")]
    Quantity(Box<types::Quantity>),
    /// `measureScoreDateTime` variant.
    #[fhir("measureScoreDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `measureScoreCodeableConcept` variant.
    #[fhir("measureScoreCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `measureScorePeriod` variant.
    #[fhir("measureScorePeriod")]
    Period(Box<types::Period>),
    /// `measureScoreRange` variant.
    #[fhir("measureScoreRange")]
    Range(Box<types::Range>),
    /// `measureScoreDuration` variant.
    #[fhir("measureScoreDuration")]
    Duration(Box<types::Duration>),
    /// `measureScoreBoolean` variant.
    #[fhir("measureScoreBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
}

/// The `MeasureReport.group.stratifier.stratum.component.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MeasureReportGroupStratifierStratumComponentValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
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
