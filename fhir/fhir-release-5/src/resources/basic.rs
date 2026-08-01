//! Basic
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Basic
//!
//! Version: 5.0.0
//!
//! Basic Resource: Basic is used for handling concepts not yet defined in FHIR, narrative-only resources that don't map to an existing resource, and custom resources not appropriate for inclusion in the FHIR specification.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Basic is used for handling concepts not yet defined in FHIR, narrative-only
/// resources that don't map to an existing resource, and custom resources not
/// appropriate for inclusion in the FHIR specification. It provides a minimal
/// framework — primarily a `code` describing the kind of thing represented,
/// plus optional subject, author, and creation date — so that implementers can
/// exchange information that FHIR does not otherwise model. The bulk of the
/// meaning is typically conveyed through extensions and the narrative text.
/// Common uses include representing business concepts (such as a household
/// or a research study protocol step) that have not yet been formalized as
/// a dedicated FHIR resource type.
///
/// # Related resources
///
/// The `code` field, of type [`CodeableConcept`](crate::r5::types::CodeableConcept),
/// identifies what kind of "thing" the instance represents, while `subject`
/// and `author` are [`Reference`](crate::r5::types::Reference) values that
/// commonly point to resources such as `Patient`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::basic::Basic;
/// use fhir::r5::types;
///
/// let value = Basic {
///     created: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `created` is the name this serializes to on the wire.
/// assert_eq!(json["created"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Basic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Basic {
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

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Kind of Resource; a coded classification describing what this Basic instance represents
    pub code: types::CodeableConcept,

    /// Identifies the patient, group, or other resource that this instance is about
    pub subject: Option<types::Reference>,

    /// The date and, optionally, time when this resource instance was created
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`).
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Indicates who was responsible for creating the resource instance
    pub author: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Basic;

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
