//! SubstanceReferenceInformation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation
//!
//! Version: 5.0.0
//!
//! SubstanceReferenceInformation Resource: Todo.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// SubstanceReferenceInformation resource.
///
/// Captures supporting scientific reference information about a substance that
/// is not itself part of the substance's formal definition, such as the genes,
/// gene elements, and biological targets that the substance is known to
/// interact with. This information is commonly sourced from pharmacological
/// and genomic databases and is used to describe mechanism-of-action data,
/// pharmacogenomic associations, and target interactions for a substance in
/// medicinal product regulation and pharmaceutical data exchange. It is
/// typically referenced from, or referenced by, other substance and medicinal
/// product definition resources rather than being used as a standalone
/// clinical record.
///
/// # Related resources
///
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) is used throughout
///   this resource to code genes, targets, organisms, and interaction types.
/// - [`Reference`](crate::r5::types::Reference) is used to cite the source
///   material (e.g. literature or documents) supporting each gene, gene
///   element, or target entry.
/// - `SubstanceDefinition` and `MedicinalProductDefinition` are the resources
///   that most commonly link to this reference information.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_reference_information::SubstanceReferenceInformation;
/// use fhir::r5::types;
///
/// let value = SubstanceReferenceInformation {
///     comment: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comment` is the name this serializes to on the wire.
/// assert_eq!(json["comment"], ::serde_json::json!("abc"));
///
/// let back: SubstanceReferenceInformation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformation {
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

    /// Textual comment about the substance's reference information.
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Genes associated with the substance, such as those it modulates.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gene: Vec<SubstanceReferenceInformationGene>,

    /// Specific gene elements (e.g. codons, introns) related to the substance.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gene_element: Vec<SubstanceReferenceInformationGeneElement>,

    /// Biological targets the substance is known to interact with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<SubstanceReferenceInformationTarget>,
}

/// A gene associated with the substance, including its sequence origin.
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_reference_information::SubstanceReferenceInformationGene;
/// use fhir::r5::types;
///
/// let value = SubstanceReferenceInformationGene {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceReferenceInformationGene = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationGene {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub gene_sequence_origin: Option<types::CodeableConcept>,

    /// Todo
    pub gene: Option<types::CodeableConcept>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// Todo
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_reference_information::SubstanceReferenceInformationGeneElement;
/// use fhir::r5::types;
///
/// let value = SubstanceReferenceInformationGeneElement {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceReferenceInformationGeneElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationGeneElement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub r#type: Option<types::CodeableConcept>,

    /// Todo
    pub element: Option<types::Identifier>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// A biological target that the substance interacts with, and the nature of
/// that interaction.
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance_reference_information::SubstanceReferenceInformationTarget;
/// use fhir::r5::types;
///
/// let value = SubstanceReferenceInformationTarget {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceReferenceInformationTarget = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier for the biological target (e.g. a receptor or enzyme).
    pub target: Option<types::Identifier>,

    /// Type of the target, such as receptor, enzyme, or transporter.
    pub r#type: Option<types::CodeableConcept>,

    /// Type of interaction between the substance and the target.
    pub interaction: Option<types::CodeableConcept>,

    /// Todo
    pub organism: Option<types::CodeableConcept>,

    /// Todo
    pub organism_type: Option<types::CodeableConcept>,

    /// The `SubstanceReferenceInformation.target.amount[x]` choice element (0..1); see [`SubstanceReferenceInformationTargetAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceReferenceInformationTargetAmount>,

    /// Todo
    pub amount_type: Option<types::CodeableConcept>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SubstanceReferenceInformation;

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
/// The `SubstanceReferenceInformation.target.amount[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceReferenceInformationTargetAmount {
    /// `amountQuantity` variant.
    #[fhir("amountQuantity")]
    Quantity(Box<types::Quantity>),
    /// `amountRange` variant.
    #[fhir("amountRange")]
    Range(Box<types::Range>),
    /// `amountString` variant.
    #[fhir("amountString")]
    String(crate::r5::choice::Primitive<types::String>),
}
