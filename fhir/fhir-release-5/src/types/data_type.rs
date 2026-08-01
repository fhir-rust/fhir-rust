//! DataType
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DataType
//!
//! Version: 5.0.0
//!
//! DataType Type: The base class for all re-useable types defined as part of the FHIR Specification.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// DataType is the abstract base class for all reusable complex and primitive
/// datatypes defined as part of the FHIR specification. It sits between
/// `Element` and the concrete datatypes such as `Quantity`, `Period`, and
/// `CodeableConcept`, providing the common structural elements (an optional
/// `id` and a set of `extension`s) that every FHIR datatype inherits.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::data_type::DataType;
/// use fhir::r5::types;
///
/// let value = DataType {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DataType = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DataType {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DataType;

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
