//! Slot
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Slot
//!
//!
//!
//! A slot of time on a schedule that may be available for booking appointments
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Slot Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::slot::Slot;
/// use fhir::r2::types;
///
/// let value = Slot {
///     overbooked: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `overbooked` is the name this serializes to on the wire.
/// assert_eq!(json["overbooked"], ::serde_json::json!(true));
///
/// let back: Slot = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Slot {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The type of appointments that can be booked into this slot (ideally
    /// this would be an identifiable service - which is at a location, rather
    /// than the location itself). If provided then this overrides the value
    /// provided on the availability resource
    pub r#type: Option<types::CodeableConcept>,

    /// The schedule resource that this slot defines an interval of status
    /// information
    pub schedule: types::Reference<crate::r2::resources::Schedule>,

    /// busy | free | busy-unavailable | busy-tentative
    pub free_busy_type: crate::coded::Coded<crate::r2::codes::Slotstatus>,
    /// Primitive extension sibling for [`free_busy_type`](Self::free_busy_type) (FHIR `_freeBusyType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_freeBusyType")]
    pub free_busy_type_ext: Option<types::Element>,

    /// Date/Time that the slot is to begin
    pub start: types::Instant,
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,

    /// Date/Time that the slot is to conclude
    pub end: types::Instant,
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,

    /// This slot has already been overbooked, appointments are unlikely to be
    /// accepted for this time
    pub overbooked: Option<types::Boolean>,
    /// Primitive extension sibling for [`overbooked`](Self::overbooked) (FHIR `_overbooked`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_overbooked")]
    pub overbooked_ext: Option<types::Element>,

    /// Comments on the slot to describe any extended information. Such as
    /// custom constraints on the slot
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Slot;

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
