//! Attachment
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Attachment
//!
//! Version: 5.0.0
//!
//! Attachment Type: For referring to data content defined in other formats.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This datatype represents binary or textual content, such as an image, PDF,
//! or document, either embedded inline as base64 data or referenced by URL.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Content in a defined format and encoding, either as inline base64 data or
/// as a reference to a URL where the content can be retrieved. It is used to
/// convey files such as images, PDFs, or other documents attached to a
/// resource, along with metadata like MIME type, size, and hash for
/// integrity checking.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::attachment::Attachment;
/// use fhir::r5::types;
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
pub struct Attachment {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// MIME type of the content, with charset etc.
    pub content_type: Option<types::Code>, // « MimeTypes! » « C »
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`).
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,
    /// Human language of the content.
    pub language: Option<types::Code>, // « AllLanguages! »
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,
    /// Data inline, base64 encoded.
    pub data: Option<types::Base64Binary>, // [0..1] « C »
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`).
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,
    /// URL where the data can be found.
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
    /// Number of bytes of content, if inline data was not provided.
    pub size: Option<types::Integer64>,
    /// Primitive extension sibling for [`size`](Self::size) (FHIR `_size`).
    #[serde(rename = "_size")]
    pub size_ext: Option<types::Element>,
    /// Hash of the data as SHA-1, base64 encoded.
    pub hash: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`hash`](Self::hash) (FHIR `_hash`).
    #[serde(rename = "_hash")]
    pub hash_ext: Option<types::Element>,
    /// Label to display in place of the data.
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,
    /// Date the attachment was first created.
    pub creation: Option<types::DateTime>,
    /// Primitive extension sibling for [`creation`](Self::creation) (FHIR `_creation`).
    #[serde(rename = "_creation")]
    pub creation_ext: Option<types::Element>,
    /// Height of the image in pixels (photo/video).
    pub height: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`height`](Self::height) (FHIR `_height`).
    #[serde(rename = "_height")]
    pub height_ext: Option<types::Element>,
    /// Width of the image in pixels (photo/video).
    pub width: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`width`](Self::width) (FHIR `_width`).
    #[serde(rename = "_width")]
    pub width_ext: Option<types::Element>,
    /// Number of frames if this is a graphic that includes multiple frames.
    pub frames: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`frames`](Self::frames) (FHIR `_frames`).
    #[serde(rename = "_frames")]
    pub frames_ext: Option<types::Element>,
    /// Length in seconds, if content is a recording.
    pub duration: Option<types::Decimal>,
    /// Primitive extension sibling for [`duration`](Self::duration) (FHIR `_duration`).
    #[serde(rename = "_duration")]
    pub duration_ext: Option<types::Element>,
    /// Number of printed pages.
    pub pages: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`pages`](Self::pages) (FHIR `_pages`).
    #[serde(rename = "_pages")]
    pub pages_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Attachment;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            content_type: None,
            language: None,
            data: None,
            url: None,
            size: None,
            hash: None,
            title: None,
            creation: None,
            height: None,
            width: None,
            frames: None,
            duration: None,
            pages: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
