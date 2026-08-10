//! CatalogEntry
//!
//! URL: http://hl7.org/fhir/StructureDefinition/CatalogEntry
//!
//! Version: 4.0.1
//!
//! An entry in a catalog
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Catalog entries are wrappers that contextualize items included in a
/// catalog.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::catalog_entry::CatalogEntry;
/// use fhir::r4::types;
///
/// let value = CatalogEntry {
///     valid_to: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `validTo` is the name this serializes to on the wire.
/// assert_eq!(json["validTo"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: CatalogEntry = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct CatalogEntry {
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

    /// Unique identifier of the catalog item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The type of item - medication, device, service, protocol or other
    pub r#type: Option<types::CodeableConcept>,

    /// Whether the entry represents an orderable item
    pub orderable: types::Boolean,
    /// Primitive extension sibling for [`orderable`](Self::orderable) (FHIR `_orderable`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_orderable")]
    pub orderable_ext: Option<types::Element>,

    /// The item that is being defined
    pub referenced_item: types::Reference,

    /// Any additional identifier(s) for the catalog item, in the same
    /// granularity or concept
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_identifier: Vec<types::Identifier>,

    /// Classification (category or class) of the item entry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<types::CodeableConcept>,

    /// draft | active | retired | unknown
    pub status: Option<crate::coded::Coded<crate::r4::codes::PublicationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The time period in which this catalog entry is expected to be active
    pub validity_period: Option<types::Period>,

    /// The date until which this catalog entry is expected to be active
    pub valid_to: Option<types::DateTime>,
    /// Primitive extension sibling for [`valid_to`](Self::valid_to) (FHIR `_validTo`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_validTo")]
    pub valid_to_ext: Option<types::Element>,

    /// When was this catalog last updated
    pub last_updated: Option<types::DateTime>,
    /// Primitive extension sibling for [`last_updated`](Self::last_updated) (FHIR `_lastUpdated`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastUpdated")]
    pub last_updated_ext: Option<types::Element>,

    /// Additional characteristics of the catalog entry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_characteristic: Vec<types::CodeableConcept>,

    /// Additional classification of the catalog entry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_classification: Vec<types::CodeableConcept>,

    /// An item that this catalog entry is related to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_entry: Vec<CatalogEntryRelatedEntry>,
}

/// Used for example, to point to a substance, or to a device used to
/// administer a medication.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::catalog_entry::CatalogEntryRelatedEntry;
/// use fhir::r4::types;
///
/// let value = CatalogEntryRelatedEntry {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: CatalogEntryRelatedEntry = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct CatalogEntryRelatedEntry {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// triggers | is-replaced-by
    pub relationtype: crate::coded::Coded<crate::r4::codes::RelationType>,
    /// Primitive extension sibling for [`relationtype`](Self::relationtype) (FHIR `_relationtype`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relationtype")]
    pub relationtype_ext: Option<types::Element>,

    /// The reference to the related item
    pub item: types::Reference<crate::r4::resources::CatalogEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = CatalogEntry;

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
