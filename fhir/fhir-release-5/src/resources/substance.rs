//! Substance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Substance
//!
//! Version: 5.0.0
//!
//! Substance Resource: A homogeneous material with a definite composition.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Substance
///
/// A homogeneous material with a definite composition. This resource captures
/// substances used in healthcare, whether as a specific physical instance (such
/// as a particular batch or package) or as a general kind of substance. It
/// records the substance code, category, quantity, expiry, and its constituent
/// ingredients along with their relative amounts.
///
/// Substances are used across many clinical and administrative workflows: as the
/// active or inactive ingredients that make up a medication, as allergens or
/// irritants referenced from an allergy or intolerance record, as specimens or
/// samples handled by a laboratory, or as raw materials tracked through supply
/// and manufacturing processes. A `Substance` instance may represent either a
/// specific physical package or lot (an "instance") or a general category of
/// material (a "kind"), as indicated by the `instance` field. The `code` field
/// identifies what the substance is, typically using a standard terminology
/// such as SNOMED CT or a national drug/substance coding system, while
/// `ingredient` describes the substance's own composition when it is itself a
/// mixture or compound.
///
/// # See also
///
/// Substances are commonly referenced from medication and clinical resources
/// such as `Medication`, `MedicationKnowledge`, and `AllergyIntolerance`, and
/// use shared data types including [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// [`CodeableReference`](crate::r5::types::CodeableReference), and
/// [`Quantity`](crate::r5::types::Quantity) to describe classification and amount.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance::Substance;
/// use fhir::r5::types;
///
/// let value = Substance {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: Substance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Substance {
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

    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// True if this represents a specific physical instance/package/lot of the substance rather than a general kind
    pub instance: types::Boolean,
    /// Primitive extension sibling for [`instance`](Self::instance) (FHIR `_instance`).
    #[serde(rename = "_instance")]
    pub instance_ext: Option<types::Element>,

    /// The status of the substance record: active | inactive | entered-in-error
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::SubstanceStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of the substance into one or more categories, e.g. drug, allergen, or biological
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// The substance's identity, coded using a terminology such as SNOMED CT, optionally with a direct reference
    pub code: types::CodeableReference,

    /// Textual description of the substance, comments
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// When no longer valid to use
    pub expiry: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiry`](Self::expiry) (FHIR `_expiry`).
    #[serde(rename = "_expiry")]
    pub expiry_ext: Option<types::Element>,

    /// Amount of substance in the package
    pub quantity: Option<types::Quantity>,

    /// Composition information about the substance, describing its constituent components and their relative amounts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<SubstanceIngredient>,
}

/// SubstanceIngredient
///
/// Composition information about the substance: a component of the substance and
/// its optional relative amount (concentration).
/// # Examples
///
/// ```
/// use fhir::r5::resources::substance::SubstanceIngredient;
/// use fhir::r5::types;
///
/// let value = SubstanceIngredient {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SubstanceIngredient = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Optional amount (concentration)
    pub quantity: Option<types::Ratio>,

    /// The `Substance.ingredient.substance[x]` choice element (0..1); see [`SubstanceIngredientSubstance`].
    #[serde(flatten)]
    pub substance: Option<SubstanceIngredientSubstance>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Substance;

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
/// The `Substance.ingredient.substance[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceIngredientSubstance {
    /// `substanceCodeableConcept` variant.
    #[fhir("substanceCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `substanceReference` variant.
    #[fhir("substanceReference")]
    Reference(Box<types::Reference>),
}
