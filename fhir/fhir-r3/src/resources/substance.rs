//! Substance
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Substance
//!
//!
//!
//! A homogeneous material with a definite composition
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Substance Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::substance::Substance;
/// use fhir::r3::types;
///
/// let value = Substance {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: Substance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Substance {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | inactive | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r3::codes::SubstanceStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// What class/type of substance this is
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// What substance this is
    pub code: types::CodeableConcept,

    /// Textual description of the substance, comments
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// If this describes a specific package/container of the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<SubstanceInstance>,

    /// Composition information about the substance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<SubstanceIngredient>,
}

/// A substance can be composed of other substances.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::substance::SubstanceIngredient;
/// use fhir::r3::types;
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
#[serde(from = "SubstanceIngredientDe")]
#[fhir_version("r3")]
pub struct SubstanceIngredient {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Optional amount (concentration)
    pub quantity: Option<types::Ratio>,

    /// A component of the substance
    /// The `Substance.ingredient.substance[x]` choice element (1..1); see [`SubstanceIngredientSubstance`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub substance: Option<SubstanceIngredientSubstance>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubstanceIngredientDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    quantity: Option<types::Ratio>,
    #[serde(flatten)]
    substance: crate::r3::choice::Slot<SubstanceIngredientSubstance>,
}

impl ::core::convert::From<SubstanceIngredientDe> for SubstanceIngredient {
    fn from(v: SubstanceIngredientDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            quantity: v.quantity,
            substance: v.substance.0,
        }
    }
}

/// Substance may be used to describe a kind of substance, or a specific
/// package/container of the substance: an instance.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::substance::SubstanceInstance;
/// use fhir::r3::types;
///
/// let value = SubstanceInstance {
///     expiry: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `expiry` is the name this serializes to on the wire.
/// assert_eq!(json["expiry"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: SubstanceInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct SubstanceInstance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier of the package/container
    pub identifier: Option<types::Identifier>,

    /// When no longer valid to use
    pub expiry: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiry`](Self::expiry) (FHIR `_expiry`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expiry")]
    pub expiry_ext: Option<types::Element>,

    /// Amount of substance in the package
    pub quantity: Option<types::Quantity>,
}

/// The `Substance.ingredient.substance[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum SubstanceIngredientSubstance {
    /// `substanceCodeableConcept` variant.
    #[fhir("substanceCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `substanceReference` variant.
    #[fhir("substanceReference")]
    Reference(Box<types::Reference>),
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
