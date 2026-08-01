//! CodeableReference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CodeableReference
//!
//! Version: 5.0.0
//!
//! CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A reference to a resource (by instance), or instead, a reference to a
/// concept defined in a terminology or ontology (by class).
///
/// The CodeableReference datatype allows an element to convey either a coded
/// concept, a literal/logical reference to a resource, or both at once. It is
/// used throughout FHIR R5 wherever an element may point to an actual resource
/// instance or, alternatively, describe the intended target abstractly using a
/// terminology or ontology concept.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::codeable_reference::CodeableReference;
/// use fhir::r5::types;
///
/// let value = CodeableReference {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CodeableReference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CodeableReference {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Reference to a concept (by class)
    pub concept: Option<types::CodeableConcept>,

    /// Reference to a resource (by instance)
    pub reference: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CodeableReference;

    #[test]
    fn test_default() {
        let _ = T::default();
    }

    #[test]
    fn test_serde_round_trip() {
        let value = T::default();
        let json = ::serde_json::to_value(&value).expect("to_value");
        let back: T = ::serde_json::from_value(json).expect("from_value");
        assert_eq!(value, back);
    }
}
