//! PositiveInt
//!
//! URL: http://hl7.org/fhir/StructureDefinition/positiveInt
//!
//! Version: 5.0.0
//!
//! positiveInt type: An integer with a value that is positive (e.g. >0)
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A primitive FHIR datatype wrapping a 32-bit unsigned integer restricted to positive values.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// An integer with a value that must be positive (i.e. greater than zero), used for FHIR
/// elements such as repeat counts, durations, or ordinal positions where zero or negative
/// values are not meaningful. Serializes as a plain JSON number.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::positive_int::PositiveInt;
///
/// let value = PositiveInt::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PositiveInt = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PositiveInt(pub u32);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(PositiveInt::default(), PositiveInt(0));
    }

    #[test]
    fn test_serde() {
        let value = PositiveInt(7);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(7));
        let back: PositiveInt = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
