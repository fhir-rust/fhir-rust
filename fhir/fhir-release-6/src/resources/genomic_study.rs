//! GenomicStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/GenomicStudy
//!
//! Version: 6.0.0-ballot3
//!
//! Genomic Study
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A set of analyses performed to analyze and generate genomic data.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::genomic_study::GenomicStudy;
/// use fhir::r6::types;
///
/// let value = GenomicStudy {
///     start_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `startDate` is the name this serializes to on the wire.
/// assert_eq!(json["startDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: GenomicStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GenomicStudy {
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

    /// Identifiers for this genomic study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// registered | available | cancelled | entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::GenomicstudyStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The type of the study (e.g., Familial variant segregation, Functional
    /// variation detection, or Gene expression profiling)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The primary subject of the genomic study
    pub subject: types::Reference,

    /// The healthcare event with which this genomics study is associated
    pub encounter: Option<types::Reference>,

    /// When the genomic study was started
    pub start_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`start_date`](Self::start_date) (FHIR `_startDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_startDate")]
    pub start_date_ext: Option<types::Element>,

    /// Event resources that the genomic study is based on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Healthcare professional who requested or referred the genomic study
    pub referrer: Option<types::Reference>,

    /// Healthcare professionals who interpreted the genomic study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interpreter: Vec<types::Reference>,

    /// Why the genomic study was performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// The defined protocol that describes the study
    pub instantiates_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    pub instantiates_canonical_ext: Option<types::Element>,

    /// The URL pointing to an externally maintained protocol that describes
    /// the study
    pub instantiates_uri: Option<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<types::Element>,

    /// Comments related to the genomic study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Description of the genomic study
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Genomic Analysis Event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub analysis: Vec<GenomicStudyAnalysis>,
}

/// The details about a specific analysis that was performed in this
/// GenomicStudy.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::genomic_study::GenomicStudyAnalysis;
/// use fhir::r6::types;
///
/// let value = GenomicStudyAnalysis {
///     instantiates_canonical: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instantiatesCanonical` is the name this serializes to on the wire.
/// assert_eq!(json["instantiatesCanonical"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: GenomicStudyAnalysis = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GenomicStudyAnalysis {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifiers for the analysis event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Type of the methods used in the analysis (e.g., FISH, Karyotyping, MSI)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub method_type: Vec<types::CodeableConcept>,

    /// Type of the genomic changes studied in the analysis (e.g., DNA, RNA, or
    /// AA change)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub change_type: Vec<types::CodeableConcept>,

    /// Genome build that is used in this analysis
    pub genome_build: Option<types::CodeableConcept>,

    /// The defined protocol that describes the analysis
    pub instantiates_canonical: Option<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    pub instantiates_canonical_ext: Option<types::Element>,

    /// The URL pointing to an externally maintained protocol that describes
    /// the analysis
    pub instantiates_uri: Option<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    pub instantiates_uri_ext: Option<types::Element>,

    /// Name of the analysis event (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What the genomic analysis is about, when it is not about the subject of
    /// record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference>,

    /// The specimen used in the analysis event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference>,

    /// The date of the analysis event
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Any notes capture with the analysis event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// The protocol that was performed for the analysis event
    pub protocol_performed: Option<types::Reference>,

    /// The genomic regions to be studied in the analysis (BED file)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions_studied: Vec<types::Reference>,

    /// Genomic regions actually called in the analysis event (BED file)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions_called: Vec<types::Reference>,

    /// Inputs for the analysis event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<GenomicStudyAnalysisInput>,

    /// Outputs for the analysis event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output: Vec<GenomicStudyAnalysisOutput>,

    /// Performer for the analysis event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<GenomicStudyAnalysisPerformer>,

    /// Devices used for the analysis (e.g., instruments, software), with
    /// settings and parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<GenomicStudyAnalysisDevice>,
}

/// Devices used for the analysis (e.g., instruments, software), with settings
/// and parameters.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::genomic_study::GenomicStudyAnalysisDevice;
/// use fhir::r6::types;
///
/// let value = GenomicStudyAnalysisDevice {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: GenomicStudyAnalysisDevice = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GenomicStudyAnalysisDevice {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Device used for the analysis
    pub device: Option<types::Reference>,

    /// Specific function for the device used for the analysis
    pub function: Option<types::CodeableConcept>,
}

/// Inputs for the analysis event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::genomic_study::GenomicStudyAnalysisInput;
/// use fhir::r6::types;
///
/// let value = GenomicStudyAnalysisInput {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: GenomicStudyAnalysisInput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GenomicStudyAnalysisInput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// File containing input data
    pub file: Option<types::Reference>,

    /// Type of input data (e.g., BAM, CRAM, or FASTA)
    pub r#type: Option<types::CodeableConcept>,

    /// The analysis event or other GenomicStudy that generated this input file
    /// The `GenomicStudy.analysis.input.generatedBy[x]` choice element (0..1); see [`GenomicStudyAnalysisInputGeneratedBy`].
    #[serde(flatten)]
    pub generated_by: Option<GenomicStudyAnalysisInputGeneratedBy>,
}

/// Outputs for the analysis event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::genomic_study::GenomicStudyAnalysisOutput;
/// use fhir::r6::types;
///
/// let value = GenomicStudyAnalysisOutput {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: GenomicStudyAnalysisOutput = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GenomicStudyAnalysisOutput {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// File containing output data
    pub file: Option<types::Reference>,

    /// Type of output data (e.g., VCF, MAF, or BAM)
    pub r#type: Option<types::CodeableConcept>,
}

/// Performer for the analysis event.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::genomic_study::GenomicStudyAnalysisPerformer;
/// use fhir::r6::types;
///
/// let value = GenomicStudyAnalysisPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: GenomicStudyAnalysisPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct GenomicStudyAnalysisPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The organization, healthcare professional, or others who participated
    /// in performing this analysis
    pub actor: Option<types::Reference>,

    /// Role of the actor for this analysis
    pub role: Option<types::CodeableConcept>,
}

/// The `GenomicStudy.analysis.input.generatedBy[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum GenomicStudyAnalysisInputGeneratedBy {
    /// `generatedByIdentifier` variant.
    #[fhir("generatedByIdentifier")]
    Identifier(Box<types::Identifier>),
    /// `generatedByReference` variant.
    #[fhir("generatedByReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = GenomicStudy;

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
