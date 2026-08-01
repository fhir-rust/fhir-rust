//! Coding
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Coding
//!
//! Version: 4.0.1
//!
//! A reference to a code defined by a terminology system
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Coding Type: A reference to a code defined by
/// a terminology system.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::coding::Coding;
/// use fhir::r4::types;
///
/// let value = Coding {
///     user_selected: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `userSelected` is the name this serializes to on the wire.
/// assert_eq!(json["userSelected"], ::serde_json::json!(true));
///
/// let back: Coding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct Coding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Identity of the terminology system
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Version of the system - if relevant
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Symbol in syntax defined by the system
    pub code: Option<types::Code>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Representation defined by the system
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// If this coding was chosen directly by the user
    pub user_selected: Option<types::Boolean>,
    /// Primitive extension sibling for [`user_selected`](Self::user_selected) (FHIR `_userSelected`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_userSelected")]
    pub user_selected_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Coding;

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
