//! Xhtml
//!
//! URL: http://hl7.org/fhir/StructureDefinition/xhtml
//!
//! Version: 5.0.0
//!
//! xhtml Type definition
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! The xhtml type is a constrained XHTML fragment used mainly to carry the
//! human-readable narrative (`Narrative.div`) of a resource.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A limited subset of XHTML content, serialized as a string, that is used
/// to hold human-readable markup such as a resource's rendered narrative.
/// It must be valid XHTML content restricted to a safe subset of elements
/// and attributes, with no active/executable content permitted.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::xhtml::Xhtml;
///
/// let value = Xhtml::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Xhtml = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Xhtml(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Xhtml::default(), Xhtml(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Xhtml("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Xhtml = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
