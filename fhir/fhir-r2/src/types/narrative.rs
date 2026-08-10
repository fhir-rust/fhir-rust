//! Narrative
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Narrative
//!
//!
//!
//! A human-readable formatted text, including images
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Narrative Type
///
/// # Examples
///
/// ```
/// use fhir::r2::types::narrative::Narrative;
/// use fhir::r2::types;
///
/// let value = Narrative {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: Narrative = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Narrative {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// generated | extensions | additional | empty
    pub status: crate::coded::Coded<crate::r2::codes::NarrativeStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Limited xhtml content
    pub div: types::Xhtml,
    /// Primitive extension sibling for [`div`](Self::div) (FHIR `_div`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_div")]
    pub div_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Narrative;

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
