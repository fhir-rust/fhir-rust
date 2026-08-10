//! Uri
//!
//! URL: http://hl7.org/fhir/StructureDefinition/uri
//!
//! Version: 5.0.0
//!
//! uri Type: String of characters used to identify a name or a resource
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A uri is a Uniform Resource Identifier used as a reference to a resource, profile, extension, or other addressable entity.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A FHIR `uri` primitive value: a Uniform Resource Identifier reference used to
/// uniquely identify or locate a resource, extension, code system, profile, or other
/// artifact. Values may be absolute URIs, URNs (e.g. `urn:oid:...`), or relative
/// references, and are compared as case-sensitive exact strings.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::uri::Uri;
///
/// let value = Uri::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Uri = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Uri(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Uri::default(), Uri(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Uri("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Uri = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
