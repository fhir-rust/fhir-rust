//! Date
//!
//! URL: http://hl7.org/fhir/StructureDefinition/date
//!
//! Version: 5.0.0
//!
//! date Type: A date or partial date (e.g. just year or year + month). There is no UTC offset. The format is a union of the schema types gYear, gYearMonth and date.  Dates SHALL be valid dates.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A simple wrapper type representing a FHIR `date` value as its raw string form.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A date or partial date (e.g. just year, or year + month), with no time
/// component and no UTC offset. Precision may be to the year, year and
/// month, or full year-month-day, and the format is a union of the XML
/// schema `gYear`, `gYearMonth`, and `date` types. Dates SHALL be valid
/// dates. Used throughout FHIR resources for date-only elements such as
/// birth dates or event dates where a time is not applicable.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::date::Date;
///
/// let value = Date::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Date = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Date(pub std::string::String);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Date::default(), Date(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Date("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Date = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
