//! Instant
//!
//! URL: http://hl7.org/fhir/StructureDefinition/instant
//!
//! Version: 5.0.0
//!
//! instant Type: An instant in time - known at least to the second
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A FHIR primitive datatype for an instant in time, precise to at least the second and always including a timezone offset.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// An instant in time, known to at least the second and including a timezone offset, formatted per RFC 3339 (e.g. `2015-02-07T13:28:17.239+02:00`). It is used for elements such as resource meta timestamps and audit event recordings where precise, unambiguous point-in-time values are required.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::instant::Instant;
///
/// let value = Instant::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Instant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Instant(
    /// The string representation of the instant value, in RFC 3339 format with a mandatory timezone offset.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Instant::default(), Instant(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Instant("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Instant = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
