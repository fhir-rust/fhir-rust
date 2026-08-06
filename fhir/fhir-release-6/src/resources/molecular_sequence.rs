//! MolecularSequence
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MolecularSequence
//!
//! Version: 6.0.0-ballot3
//!
//! Representation of a molecular sequence
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Representation of a molecular sequence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_sequence::MolecularSequence;
/// use fhir::r6::types;
///
/// let value = MolecularSequence {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequence = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique ID for this particular sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// aa | dna | rna
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::SequenceType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// A literal representation of a Molecular Sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub literal: Vec<MolecularSequenceLiteral>,

    /// Embedded file or a link (URL) which contains content to represent the
    /// sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file: Vec<types::Attachment>,

    /// A Molecular Sequence that is represented as an ordered series of edits
    /// on a specified starting sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relative: Vec<MolecularSequenceRelative>,

    /// A Molecular Sequence that is represented as an extracted portion of a
    /// different Molecular Sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extracted: Vec<MolecularSequenceExtracted>,

    /// A Molecular Sequence that is represented as a repeated sequence motif
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeated: Vec<MolecularSequenceRepeated>,

    /// A Molecular Sequence that is represented as an ordered concatenation of
    /// two or more Molecular Sequences
    pub concatenated: Option<MolecularSequenceConcatenated>,
}

/// A Molecular Sequence that is represented as an ordered concatenation of two
/// or more Molecular Sequences.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::molecular_sequence::MolecularSequenceConcatenated;
///
/// let value = MolecularSequenceConcatenated::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: MolecularSequenceConcatenated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularSequenceConcatenated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// One element of a concatenated Molecular Sequence
    pub sequence_element: ::vec1::Vec1<MolecularSequenceConcatenatedSequenceElement>,
}

/// One element of a concatenated Molecular Sequence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_sequence::MolecularSequenceConcatenatedSequenceElement;
/// use fhir::r6::types;
///
/// let value = MolecularSequenceConcatenatedSequenceElement {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceConcatenatedSequenceElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularSequenceConcatenatedSequenceElement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The Molecular Sequence corresponding to this element
    pub sequence: types::Reference,

    /// The ordinal position of this sequence element within the concatenated
    /// Molecular Sequence
    pub ordinal_index: types::Integer,
    /// Primitive extension sibling for [`ordinal_index`](Self::ordinal_index) (FHIR `_ordinalIndex`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ordinalIndex")]
    pub ordinal_index_ext: Option<types::Element>,
}

/// A Molecular Sequence that is represented as an extracted portion of a
/// different Molecular Sequence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_sequence::MolecularSequenceExtracted;
/// use fhir::r6::types;
///
/// let value = MolecularSequenceExtracted {
///     reverse_complement: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reverseComplement` is the name this serializes to on the wire.
/// assert_eq!(json["reverseComplement"], ::serde_json::json!(true));
///
/// let back: MolecularSequenceExtracted = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularSequenceExtracted {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The Molecular Sequence that serves as the parent sequence, from which
    /// the intended sequence will be extracted
    pub starting_sequence: types::Reference,

    /// The start coordinate (on the parent sequence) of the interval that
    /// defines the subsequence to be extracted
    pub start: types::Integer,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// The end coordinate (on the parent sequence) of the interval that
    /// defines the subsequence to be extracted
    pub end: types::Integer,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// The coordinate system used to define the interval that defines the
    /// subsequence to be extracted. Coordinate systems are usually 0- or
    /// 1-based
    pub coordinate_system: types::CodeableConcept,

    /// A flag that indicates whether the extracted sequence should be reverse
    /// complemented
    pub reverse_complement: Option<types::Boolean>,
    /// Primitive extension sibling for [`reverse_complement`](Self::reverse_complement) (FHIR `_reverseComplement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reverseComplement")]
    pub reverse_complement_ext: Option<types::Element>,
}

/// A literal representation of a Molecular Sequence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_sequence::MolecularSequenceLiteral;
/// use fhir::r6::types;
///
/// let value = MolecularSequenceLiteral {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceLiteral = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularSequenceLiteral {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The primary (linear) sequence, expressed as a literal string
    pub sequence_value: types::String,
    /// Primitive extension sibling for [`sequence_value`](Self::sequence_value) (FHIR `_sequenceValue`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequenceValue")]
    pub sequence_value_ext: Option<types::Element>,
}

/// A Molecular Sequence that is represented as an ordered series of edits on a
/// specified starting sequence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_sequence::MolecularSequenceRelative;
/// use fhir::r6::types;
///
/// let value = MolecularSequenceRelative {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceRelative = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularSequenceRelative {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The Molecular Sequence that serves as the starting sequence, on which
    /// edits will be applied
    pub starting_sequence: types::Reference,

    /// An edit (change) made to a sequence
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edit: Vec<MolecularSequenceRelativeEdit>,
}

/// An edit (change) made to a sequence.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_sequence::MolecularSequenceRelativeEdit;
/// use fhir::r6::types;
///
/// let value = MolecularSequenceRelativeEdit {
///     edit_order: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `editOrder` is the name this serializes to on the wire.
/// assert_eq!(json["editOrder"], ::serde_json::json!(42));
///
/// let back: MolecularSequenceRelativeEdit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularSequenceRelativeEdit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The order of this edit, relative to other edits on the starting
    /// sequence
    pub edit_order: Option<types::Integer>,
    /// Primitive extension sibling for [`edit_order`](Self::edit_order) (FHIR `_editOrder`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_editOrder")]
    pub edit_order_ext: Option<types::Element>,

    /// The coordinate system used to define the edited intervals on the
    /// starting sequence. Coordinate systems are usually 0- or 1-based
    pub coordinate_system: types::CodeableConcept,

    /// The start coordinate of the interval that will be edited
    pub start: types::Integer,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// The end coordinate of the interval that will be edited
    pub end: types::Integer,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// The sequence that defines the replacement sequence used in the edit
    /// operation
    pub replacement_sequence: types::Reference,

    /// The sequence on the 'starting' sequence for the edit operation, defined
    /// by the specified interval, that will be replaced during the edit
    pub replaced_sequence: Option<types::Reference>,
}

/// A Molecular Sequence that is represented as a repeated sequence motif.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::molecular_sequence::MolecularSequenceRepeated;
/// use fhir::r6::types;
///
/// let value = MolecularSequenceRepeated {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MolecularSequenceRepeated = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct MolecularSequenceRepeated {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The sequence that defines the repeated motif
    pub sequence_motif: types::Reference,

    /// The number of repeats (copies) of the sequence motif
    pub copy_count: types::Integer,
    /// Primitive extension sibling for [`copy_count`](Self::copy_count) (FHIR `_copyCount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyCount")]
    pub copy_count_ext: Option<types::Element>,
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
