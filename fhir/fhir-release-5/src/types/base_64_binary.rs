//! Base64Binary
//!
//! URL: http://hl7.org/fhir/StructureDefinition/base64Binary
//!
//! Version: 5.0.0
//!
//! base64Binary Type: A stream of bytes
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This module defines the base64Binary primitive type, used to carry raw binary content encoded as base64 text.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A stream of bytes, base64 encoded as a string. This FHIR primitive type is used
/// wherever an element needs to carry arbitrary binary content, such as attachments,
/// signatures, or other opaque data, in a JSON- and XML-safe textual form.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::base_64_binary::Base64Binary;
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
        let value = Base64Binary("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Base64Binary = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
