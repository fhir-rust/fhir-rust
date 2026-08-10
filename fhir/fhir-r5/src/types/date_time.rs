//! DateTime
//!
//! URL: http://hl7.org/fhir/StructureDefinition/dateTime
//!
//! Version: 5.0.0
//!
//! dateTime Type: A date, date-time or partial date (e.g. just year or year + month).  If hours and minutes are specified, a UTC offset SHALL be populated. The format is a union of the schema types gYear, gYearMonth, date and dateTime. Seconds must be provided due to schema type constraints but may be zero-filled and may be ignored.                 Dates SHALL be valid dates.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A date, date-time, or partial date value used throughout FHIR resources to record when events occurred.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A date, date-time, or partial date (e.g. just a year, or a year and month).
///
/// If hours and minutes are specified, a UTC offset must be present. The
/// underlying value follows a union of the ISO 8601 gYear, gYearMonth, date,
/// and dateTime formats, and any date component present must be a valid
/// calendar date.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::date_time::DateTime;
///
/// let value = DateTime::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DateTime = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DateTime(
    /// The literal date/time string value, e.g. "2015-02-07T13:28:17-05:00".
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(DateTime::default(), DateTime(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = DateTime("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: DateTime = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
