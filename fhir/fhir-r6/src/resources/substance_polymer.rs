//! SubstancePolymer
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstancePolymer
//!
//! Version: 6.0.0-ballot3
//!
//! Properties of a substance specific to it being a polymer
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Properties of a substance specific to it being a polymer.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_polymer::SubstancePolymer;
/// use fhir::r6::types;
///
/// let value = SubstancePolymer {
///     modification: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `modification` is the name this serializes to on the wire.
/// assert_eq!(json["modification"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstancePolymer {
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

    /// A business idenfier for this polymer, but typically this is handled by
    /// a SubstanceDefinition identifier
    pub identifier: Option<types::Identifier>,

    /// Overall type of the polymer
    pub class: Option<types::CodeableConcept>,

    /// Polymer geometry, e.g. linear, branched, cross-linked, network or
    /// dendritic
    pub geometry: Option<types::CodeableConcept>,

    /// Descrtibes the copolymer sequence type (polymer connectivity)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub copolymer_connectivity: Vec<types::CodeableConcept>,

    /// Todo - this is intended to connect to a repeating full modification
    /// structure, also used by Protein and Nucleic Acid . String is just a
    /// placeholder
    pub modification: Option<types::String>,
    /// Primitive extension sibling for [`modification`](Self::modification) (FHIR `_modification`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_modification")]
    pub modification_ext: Option<types::Element>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub monomer_set: Vec<SubstancePolymerMonomerSet>,

    /// Specifies and quantifies the repeated units and their configuration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeat: Vec<SubstancePolymerRepeat>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_polymer::SubstancePolymerMonomerSet;
/// use fhir::r6::types;
///
/// let value = SubstancePolymerMonomerSet {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerMonomerSet = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstancePolymerMonomerSet {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Captures the type of ratio to the entire polymer, e.g. Monomer/Polymer
    /// ratio, SRU/Polymer Ratio
    pub ratio_type: Option<types::CodeableConcept>,

    /// The starting materials - monomer(s) used in the synthesis of the
    /// polymer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub starting_material: Vec<SubstancePolymerMonomerSetStartingMaterial>,
}

/// The starting materials - monomer(s) used in the synthesis of the polymer.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_polymer::SubstancePolymerMonomerSetStartingMaterial;
/// use fhir::r6::types;
///
/// let value = SubstancePolymerMonomerSetStartingMaterial {
///     is_defining: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isDefining` is the name this serializes to on the wire.
/// assert_eq!(json["isDefining"], ::serde_json::json!(true));
///
/// let back: SubstancePolymerMonomerSetStartingMaterial = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of substance for this starting material
    pub code: Option<types::CodeableConcept>,

    /// Substance high level category, e.g. chemical substance
    pub category: Option<types::CodeableConcept>,

    /// Used to specify whether the attribute described is a defining element
    /// for the unique identification of the polymer
    pub is_defining: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_defining`](Self::is_defining) (FHIR `_isDefining`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isDefining")]
    pub is_defining_ext: Option<types::Element>,

    /// A percentage
    pub amount: Option<types::Quantity>,
}

/// Specifies and quantifies the repeated units and their configuration.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_polymer::SubstancePolymerRepeat;
/// use fhir::r6::types;
///
/// let value = SubstancePolymerRepeat {
///     average_molecular_formula: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `averageMolecularFormula` is the name this serializes to on the wire.
/// assert_eq!(json["averageMolecularFormula"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerRepeat = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstancePolymerRepeat {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A representation of an (average) molecular formula from a polymer
    pub average_molecular_formula: Option<types::String>,
    /// Primitive extension sibling for [`average_molecular_formula`](Self::average_molecular_formula) (FHIR `_averageMolecularFormula`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_averageMolecularFormula")]
    pub average_molecular_formula_ext: Option<types::Element>,

    /// How the quantitative amount of Structural Repeat Units is captured
    /// (e.g. Exact, Numeric, Average)
    pub repeat_unit_amount_type: Option<types::CodeableConcept>,

    /// An SRU - Structural Repeat Unit
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repeat_unit: Vec<SubstancePolymerRepeatRepeatUnit>,
}

/// An SRU - Structural Repeat Unit.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_polymer::SubstancePolymerRepeatRepeatUnit;
/// use fhir::r6::types;
///
/// let value = SubstancePolymerRepeatRepeatUnit {
///     unit: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `unit` is the name this serializes to on the wire.
/// assert_eq!(json["unit"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerRepeatRepeatUnit = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstancePolymerRepeatRepeatUnit {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Structural repeat units are essential elements for defining polymers
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,

    /// The orientation of the polymerisation, e.g. head-tail, head-head,
    /// random
    pub orientation: Option<types::CodeableConcept>,

    /// Number of repeats of this unit
    pub amount: Option<types::Integer>,
    /// Primitive extension sibling for [`amount`](Self::amount) (FHIR `_amount`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_amount")]
    pub amount_ext: Option<types::Element>,

    /// Applies to homopolymer and block co-polymers where the degree of
    /// polymerisation within a block can be described
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub degree_of_polymerisation: Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,

    /// A graphical structure for this SRU
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub structural_representation: Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
}

/// Applies to homopolymer and block co-polymers where the degree of
/// polymerisation within a block can be described.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_polymer::SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation;
/// use fhir::r6::types;
///
/// let value = SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
///     average: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `average` is the name this serializes to on the wire.
/// assert_eq!(json["average"], ::serde_json::json!(42));
///
/// let back: SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of the degree of polymerisation shall be described, e.g.
    /// SRU/Polymer Ratio
    pub r#type: Option<types::CodeableConcept>,

    /// An average amount of polymerisation
    pub average: Option<types::Integer>,
    /// Primitive extension sibling for [`average`](Self::average) (FHIR `_average`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_average")]
    pub average_ext: Option<types::Element>,

    /// A low expected limit of the amount
    pub low: Option<types::Integer>,
    /// Primitive extension sibling for [`low`](Self::low) (FHIR `_low`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_low")]
    pub low_ext: Option<types::Element>,

    /// A high expected limit of the amount
    pub high: Option<types::Integer>,
    /// Primitive extension sibling for [`high`](Self::high) (FHIR `_high`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_high")]
    pub high_ext: Option<types::Element>,
}

/// A graphical structure for this SRU.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_polymer::SubstancePolymerRepeatRepeatUnitStructuralRepresentation;
/// use fhir::r6::types;
///
/// let value = SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
///     representation: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `representation` is the name this serializes to on the wire.
/// assert_eq!(json["representation"], ::serde_json::json!("abc"));
///
/// let back: SubstancePolymerRepeatRepeatUnitStructuralRepresentation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of structure (e.g. Full, Partial, Representative)
    pub r#type: Option<types::CodeableConcept>,

    /// The structural representation as text string in a standard format e.g.
    /// InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF
    pub representation: Option<types::String>,
    /// Primitive extension sibling for [`representation`](Self::representation) (FHIR `_representation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_representation")]
    pub representation_ext: Option<types::Element>,

    /// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF,
    /// PDB, mmCIF
    pub format: Option<types::CodeableConcept>,

    /// An attached file with the structural representation
    pub attachment: Option<types::Attachment>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstancePolymer;

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
