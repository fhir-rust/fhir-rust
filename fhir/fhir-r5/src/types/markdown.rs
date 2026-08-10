//! Markdown
//!
//! URL: http://hl7.org/fhir/StructureDefinition/markdown
//!
//! Version: 5.0.0
//!
//! markdown type: A string that may contain Github Flavored Markdown syntax for optional processing by a mark down presentation engine
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};

/// A FHIR primitive datatype representing a string of text that may contain
/// GitHub Flavored Markdown syntax, intended for optional rendering by a
/// markdown-aware presentation engine. It is used wherever richer, human
/// authored text (such as descriptions, purposes, or usage notes) needs to
/// support basic formatting like emphasis, lists, and links.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::markdown::Markdown;
///
/// let value = Markdown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: Markdown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Markdown(
    /// The raw markdown-formatted text content.
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
        let value = Markdown("abc".to_string());
        let json = ::serde_json::to_value(&value).expect("to_value");
        assert_eq!(json, json!("abc"));
        let back: Markdown = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
