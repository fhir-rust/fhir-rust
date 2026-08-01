//! Count
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Count
//!
//! Version: 5.0.0
//!
//! Count Type: A measured amount (or an amount that can potentially be measured).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A measured amount (or an amount that can potentially be measured). Note that
/// measured amounts include amounts that are not precisely quantified, including
/// amounts involving arbitrary units and floating currencies. The `Count`
/// datatype is a specialization used to represent a whole number of discrete
/// items, and in FHIR R5 it carries the same structural fields as `Quantity`
/// (value, comparator, unit, system, and code).
///
/// # Examples
///
/// ```
/// use fhir::r5::types::count::Count;
/// use fhir::r5::types;
///
/// let value = Count {
///     unit: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `unit` is the name this serializes to on the wire.
/// assert_eq!(json["unit"], ::serde_json::json!("abc"));
///
/// let back: Count = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Numerical value (with implicit precision)
    pub value: Option<types::Decimal>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// < | <= | >= | > | ad - how to understand the value
    pub comparator: Option<crate::r5::coded::Coded<crate::r5::codes::QuantityComparator>>,
    /// Primitive extension sibling for [`comparator`](Self::comparator) (FHIR `_comparator`).
    #[serde(rename = "_comparator")]
    pub comparator_ext: Option<types::Element>,

    /// Unit representation
    pub unit: Option<types::String>,
    /// Primitive extension sibling for [`unit`](Self::unit) (FHIR `_unit`).
    #[serde(rename = "_unit")]
    pub unit_ext: Option<types::Element>,

    /// System that defines coded unit form
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Coded form of the unit
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Count;

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
