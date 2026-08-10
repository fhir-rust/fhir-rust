//! time
//!
//! URL: http://hl7.org/fhir/StructureDefinition/time
//!
//! Version: 6.0.0-ballot3
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

use ::serde::{Deserialize, Serialize};

/// time Type: A time during the day, with no date specified
///
/// # Examples
///
/// ```
/// use fhir::r6::types::time::Time;
///
/// let value = Time::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Time = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Time(
    /// The time of day, without a date or timezone.
    pub std::string::String,
);

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
        let value = Time("09:29:23".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("09:29:23"));
        let back: Time = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
