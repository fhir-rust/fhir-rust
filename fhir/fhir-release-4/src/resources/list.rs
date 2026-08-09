//! List
//!
//! URL: http://hl7.org/fhir/StructureDefinition/List
//!
//! Version: 4.0.1
//!
//! A list is a curated collection of resources
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A list is a curated collection of resources.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::list::List;
/// use fhir::r4::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct List {
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

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// current | retired | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::ListStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// working | snapshot | changes
    pub mode: crate::coded::Coded<crate::r4::codes::ListMode>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Descriptive name for the list
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// What the purpose of this list is
    pub code: Option<types::CodeableConcept>,

    /// If all resources have the same subject
    pub subject: Option<types::Reference>,

    /// Context in which list created
    pub encounter: Option<types::Reference<crate::r4::resources::Encounter>>,

    /// When the list was prepared
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Who and/or what defined the list contents (aka Author)
    pub source: Option<types::Reference>,

    /// What order the list has
    pub ordered_by: Option<types::CodeableConcept>,

    /// Comments about the list
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Entries in the list
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<ListEntry>,

    /// Why list is empty
    pub empty_reason: Option<types::CodeableConcept>,
}

/// Entries in this list.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::list::ListEntry;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    /// Primitive extension sibling for [`deleted`](Self::deleted) (FHIR `_deleted`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deleted")]
    pub deleted_ext: Option<types::Element>,

    /// When item added to list
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
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
