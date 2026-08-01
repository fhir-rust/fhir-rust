//! SubstanceProtein
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceProtein
//!
//! Version: 6.0.0-ballot3
//!
//! A SubstanceProtein is defined as a single unit of a linear amino acid
//! sequence, or a combination of subunits that are either covalently linked or
//! have a defined invariant stoichiometric relationship. This includes all
//! synthetic, recombinant and purified SubstanceProteins of defined sequence,
//! whether the use is therapeutic or prophylactic. This set of elements will
//! be used to describe albumins, coagulation factors, cytokines, growth
//! factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids,
//! recombinant vaccines, and immunomodulators
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A SubstanceProtein is defined as a single unit of a linear amino acid
/// sequence, or a combination of subunits that are either covalently linked or
/// have a defined invariant stoichiometric relationship. This includes all
/// synthetic, recombinant and purified SubstanceProteins of defined sequence,
/// whether the use is therapeutic or prophylactic. This set of elements will
/// be used to describe albumins, coagulation factors, cytokines, growth
/// factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids,
/// recombinant vaccines, and immunomodulators.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_protein::SubstanceProtein;
/// use fhir::r6::types;
///
/// let value = SubstanceProtein {
///     number_of_subunits: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfSubunits` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfSubunits"], ::serde_json::json!(42));
///
/// let back: SubstanceProtein = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceProtein {
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

    /// The SubstanceProtein descriptive elements will only be used when a
    /// complete or partial amino acid sequence is available or derivable from
    /// a nucleic acid sequence
    pub sequence_type: Option<types::CodeableConcept>,

    /// Number of linear sequences of amino acids linked through peptide bonds.
    /// The number of subunits constituting the SubstanceProtein shall be
    /// described. It is possible that the number of subunits can be variable
    pub number_of_subunits: Option<types::Integer>,
    /// Primitive extension sibling for [`number_of_subunits`](Self::number_of_subunits) (FHIR `_numberOfSubunits`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfSubunits")]
    pub number_of_subunits_ext: Option<types::Element>,

    /// The disulphide bond between two cysteine residues either on the same
    /// subunit or on two different subunits shall be described. The position
    /// of the disulfide bonds in the SubstanceProtein shall be listed in
    /// increasing order of subunit number and position within subunit followed
    /// by the abbreviation of the amino acids involved. The disulfide linkage
    /// positions shall actually contain the amino acid Cysteine at the
    /// respective positions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disulfide_linkage: Vec<types::String>,
    /// Primitive extension sibling for [`disulfide_linkage`](Self::disulfide_linkage) (FHIR `_disulfideLinkage`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_disulfideLinkage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disulfide_linkage_ext: Vec<Option<types::Element>>,

    /// This subclause refers to the description of each subunit constituting
    /// the SubstanceProtein. A subunit is a linear sequence of amino acids
    /// linked through peptide bonds. The Subunit information shall be provided
    /// when the finished SubstanceProtein is a complex of multiple sequences;
    /// subunits are not used to delineate domains within a single sequence.
    /// Subunits are listed in order of decreasing length; sequences of the
    /// same length will be ordered by decreasing molecular weight; subunits
    /// that have identical sequences will be repeated multiple times
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subunit: Vec<SubstanceProteinSubunit>,
}

/// This subclause refers to the description of each subunit constituting the
/// SubstanceProtein. A subunit is a linear sequence of amino acids linked
/// through peptide bonds. The Subunit information shall be provided when the
/// finished SubstanceProtein is a complex of multiple sequences; subunits are
/// not used to delineate domains within a single sequence. Subunits are listed
/// in order of decreasing length; sequences of the same length will be ordered
/// by decreasing molecular weight; subunits that have identical sequences will
/// be repeated multiple times.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_protein::SubstanceProteinSubunit;
/// use fhir::r6::types;
///
/// let value = SubstanceProteinSubunit {
///     n_terminal_modification: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `nTerminalModification` is the name this serializes to on the wire.
/// assert_eq!(json["nTerminalModification"], ::serde_json::json!("abc"));
///
/// let back: SubstanceProteinSubunit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceProteinSubunit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Index of primary sequences of amino acids linked through peptide bonds
    /// in order of decreasing length. Sequences of the same length will be
    /// ordered by molecular weight. Subunits that have identical sequences
    /// will be repeated and have sequential subscripts
    pub subunit: Option<types::Integer>,
    /// Primitive extension sibling for [`subunit`](Self::subunit) (FHIR `_subunit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subunit")]
    pub subunit_ext: Option<types::Element>,

    /// The sequence information shall be provided enumerating the amino acids
    /// from N- to C-terminal end using standard single-letter amino acid
    /// codes. Uppercase shall be used for L-amino acids and lowercase for
    /// D-amino acids. Transcribed SubstanceProteins will always be described
    /// using the translated sequence; for synthetic peptide containing amino
    /// acids that are not represented with a single letter code an X should be
    /// used within the sequence. The modified amino acids will be
    /// distinguished by their position in the sequence
    pub sequence: Option<types::String>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Length of linear sequences of amino acids contained in the subunit
    pub length: Option<types::Integer>,
    /// Primitive extension sibling for [`length`](Self::length) (FHIR `_length`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_length")]
    pub length_ext: Option<types::Element>,

    /// The sequence information shall be provided enumerating the amino acids
    /// from N- to C-terminal end using standard single-letter amino acid
    /// codes. Uppercase shall be used for L-amino acids and lowercase for
    /// D-amino acids. Transcribed SubstanceProteins will always be described
    /// using the translated sequence; for synthetic peptide containing amino
    /// acids that are not represented with a single letter code an X should be
    /// used within the sequence. The modified amino acids will be
    /// distinguished by their position in the sequence
    pub sequence_attachment: Option<types::Attachment>,

    /// Unique identifier for molecular fragment modification based on the ISO
    /// 11238 Substance ID
    pub n_terminal_modification_id: Option<types::Identifier>,

    /// The name of the fragment modified at the N-terminal of the
    /// SubstanceProtein shall be specified
    pub n_terminal_modification: Option<types::String>,
    /// Primitive extension sibling for [`n_terminal_modification`](Self::n_terminal_modification) (FHIR `_nTerminalModification`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_nTerminalModification")]
    pub n_terminal_modification_ext: Option<types::Element>,

    /// Unique identifier for molecular fragment modification based on the ISO
    /// 11238 Substance ID
    pub c_terminal_modification_id: Option<types::Identifier>,

    /// The modification at the C-terminal shall be specified
    pub c_terminal_modification: Option<types::String>,
    /// Primitive extension sibling for [`c_terminal_modification`](Self::c_terminal_modification) (FHIR `_cTerminalModification`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_cTerminalModification")]
    pub c_terminal_modification_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceProtein;

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
