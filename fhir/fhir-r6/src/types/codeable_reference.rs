//! CodeableReference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CodeableReference
//!
//! Version: 6.0.0-ballot3
//!
//! Reference to a resource or a concept
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// CodeableReference Type: A reference to a resource (by instance), or
/// instead, a reference to a concept defined in a terminology or ontology (by
/// class).
///
/// # Examples
///
/// ```
/// use fhir::r6::types::codeable_reference::CodeableReference;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
