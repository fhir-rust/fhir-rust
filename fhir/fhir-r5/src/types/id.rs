//! Id
//!
//! URL: http://hl7.org/fhir/StructureDefinition/id
//!
//! Version: 5.0.0
//!
//! id type: Any combination of letters, numerals, "-" and ".", with a length limit of 64 characters.  (This might be an integer, an unprefixed OID, UUID or any other identifier pattern that meets these constraints.)  Ids are case-insensitive.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A FHIR primitive datatype representing a short identifier string.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A FHIR `id` primitive: any combination of letters, numerals, "-" and ".",
/// limited to 64 characters and treated as case-insensitive. It is commonly
/// used for logical resource ids, as well as ids for extensions, elements,
/// and other constructs that need a short, constrained, machine-readable
/// identifier.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::id::Id;
///
/// let value = Id::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Id = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Id(
    /// The raw string value of the id.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Id::default(), Id(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Id("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Id = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
