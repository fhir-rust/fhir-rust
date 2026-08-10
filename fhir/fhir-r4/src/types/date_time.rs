//! dateTime
//!
//! URL: http://hl7.org/fhir/StructureDefinition/dateTime
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for dateTime Type: A date, date-time or partial
/// date (e.g. just year or year + month). If hours and minutes are specified,
/// a time zone SHALL be populated. The format is a union of the schema types
/// gYear, gYearMonth, date and dateTime. Seconds must be provided due to
/// schema type constraints but may be zero-filled and may be ignored. Dates
/// SHALL be valid dates.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::date_time::DateTime;
///
/// let value = DateTime::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DateTime = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DateTime(
    /// The date and time, at whatever precision it was written.
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
        let value = DateTime("2019-11-01T09:29:23Z".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("2019-11-01T09:29:23Z"));
        let back: DateTime = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
