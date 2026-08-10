//! RatioRange
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RatioRange
//!
//! Version: 5.0.0
//!
//! RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A range of ratios expressed as a low and high numerator over a common denominator, used where a quantity varies within bounds relative to another quantity (e.g. a dosage range per body weight).

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A range of ratios expressed as a low and high numerator and a denominator, such as
/// a minimum and maximum dose relative to body weight. Used when a ratio-based value can
/// vary within a range, rather than being fixed. Each numerator and the denominator are
/// expressed as simple quantities.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::ratio_range::RatioRange;
/// use fhir::r5::types;
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
pub struct RatioRange {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The lower limit of the numerator quantity, forming the low bound of the ratio range.
    pub low_numerator: Option<types::Quantity>, // Quantity(SimpleQuantity) [0..1] « C »
    /// The upper limit of the numerator quantity, forming the high bound of the ratio range.
    pub high_numerator: Option<types::Quantity>, // Quantity(SimpleQuantity) [0..1] « C »
    /// The denominator quantity that both the low and high numerators are ratioed against.
    pub denominator: Option<types::Quantity>, // Quantity(SimpleQuantity) [0..1] « C »
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RatioRange;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            low_numerator: None,
            high_numerator: None,
            denominator: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
