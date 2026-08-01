//! PrimitiveType
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PrimitiveType
//!
//! Version: 5.0.0
//!
//! PrimitiveType Type: The base type for all re-useable types defined that have a simple property.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// PrimitiveType is the abstract base for all re-useable FHIR types that carry a
/// single simple value (for example `string`, `boolean`, `integer`, and the
/// other primitive datatypes). It extends the base `Element` structure, so it
/// provides an optional `id` for inter-element referencing and a set of
/// `extension`s that let implementations attach additional content to any
/// primitive value. Concrete primitive datatypes derive from this type and add
/// their own `value` property.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::primitive_type::PrimitiveType;
/// use fhir::r5::types;
///
/// let value = PrimitiveType {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PrimitiveType = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct PrimitiveType {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PrimitiveType;

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
