//! PersonalRelationship
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PersonalRelationship
//!
//! Version: 6.0.0-ballot3
//!
//! A relationship between two indiduals
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Details about the relationship between 2 individuals.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::personal_relationship::PersonalRelationship;
/// use fhir::r6::types;
///
/// let value = PersonalRelationship {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PersonalRelationship = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PersonalRelationship {
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

    /// The individual that is the the source of the relationship
    pub source: types::Reference,

    /// The relationship between the source and the target individuals
    pub relationship_type: types::CodeableConcept,

    /// The individual that is the the target of the relationship
    pub target: types::Reference,

    /// The period of time during which the relationship is active
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub period: Vec<types::Period>,

    /// The confidence level of this relationship
    pub confidence: Option<types::CodeableConcept>,

    /// Who has asserted the details of the relationship
    pub asserter: Option<types::Reference>,

    /// This relationship is applicable to the referenced group(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub group: Vec<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PersonalRelationship;

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
