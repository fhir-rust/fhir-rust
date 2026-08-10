//! Oid
//!
//! URL: http://hl7.org/fhir/StructureDefinition/oid
//!
//! Version: 5.0.0
//!
//! oid type: An OID represented as a URI
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// An OID (Object Identifier) represented as a URI, using the `urn:oid:` scheme.
///
/// This primitive type is used for FHIR elements that reference globally unique
/// identifiers registered under the ISO/ITU-T OID hierarchy, such as coding
/// systems or other externally assigned identifiers. The value is a string in
/// the form `urn:oid:1.2.3.4.5`.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::oid::Oid;
///
/// let value = Oid::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Oid = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Oid(
    /// The OID value as a URI string, e.g. `urn:oid:1.2.3.4.5`.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Oid::default(), Oid(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Oid("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Oid = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
