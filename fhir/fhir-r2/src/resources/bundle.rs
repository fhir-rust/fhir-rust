//! Bundle
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Bundle
//!
//!
//!
//! Contains a collection of resources
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Bundle Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::bundle::Bundle;
/// use fhir::r2::types;
///
/// let value = Bundle {
///     total: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `total` is the name this serializes to on the wire.
/// assert_eq!(json["total"], ::serde_json::json!(0));
///
/// let back: Bundle = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Bundle {
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

    /// document | message | transaction | transaction-response | batch |
    /// batch-response | history | searchset | collection
    pub r#type: crate::coded::Coded<crate::r2::codes::BundleType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// If search, the total number of matches
    pub total: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`total`](Self::total) (FHIR `_total`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_total")]
    pub total_ext: Option<types::Element>,

    /// Links related to this Bundle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<BundleLink>,

    /// Entry in the bundle - will have a resource, or information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<BundleEntry>,

    /// Digital Signature
    pub signature: Option<types::Signature>,
}

/// An entry in a bundle resource - will either contain a resource, or
/// information about a resource (transactions and history only).
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::bundle::BundleEntry;
/// use fhir::r2::types;
///
/// let value = BundleEntry {
///     full_url: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `fullUrl` is the name this serializes to on the wire.
/// assert_eq!(json["fullUrl"], ::serde_json::json!("http://example.org"));
///
/// let back: BundleEntry = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct BundleEntry {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Links related to this entry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<BundleLink>,

    /// Absolute URL for resource (server address, or UUID/OID)
    pub full_url: Option<types::Uri>,
    /// Primitive extension sibling for [`full_url`](Self::full_url) (FHIR `_fullUrl`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fullUrl")]
    pub full_url_ext: Option<types::Element>,

    /// A resource in the bundle
    pub resource: Option<::serde_json::Value>,

    /// Search related information
    pub search: Option<BundleEntrySearch>,

    /// Transaction Related Information
    pub request: Option<BundleEntryRequest>,

    /// Transaction Related Information
    pub response: Option<BundleEntryResponse>,
}

/// Additional information about how this entry should be processed as part of
/// a transaction.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::bundle::BundleEntryRequest;
/// use fhir::r2::types;
///
/// let value = BundleEntryRequest {
///     if_none_match: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `ifNoneMatch` is the name this serializes to on the wire.
/// assert_eq!(json["ifNoneMatch"], ::serde_json::json!("abc"));
///
/// let back: BundleEntryRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct BundleEntryRequest {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// GET | POST | PUT | DELETE
    pub method: crate::coded::Coded<crate::r2::codes::HttpVerb>,
    /// Primitive extension sibling for [`method`](Self::method) (FHIR `_method`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_method")]
    pub method_ext: Option<types::Element>,

    /// URL for HTTP equivalent of this entry
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// For managing cache currency
    pub if_none_match: Option<types::String>,
    /// Primitive extension sibling for [`if_none_match`](Self::if_none_match) (FHIR `_ifNoneMatch`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ifNoneMatch")]
    pub if_none_match_ext: Option<types::Element>,

    /// For managing update contention
    pub if_modified_since: Option<types::Instant>,
    /// Primitive extension sibling for [`if_modified_since`](Self::if_modified_since) (FHIR `_ifModifiedSince`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ifModifiedSince")]
    pub if_modified_since_ext: Option<types::Element>,

    /// For managing update contention
    pub if_match: Option<types::String>,
    /// Primitive extension sibling for [`if_match`](Self::if_match) (FHIR `_ifMatch`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ifMatch")]
    pub if_match_ext: Option<types::Element>,

    /// For conditional creates
    pub if_none_exist: Option<types::String>,
    /// Primitive extension sibling for [`if_none_exist`](Self::if_none_exist) (FHIR `_ifNoneExist`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ifNoneExist")]
    pub if_none_exist_ext: Option<types::Element>,
}

/// Additional information about how this entry should be processed as part of
/// a transaction.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::bundle::BundleEntryResponse;
/// use fhir::r2::types;
///
/// let value = BundleEntryResponse {
///     last_modified: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lastModified` is the name this serializes to on the wire.
/// assert_eq!(json["lastModified"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: BundleEntryResponse = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct BundleEntryResponse {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Status return code for entry
    pub status: types::String,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The location, if the operation returns a location
    pub location: Option<types::Uri>,
    /// Primitive extension sibling for [`location`](Self::location) (FHIR `_location`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_location")]
    pub location_ext: Option<types::Element>,

    /// The etag for the resource (if relevant)
    pub etag: Option<types::String>,
    /// Primitive extension sibling for [`etag`](Self::etag) (FHIR `_etag`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_etag")]
    pub etag_ext: Option<types::Element>,

    /// Server's date time modified
    pub last_modified: Option<types::Instant>,
    /// Primitive extension sibling for [`last_modified`](Self::last_modified) (FHIR `_lastModified`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastModified")]
    pub last_modified_ext: Option<types::Element>,
}

/// Information about the search process that lead to the creation of this
/// entry.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::bundle::BundleEntrySearch;
/// use fhir::r2::types;
///
/// let value = BundleEntrySearch {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: BundleEntrySearch = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct BundleEntrySearch {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// match | include | outcome - why this is in the result set
    pub mode: Option<crate::coded::Coded<crate::r2::codes::SearchEntryMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Search ranking (between 0 and 1)
    pub score: Option<types::Decimal>,
    /// Primitive extension sibling for [`score`](Self::score) (FHIR `_score`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_score")]
    pub score_ext: Option<types::Element>,
}

/// A series of links that provide context to this bundle.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::bundle::BundleLink;
/// use fhir::r2::types;
///
/// let value = BundleLink {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: BundleLink = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct BundleLink {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// http://www.iana.org/assignments/link-relations/link-relations.xhtml
    pub relation: types::String,
    /// Primitive extension sibling for [`relation`](Self::relation) (FHIR `_relation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_relation")]
    pub relation_ext: Option<types::Element>,

    /// Reference details for the link
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Bundle;

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
