//! Range
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Range
//!
//! Version: 5.0.0
//!
//! Range Type: A set of ordered Quantities defined by a low and high limit.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A Range represents a bounded interval of quantities, expressed as a low and high limit, and is used wherever a value is best described as falling somewhere between two measured or observed quantities.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A set of ordered quantities defined by a low and high limit, used to express a value that
/// falls somewhere within a bounded interval, such as a normal reference range for an
/// observation or a dosage range. Both the low and high limits are simple quantities that
/// share the same unit of measure, and either limit may be omitted when the range is
/// open-ended.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::range::Range;
/// use fhir::r5::types;
///
/// let value = Range {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Range = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The low limit of the range; omitted if the range has no lower bound (0..1). // « C »
    pub low: Option<types::Quantity>,
    /// The high limit of the range; omitted if the range has no upper bound (0..1). // « C »
    pub high: Option<types::Quantity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Range;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            low: None,
            high: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            // Both bounds are optional (0..1), so an empty object is a valid Range.
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
