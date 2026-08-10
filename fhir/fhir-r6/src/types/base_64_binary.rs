//! base64Binary
//!
//! URL: http://hl7.org/fhir/StructureDefinition/base64Binary
//!
//! Version: 6.0.0-ballot3
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

use ::serde::{Deserialize, Serialize};

/// base64Binary Type: A stream of bytes
///
/// # Examples
///
/// ```
/// use fhir::r6::types::base_64_binary::Base64Binary;
///
/// let value = Base64Binary::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Base64Binary = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Base64Binary(
    /// The base64-encoded string representation of the binary content.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(
            Base64Binary::default(),
            Base64Binary(std::string::String::new())
        );
    }

    #[test]
    fn test_serde() {
        let value = Base64Binary("YWJj".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("YWJj"));
        let back: Base64Binary = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
