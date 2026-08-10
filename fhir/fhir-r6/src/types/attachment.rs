//! Attachment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Attachment
//!
//! Version: 6.0.0-ballot3
//!
//! Content in a format defined elsewhere
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Attachment Type: For referring to data content defined in other formats.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::attachment::Attachment;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct Attachment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
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
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Number of bytes of content (if url provided)
    pub size: Option<types::Integer64>,
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

    /// Height of the image in pixels (photo/video)
    pub height: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`height`](Self::height) (FHIR `_height`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_height")]
    pub height_ext: Option<types::Element>,

    /// Width of the image in pixels (photo/video)
    pub width: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`width`](Self::width) (FHIR `_width`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_width")]
    pub width_ext: Option<types::Element>,

    /// Number of frames if > 1 (photo)
    pub frames: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`frames`](Self::frames) (FHIR `_frames`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_frames")]
    pub frames_ext: Option<types::Element>,

    /// Length in seconds (audio / video)
    pub duration: Option<types::Decimal>,
    /// Primitive extension sibling for [`duration`](Self::duration) (FHIR `_duration`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_duration")]
    pub duration_ext: Option<types::Element>,

    /// Number of printed pages
    pub pages: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`pages`](Self::pages) (FHIR `_pages`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_pages")]
    pub pages_ext: Option<types::Element>,
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
