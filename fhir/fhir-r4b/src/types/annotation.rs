//! Annotation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Annotation
//!
//! Version: 4.3.0
//!
//! Text node with attribution
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Annotation Type: A text note which also
/// contains information about who made the statement and when.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::annotation::Annotation;
/// use fhir::r4b::types;
///
/// let value = Annotation {
///     time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `time` is the name this serializes to on the wire.
/// assert_eq!(json["time"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Annotation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "AnnotationDe")]
#[fhir_version("r4b")]
pub struct Annotation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Individual responsible for the annotation
    /// The `Annotation.author[x]` choice element (0..1); see [`AnnotationAuthor`].
    #[serde(flatten)]
    pub author: Option<AnnotationAuthor>,

    /// When the annotation was made
    pub time: Option<types::DateTime>,
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,

    /// The annotation - text content (as markdown)
    pub text: types::Markdown,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnnotationDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(flatten)]
    author: crate::r4b::choice::Slot<AnnotationAuthor>,
    time: Option<types::DateTime>,
    #[serde(rename = "_time")]
    time_ext: Option<types::Element>,
    text: types::Markdown,
    #[serde(rename = "_text")]
    text_ext: Option<types::Element>,
}

impl ::core::convert::From<AnnotationDe> for Annotation {
    fn from(v: AnnotationDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            author: v.author.0,
            time: v.time,
            time_ext: v.time_ext,
            text: v.text,
            text_ext: v.text_ext,
        }
    }
}

/// The `Annotation.author[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum AnnotationAuthor {
    /// `authorReference` variant.
    #[fhir("authorReference")]
    Reference(Box<types::Reference>),
    /// `authorString` variant.
    #[fhir("authorString")]
    String(crate::r4b::choice::Primitive<types::String>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Annotation;

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
