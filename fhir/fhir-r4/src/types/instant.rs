//! instant
//!
//! URL: http://hl7.org/fhir/StructureDefinition/instant
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for instant Type: An instant in time - known at
/// least to the second
///
/// # Examples
///
/// ```
/// use fhir::r4::types::instant::Instant;
///
/// let value = Instant::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Instant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Instant(
    /// The instant, always to at least second precision with a timezone.
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
        let value = Instant("2019-11-01T09:29:23Z".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("2019-11-01T09:29:23Z"));
        let back: Instant = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
