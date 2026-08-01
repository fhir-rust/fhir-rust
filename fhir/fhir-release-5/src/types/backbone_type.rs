//! BackboneType
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BackboneType
//!
//! Version: 5.0.0
//!
//! BackboneType Type: Base definition for the few data types that are allowed to carry modifier extensions.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This is the base type used by complex datatypes that need to support modifier extensions.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base definition for the few FHIR data types that are allowed to carry
/// modifier extensions. Complex datatypes that represent a "backbone"
/// element extend this type so that implementations must not ignore any
/// modifier extension present, since doing so could change the meaning of
/// the data. It is not used directly as a standalone datatype.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::backbone_type::BackboneType;
/// use fhir::r5::types;
///
/// let value = BackboneType {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BackboneType = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct BackboneType {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// Extensions that cannot be ignored even if unrecognized, altering the meaning of the containing element.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = BackboneType;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            modifier_extension: vec![],
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!(
                {
                    "modifierExtension": []
                }
            );
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
