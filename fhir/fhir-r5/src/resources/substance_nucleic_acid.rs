//! SubstanceNucleicAcid
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceNucleicAcid
//!
//! Version: 5.0.0
//!
//! SubstanceNucleicAcid Resource: Nucleic acids are defined by three distinct elements: the base, sugar and linkage.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubstanceNucleicAcid describes the chemical structure of a nucleic acid
/// substance. Nucleic acids are defined by three distinct elements: the base,
/// sugar and linkage, and individual substance/moiety IDs are created for each
/// of these elements. The nucleotide sequence is always entered in the 5'-3'
/// direction. This resource is primarily used in medicinal product regulatory
/// submissions to characterize substances such as oligonucleotides.
///
/// Structurally, a nucleic acid substance is composed of one or more subunits
/// (linear chains of nucleotides), each of which may in turn describe its
/// constituent sugars and the linkages connecting them. This layered model
/// mirrors the way regulators and manufacturers document complex synthetic
/// oligonucleotide therapeutics (for example antisense oligonucleotides or
/// siRNA drug substances), where the exact sequence, stereochemistry, and
/// chemical linkages must be precisely characterized for identity and quality
/// control purposes. As with other substance-family resources, instances of
/// `SubstanceNucleicAcid` are typically referenced from a general `Substance`
/// or from medicinal product definition resources rather than being used
/// directly in clinical patient records.
///
/// See also: [`CodeableConcept`](crate::r5::types::CodeableConcept) for
/// coded elements such as `sequence_type`, and the related `Substance`,
/// `SubstanceProtein`, and `SubstancePolymer` resources that together model
/// the composition of complex substances.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_nucleic_acid::SubstanceNucleicAcid;
/// use fhir::r5::types;
///
/// let value = SubstanceNucleicAcid {
///     number_of_subunits: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfSubunits` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfSubunits"], ::serde_json::json!(42));
///
/// let back: SubstanceNucleicAcid = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcid {
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

    /// The type of the sequence, e.g. DNA or RNA, specified using a controlled vocabulary
    pub sequence_type: Option<types::CodeableConcept>,

    /// The number of linear nucleotide sequences (subunits) linked through phosphodiester bonds
    pub number_of_subunits: Option<types::Integer>,
    /// Primitive extension sibling for [`number_of_subunits`](Self::number_of_subunits) (FHIR `_numberOfSubunits`).
    #[serde(rename = "_numberOfSubunits")]
    pub number_of_subunits_ext: Option<types::Element>,

    /// The area of hybridisation shall be described if applicable for double stranded RNA or DNA
    pub area_of_hybridisation: Option<types::String>,
    /// Primitive extension sibling for [`area_of_hybridisation`](Self::area_of_hybridisation) (FHIR `_areaOfHybridisation`).
    #[serde(rename = "_areaOfHybridisation")]
    pub area_of_hybridisation_ext: Option<types::Element>,

    /// (TBC)
    pub oligo_nucleotide_type: Option<types::CodeableConcept>,

    /// The individual nucleotide subunits that make up this substance, listed in order of decreasing length
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subunit: Vec<SubstanceNucleicAcidSubunit>,
}

/// Subunits are listed in order of decreasing length; sequences of the same
/// length will be ordered by molecular weight; subunits that have identical
/// sequences will be repeated multiple times.
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_nucleic_acid::SubstanceNucleicAcidSubunit;
/// use fhir::r5::types;
///
/// let value = SubstanceNucleicAcidSubunit {
///     subunit: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `subunit` is the name this serializes to on the wire.
/// assert_eq!(json["subunit"], ::serde_json::json!(42));
///
/// let back: SubstanceNucleicAcidSubunit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Index of linear sequences of nucleic acids in order of decreasing length
    pub subunit: Option<types::Integer>,
    /// Primitive extension sibling for [`subunit`](Self::subunit) (FHIR `_subunit`).
    #[serde(rename = "_subunit")]
    pub subunit_ext: Option<types::Element>,

    /// Actual nucleotide sequence notation from 5' to 3' end using standard single letter codes
    pub sequence: Option<types::String>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// The length of the sequence shall be captured
    pub length: Option<types::Integer>,
    /// Primitive extension sibling for [`length`](Self::length) (FHIR `_length`).
    #[serde(rename = "_length")]
    pub length_ext: Option<types::Element>,

    /// (TBC)
    pub sequence_attachment: Option<types::Attachment>,

    /// The nucleotide present at the 5' terminal shall be specified based on a controlled vocabulary
    pub five_prime: Option<types::CodeableConcept>,

    /// The nucleotide present at the 3' terminal shall be specified based on a controlled vocabulary
    pub three_prime: Option<types::CodeableConcept>,

    /// The linkages between sugar residues will also be captured
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linkage: Vec<SubstanceNucleicAcidSubunitLinkage>,

    /// 5.3.6.8.1 Sugar ID (Mandatory)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sugar: Vec<SubstanceNucleicAcidSubunitSugar>,
}

/// The linkages between sugar residues will also be captured.
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_nucleic_acid::SubstanceNucleicAcidSubunitLinkage;
/// use fhir::r5::types;
///
/// let value = SubstanceNucleicAcidSubunitLinkage {
///     residue_site: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `residueSite` is the name this serializes to on the wire.
/// assert_eq!(json["residueSite"], ::serde_json::json!("abc"));
///
/// let back: SubstanceNucleicAcidSubunitLinkage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunitLinkage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The entity that links the sugar residues together
    pub connectivity: Option<types::String>,
    /// Primitive extension sibling for [`connectivity`](Self::connectivity) (FHIR `_connectivity`).
    #[serde(rename = "_connectivity")]
    pub connectivity_ext: Option<types::Element>,

    /// Each linkage will be registered as a fragment and have an ID
    pub identifier: Option<types::Identifier>,

    /// Each linkage will be registered as a fragment and have at least one name
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Residues shall be captured as described in 5.3.6.8.3
    pub residue_site: Option<types::String>,
    /// Primitive extension sibling for [`residue_site`](Self::residue_site) (FHIR `_residueSite`).
    #[serde(rename = "_residueSite")]
    pub residue_site_ext: Option<types::Element>,
}

/// 5.3.6.8.1 Sugar ID (Mandatory).
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_nucleic_acid::SubstanceNucleicAcidSubunitSugar;
/// use fhir::r5::types;
///
/// let value = SubstanceNucleicAcidSubunitSugar {
///     residue_site: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `residueSite` is the name this serializes to on the wire.
/// assert_eq!(json["residueSite"], ::serde_json::json!("abc"));
///
/// let back: SubstanceNucleicAcidSubunitSugar = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceNucleicAcidSubunitSugar {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The Substance ID of the sugar or sugar-like component that make up the nucleotide
    pub identifier: Option<types::Identifier>,

    /// The name of the sugar or sugar-like component that make up the nucleotide
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// The residues that contain a given sugar will be captured
    pub residue_site: Option<types::String>,
    /// Primitive extension sibling for [`residue_site`](Self::residue_site) (FHIR `_residueSite`).
    #[serde(rename = "_residueSite")]
    pub residue_site_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceNucleicAcid;

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
