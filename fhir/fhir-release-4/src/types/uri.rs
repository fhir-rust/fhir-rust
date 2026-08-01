//! uri
//!
//! URL: http://hl7.org/fhir/StructureDefinition/uri
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for uri Type: String of characters used to
/// identify a name or a resource
///
/// # Examples
///
/// ```
/// use fhir::r4::types::uri::Uri;
///
/// let value = Uri::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Uri = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Uri(
    /// The URI, which may be absolute or relative.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Uri::default(), Uri(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Uri("http://example.org".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("http://example.org"));
        let back: Uri = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
