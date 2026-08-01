//! Flag
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Flag
//!
//! Version: 5.0.0
//!
//! Flag Resource: Prospective warnings of potential issues when providing care to the patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Prospective warnings of potential issues when providing care to a patient.
/// A Flag is a clinical or administrative alert — such as a special precaution,
/// behavioral concern, or care restriction — that a care provider needs to be
/// aware of when interacting with the subject of the flag. It carries a coded or
/// textual message, a status, an optional active period, and references to the
/// subject, relevant encounter, and author.
///
/// Unlike a `Condition` or `AllergyIntolerance`, a Flag is not a clinical finding
/// in itself; rather it is a mechanism for surfacing important information — such
/// as "patient is a fall risk" or "record requires special handling" — prominently
/// to anyone accessing the subject's record, regardless of the specific clinical
/// context being reviewed. Flags are commonly rendered by clinical systems as
/// banners or alerts on a patient's chart, and their `status` element tracks
/// whether the alert is currently `active`, `inactive`, or was `entered-in-error`.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) and
///   [`Group`](crate::r5::resources::group::Group), which are common subjects of a Flag.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept), used for the flag's
///   category and coded message.
/// - [`Reference`](crate::r5::types::Reference), used to link the flag to its
///   subject, encounter, and author.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::flag::Flag;
/// use fhir::r5::types;
///
/// let value = Flag {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Flag = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
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

    /// The lifecycle status of the flag: active | inactive | entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::FlagStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// A broad classification of the flag, such as clinical, administrative, or safety.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// The coded or free-text message that the flag is intended to convey to a reader.
    pub code: types::CodeableConcept,

    /// A reference to the person, group, or other resource the flag concerns.
    pub subject: types::Reference,

    /// The time period during which the flag should be considered active.
    pub period: Option<types::Period>,

    /// Alert relevant during encounter
    pub encounter: Option<types::Reference>,

    /// Flag creator
    pub author: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Flag;

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
