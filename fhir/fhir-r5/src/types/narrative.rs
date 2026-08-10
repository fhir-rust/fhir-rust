//! Narrative
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Narrative
//!
//! Version: 5.0.0
//!
//! Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A human-readable summary of the resource conveying the essential clinical
/// and business information for the resource.
///
/// The `Narrative` datatype carries an XHTML-formatted rendering of a FHIR
/// resource so that its key information remains accessible to humans even when
/// a system cannot process every structured element. It pairs a generation
/// `status` code with an XHTML `div`, and is typically embedded as the `text`
/// element of a resource. Narratives are central to clinical safety and
/// interoperability because they guarantee a readable fallback view.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::narrative::Narrative;
/// use fhir::r5::types;
///
/// let value = Narrative {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Narrative = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Narrative {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// generated | extensions | additional | empty
    pub status: crate::r5::coded::Coded<crate::r5::codes::NarrativeStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Limited xhtml content
    pub div: types::Xhtml,
    /// Primitive extension sibling for [`div`](Self::div) (FHIR `_div`).
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
