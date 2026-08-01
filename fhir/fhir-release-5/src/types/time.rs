//! Time
//!
//! URL: http://hl7.org/fhir/StructureDefinition/time
//!
//! Version: 5.0.0
//!
//! time Type: A time during the day, with no date specified
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This datatype wraps a plain string holding a time of day, independent of any date or timezone.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A time during the day, in the format `hh:mm:ss`, with no date or timezone
/// specified. Fractions of a second may be included to an arbitrary
/// precision. This is used for FHIR elements that record a time of day
/// independent of the day it occurs on, such as a recurring daily schedule.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::time::Time;
///
/// let value = Time::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Time = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Time(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Time::default(), Time(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Time("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Time = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
