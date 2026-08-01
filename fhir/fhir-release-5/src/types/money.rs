//! Money
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Money
//!
//! Version: 5.0.0
//!
//! Money Type: An amount of economic utility in some recognized currency.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! Represents a monetary amount paired with an ISO 4217 currency code.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An amount of economic utility in some recognized currency, used
/// throughout FHIR resources to represent prices, costs, balances, and
/// other financial values. The numeric `value` is paired with a
/// `currency` code so the amount can be interpreted unambiguously.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::money::Money;
/// use fhir::r5::types;
///
/// let value = Money {
///     currency: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `currency` is the name this serializes to on the wire.
/// assert_eq!(json["currency"], ::serde_json::json!("final"));
///
/// let back: Money = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// Numerical value (with implicit precision).
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
    /// ISO 4217 currency code identifying the currency of the value.
    pub currency: Option<types::Code>, // « Currencies! »
    /// Primitive extension sibling for [`currency`](Self::currency) (FHIR `_currency`).
    #[serde(rename = "_currency")]
    pub currency_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Money;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            value: None,
            currency: None,
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
