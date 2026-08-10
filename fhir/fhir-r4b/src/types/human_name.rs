//! HumanName
//!
//! URL: http://hl7.org/fhir/StructureDefinition/HumanName
//!
//! Version: 4.3.0
//!
//! Name of a human - parts and usage
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for HumanName Type: A human's name with the
/// ability to identify parts and usage.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::human_name::HumanName;
/// use fhir::r4b::types;
///
/// let value = HumanName {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: HumanName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct HumanName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// usual | official | temp | nickname | anonymous | old | maiden
    pub r#use: Option<crate::coded::Coded<crate::r4b::codes::NameUse>>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Text representation of the full name
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Family name (often called 'Surname')
    pub family: Option<types::String>,
    /// Primitive extension sibling for [`family`](Self::family) (FHIR `_family`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_family")]
    pub family_ext: Option<types::Element>,

    /// Given names (not always 'first'). Includes middle names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub given: Vec<types::String>,
    /// Primitive extension sibling for [`given`](Self::given) (FHIR `_given`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_given")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub given_ext: Vec<Option<types::Element>>,

    /// Parts that come before the name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prefix: Vec<types::String>,
    /// Primitive extension sibling for [`prefix`](Self::prefix) (FHIR `_prefix`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_prefix")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prefix_ext: Vec<Option<types::Element>>,

    /// Parts that come after the name
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suffix: Vec<types::String>,
    /// Primitive extension sibling for [`suffix`](Self::suffix) (FHIR `_suffix`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_suffix")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suffix_ext: Vec<Option<types::Element>>,

    /// Time period when name was/is in use
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = HumanName;

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
