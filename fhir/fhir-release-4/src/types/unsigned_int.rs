//! unsignedInt
//!
//! URL: http://hl7.org/fhir/StructureDefinition/unsignedInt
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for unsignedInt type: An integer with a value that
/// is not negative (e.g. >= 0)
///
/// # Examples
///
/// ```
/// use fhir::r4::types::unsigned_int::UnsignedInt;
///
/// let value = UnsignedInt::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: UnsignedInt = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnsignedInt(
    /// The value, which the specification constrains to be zero or greater.
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
        let value = UnsignedInt(0);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(0));
        let back: UnsignedInt = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
