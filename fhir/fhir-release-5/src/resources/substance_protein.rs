//! SubstanceProtein
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceProtein
//!
//! Version: 5.0.0
//!
//! SubstanceProtein Resource: A single unit of a linear amino acid sequence, or a combination of covalently linked or defined stoichiometric subunits.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubstanceProtein
///
/// A SubstanceProtein is defined as a single unit of a linear amino acid
/// sequence, or a combination of subunits that are either covalently linked or
/// have a defined invariant stoichiometric relationship. This includes all
/// synthetic, recombinant and purified SubstanceProteins of defined sequence,
/// whether the use is therapeutic or prophylactic. It is used to describe
/// albumins, coagulation factors, cytokines, growth factors, and similar
/// biological substances.
///
/// This resource is used within regulated substance definitions and product
/// authoring to capture the molecular characterization of a protein-based
/// substance, including how many subunits it comprises and, for each subunit,
/// its amino acid sequence, length, and any N-terminal or C-terminal
/// modifications. It supports the identification and comparison of proteins
/// used as active ingredients, excipients, or reference substances in
/// pharmaceutical and biologic products.
///
/// # Related resources
///
/// A `SubstanceProtein` is typically referenced from a substance definition
/// resource (such as `SubstanceDefinition`) rather than being used directly by
/// clinical resources; classification and descriptive terms throughout this
/// resource are represented as [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// values, and cross-references to other ISO 11238 substance identifiers use
/// [`Identifier`](crate::r5::types::Identifier).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_protein::SubstanceProtein;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceProtein {
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
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The SubstanceProtein descriptive elements will only be used when a complete or partial amino acid sequence is available or derivable from a nucleic acid sequence; coded using a [`CodeableConcept`](crate::r5::types::CodeableConcept)
    pub sequence_type: Option<types::CodeableConcept>,

    /// Number of linear sequences of amino acids linked through peptide bonds that together make up this protein
    pub number_of_subunits: Option<types::Integer>,
    /// Primitive extension sibling for [`number_of_subunits`](Self::number_of_subunits) (FHIR `_numberOfSubunits`).
    #[serde(rename = "_numberOfSubunits")]
    pub number_of_subunits_ext: Option<types::Element>,

    /// The disulphide bond between two cysteine residues shall be described, identifying the connecting residue positions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disulfide_linkage: Vec<types::String>,
    /// Primitive extension sibling for [`disulfide_linkage`](Self::disulfide_linkage) (FHIR `_disulfideLinkage`).
    #[serde(rename = "_disulfideLinkage")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disulfide_linkage_ext: Vec<Option<types::Element>>,

    /// The individual amino acid subunits that together constitute this SubstanceProtein, described in detail by [`SubstanceProteinSubunit`]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subunit: Vec<SubstanceProteinSubunit>,
}

/// SubstanceProteinSubunit
///
/// This subclause refers to the description of each subunit constituting the
/// SubstanceProtein. A subunit is a linear sequence of amino acids linked
/// through peptide bonds.
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_protein::SubstanceProteinSubunit;
/// use fhir::r5::types;
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
pub struct SubstanceProteinSubunit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Index of primary sequences of amino acids linked through peptide bonds in order of decreasing length
    pub subunit: Option<types::Integer>,
    /// Primitive extension sibling for [`subunit`](Self::subunit) (FHIR `_subunit`).
    #[serde(rename = "_subunit")]
    pub subunit_ext: Option<types::Element>,

    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes
    pub sequence: Option<types::String>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// Length of linear sequences of amino acids contained in the subunit
    pub length: Option<types::Integer>,
    /// Primitive extension sibling for [`length`](Self::length) (FHIR `_length`).
    #[serde(rename = "_length")]
    pub length_ext: Option<types::Element>,

    /// The sequence information shall be provided as an attachment
    pub sequence_attachment: Option<types::Attachment>,

    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    pub n_terminal_modification_id: Option<types::Identifier>,

    /// The name of the fragment modified at the N-terminal of the SubstanceProtein shall be specified
    pub n_terminal_modification: Option<types::String>,
    /// Primitive extension sibling for [`n_terminal_modification`](Self::n_terminal_modification) (FHIR `_nTerminalModification`).
    #[serde(rename = "_nTerminalModification")]
    pub n_terminal_modification_ext: Option<types::Element>,

    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    pub c_terminal_modification_id: Option<types::Identifier>,

    /// The modification at the C-terminal shall be specified
    pub c_terminal_modification: Option<types::String>,
    /// Primitive extension sibling for [`c_terminal_modification`](Self::c_terminal_modification) (FHIR `_cTerminalModification`).
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
