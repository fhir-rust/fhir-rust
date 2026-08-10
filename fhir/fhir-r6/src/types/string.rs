//! string
//!
//! URL: http://hl7.org/fhir/StructureDefinition/string
//!
//! Version: 6.0.0-ballot3
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

use ::serde::{Deserialize, Serialize};

/// string Type: A sequence of Unicode characters
///
/// # Examples
///
/// ```
/// use fhir::r6::types::string::String;
///
/// let value = String::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: String = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct String(
    /// The underlying Unicode string value.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(String::default(), String(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = String("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: String = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
