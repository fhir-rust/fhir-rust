//! canonical
//!
//! URL: http://hl7.org/fhir/StructureDefinition/canonical
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for canonical type: A URI that is a reference to a
/// canonical URL on a FHIR resource
///
/// # Examples
///
/// ```
/// use fhir::r4::types::canonical::Canonical;
///
/// let value = Canonical::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Canonical = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Canonical(
    /// The canonical URL, optionally with a `|version` suffix.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Canonical::default(), Canonical(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Canonical("http://example.org/vs".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("http://example.org/vs"));
        let back: Canonical = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
