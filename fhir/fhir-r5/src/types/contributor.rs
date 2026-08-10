//! Contributor
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Contributor
//!
//! Version: 5.0.0
//!
//! Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A contributor to the content of a knowledge asset, including authors,
/// editors, reviewers, and endorsers.
///
/// The `Contributor` datatype captures attribution for a knowledge asset by
/// naming an individual or organization and describing the kind of contribution
/// they made (author, editor, reviewer, or endorser). It also carries contact
/// details so the contributor can be reached. It is commonly used in metadata
/// resources such as knowledge artifacts, guidelines, and measures.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::contributor::Contributor;
/// use fhir::r5::types;
///
/// let value = Contributor {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Contributor = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// author | editor | reviewer | endorser
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ContributorType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Who contributed the content
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Contact details of the contributor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Contributor;

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
