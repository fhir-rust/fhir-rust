//! Integer64
//!
//! URL: http://hl7.org/fhir/StructureDefinition/integer64
//!
//! Version: 5.0.0
//!
//! integer64 Type: A very large whole number
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A primitive datatype for a signed 64-bit whole number, larger than the range supported by the `integer` type.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

use ::serde_with::{DisplayFromStr, serde_as};

/// A signed 64-bit integer value. Used for FHIR elements that need whole numbers
/// larger than the 32-bit range supported by the `integer` type, such as very large
/// counts or identifiers. On the wire it is represented as a JSON string to avoid
/// precision loss in JSON number parsers.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::integer_64::Integer64;
///
/// let value = Integer64::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Integer64 = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer64(#[serde_as(as = "DisplayFromStr")] pub i64);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Integer64::default(), Integer64(0));
    }

    #[test]
    fn test_serde() {
        let value = Integer64(9_007_199_254_740_993);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("9007199254740993"));
        let back: Integer64 = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
