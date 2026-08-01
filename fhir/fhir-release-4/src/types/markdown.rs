//! markdown
//!
//! URL: http://hl7.org/fhir/StructureDefinition/markdown
//!
//! Version: 4.0.1
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

use ::serde::{Deserialize, Serialize};

/// Base StructureDefinition for markdown type: A string that may contain
/// Github Flavored Markdown syntax for optional processing by a mark down
/// presentation engine
///
/// # Examples
///
/// ```
/// use fhir::r4::types::markdown::Markdown;
///
/// let value = Markdown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Markdown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Markdown(
    /// The Markdown source text.
    pub std::string::String,
);

#[cfg(test)]
mod tests {
    use super::*;
    use ::serde_json::json;

    #[test]
    fn test_default() {
        assert_eq!(Markdown::default(), Markdown(std::string::String::new()));
    }

    #[test]
    fn test_serde() {
        let value = Markdown("# Heading".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("# Heading"));
        let back: Markdown = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
