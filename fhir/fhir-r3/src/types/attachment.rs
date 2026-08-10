//! Attachment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Attachment
//!
//!
//!
//! Content in a format defined elsewhere
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Attachment Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::attachment::Attachment;
/// use fhir::r3::types;
///
/// let value = Attachment {
///     content_type: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `contentType` is the name this serializes to on the wire.
/// assert_eq!(json["contentType"], ::serde_json::json!("final"));
///
/// let back: Attachment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Attachment {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Mime type of the content, with charset etc.
    pub content_type: Option<types::Code>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// Human language of the content (BCP-47)
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Data inline, base64ed
    pub data: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,

    /// Uri where the data can be found
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Number of bytes of content (if url provided)
    pub size: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`size`](Self::size) (FHIR `_size`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_size")]
    pub size_ext: Option<types::Element>,

    /// Hash of the data (sha-1, base64ed)
    pub hash: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`hash`](Self::hash) (FHIR `_hash`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hash")]
    pub hash_ext: Option<types::Element>,

    /// Label to display in place of the data
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Date attachment was first created
    pub creation: Option<types::DateTime>,
    /// Primitive extension sibling for [`creation`](Self::creation) (FHIR `_creation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_creation")]
    pub creation_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Attachment;

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
