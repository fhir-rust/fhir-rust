//! String
//!
//! URL: http://hl7.org/fhir/StructureDefinition/string
//!
//! Version: 5.0.0
//!
//! string Type: A sequence of Unicode characters
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A primitive datatype for a sequence of Unicode characters used for short text values.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A FHIR primitive `string` value: a sequence of Unicode characters. Strings
/// are typically used for short human-readable text such as names, titles,
/// and identifiers, and should generally not exceed 1MB in size. Leading and
/// trailing whitespace is considered significant and is preserved.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::string::String;
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
