//! MolecularDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MolecularDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! Definitional content for a molecular entity
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Definitional content for a molecular entity, such as a nucleotide or
/// protein sequence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinition;
/// use fhir::r6::types;
///
/// let value = MolecularDefinition {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: MolecularDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinition {
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

    /// Unique ID of an instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Description of the Molecular Definition instance
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The type of molecule (e.g., DNA, RNA, amino acid)
    pub molecule_type: Option<types::CodeableConcept>,

    /// Classification of the molecule into types other than those defined by
    /// moleculeType
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The structural topology of the molecular entity (e.g., linear,
    /// circular)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology: Vec<types::CodeableConcept>,

    /// Constituents of an aggregate molecular concept (e.g., haplotype,
    /// genotype)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<types::Reference>,

    /// A defined location on a molecular entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<MolecularDefinitionLocation>,

    /// A representation of a molecular entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub representation: Vec<MolecularDefinitionRepresentation>,
}

/// A defined location on a molecular entity. Location definitions may vary
/// with respect to coordinate space and precision or level of granularity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocation;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A coordinate-based location on a sequence
    pub sequence_location: Option<MolecularDefinitionLocationSequenceLocation>,

    /// A cytoband-based location on a sequence
    pub cytoband_location: Option<MolecularDefinitionLocationCytobandLocation>,
}

/// A location on a sequence, defined using cytobands.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationCytobandLocation;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationCytobandLocation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationCytobandLocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationCytobandLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Reference Genome
    pub genome_assembly: MolecularDefinitionLocationCytobandLocationGenomeAssembly,

    /// Cytoband Interval
    pub cytoband_interval: MolecularDefinitionLocationCytobandLocationCytobandInterval,
}

/// The Cytoband Interval.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationCytobandLocationCytobandInterval;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationCytobandLocationCytobandInterval {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationCytobandLocationCytobandInterval = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationCytobandLocationCytobandInterval {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Chromosome
    pub chromosome: types::CodeableConcept,

    /// Start
    pub start_cytoband:
        Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytoband>,

    /// End
    pub end_cytoband:
        Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytoband>,
}

/// The end of this cytoband Interval.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytoband;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytoband {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytoband = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytoband {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Arm
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.arm[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandArm`].
    #[serde(flatten)]
    pub arm: Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandArm>,

    /// Region
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.region[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandRegion`].
    #[serde(flatten)]
    pub region:
        Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandRegion>,

    /// Band
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.band[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandBand`].
    #[serde(flatten)]
    pub band: Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandBand>,

    /// SuBand
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.subBand[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandSubBand`].
    #[serde(flatten)]
    pub sub_band:
        Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandSubBand>,
}

/// The start of this cytoband Interval.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytoband;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytoband {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytoband = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytoband {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Arm
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.arm[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandArm`].
    #[serde(flatten)]
    pub arm: Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandArm>,

    /// Region
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.region[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandRegion`].
    #[serde(flatten)]
    pub region:
        Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandRegion>,

    /// Band
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.band[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandBand`].
    #[serde(flatten)]
    pub band: Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandBand>,

    /// Sub-band
    /// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.subBand[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandSubBand`].
    #[serde(flatten)]
    pub sub_band:
        Option<MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandSubBand>,
}

/// The reference genome assemble.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationCytobandLocationGenomeAssembly;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationCytobandLocationGenomeAssembly {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationCytobandLocationGenomeAssembly = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationCytobandLocationGenomeAssembly {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Species of the organism
    pub organism: Option<types::CodeableConcept>,

    /// Build number
    pub build: Option<types::CodeableConcept>,

    /// Accession
    pub accession: Option<types::CodeableConcept>,

    /// Genome assembly description
    /// The `MolecularDefinition.location.cytobandLocation.genomeAssembly.description[x]` choice element (0..1); see [`MolecularDefinitionLocationCytobandLocationGenomeAssemblyDescription`].
    #[serde(flatten)]
    pub description: Option<MolecularDefinitionLocationCytobandLocationGenomeAssemblyDescription>,
}

/// A location on a sequence, defined using a nucleotide coordinate system.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationSequenceLocation;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationSequenceLocation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationSequenceLocation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationSequenceLocation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The sequence on which the location is defined
    pub sequence_context: types::Reference,

    /// An interval on a sequence
    pub coordinate_interval: Option<MolecularDefinitionLocationSequenceLocationCoordinateInterval>,

    /// The strand at the coordinateInterval
    pub strand: Option<types::CodeableConcept>,
}

/// An interval on a sequence, defined by coordinate-based start and end
/// coordinates.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationSequenceLocationCoordinateInterval;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationSequenceLocationCoordinateInterval {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationSequenceLocationCoordinateInterval = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationSequenceLocationCoordinateInterval {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The coordinate system used to define the location
    pub coordinate_system:
        Option<MolecularDefinitionLocationSequenceLocationCoordinateIntervalCoordinateSystem>,

    /// The start location of the interval
    /// The `MolecularDefinition.location.sequenceLocation.coordinateInterval.start[x]` choice element (0..1); see [`MolecularDefinitionLocationSequenceLocationCoordinateIntervalStart`].
    #[serde(flatten)]
    pub start: Option<MolecularDefinitionLocationSequenceLocationCoordinateIntervalStart>,

    /// The end location of the interval
    /// The `MolecularDefinition.location.sequenceLocation.coordinateInterval.end[x]` choice element (0..1); see [`MolecularDefinitionLocationSequenceLocationCoordinateIntervalEnd`].
    #[serde(flatten)]
    pub end: Option<MolecularDefinitionLocationSequenceLocationCoordinateIntervalEnd>,
}

/// A definition of the coordinate system. Examples include 1-based character
/// counting, and 0-based interval counting.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionLocationSequenceLocationCoordinateIntervalCoordinateSystem;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionLocationSequenceLocationCoordinateIntervalCoordinateSystem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionLocationSequenceLocationCoordinateIntervalCoordinateSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionLocationSequenceLocationCoordinateIntervalCoordinateSystem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of coordinate system used
    pub system: Option<types::CodeableConcept>,

    /// The location of the origin of the coordinate system
    pub origin: Option<types::CodeableConcept>,

    /// The normalization method used for determining a location within the
    /// coordinate system
    pub normalization_method: Option<types::CodeableConcept>,
}

/// A representation of a molecular entity, specifically including sequence.
/// Note this element is intended to define the entity primarily through
/// computable, discrete elements that express domain semantics rather than
/// replicating a particular file format or relational schema.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentation;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The domain concept that is the focus of a given instance of the
    /// representation
    pub focus: Option<types::CodeableConcept>,

    /// A code (e.g., sequence accession number) used to represent a molecular
    /// entity
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// A molecular entity defined as a string literal
    pub literal: Option<MolecularDefinitionRepresentationLiteral>,

    /// A resolvable representation of a molecular entity (e.g., URI, attached
    /// and formatted file)
    pub resolvable: Option<types::Reference>,

    /// A molecular entity that is represented as a portion of a different
    /// entity
    pub extracted: Option<MolecularDefinitionRepresentationExtracted>,

    /// A representation as a repeated motif
    pub repeated: Option<MolecularDefinitionRepresentationRepeated>,

    /// An ordered concatenation of molecular entities
    pub concatenated: Option<MolecularDefinitionRepresentationConcatenated>,

    /// A molecular entity represented as an ordered series of edits on a
    /// specified starting entity
    pub relative: Option<MolecularDefinitionRepresentationRelative>,
}

/// A representation comprised of an ordered concatenation of two or more
/// molecular entities.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationConcatenated;
///
/// let value = MolecularDefinitionRepresentationConcatenated::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MolecularDefinitionRepresentationConcatenated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationConcatenated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// One of the concatenated entities
    pub sequence_element:
        ::vec1::Vec1<MolecularDefinitionRepresentationConcatenatedSequenceElement>,
}

/// One of the concatenated entities within the concatenated representation.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationConcatenatedSequenceElement;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationConcatenatedSequenceElement {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationConcatenatedSequenceElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationConcatenatedSequenceElement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A reference to the sequence that defines this specific concatenated
    /// element
    pub sequence: types::Reference,

    /// The ordinal index of the element within the concatenated representation
    pub ordinal_index: types::Integer,
    /// Primitive extension sibling for [`ordinal_index`](Self::ordinal_index) (FHIR `_ordinalIndex`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ordinalIndex")]
    pub ordinal_index_ext: Option<types::Element>,
}

/// A molecular entity that is represented as a portion of a different entity.
/// For example, this element can represent a subsequence (e.g., genetic
/// region) that is part of and conceptually extracted from a longer sequence
/// (e.g., chromosome sequence). The “parent” entity is specified in
/// startingMolecule and the location of the intended molecular entity on the
/// parent entity is defined by coordinateInterval.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationExtracted;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationExtracted {
///     reverse_complement: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reverseComplement` is the name this serializes to on the wire.
/// assert_eq!(json["reverseComplement"], ::serde_json::json!(true));
///
/// let back: MolecularDefinitionRepresentationExtracted = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationExtracted {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The molecular entity that serves as the conceptual 'parent' from which
    /// the intended entity is derived
    pub starting_molecule: types::Reference,

    /// The interval on startingMolecule that defines the portion to be
    /// extracted to produce the intended entity
    pub coordinate_interval: Option<MolecularDefinitionRepresentationExtractedCoordinateInterval>,

    /// A flag that indicates whether the extracted sequence should be reverse
    /// complemented
    pub reverse_complement: Option<types::Boolean>,
    /// Primitive extension sibling for [`reverse_complement`](Self::reverse_complement) (FHIR `_reverseComplement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reverseComplement")]
    pub reverse_complement_ext: Option<types::Element>,
}

/// The interval on startingMolecule that defines the portion to be extracted
/// in order to create the intended entity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationExtractedCoordinateInterval;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationExtractedCoordinateInterval {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationExtractedCoordinateInterval = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationExtractedCoordinateInterval {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The coordinate system used to define the location
    pub coordinate_system:
        Option<MolecularDefinitionRepresentationExtractedCoordinateIntervalCoordinateSystem>,

    /// The start location of the interval
    /// The `MolecularDefinition.representation.extracted.coordinateInterval.start[x]` choice element (0..1); see [`MolecularDefinitionRepresentationExtractedCoordinateIntervalStart`].
    #[serde(flatten)]
    pub start: Option<MolecularDefinitionRepresentationExtractedCoordinateIntervalStart>,

    /// The end location of the interval
    /// The `MolecularDefinition.representation.extracted.coordinateInterval.end[x]` choice element (0..1); see [`MolecularDefinitionRepresentationExtractedCoordinateIntervalEnd`].
    #[serde(flatten)]
    pub end: Option<MolecularDefinitionRepresentationExtractedCoordinateIntervalEnd>,
}

/// The coordinate system used to define the location, which may vary depending
/// on application or context of use.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationExtractedCoordinateIntervalCoordinateSystem;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationExtractedCoordinateIntervalCoordinateSystem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationExtractedCoordinateIntervalCoordinateSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationExtractedCoordinateIntervalCoordinateSystem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of coordinate system used
    pub system: Option<types::CodeableConcept>,

    /// The location of the origin of the coordinate system
    pub origin: Option<types::CodeableConcept>,

    /// The normalization method used for determining a location within the
    /// coordinate system
    pub normalization_method: Option<types::CodeableConcept>,
}

/// A molecular entity defined as a string literal.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationLiteral;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationLiteral {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationLiteral = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationLiteral {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The encoding used in the value
    pub encoding: Option<types::CodeableConcept>,

    /// A string literal representation of the molecular entity, using the
    /// encoding specified in encoding
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A molecular entity represented as an ordered series of edits on a specified
/// starting entity. This representation can be used to define one entity
/// relative to another.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationRelative;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationRelative {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationRelative = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationRelative {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The molecular entity on which edits will be applied
    pub starting_molecule: types::Reference,

    /// A defined edit (change) to be applied
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edit: Vec<MolecularDefinitionRepresentationRelativeEdit>,
}

/// A defined edit (change) to be applied to the molecular entity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationRelativeEdit;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationRelativeEdit {
///     edit_order: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `editOrder` is the name this serializes to on the wire.
/// assert_eq!(json["editOrder"], ::serde_json::json!(42));
///
/// let back: MolecularDefinitionRepresentationRelativeEdit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationRelativeEdit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Defines the order of edits when multiple edits are to be applied to the
    /// startingMolecule
    pub edit_order: Option<types::Integer>,
    /// Primitive extension sibling for [`edit_order`](Self::edit_order) (FHIR `_editOrder`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_editOrder")]
    pub edit_order_ext: Option<types::Element>,

    /// The interval on startingMolecule that defines the portion to be
    /// extracted to produce the intended entity
    pub coordinate_interval:
        Option<MolecularDefinitionRepresentationRelativeEditCoordinateInterval>,

    /// The molecular entity that serves as the replacement in the edit
    /// operation
    pub replacement_molecule: types::Reference,

    /// The portion of the molecular entity that is replaced by the
    /// replacementMolecule
    pub replaced_molecule: Option<types::Reference>,
}

/// The interval on startingMolecule that defines the portion to be extracted
/// in order to create the intended entity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationRelativeEditCoordinateInterval;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationRelativeEditCoordinateInterval {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationRelativeEditCoordinateInterval = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationRelativeEditCoordinateInterval {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The coordinate system used to define the location
    pub coordinate_system:
        Option<MolecularDefinitionRepresentationRelativeEditCoordinateIntervalCoordinateSystem>,

    /// The start location of the interval
    /// The `MolecularDefinition.representation.relative.edit.coordinateInterval.start[x]` choice element (0..1); see [`MolecularDefinitionRepresentationRelativeEditCoordinateIntervalStart`].
    #[serde(flatten)]
    pub start: Option<MolecularDefinitionRepresentationRelativeEditCoordinateIntervalStart>,

    /// The end location of the interval
    /// The `MolecularDefinition.representation.relative.edit.coordinateInterval.end[x]` choice element (0..1); see [`MolecularDefinitionRepresentationRelativeEditCoordinateIntervalEnd`].
    #[serde(flatten)]
    pub end: Option<MolecularDefinitionRepresentationRelativeEditCoordinateIntervalEnd>,
}

/// The coordinate system used to define the location, which may vary depending
/// on application or context of use.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationRelativeEditCoordinateIntervalCoordinateSystem;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationRelativeEditCoordinateIntervalCoordinateSystem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationRelativeEditCoordinateIntervalCoordinateSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationRelativeEditCoordinateIntervalCoordinateSystem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of coordinate system used
    pub system: Option<types::CodeableConcept>,

    /// The location of the origin of the coordinate system
    pub origin: Option<types::CodeableConcept>,

    /// The normalization method used for determining a location within the
    /// coordinate system
    pub normalization_method: Option<types::CodeableConcept>,
}

/// A representation of a molecular entity that is expressed as a number of
/// copies of a repeated motif.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_definition::MolecularDefinitionRepresentationRepeated;
/// use fhir::r6::types;
///
/// let value = MolecularDefinitionRepresentationRepeated {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularDefinitionRepresentationRepeated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularDefinitionRepresentationRepeated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The motif that is repeated
    pub sequence_motif: types::Reference,

    /// The number of copies of the motif
    pub copy_count: types::Integer,
    /// Primitive extension sibling for [`copy_count`](Self::copy_count) (FHIR `_copyCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyCount")]
    pub copy_count_ext: Option<types::Element>,
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.arm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandArm {
    /// `armCode` variant.
    #[fhir("armCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `armString` variant.
    #[fhir("armString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.region[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandRegion {
    /// `regionCode` variant.
    #[fhir("regionCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `regionString` variant.
    #[fhir("regionString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.band[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandBand {
    /// `bandCode` variant.
    #[fhir("bandCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `bandString` variant.
    #[fhir("bandString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.endCytoband.subBand[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalEndCytobandSubBand {
    /// `subBandCode` variant.
    #[fhir("subBandCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `subBandString` variant.
    #[fhir("subBandString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.arm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandArm {
    /// `armCode` variant.
    #[fhir("armCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `armString` variant.
    #[fhir("armString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.region[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandRegion {
    /// `regionCode` variant.
    #[fhir("regionCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `regionString` variant.
    #[fhir("regionString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.band[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandBand {
    /// `bandCode` variant.
    #[fhir("bandCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `bandString` variant.
    #[fhir("bandString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.cytobandInterval.startCytoband.subBand[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationCytobandIntervalStartCytobandSubBand {
    /// `subBandCode` variant.
    #[fhir("subBandCode")]
    Code(crate::r6::choice::Primitive<types::Code>),
    /// `subBandString` variant.
    #[fhir("subBandString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.cytobandLocation.genomeAssembly.description[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationCytobandLocationGenomeAssemblyDescription {
    /// `descriptionMarkdown` variant.
    #[fhir("descriptionMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `descriptionString` variant.
    #[fhir("descriptionString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `MolecularDefinition.location.sequenceLocation.coordinateInterval.start[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationSequenceLocationCoordinateIntervalStart {
    /// `startQuantity` variant.
    #[fhir("startQuantity")]
    Quantity(Box<types::Quantity>),
    /// `startRange` variant.
    #[fhir("startRange")]
    Range(Box<types::Range>),
}

/// The `MolecularDefinition.location.sequenceLocation.coordinateInterval.end[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionLocationSequenceLocationCoordinateIntervalEnd {
    /// `endQuantity` variant.
    #[fhir("endQuantity")]
    Quantity(Box<types::Quantity>),
    /// `endRange` variant.
    #[fhir("endRange")]
    Range(Box<types::Range>),
}

/// The `MolecularDefinition.representation.extracted.coordinateInterval.start[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionRepresentationExtractedCoordinateIntervalStart {
    /// `startQuantity` variant.
    #[fhir("startQuantity")]
    Quantity(Box<types::Quantity>),
    /// `startRange` variant.
    #[fhir("startRange")]
    Range(Box<types::Range>),
}

/// The `MolecularDefinition.representation.extracted.coordinateInterval.end[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionRepresentationExtractedCoordinateIntervalEnd {
    /// `endQuantity` variant.
    #[fhir("endQuantity")]
    Quantity(Box<types::Quantity>),
    /// `endRange` variant.
    #[fhir("endRange")]
    Range(Box<types::Range>),
}

/// The `MolecularDefinition.representation.relative.edit.coordinateInterval.start[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionRepresentationRelativeEditCoordinateIntervalStart {
    /// `startQuantity` variant.
    #[fhir("startQuantity")]
    Quantity(Box<types::Quantity>),
    /// `startRange` variant.
    #[fhir("startRange")]
    Range(Box<types::Range>),
}

/// The `MolecularDefinition.representation.relative.edit.coordinateInterval.end[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum MolecularDefinitionRepresentationRelativeEditCoordinateIntervalEnd {
    /// `endQuantity` variant.
    #[fhir("endQuantity")]
    Quantity(Box<types::Quantity>),
    /// `endRange` variant.
    #[fhir("endRange")]
    Range(Box<types::Range>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MolecularDefinition;

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
