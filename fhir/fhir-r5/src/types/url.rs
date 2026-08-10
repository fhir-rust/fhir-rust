//! Url
//!
//! URL: http://hl7.org/fhir/StructureDefinition/url
//!
//! Version: 5.0.0
//!
//! url type: A URI that is a literal reference
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A `url` is a FHIR primitive type used to hold a Uniform Resource Locator,
//! a literal web address such as an `http://` or `https://` link.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A URI that is a literal reference, such as a web address (RFC 3986),
/// used wherever a FHIR element needs to hold a URL value, for example
/// a resource's canonical address or a link to an external resource.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::url::Url;
///
/// let value = Url::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Url = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Url(
    /// The literal URL string value.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Url::default(), Url(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Url("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Url = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
