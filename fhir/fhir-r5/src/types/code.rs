//! Code
//!
//! URL: http://hl7.org/fhir/StructureDefinition/code
//!
//! Version: 5.0.0
//!
//! code type: A string which has at least one character and no leading or trailing whitespace and where there is no whitespace other than single spaces in the contents
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A restricted string value drawn from a defined set of codes, used throughout FHIR to represent coded elements such as status, kind, or category.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A FHIR primitive `code` value: a string with at least one character, no leading or
/// trailing whitespace, and no internal whitespace other than single spaces. Codes are
/// used to represent values from a constrained set, often bound to a FHIR value set,
/// such as a resource's `status` or `gender` element.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::code::Code;
///
/// let value = Code::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Code = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Code(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Code::default(), Code(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Code("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Code = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
