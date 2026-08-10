//! integer64
//!
//! URL: http://hl7.org/fhir/StructureDefinition/integer64
//!
//! Version: 6.0.0-ballot3
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

use ::serde::{Deserialize, Serialize};
use ::serde_with::{DisplayFromStr, serde_as};

/// integer64 Type: A very large whole number
///
/// # Examples
///
/// ```
/// use fhir::r6::types::integer_64::Integer64;
///
/// let value = Integer64::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Integer64 = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Integer64(
    /// The signed 64-bit value.
    // FHIR carries this as a JSON string so that it survives
    // consumers whose numbers are 64-bit floats.
    #[serde_as(as = "DisplayFromStr")]
    pub i64,
);

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
        let value = Integer64(9_000_000_000);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("9000000000"));
        let back: Integer64 = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
