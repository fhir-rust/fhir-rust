//! MedicinalProductInteraction
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicinalProductInteraction
//!
//! Version: 4.0.1
//!
//! MedicinalProductInteraction
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The interactions of the medicinal product with other medicinal products, or
/// other forms of interactions.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_interaction::MedicinalProductInteraction;
/// use fhir::r4::types;
///
/// let value = MedicinalProductInteraction {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductInteraction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductInteraction {
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

    /// The medication for which this is a described interaction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// The interaction described
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The specific medication, food or laboratory test that interacts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interactant: Vec<MedicinalProductInteractionInteractant>,

    /// The type of the interaction e.g. drug-drug interaction, drug-food
    /// interaction, drug-lab test interaction
    pub r#type: Option<types::CodeableConcept>,

    /// The effect of the interaction, for example "reduced gastric absorption
    /// of primary medication"
    pub effect: Option<types::CodeableConcept>,

    /// The incidence of the interaction, e.g. theoretical, observed
    pub incidence: Option<types::CodeableConcept>,

    /// Actions for managing the interaction
    pub management: Option<types::CodeableConcept>,
}

/// The specific medication, food or laboratory test that interacts.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::medicinal_product_interaction::MedicinalProductInteractionInteractant;
/// use fhir::r4::types;
///
/// let value = MedicinalProductInteractionInteractant {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicinalProductInteractionInteractant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct MedicinalProductInteractionInteractant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific medication, food or laboratory test that interacts
    /// The `MedicinalProductInteraction.interactant.item[x]` choice element (1..1); see [`MedicinalProductInteractionInteractantItem`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub item: Option<MedicinalProductInteractionInteractantItem>,
}

/// The `MedicinalProductInteraction.interactant.item[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum MedicinalProductInteractionInteractantItem {
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicinalProductInteraction;

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
