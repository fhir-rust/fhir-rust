//! Money
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Money
//!
//! Version: 6.0.0-ballot3
//!
//! An amount of economic utility in some recognized currency
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Money Type: An amount of economic utility in some recognized currency.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::money::Money;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct Money {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Numerical value (with implicit precision)
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// ISO 4217 Currency Code
    pub currency: Option<types::Code>,
    /// Primitive extension sibling for [`currency`](Self::currency) (FHIR `_currency`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_currency")]
    pub currency_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Money;

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
