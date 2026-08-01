//! Element
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Element
//!
//! Version: 5.0.0
//!
//! Element Type: Base definition for all elements in a resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The base definition for all elements contained in a resource. Every FHIR
/// element carries an optional `id` for inter-element referencing and an
/// optional set of `extension`s that add data not present in the base
/// definition.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::element::Element;
/// use fhir::r5::types;
///
/// let value = Element {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Element = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// Unique id for inter-element referencing.
    pub id: Option<types::String>,

    /// Additional content defined by implementations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Element;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            id: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
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
