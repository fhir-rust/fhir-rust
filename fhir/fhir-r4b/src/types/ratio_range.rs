//! RatioRange
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RatioRange
//!
//! Version: 4.3.0
//!
//! Range of ratio values
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for RatioRange Type: A range of ratios expressed
/// as a low and high numerator and a denominator.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::ratio_range::RatioRange;
/// use fhir::r4b::types;
///
/// let value = RatioRange {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: RatioRange = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct RatioRange {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Low Numerator limit
    pub low_numerator: Option<types::Quantity>,

    /// High Numerator limit
    pub high_numerator: Option<types::Quantity>,

    /// Denominator value
    pub denominator: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RatioRange;

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
