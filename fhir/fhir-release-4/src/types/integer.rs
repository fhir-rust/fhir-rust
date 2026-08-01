//! integer
//!
//! URL: http://hl7.org/fhir/StructureDefinition/integer
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for integer Type: A whole number
///
/// # Examples
///
/// ```
/// use fhir::r4::types::integer::Integer;
///
/// let value = Integer::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Integer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer(
    /// The signed 32-bit value.
    pub i32,
);

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
