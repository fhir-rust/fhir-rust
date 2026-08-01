//! Reference
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Reference
//!
//!
//!
//! A reference from one resource to another
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Reference Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::reference::Reference;
/// use fhir::r3::types;
///
/// let value = Reference {
///     reference: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `reference` is the name this serializes to on the wire.
/// assert_eq!(json["reference"], ::serde_json::json!("abc"));
///
/// let back: Reference = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Reference {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<types::String>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Logical reference, when literal reference is not known
    pub identifier: Option<types::Identifier>,

    /// Text alternative for the resource
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Reference;

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
