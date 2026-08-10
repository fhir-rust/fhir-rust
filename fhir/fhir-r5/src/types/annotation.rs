//! Annotation
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Annotation
//!
//! Version: 5.0.0
//!
//! Annotation Type: A  text note which also  contains information about who made the statement and when.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A text note that also records information about who made the statement
/// and when it was made. Annotations are used throughout FHIR resources to
/// capture free-text comments, clinical notes, or remarks alongside optional
/// attribution to an author and a timestamp. The author may be identified
/// either by a reference to a resource or by a plain text name.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::annotation::Annotation;
/// use fhir::r5::types;
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
pub struct Annotation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The individual responsible for the annotation (`author[x]` choice,
    /// 0..1): either a [`Reference`](types::Reference) to a Practitioner,
    /// PractitionerRole, Patient, RelatedPerson, or Organization
    /// (`authorReference`), or free-text [`String`](types::String)
    /// (`authorString`).
    #[serde(flatten)]
    pub author: Option<AnnotationAuthor>,

    /// The date and time this annotation was made. Cardinality 0..1, data
    /// type `dateTime`.
    ///
    pub time: Option<types::DateTime>,
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`).
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,

    /// The text content of the annotation, which may include markdown
    /// formatting. This is the required content of the note itself.
    ///
    pub text: types::Markdown,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

/// The `Annotation.author[x]` choice: who made the annotation.
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
pub enum AnnotationAuthor {
    /// `authorReference` — a reference to the responsible party.
    #[fhir("authorReference")]
    Reference(Box<types::Reference>),
    /// `authorString` — the responsible party as free text.
    #[fhir("authorString")]
    String(crate::r5::choice::Primitive<types::String>),
}

impl Annotation {
    /// The `authorReference` variant, if set.
    #[deprecated(note = "match on `author` (the AnnotationAuthor choice enum) instead")]
    #[must_use]
    pub fn author_reference(&self) -> Option<&types::Reference> {
        match &self.author {
            Some(AnnotationAuthor::Reference(r)) => Some(r),
            _ => None,
        }
    }

    /// The `authorString` variant's value, if set.
    #[deprecated(note = "match on `author` (the AnnotationAuthor choice enum) instead")]
    #[must_use]
    pub fn author_string(&self) -> Option<&types::String> {
        match &self.author {
            Some(AnnotationAuthor::String(p)) => Some(&p.value),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Annotation;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            author: None,
            time: None,
            text: types::Markdown::default(),
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;

        #[test]
        fn test_serde_json_round_trip() {
            let value = T::default();
            let json = ::serde_json::to_value(&value).expect("to_value");
            let back: T = ::serde_json::from_value(json).expect("from_value");
            assert_eq!(value, back);
        }
    }
}
