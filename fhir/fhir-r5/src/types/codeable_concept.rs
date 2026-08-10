//! CodeableConcept
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CodeableConcept
//!
//! Version: 5.0.0
//!
//! CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A CodeableConcept pairs zero or more codings from terminologies with an optional plain text representation.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A concept represented by one or more codings from a terminology or ontology, optionally
/// accompanied by a plain text representation. CodeableConcept is used throughout FHIR whenever
/// a coded value may need to carry multiple equivalent codings (for example from different
/// code systems) or when no suitable code exists and free text must be used instead.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::codeable_concept::CodeableConcept;
/// use fhir::r5::types;
///
/// let value = CodeableConcept {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: CodeableConcept = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CodeableConcept {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// A code defined by a terminology system; multiple codings may represent the same concept.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coding: Vec<types::Coding>,

    /// A plain text representation of the concept, used when no coding fully captures the
    /// meaning or as a human-readable fallback alongside the codings.
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CodeableConcept;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            coding: vec![],
            text: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({
                "coding": []
            });
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
