//! SimpleQuantity
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SimpleQuantity
//!
//! Version: 4.3.0
//!
//! A fixed quantity (no comparator)
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A fixed quantity (no comparator)
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::simple_quantity::SimpleQuantity;
/// use fhir::r4b::types;
///
/// let value = SimpleQuantity {
///     unit: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `unit` is the name this serializes to on the wire.
/// assert_eq!(json["unit"], ::serde_json::json!("abc"));
///
/// let back: SimpleQuantity = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct SimpleQuantity {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Numerical value (with implicit precision)
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Unit representation
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,

    /// System that defines coded unit form
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Coded form of the unit
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SimpleQuantity;

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
