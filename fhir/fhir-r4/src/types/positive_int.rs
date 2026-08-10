//! positiveInt
//!
//! URL: http://hl7.org/fhir/StructureDefinition/positiveInt
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for positiveInt type: An integer with a value that
/// is positive (e.g. >0)
///
/// # Examples
///
/// ```
/// use fhir::r4::types::positive_int::PositiveInt;
///
/// let value = PositiveInt::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: PositiveInt = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PositiveInt(
    /// The value, which the specification constrains to be greater than zero.
    pub u32,
);

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
        let value = PositiveInt(1);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(1));
        let back: PositiveInt = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
