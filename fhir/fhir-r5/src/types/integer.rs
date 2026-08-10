//! Integer
//!
//! URL: http://hl7.org/fhir/StructureDefinition/integer
//!
//! Version: 5.0.0
//!
//! integer Type: A whole number
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A FHIR primitive datatype that wraps a signed 32-bit whole number.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A FHIR primitive `integer` value: a signed whole number, ranging from
/// -2,147,483,648 to 2,147,483,647 (a 32-bit signed integer). It is used
/// throughout FHIR resources for elements such as counts, sequence
/// numbers, and other discrete numeric quantities that do not carry
/// units or fractional precision.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::integer::Integer;
///
/// let value = Integer::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Integer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer(pub i32);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Integer::default(), Integer(0));
    }

    #[test]
    fn test_serde() {
        let value = Integer(42);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(42));
        let back: Integer = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
