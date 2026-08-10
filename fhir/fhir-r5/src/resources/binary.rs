//! Binary
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Binary
//!
//! Version: 5.0.0
//!
//! Binary Resource: A resource that represents the data of a single raw artifact as digital content accessible in its native format.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A resource that represents the data of a single raw artifact as digital
/// content accessible in its native format.
///
/// A Binary resource can contain any content, whether text, image, pdf, zip
/// archive, etc. Unlike most FHIR resources it is not modeled as structured
/// data but instead carries an opaque payload identified by a MIME content
/// type. In FHIR R5 it is typically used to serve document attachments,
/// images, or other blobs that other resources reference.
///
/// Binary is unusual among FHIR resources in that it has no narrative and
/// almost no structured metadata; its identity is essentially the pairing
/// of a MIME type with a payload. Other resources typically point at a
/// Binary indirectly through an `Attachment.url` or a `Reference`, rather
/// than embedding the raw bytes inline, which keeps larger payloads (such
/// as scanned documents, photographs, or PDFs) out of the primary clinical
/// or administrative resource. Because access to the raw content may need
/// to be governed independently of the referencing resource, Binary also
/// carries an optional `security_context` that servers can use to apply
/// access control consistent with another resource, such as a
/// [`Patient`](crate::r5::resources::patient::Patient) or a `Document`.
///
/// See also: `DocumentReference` and `Media`, which commonly reference a
/// Binary for their underlying content, and `Attachment`, the data type
/// often used elsewhere to carry a URL pointing at a Binary resource.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::binary::Binary;
/// use fhir::r5::types;
///
/// let value = Binary {
///     data: Some(types::Base64Binary("YWJj".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `data` is the name this serializes to on the wire.
/// assert_eq!(json["data"], ::serde_json::json!("YWJj"));
///
/// let back: Binary = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Binary {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// MimeType of the binary content, expressed as a MIME/media type such as `image/png` or `application/pdf`
    pub content_type: types::Code,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`).
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// Identifies another resource, such as a Patient, whose access-control policy should govern this content
    pub security_context: Option<types::Reference>,

    /// The actual content, base64 encoded, that constitutes this raw artifact
    pub data: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`).
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Binary;

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
