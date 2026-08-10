//! ProdCharacteristic
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ProdCharacteristic
//!
//! Version: 4.3.0
//!
//! The marketing status describes the date when a medicinal product is
//! actually put on the market or the date as of which it is no longer
//! available
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ProdCharacteristic Type: The marketing status
/// describes the date when a medicinal product is actually put on the market
/// or the date as of which it is no longer available.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::prod_characteristic::ProdCharacteristic;
/// use fhir::r4b::types;
///
/// let value = ProdCharacteristic {
///     shape: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `shape` is the name this serializes to on the wire.
/// assert_eq!(json["shape"], ::serde_json::json!("abc"));
///
/// let back: ProdCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct ProdCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Where applicable, the height can be specified using a numerical value
    /// and its unit of measurement The unit of measurement shall be specified
    /// in accordance with ISO 11240 and the resulting terminology The symbol
    /// and the symbol identifier shall be used
    pub height: Option<types::Quantity>,

    /// Where applicable, the width can be specified using a numerical value
    /// and its unit of measurement The unit of measurement shall be specified
    /// in accordance with ISO 11240 and the resulting terminology The symbol
    /// and the symbol identifier shall be used
    pub width: Option<types::Quantity>,

    /// Where applicable, the depth can be specified using a numerical value
    /// and its unit of measurement The unit of measurement shall be specified
    /// in accordance with ISO 11240 and the resulting terminology The symbol
    /// and the symbol identifier shall be used
    pub depth: Option<types::Quantity>,

    /// Where applicable, the weight can be specified using a numerical value
    /// and its unit of measurement The unit of measurement shall be specified
    /// in accordance with ISO 11240 and the resulting terminology The symbol
    /// and the symbol identifier shall be used
    pub weight: Option<types::Quantity>,

    /// Where applicable, the nominal volume can be specified using a numerical
    /// value and its unit of measurement The unit of measurement shall be
    /// specified in accordance with ISO 11240 and the resulting terminology
    /// The symbol and the symbol identifier shall be used
    pub nominal_volume: Option<types::Quantity>,

    /// Where applicable, the external diameter can be specified using a
    /// numerical value and its unit of measurement The unit of measurement
    /// shall be specified in accordance with ISO 11240 and the resulting
    /// terminology The symbol and the symbol identifier shall be used
    pub external_diameter: Option<types::Quantity>,

    /// Where applicable, the shape can be specified An appropriate controlled
    /// vocabulary shall be used The term and the term identifier shall be used
    pub shape: Option<types::String>,
    /// Primitive extension sibling for [`shape`](Self::shape) (FHIR `_shape`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_shape")]
    pub shape_ext: Option<types::Element>,

    /// Where applicable, the color can be specified An appropriate controlled
    /// vocabulary shall be used The term and the term identifier shall be used
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub color: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`color`](Self::color) (FHIR `_color`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_color")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub color_ext: Vec<Option<types::Element>>,

    /// Where applicable, the imprint can be specified as text
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub imprint: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`imprint`](Self::imprint) (FHIR `_imprint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_imprint")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub imprint_ext: Vec<Option<types::Element>>,

    /// Where applicable, the image can be provided The format of the image
    /// attachment shall be specified by regional implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<types::Attachment>,

    /// Where applicable, the scoring can be specified An appropriate
    /// controlled vocabulary shall be used The term and the term identifier
    /// shall be used
    pub scoring: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ProdCharacteristic;

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
