//! Identifier
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Identifier
//!
//!
//!
//! An identifier intended for computation
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Identifier Type
///
/// # Examples
///
/// ```
/// use fhir::r2::types::identifier::Identifier;
/// use fhir::r2::types;
///
/// let value = Identifier {
///     system: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `system` is the name this serializes to on the wire.
/// assert_eq!(json["system"], ::serde_json::json!("http://example.org"));
///
/// let back: Identifier = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Identifier {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// usual | official | temp | secondary (If known)
    pub r#use: Option<crate::coded::Coded<crate::r2::codes::IdentifierUse>>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Description of identifier
    pub r#type: Option<types::CodeableConcept>,

    /// The namespace for the identifier
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// The value that is unique
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Time period when id is/was valid for use
    pub period: Option<types::Period>,

    /// Organization that issued id (may be just text)
    pub assigner: Option<types::Reference<crate::r2::resources::Organization>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Identifier;

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
