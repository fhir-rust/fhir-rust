//! ProductShelfLife
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ProductShelfLife
//!
//! Version: 5.0.0
//!
//! ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The shelf-life and storage information for a medicinal product item or
/// container can be described using this class.
///
/// `ProductShelfLife` is a complex datatype used within medicinal product
/// definitions to capture how long an item remains usable and under what
/// conditions it must be stored. It records the type of shelf life being
/// described, the duration of that period, and any special storage precautions,
/// each expressed using appropriate controlled vocabularies.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::product_shelf_life::ProductShelfLife;
/// use fhir::r5::types;
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
pub struct ProductShelfLife {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized.
    ///
    /// A modifier extension changes the meaning of the element that carries
    /// it, so a consumer that does not understand one must refuse the
    /// resource rather than proceed. Dropping them silently is the worst of
    /// both: the resource looks understood and no longer means what it said.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of shelf life being described, e.g. in-use or transformation
    /// shelf life, from an appropriate controlled vocabulary.
    pub r#type: Option<types::CodeableConcept>,

    /// The `ProductShelfLife.period[x]` choice element (0..1); see [`ProductShelfLifePeriod`].
    #[serde(flatten)]
    pub period: Option<ProductShelfLifePeriod>,

    /// Special precautions for storage of the item, if any.
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
/// The `ProductShelfLife.period[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ProductShelfLifePeriod {
    /// `periodDuration` variant.
    #[fhir("periodDuration")]
    Duration(Box<types::Duration>),
    /// `periodString` variant.
    #[fhir("periodString")]
    String(crate::r5::choice::Primitive<types::String>),
}

#[cfg(test)]
mod drift_tests {
    use super::*;

    /// `ProductShelfLife` had no `modifierExtension` at all. A modifier
    /// extension changes what the element means, so dropping one leaves a
    /// resource that reads as understood and is not.
    #[test]
    fn a_modifier_extension_survives() {
        let json = r#"{"modifierExtension":[{"url":"http://example.org/m","valueBoolean":true}]}"#;
        let parsed: ProductShelfLife = ::serde_json::from_str(json).expect("deserialize");
        assert_eq!(
            parsed.modifier_extension.len(),
            1,
            "modifierExtension dropped"
        );
        assert_eq!(::serde_json::to_string(&parsed).expect("serialize"), json);
    }
}
