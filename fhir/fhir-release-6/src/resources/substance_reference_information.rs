//! SubstanceReferenceInformation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SubstanceReferenceInformation
//!
//! Version: 6.0.0-ballot3
//!
//! Todo
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_reference_information::SubstanceReferenceInformation;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SubstanceReferenceInformation {
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

    /// Todo
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gene: Vec<SubstanceReferenceInformationGene>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gene_element: Vec<SubstanceReferenceInformationGeneElement>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<SubstanceReferenceInformationTarget>,
}

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_reference_information::SubstanceReferenceInformationGene;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_reference_information::SubstanceReferenceInformationGeneElement;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Todo.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::substance_reference_information::SubstanceReferenceInformationTarget;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct SubstanceReferenceInformationTarget {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Todo
    pub target: Option<types::Identifier>,

    /// Todo
    pub r#type: Option<types::CodeableConcept>,

    /// Todo
    pub interaction: Option<types::CodeableConcept>,

    /// Todo
    pub organism: Option<types::CodeableConcept>,

    /// Todo
    pub organism_type: Option<types::CodeableConcept>,

    /// Todo
    /// The `SubstanceReferenceInformation.target.amount[x]` choice element (0..1); see [`SubstanceReferenceInformationTargetAmount`].
    #[serde(flatten)]
    pub amount: Option<SubstanceReferenceInformationTargetAmount>,

    /// Todo
    pub amount_type: Option<types::CodeableConcept>,

    /// Todo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<types::Reference>,
}

/// The `SubstanceReferenceInformation.target.amount[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    String(crate::r6::choice::Primitive<types::String>),
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
