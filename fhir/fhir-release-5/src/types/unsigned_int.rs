//! UnsignedInt
//!
//! URL: http://hl7.org/fhir/StructureDefinition/unsignedInt
//!
//! Version: 5.0.0
//!
//! unsignedInt type: An integer with a value that is not negative (e.g. >= 0)
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A primitive datatype wrapping a whole number that must be zero or greater.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A whole number in the range 0..2,147,483,647, used for FHIR elements
/// such as counts, durations, and indices where negative values are not
/// meaningful. It serializes as a plain JSON number, not as a string.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::unsigned_int::UnsignedInt;
///
/// let value = UnsignedInt::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: UnsignedInt = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnsignedInt(
    /// The non-negative integer value.
    pub u32,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(UnsignedInt::default(), UnsignedInt(0));
    }

    #[test]
    fn test_serde() {
        let value = UnsignedInt(7);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(7));
        let back: UnsignedInt = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
