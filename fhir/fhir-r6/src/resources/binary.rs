//! Binary
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Binary
//!
//! Version: 6.0.0-ballot3
//!
//! Pure binary content defined by a format other than FHIR
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A resource that represents the data of a single raw artifact as digital
/// content accessible in its native format. A Binary resource can contain any
/// content, whether text, image, pdf, zip archive, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::binary::Binary;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Binary {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// MimeType of the binary content
    pub content_type: types::Code,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// Identifies another resource to use as proxy when enforcing access
    /// control
    pub security_context: Option<types::Reference>,

    /// The actual content
    pub data: Option<types::Base64Binary>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`):
    /// carries `id` and/or `extension` for the primitive value.
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
