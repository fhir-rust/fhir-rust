//! Uuid
//!
//! URL: http://hl7.org/fhir/StructureDefinition/uuid
//!
//! Version: 5.0.0
//!
//! uuid type: A UUID, represented as a URI
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A RFC 4122 UUID, represented as a URI, used to uniquely identify resources or elements.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A version 4 UUID (Universally Unique Identifier), represented as a URI in the
/// form `urn:uuid:...`. It is typically used as a logical identifier for resources
/// or elements when a globally unique value is needed but no other identifier scheme
/// applies, such as for temporary or locally-generated identifiers.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::uuid::Uuid;
///
/// let value = Uuid::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Uuid = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Uuid(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Uuid::default(), Uuid(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Uuid("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Uuid = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
