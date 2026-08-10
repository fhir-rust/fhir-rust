//! Quantity
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Quantity
//!
//! Version: 5.0.0
//!
//! Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary units and floating currencies.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A measured amount, or an amount that can potentially be measured. This
/// includes amounts that are not precisely quantified, such as amounts
/// involving arbitrary units or floating currencies. `Quantity` is a base
/// datatype used throughout FHIR resources for representing dosages,
/// observation results, and similar numeric measurements with units.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::quantity::Quantity;
/// use fhir::r5::types;
///
/// let value = Quantity {
///     unit: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `unit` is the name this serializes to on the wire.
/// assert_eq!(json["unit"], ::serde_json::json!("abc"));
///
/// let back: Quantity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The numeric value of the quantity, including any arbitrary precision.
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
    /// How the value should be understood, e.g. less than, greater than. // « QuantityComparator! »
    pub comparator: Option<crate::r5::coded::Coded<crate::r5::codes::QuantityComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`).
    #[serde(rename = "_comparator")]
    pub comparator_ext: Option<types::Element>,
    /// A human-readable form of the unit, as displayed to the user.
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`).
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,
    /// The identification of the system that provides the coded form of the unit. // « C »
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,
    /// A computer-processable form of the unit in the system, matching the system's code. // « C »
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Quantity;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            value: None,
            comparator: None,
            unit: None,
            system: None,
            code: None,
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
