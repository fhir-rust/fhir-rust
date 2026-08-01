//! date
//!
//! URL: http://hl7.org/fhir/StructureDefinition/date
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for date Type: A date or partial date (e.g. just
/// year or year + month). There is no time zone. The format is a union of the
/// schema types gYear, gYearMonth and date. Dates SHALL be valid dates.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::date::Date;
///
/// let value = Date::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Date = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Date(
    /// The date, at whatever precision it was written.
    pub std::string::String,
);

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
        let value = Date("2019-11-01".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("2019-11-01"));
        let back: Date = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
