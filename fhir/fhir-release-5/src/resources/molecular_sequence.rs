//! MolecularSequence
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
//!
//! Version: 5.0.0
//!
//! MolecularSequence Resource: Representation of a molecular sequence.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Representation of a molecular sequence.
///
/// The MolecularSequence resource captures a raw or relative molecular sequence
/// (amino acid, DNA, or RNA) together with the subject, specimen, device, and
/// performer context in which it was observed. Its clinical and research purpose
/// is to carry the actual sequence content produced by genomic testing so that
/// it can be exchanged, stored, and interpreted alongside the observations and
/// diagnostic reports that describe its meaning. A sequence may be conveyed in
/// three complementary ways: literally as an inline string, as an embedded or
/// externally linked file, or relatively by referencing a known starting
/// sequence (such as a genome assembly build like GRCh38) and expressing the
/// differences as a set of edits over defined coordinate ranges. In typical
/// FHIR R5 genomics workflows this resource is referenced from an Observation
/// or other clinical resource, letting downstream systems reason about variants,
/// alignments, and provenance without duplicating large sequence payloads.
///
/// # Related resources
///
/// The subject, specimen, device, and performer are expressed as
/// [`Reference`](crate::r5::types::Reference) values, commonly pointing to a
/// `Patient`, `Specimen`, `Device`, or `Practitioner`. Attached sequence
/// content uses [`Attachment`](crate::r5::types::Attachment), and coordinate
/// systems, genome assemblies, and chromosomes are described with
/// [`CodeableConcept`](crate::r5::types::CodeableConcept). This resource is
/// frequently referenced by an `Observation` in genomics-oriented profiles.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::molecular_sequence::MolecularSequence;
/// use fhir::r5::types;
///
/// let value = MolecularSequence {
///     literal: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `literal` is the name this serializes to on the wire.
/// assert_eq!(json["literal"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequence {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique business identifier(s) assigned to this particular sequence, distinct from the resource's logical id
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Kind of molecule represented, coded as aa (amino acid), dna, or rna
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::SequenceType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Subject the sequence is associated with, typically a Patient or other record target
    pub subject: Option<types::Reference>,

    /// What the molecular sequence is about, when it is not about the subject of record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference>,

    /// Specimen used for sequencing
    pub specimen: Option<types::Reference>,

    /// The method or platform used for sequencing, referencing the Device that performed it
    pub device: Option<types::Reference>,

    /// Who should be responsible for test result
    pub performer: Option<types::Reference>,

    /// Sequence that was observed, provided inline as a literal string of residues
    pub literal: Option<types::String>,
    /// Primitive extension sibling for [`literal`](Self::literal) (FHIR `_literal`).
    #[serde(rename = "_literal")]
    pub literal_ext: Option<types::Element>,

    /// Embedded file or an external link containing content that represents the sequence, such as a FASTA or VCF file
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formatted: Vec<types::Attachment>,

    /// A sequence defined relative to a starting sequence via a set of edits
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relative: Vec<MolecularSequenceRelative>,
}

/// A sequence defined relative to another sequence.
/// # Examples
///
/// ```
/// use fhir::r5::resources::molecular_sequence::MolecularSequenceRelative;
/// use fhir::r5::types;
///
/// let value = MolecularSequenceRelative {
///     ordinal_position: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `ordinalPosition` is the name this serializes to on the wire.
/// assert_eq!(json["ordinalPosition"], ::serde_json::json!(42));
///
/// let back: MolecularSequenceRelative = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelative {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Ways of identifying nucleotides or amino acids within a sequence
    pub coordinate_system: types::CodeableConcept,

    /// Indicates the order in which the sequence should be considered when putting multiple 'relative' elements together
    pub ordinal_position: Option<types::Integer>,
    /// Primitive extension sibling for [`ordinal_position`](Self::ordinal_position) (FHIR `_ordinalPosition`).
    #[serde(rename = "_ordinalPosition")]
    pub ordinal_position_ext: Option<types::Element>,

    /// Indicates the nucleotide range in the composed sequence when multiple 'relative' elements are used together
    pub sequence_range: Option<types::Range>,

    /// A sequence used as starting sequence
    pub starting_sequence: Option<MolecularSequenceRelativeStartingSequence>,

    /// Changes in sequence from the starting sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edit: Vec<MolecularSequenceRelativeEdit>,
}

/// A sequence used as starting sequence.
/// # Examples
///
/// ```
/// use fhir::r5::resources::molecular_sequence::MolecularSequenceRelativeStartingSequence;
/// use fhir::r5::types;
///
/// let value = MolecularSequenceRelativeStartingSequence {
///     window_start: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `windowStart` is the name this serializes to on the wire.
/// assert_eq!(json["windowStart"], ::serde_json::json!(42));
///
/// let back: MolecularSequenceRelativeStartingSequence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelativeStartingSequence {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The genome assembly used for starting sequence, e.g. GRCh38
    pub genome_assembly: Option<types::CodeableConcept>,

    /// Chromosome Identifier
    pub chromosome: Option<types::CodeableConcept>,

    /// The `MolecularSequence.relative.startingSequence.sequence[x]` choice element (0..1); see [`MolecularSequenceRelativeStartingSequenceSequence`].
    #[serde(flatten)]
    pub sequence: Option<MolecularSequenceRelativeStartingSequenceSequence>,

    /// Start position of the window on the starting sequence
    pub window_start: Option<types::Integer>,
    /// Primitive extension sibling for [`window_start`](Self::window_start) (FHIR `_windowStart`).
    #[serde(rename = "_windowStart")]
    pub window_start_ext: Option<types::Element>,

    /// End position of the window on the starting sequence
    pub window_end: Option<types::Integer>,
    /// Primitive extension sibling for [`window_end`](Self::window_end) (FHIR `_windowEnd`).
    #[serde(rename = "_windowEnd")]
    pub window_end_ext: Option<types::Element>,

    /// sense | antisense
    pub orientation: Option<crate::r5::coded::Coded<crate::r5::codes::OrientationType>>,
    /// Primitive extension sibling for [`orientation`](Self::orientation) (FHIR `_orientation`).
    #[serde(rename = "_orientation")]
    pub orientation_ext: Option<types::Element>,

    /// watson | crick
    pub strand: Option<crate::r5::coded::Coded<crate::r5::codes::StrandType>>,
    /// Primitive extension sibling for [`strand`](Self::strand) (FHIR `_strand`).
    #[serde(rename = "_strand")]
    pub strand_ext: Option<types::Element>,
}

/// Changes in sequence from the starting sequence.
/// # Examples
///
/// ```
/// use fhir::r5::resources::molecular_sequence::MolecularSequenceRelativeEdit;
/// use fhir::r5::types;
///
/// let value = MolecularSequenceRelativeEdit {
///     replacement_sequence: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `replacementSequence` is the name this serializes to on the wire.
/// assert_eq!(json["replacementSequence"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceRelativeEdit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MolecularSequenceRelativeEdit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Start position of the edit on the starting sequence
    pub start: Option<types::Integer>,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`).
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// End position of the edit on the starting sequence
    pub end: Option<types::Integer>,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`).
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// Allele that was observed
    pub replacement_sequence: Option<types::String>,
    /// Primitive extension sibling for [`replacement_sequence`](Self::replacement_sequence) (FHIR `_replacementSequence`).
    #[serde(rename = "_replacementSequence")]
    pub replacement_sequence_ext: Option<types::Element>,

    /// Allele in the starting sequence
    pub replaced_sequence: Option<types::String>,
    /// Primitive extension sibling for [`replaced_sequence`](Self::replaced_sequence) (FHIR `_replacedSequence`).
    #[serde(rename = "_replacedSequence")]
    pub replaced_sequence_ext: Option<types::Element>,
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
/// The `MolecularSequence.relative.startingSequence.sequence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MolecularSequenceRelativeStartingSequenceSequence {
    /// `sequenceCodeableConcept` variant.
    #[fhir("sequenceCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `sequenceString` variant.
    #[fhir("sequenceString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `sequenceReference` variant.
    #[fhir("sequenceReference")]
    Reference(Box<types::Reference>),
}
