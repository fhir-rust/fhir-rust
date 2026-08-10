//! ProductShelfLife
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ProductShelfLife
//!
//! Version: 4.3.0
//!
//! The shelf-life and storage information for a medicinal product item or
//! container can be described using this class
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ProductShelfLife Type: The shelf-life and
/// storage information for a medicinal product item or container can be
/// described using this class.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::product_shelf_life::ProductShelfLife;
/// use fhir::r4b::types;
///
/// let value = ProductShelfLife {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ProductShelfLife = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ProductShelfLife {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier for the packaged Medicinal Product
    pub identifier: Option<types::Identifier>,

    /// This describes the shelf life, taking into account various scenarios
    /// such as shelf life of the packaged Medicinal Product itself, shelf life
    /// after transformation where necessary and shelf life after the first
    /// opening of a bottle, etc. The shelf life type shall be specified using
    /// an appropriate controlled vocabulary The controlled term and the
    /// controlled term identifier shall be specified
    pub r#type: types::CodeableConcept,

    /// The shelf life time period can be specified using a numerical value for
    /// the period of time and its unit of time measurement The unit of
    /// measurement shall be specified in accordance with ISO 11240 and the
    /// resulting terminology The symbol and the symbol identifier shall be
    /// used
    pub period: types::Quantity,

    /// Special precautions for storage, if any, can be specified using an
    /// appropriate controlled vocabulary The controlled term and the
    /// controlled term identifier shall be specified
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_precautions_for_storage: Vec<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ProductShelfLife;

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
