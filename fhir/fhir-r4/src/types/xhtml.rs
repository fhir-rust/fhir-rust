//! xhtml
//!
//! URL: http://hl7.org/fhir/StructureDefinition/xhtml
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for xhtml Type
///
/// # Examples
///
/// ```
/// use fhir::r4::types::xhtml::Xhtml;
///
/// let value = Xhtml::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Xhtml = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Xhtml(
    /// The XHTML fragment, which must be a well-formed `<div>`.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Xhtml::default(), Xhtml(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Xhtml("<div xmlns=\"http://www.w3.org/1999/xhtml\"/>".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("<div xmlns=\"http://www.w3.org/1999/xhtml\"/>"));
        let back: Xhtml = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
