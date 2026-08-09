//! Identifier
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Identifier
//!
//! Version: 5.0.0
//!
//! Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Identifier is a FHIR R5 complex datatype that identifies some entity
/// uniquely and unambiguously. It is most commonly used to carry business
/// identifiers such as medical record numbers, order numbers, and driver's
/// license numbers, distinguishing them from the technical logical identifiers
/// assigned by FHIR servers. Each Identifier can pin the value to a namespace
/// via `system`, classify its role via `use` and `type`, and record the period
/// of validity and the organization that assigned it.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::identifier::Identifier;
/// use fhir::r5::types;
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
pub struct Identifier {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// usual | official | temp | secondary | old (If known)
    pub r#use: Option<crate::r5::coded::Coded<crate::r5::codes::IdentifierUse>>,
    /// Primitive extension sibling for [`use`](Self::r#use) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Description of identifier
    pub r#type: Option<types::CodeableConcept>,

    /// The namespace for the identifier value
    pub system: Option<types::Uri>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// The value that is unique
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Time period when id is/was valid for use
    pub period: Option<types::Period>,

    /// Organization that issued id (may be just text)
    pub assigner: Option<types::Reference<crate::r5::resources::Organization>>,
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
