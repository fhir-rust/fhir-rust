//! MolecularSequence
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
//!
//! Version: 4.0.1
//!
//! Information about a biological sequence
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Raw data describing a biological sequence.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequence;
/// use fhir::r4::types;
///
/// let value = MolecularSequence {
///     observed_seq: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `observedSeq` is the name this serializes to on the wire.
/// assert_eq!(json["observedSeq"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequence {
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

    /// Unique ID for this particular sequence. This is a FHIR-defined id
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// aa | dna | rna
    pub r#type: Option<crate::coded::Coded<crate::r4::codes::SequenceType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Base number of coordinate system (0 for 0-based numbering or
    /// coordinates, inclusive start, exclusive end, 1 for 1-based numbering,
    /// inclusive start, inclusive end)
    pub coordinate_system: types::Integer,
    /// Primitive extension sibling for [`coordinate_system`](Self::coordinate_system) (FHIR `_coordinateSystem`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_coordinateSystem")]
    pub coordinate_system_ext: Option<types::Element>,

    /// Who and/or what this is about
    pub patient: Option<types::Reference<crate::r4::resources::Patient>>,

    /// Specimen used for sequencing
    pub specimen: Option<types::Reference<crate::r4::resources::Specimen>>,

    /// The method for sequencing
    pub device: Option<types::Reference<crate::r4::resources::Device>>,

    /// Who should be responsible for test result
    pub performer: Option<types::Reference<crate::r4::resources::Organization>>,

    /// The number of copies of the sequence of interest. (RNASeq)
    pub quantity: Option<types::Quantity>,

    /// A sequence used as reference
    pub reference_seq: Option<MolecularSequenceReferenceSeq>,

    /// Variant in sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variant: Vec<MolecularSequenceVariant>,

    /// Sequence that was observed
    pub observed_seq: Option<types::String>,
    /// Primitive extension sibling for [`observed_seq`](Self::observed_seq) (FHIR `_observedSeq`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_observedSeq")]
    pub observed_seq_ext: Option<types::Element>,

    /// An set of value as quality of sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quality: Vec<MolecularSequenceQuality>,

    /// Average number of reads representing a given nucleotide in the
    /// reconstructed sequence
    pub read_coverage: Option<types::Integer>,
    /// Primitive extension sibling for [`read_coverage`](Self::read_coverage) (FHIR `_readCoverage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_readCoverage")]
    pub read_coverage_ext: Option<types::Element>,

    /// External repository which contains detailed report related with
    /// observedSeq in this resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repository: Vec<MolecularSequenceRepository>,

    /// Pointer to next atomic sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pointer: Vec<types::Reference<crate::r4::resources::MolecularSequence>>,

    /// Structural variant
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structure_variant: Vec<MolecularSequenceStructureVariant>,
}

/// An experimental feature attribute that defines the quality of the feature
/// in a quantitative way, such as a phred quality score
/// ([SO:0001686](http://www.sequenceontology.org/browser/current_svn/term/SO:0001686)).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceQuality;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceQuality {
///     start: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `start` is the name this serializes to on the wire.
/// assert_eq!(json["start"], ::serde_json::json!(42));
///
/// let back: MolecularSequenceQuality = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceQuality {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// indel | snp | unknown
    pub r#type: crate::coded::Coded<crate::r4::codes::QualityType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Standard sequence for comparison
    pub standard_sequence: Option<types::CodeableConcept>,

    /// Start position of the sequence
    pub start: Option<types::Integer>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// End position of the sequence
    pub end: Option<types::Integer>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Quality score for the comparison
    pub score: Option<types::Quantity>,

    /// Method to get quality
    pub method: Option<types::CodeableConcept>,

    /// True positives from the perspective of the truth data
    #[serde(rename = "truthTP")]
    pub truth_tp: Option<types::Decimal>,
    /// Primitive extension sibling for [`truth_tp`](Self::truth_tp) (FHIR `_truthTP`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_truthTP")]
    pub truth_tp_ext: Option<types::Element>,

    /// True positives from the perspective of the query data
    #[serde(rename = "queryTP")]
    pub query_tp: Option<types::Decimal>,
    /// Primitive extension sibling for [`query_tp`](Self::query_tp) (FHIR `_queryTP`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_queryTP")]
    pub query_tp_ext: Option<types::Element>,

    /// False negatives
    #[serde(rename = "truthFN")]
    pub truth_fn: Option<types::Decimal>,
    /// Primitive extension sibling for [`truth_fn`](Self::truth_fn) (FHIR `_truthFN`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_truthFN")]
    pub truth_fn_ext: Option<types::Element>,

    /// False positives
    #[serde(rename = "queryFP")]
    pub query_fp: Option<types::Decimal>,
    /// Primitive extension sibling for [`query_fp`](Self::query_fp) (FHIR `_queryFP`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_queryFP")]
    pub query_fp_ext: Option<types::Element>,

    /// False positives where the non-REF alleles in the Truth and Query Call
    /// Sets match
    #[serde(rename = "gtFP")]
    pub gt_fp: Option<types::Decimal>,
    /// Primitive extension sibling for [`gt_fp`](Self::gt_fp) (FHIR `_gtFP`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_gtFP")]
    pub gt_fp_ext: Option<types::Element>,

    /// Precision of comparison
    pub precision: Option<types::Decimal>,
    /// Primitive extension sibling for [`precision`](Self::precision) (FHIR `_precision`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precision")]
    pub precision_ext: Option<types::Element>,

    /// Recall of comparison
    pub recall: Option<types::Decimal>,
    /// Primitive extension sibling for [`recall`](Self::recall) (FHIR `_recall`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recall")]
    pub recall_ext: Option<types::Element>,

    /// F-score
    pub f_score: Option<types::Decimal>,
    /// Primitive extension sibling for [`f_score`](Self::f_score) (FHIR `_fScore`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fScore")]
    pub f_score_ext: Option<types::Element>,

    /// Receiver Operator Characteristic (ROC) Curve
    pub roc: Option<MolecularSequenceQualityRoc>,
}

/// Receiver Operator Characteristic (ROC) Curve to give
/// sensitivity/specificity tradeoff.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceQualityRoc;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceQualityRoc {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceQualityRoc = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceQualityRoc {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Genotype quality score
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub score: ::fhir_core::PrimVec<types::Integer>,
    /// Primitive extension sibling for [`score`](Self::score) (FHIR `_score`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_score")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub score_ext: Vec<Option<types::Element>>,

    /// Roc score true positive numbers
    #[serde(rename = "numTP")]
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub num_tp: ::fhir_core::PrimVec<types::Integer>,
    /// Primitive extension sibling for [`num_tp`](Self::num_tp) (FHIR `_numTP`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numTP")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub num_tp_ext: Vec<Option<types::Element>>,

    /// Roc score false positive numbers
    #[serde(rename = "numFP")]
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub num_fp: ::fhir_core::PrimVec<types::Integer>,
    /// Primitive extension sibling for [`num_fp`](Self::num_fp) (FHIR `_numFP`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numFP")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub num_fp_ext: Vec<Option<types::Element>>,

    /// Roc score false negative numbers
    #[serde(rename = "numFN")]
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub num_fn: ::fhir_core::PrimVec<types::Integer>,
    /// Primitive extension sibling for [`num_fn`](Self::num_fn) (FHIR `_numFN`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numFN")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub num_fn_ext: Vec<Option<types::Element>>,

    /// Precision of the GQ score
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub precision: ::fhir_core::PrimVec<types::Decimal>,
    /// Primitive extension sibling for [`precision`](Self::precision) (FHIR `_precision`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_precision")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub precision_ext: Vec<Option<types::Element>>,

    /// Sensitivity of the GQ score
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub sensitivity: ::fhir_core::PrimVec<types::Decimal>,
    /// Primitive extension sibling for [`sensitivity`](Self::sensitivity) (FHIR `_sensitivity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sensitivity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sensitivity_ext: Vec<Option<types::Element>>,

    /// FScore of the GQ score
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub f_measure: ::fhir_core::PrimVec<types::Decimal>,
    /// Primitive extension sibling for [`f_measure`](Self::f_measure) (FHIR `_fMeasure`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fMeasure")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub f_measure_ext: Vec<Option<types::Element>>,
}

/// A sequence that is used as a reference to describe variants that are
/// present in a sequence analyzed.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceReferenceSeq;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceReferenceSeq {
///     genome_build: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `genomeBuild` is the name this serializes to on the wire.
/// assert_eq!(json["genomeBuild"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceReferenceSeq = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceReferenceSeq {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Chromosome containing genetic finding
    pub chromosome: Option<types::CodeableConcept>,

    /// The Genome Build used for reference, following GRCh build versions e.g.
    /// 'GRCh 37'
    pub genome_build: Option<types::String>,
    /// Primitive extension sibling for [`genome_build`](Self::genome_build) (FHIR `_genomeBuild`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_genomeBuild")]
    pub genome_build_ext: Option<types::Element>,

    /// sense | antisense
    pub orientation: Option<crate::coded::Coded<crate::r4::codes::OrientationType>>,
    /// Primitive extension sibling for [`orientation`](Self::orientation) (FHIR `_orientation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_orientation")]
    pub orientation_ext: Option<types::Element>,

    /// Reference identifier
    pub reference_seq_id: Option<types::CodeableConcept>,

    /// A pointer to another MolecularSequence entity as reference sequence
    pub reference_seq_pointer: Option<types::Reference<crate::r4::resources::MolecularSequence>>,

    /// A string to represent reference sequence
    pub reference_seq_string: Option<types::String>,
    /// Primitive extension sibling for [`reference_seq_string`](Self::reference_seq_string) (FHIR `_referenceSeqString`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_referenceSeqString")]
    pub reference_seq_string_ext: Option<types::Element>,

    /// watson | crick
    pub strand: Option<crate::coded::Coded<crate::r4::codes::StrandType>>,
    /// Primitive extension sibling for [`strand`](Self::strand) (FHIR `_strand`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_strand")]
    pub strand_ext: Option<types::Element>,

    /// Start position of the window on the reference sequence
    pub window_start: Option<types::Integer>,
    /// Primitive extension sibling for [`window_start`](Self::window_start) (FHIR `_windowStart`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_windowStart")]
    pub window_start_ext: Option<types::Element>,

    /// End position of the window on the reference sequence
    pub window_end: Option<types::Integer>,
    /// Primitive extension sibling for [`window_end`](Self::window_end) (FHIR `_windowEnd`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_windowEnd")]
    pub window_end_ext: Option<types::Element>,
}

/// Configurations of the external repository. The repository shall store
/// target's observedSeq or records related with target's observedSeq.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceRepository;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceRepository {
///     dataset_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `datasetId` is the name this serializes to on the wire.
/// assert_eq!(json["datasetId"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceRepository = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceRepository {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// directlink | openapi | login | oauth | other
    pub r#type: crate::coded::Coded<crate::r4::codes::RepositoryType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// URI of the repository
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Repository's name
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Id of the dataset that used to call for dataset in repository
    pub dataset_id: Option<types::String>,
    /// Primitive extension sibling for [`dataset_id`](Self::dataset_id) (FHIR `_datasetId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_datasetId")]
    pub dataset_id_ext: Option<types::Element>,

    /// Id of the variantset that used to call for variantset in repository
    pub variantset_id: Option<types::String>,
    /// Primitive extension sibling for [`variantset_id`](Self::variantset_id) (FHIR `_variantsetId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_variantsetId")]
    pub variantset_id_ext: Option<types::Element>,

    /// Id of the read
    pub readset_id: Option<types::String>,
    /// Primitive extension sibling for [`readset_id`](Self::readset_id) (FHIR `_readsetId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_readsetId")]
    pub readset_id_ext: Option<types::Element>,
}

/// Information about chromosome structure variation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceStructureVariant;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceStructureVariant {
///     exact: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `exact` is the name this serializes to on the wire.
/// assert_eq!(json["exact"], ::serde_json::json!(true));
///
/// let back: MolecularSequenceStructureVariant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceStructureVariant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Structural variant change type
    pub variant_type: Option<types::CodeableConcept>,

    /// Does the structural variant have base pair resolution breakpoints?
    pub exact: Option<types::Boolean>,
    /// Primitive extension sibling for [`exact`](Self::exact) (FHIR `_exact`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exact")]
    pub exact_ext: Option<types::Element>,

    /// Structural variant length
    pub length: Option<types::Integer>,
    /// Primitive extension sibling for [`length`](Self::length) (FHIR `_length`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_length")]
    pub length_ext: Option<types::Element>,

    /// Structural variant outer
    pub outer: Option<MolecularSequenceStructureVariantOuter>,

    /// Structural variant inner
    pub inner: Option<MolecularSequenceStructureVariantInner>,
}

/// Structural variant inner.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceStructureVariantInner;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceStructureVariantInner {
///     start: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `start` is the name this serializes to on the wire.
/// assert_eq!(json["start"], ::serde_json::json!(42));
///
/// let back: MolecularSequenceStructureVariantInner = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceStructureVariantInner {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Structural variant inner start
    pub start: Option<types::Integer>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// Structural variant inner end
    pub end: Option<types::Integer>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,
}

/// Structural variant outer.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceStructureVariantOuter;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceStructureVariantOuter {
///     start: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `start` is the name this serializes to on the wire.
/// assert_eq!(json["start"], ::serde_json::json!(42));
///
/// let back: MolecularSequenceStructureVariantOuter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceStructureVariantOuter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Structural variant outer start
    pub start: Option<types::Integer>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// Structural variant outer end
    pub end: Option<types::Integer>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,
}

/// The definition of variant here originates from Sequence ontology
/// ([variant_of](http://www.sequenceontology.org/browser/current_svn/term/variant_of)).
/// This element can represent amino acid or nucleic sequence change(including
/// insertion,deletion,SNP,etc.) It can represent some complex mutation or
/// segment variation with the assist of CIGAR string.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::molecular_sequence::MolecularSequenceVariant;
/// use fhir::r4::types;
///
/// let value = MolecularSequenceVariant {
///     observed_allele: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `observedAllele` is the name this serializes to on the wire.
/// assert_eq!(json["observedAllele"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceVariant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MolecularSequenceVariant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Start position of the variant on the reference sequence
    pub start: Option<types::Integer>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// End position of the variant on the reference sequence
    pub end: Option<types::Integer>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Allele that was observed
    pub observed_allele: Option<types::String>,
    /// Primitive extension sibling for [`observed_allele`](Self::observed_allele) (FHIR `_observedAllele`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_observedAllele")]
    pub observed_allele_ext: Option<types::Element>,

    /// Allele in the reference sequence
    pub reference_allele: Option<types::String>,
    /// Primitive extension sibling for [`reference_allele`](Self::reference_allele) (FHIR `_referenceAllele`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_referenceAllele")]
    pub reference_allele_ext: Option<types::Element>,

    /// Extended CIGAR string for aligning the sequence with reference bases
    pub cigar: Option<types::String>,
    /// Primitive extension sibling for [`cigar`](Self::cigar) (FHIR `_cigar`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cigar")]
    pub cigar_ext: Option<types::Element>,

    /// Pointer to observed variant information
    pub variant_pointer: Option<types::Reference<crate::r4::resources::Observation>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MolecularSequence;

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
