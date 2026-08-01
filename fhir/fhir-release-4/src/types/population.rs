//! Population
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Population
//!
//! Version: 4.0.1
//!
//! A definition of a set of people that apply to some clinically related
//! context, for example people contraindicated for a certain medication
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Population Type: A populatioof people with
/// some set of grouping criteria.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::population::Population;
/// use fhir::r4::types;
///
/// let value = Population {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Population = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Population {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The age of the specific population
    /// The `Population.age[x]` choice element (0..1); see [`PopulationAge`].
    #[serde(flatten)]
    pub age: Option<PopulationAge>,

    /// The gender of the specific population
    pub gender: Option<types::CodeableConcept>,

    /// Race of the specific population
    pub race: Option<types::CodeableConcept>,

    /// The existing physiological conditions of the specific population to
    /// which this applies
    pub physiological_condition: Option<types::CodeableConcept>,
}

/// The `Population.age[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum PopulationAge {
    /// `ageRange` variant.
    #[fhir("ageRange")]
    Range(Box<types::Range>),
    /// `ageCodeableConcept` variant.
    #[fhir("ageCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Population;

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
