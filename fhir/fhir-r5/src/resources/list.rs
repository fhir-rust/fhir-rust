//! List
//!
//! URL: http://hl7.org/fhir/StructureDefinition/List
//!
//! Version: 5.0.0
//!
//! List Resource: A List is a curated collection of resources, for things such as problem lists, allergy lists, facility list, organization list, etc.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A List is a curated, human- or system-maintained collection of resources.
///
/// The List resource groups references to other resources into a single named
/// collection, such as a problem list, allergy list, medication list, a set of
/// results, or a working set of items assembled for a particular purpose. Unlike
/// a Bundle, which is a transient container used to move a set of resources
/// together, a List is a persistent, first-class resource that records the
/// intent and provenance behind the grouping: how and why the collection was
/// assembled, including its status (current, retired, or entered-in-error), its
/// mode (a working list, a point-in-time snapshot, or a change set), the ordering
/// applied to entries, the source that authored it, and the date it was prepared.
/// Each entry can carry workflow flags, an indication that an item has been
/// deleted, and the date the item was added, which lets the List act as a
/// managed, curated view whose membership evolves over time. In FHIR R5 it is
/// used both for clinically curated lists and for administrative or operational
/// collections, and an empty list can explain its emptiness through a reason code.
///
/// # See also
///
/// The list `subject` and `source` fields, and each entry's `item`, are typically
/// references to resources such as [`Patient`](crate::r5::resources::patient::Patient),
/// [`Condition`](crate::r5::resources::condition::Condition), or an
/// [`Encounter`](crate::r5::resources::encounter::Encounter). Coded fields such as
/// `code`, `ordered_by`, and `empty_reason` use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and references use
/// [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::list::List;
/// use fhir::r5::types;
///
/// let value = List {
///     title: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `title` is the name this serializes to on the wire.
/// assert_eq!(json["title"], ::serde_json::json!("abc"));
///
/// let back: List = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct List {
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
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Lifecycle status of the list: current, retired, or entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::ListStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// How the list was assembled: a working list, a point-in-time snapshot, or a change set.
    pub mode: crate::r5::coded::Coded<crate::r5::codes::ListMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Descriptive name for the list
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What the purpose of this list is
    pub code: Option<types::CodeableConcept>,

    /// If all resources have the same subject(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<types::Reference>,

    /// Context in which list created
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// When the list was prepared
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who and/or what defined the list contents (aka Author)
    pub source: Option<types::Reference>,

    /// What order the list has
    pub ordered_by: Option<types::CodeableConcept>,

    /// Comments about the list
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Entries in the list, each referencing an item and its optional flags, deletion state, and date.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<ListEntry>,

    /// Why list is empty
    pub empty_reason: Option<types::CodeableConcept>,
}

/// Entries in the list.
///
/// Each entry references an actual item in the list and may carry status or
/// workflow flags, an indication that the item has been deleted, and the date
/// the item was added.
/// # Examples
///
/// ```
/// use fhir::r5::resources::list::ListEntry;
/// use fhir::r5::types;
///
/// let value = ListEntry {
///     deleted: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `deleted` is the name this serializes to on the wire.
/// assert_eq!(json["deleted"], ::serde_json::json!(true));
///
/// let back: ListEntry = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ListEntry {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Status/Workflow information about this item
    pub flag: Option<types::CodeableConcept>,

    /// If this item is actually marked as deleted
    pub deleted: Option<types::Boolean>,
    /// Primitive extension sibling for [`deleted`](Self::deleted) (FHIR `_deleted`).
    #[serde(rename = "_deleted")]
    pub deleted_ext: Option<types::Element>,

    /// When item added to list
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Actual entry
    pub item: types::Reference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = List;

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
