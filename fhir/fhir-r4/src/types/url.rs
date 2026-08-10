//! url
//!
//! URL: http://hl7.org/fhir/StructureDefinition/url
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for url type: A URI that is a literal reference
///
/// # Examples
///
/// ```
/// use fhir::r4::types::url::Url;
///
/// let value = Url::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Url = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Url(
    /// The URL.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Url::default(), Url(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Url("http://example.org".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("http://example.org"));
        let back: Url = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
