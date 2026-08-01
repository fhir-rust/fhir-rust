//! Boolean
//!
//! URL: http://hl7.org/fhir/StructureDefinition/boolean
//!
//! Version: 5.0.0
//!
//! boolean Type: Value of "true" or "false"
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A simple wrapper around a primitive true/false value, used throughout FHIR resources.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A FHIR primitive datatype representing a value of "true" or "false".
/// It is used wherever a resource or data type element needs to convey a simple
/// binary flag or condition, such as `active`, `deceased`, or `experimental`.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::boolean::Boolean;
///
/// let value = Boolean::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Boolean = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Boolean(pub bool);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Boolean::default(), Boolean(false));
    }

    #[test]
    fn test_serde() {
        let value = Boolean(true);
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!(true));
        let back: Boolean = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
