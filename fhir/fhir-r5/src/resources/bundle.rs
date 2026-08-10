//! Bundle
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Bundle
//!
//! Version: 5.0.0
//!
//! Bundle Resource: A container for a collection of resources.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A container for a collection of resources.
///
/// A Bundle groups together a set of resources into a single instance for
/// transport, exchange, or storage. Bundles are used for many purposes in FHIR
/// R5, including documents, messages, transaction and batch operations and their
/// responses, search result sets, and collections of resources maintained
/// together as history. The `type` element declares which of these uses applies
/// and governs the expected content and processing of the entries.
///
/// Clinically and administratively, a Bundle is the primary mechanism by which
/// FHIR systems exchange more than one resource at a time: a clinical document
/// (such as a discharge summary) is a Bundle whose first entry is a `Composition`
/// referencing supporting resources such as [`Patient`](crate::r5::resources::patient::Patient)
/// and observations; a transaction or batch Bundle lets a client submit a set of
/// create/update/delete operations to a server atomically or independently; and a
/// searchset Bundle is the standard shape of the results returned from a FHIR
/// search interaction, including pagination via `link` entries and, optionally,
/// the `total` number of matches. Each `entry` in the bundle carries either the
/// resource itself, information about how it should be processed (for
/// transactions and batches), or the outcome of processing it (for responses and
/// history).
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) is a common resource
///   referenced from within document and search Bundles.
/// - `Composition` (when present) typically forms the first entry of a document
///   Bundle.
/// - [`Identifier`](crate::r5::types::Identifier), [`Meta`](crate::r5::types::Meta),
///   and [`Signature`](crate::r5::types::Signature) describe and, optionally,
///   authenticate the Bundle as a whole.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::bundle::Bundle;
/// use fhir::r5::types;
///
/// let value = Bundle {
///     timestamp: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `timestamp` is the name this serializes to on the wire.
/// assert_eq!(json["timestamp"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Bundle = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
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

    /// Persistent identifier for the bundle
    pub identifier: Option<types::Identifier>,

    /// The kind of Bundle: document | message | transaction | transaction-response
    /// | batch | batch-response | history | searchset | collection |
    /// subscription-notification. This value determines how the entries must be
    /// interpreted and processed.
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::BundleType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// When the bundle was assembled
    pub timestamp: Option<types::Instant>,
    /// Primitive extension sibling for [`timestamp`](Self::timestamp) (FHIR `_timestamp`).
    #[serde(rename = "_timestamp")]
    pub timestamp_ext: Option<types::Element>,

    /// If search, the total number of matches across all pages of results
    pub total: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`total`](Self::total) (FHIR `_total`).
    #[serde(rename = "_total")]
    pub total_ext: Option<types::Element>,

    /// Links related to this Bundle, such as `self` and `next` for paginated
    /// search results
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<BundleLink>,

    /// The entries making up the bundle; each one carries a resource, request,
    /// response, or search metadata depending on the bundle's `type`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry: Vec<BundleEntry>,

    /// Digital Signature
    pub signature: Option<types::Signature>,

    /// Issues with the Bundle
    pub issues: Option<::serde_json::Value>,
}

/// Links related to this Bundle.
///
/// A series of links that provide context to this bundle, such as pagination
/// links for a search result set.
/// # Examples
///
/// ```
/// use fhir::r5::resources::bundle::BundleLink;
/// use fhir::r5::types;
///
/// let value = BundleLink {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BundleLink = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BundleLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// See http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-1
    pub relation: crate::r5::coded::Coded<crate::r5::codes::IanaLinkRelations>,
    /// Primitive extension sibling for [`relation`](Self::relation) (FHIR `_relation`).
    #[serde(rename = "_relation")]
    pub relation_ext: Option<types::Element>,

    /// Reference details for the link
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// Entry in the bundle - will have a resource or information.
///
/// An entry in a bundle resource - will either contain a resource or information
/// about a resource (transactions and history only).
/// # Examples
///
/// ```
/// use fhir::r5::resources::bundle::BundleEntry;
/// use fhir::r5::types;
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
pub struct BundleEntry {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Links related to this entry
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<BundleLink>,

    /// URI for resource (e.g. the absolute URL server address, URI for
    /// UUID/OID, etc.)
    pub full_url: Option<types::Uri>,
    /// Primitive extension sibling for [`full_url`](Self::full_url) (FHIR `_fullUrl`).
    #[serde(rename = "_fullUrl")]
    pub full_url_ext: Option<types::Element>,

    /// A resource in the bundle
    pub resource: Option<::serde_json::Value>,

    /// Search related information
    pub search: Option<BundleEntrySearch>,

    /// Additional execution information (transaction/batch/history)
    pub request: Option<BundleEntryRequest>,

    /// Results of execution (transaction/batch/history)
    pub response: Option<BundleEntryResponse>,
}

/// Search related information.
///
/// Information about the search process that lead to the creation of this entry.
/// # Examples
///
/// ```
/// use fhir::r5::resources::bundle::BundleEntrySearch;
/// use fhir::r5::types;
///
/// let value = BundleEntrySearch {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BundleEntrySearch = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntrySearch {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// match | include - why this is in the result set
    pub mode: Option<crate::r5::coded::Coded<crate::r5::codes::SearchEntryMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Search ranking (between 0 and 1)
    pub score: Option<types::Decimal>,
    /// Primitive extension sibling for [`score`](Self::score) (FHIR `_score`).
    #[serde(rename = "_score")]
    pub score_ext: Option<types::Element>,
}

/// Additional execution information (transaction/batch/history).
///
/// Additional information about how this entry should be processed as part of a
/// transaction or batch. For history, it shows how the entry was processed to
/// create the version contained in the entry.
/// # Examples
///
/// ```
/// use fhir::r5::resources::bundle::BundleEntryRequest;
/// use fhir::r5::types;
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
pub struct BundleEntryRequest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// GET | HEAD | POST | PUT | DELETE | PATCH
    pub method: crate::r5::coded::Coded<crate::r5::codes::HttpVerb>,
    /// Primitive extension sibling for [`method`](Self::method) (FHIR `_method`).
    #[serde(rename = "_method")]
    pub method_ext: Option<types::Element>,

    /// URL for HTTP equivalent of this entry
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// For managing cache validation
    pub if_none_match: Option<types::String>,
    /// Primitive extension sibling for [`if_none_match`](Self::if_none_match) (FHIR `_ifNoneMatch`).
    #[serde(rename = "_ifNoneMatch")]
    pub if_none_match_ext: Option<types::Element>,

    /// For managing cache currency
    pub if_modified_since: Option<types::Instant>,
    /// Primitive extension sibling for [`if_modified_since`](Self::if_modified_since) (FHIR `_ifModifiedSince`).
    #[serde(rename = "_ifModifiedSince")]
    pub if_modified_since_ext: Option<types::Element>,

    /// For managing update contention
    pub if_match: Option<types::String>,
    /// Primitive extension sibling for [`if_match`](Self::if_match) (FHIR `_ifMatch`).
    #[serde(rename = "_ifMatch")]
    pub if_match_ext: Option<types::Element>,

    /// For conditional creates
    pub if_none_exist: Option<types::String>,
    /// Primitive extension sibling for [`if_none_exist`](Self::if_none_exist) (FHIR `_ifNoneExist`).
    #[serde(rename = "_ifNoneExist")]
    pub if_none_exist_ext: Option<types::Element>,
}

/// Results of execution (transaction/batch/history).
///
/// Indicates the results of processing the corresponding 'request' entry in the
/// batch or transaction being responded to or what the results of an operation
/// where when returning history.
/// # Examples
///
/// ```
/// use fhir::r5::resources::bundle::BundleEntryResponse;
/// use fhir::r5::types;
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
pub struct BundleEntryResponse {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Status response code (text optional)
    pub status: types::String,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The location (if the operation returns a location)
    pub location: Option<types::Uri>,
    /// Primitive extension sibling for [`location`](Self::location) (FHIR `_location`).
    #[serde(rename = "_location")]
    pub location_ext: Option<types::Element>,

    /// The Etag for the resource (if relevant)
    pub etag: Option<types::String>,
    /// Primitive extension sibling for [`etag`](Self::etag) (FHIR `_etag`).
    #[serde(rename = "_etag")]
    pub etag_ext: Option<types::Element>,

    /// Server's date time modified
    pub last_modified: Option<types::Instant>,
    /// Primitive extension sibling for [`last_modified`](Self::last_modified) (FHIR `_lastModified`).
    #[serde(rename = "_lastModified")]
    pub last_modified_ext: Option<types::Element>,

    /// OperationOutcome with hints and warnings (for batch/transaction)
    pub outcome: Option<::serde_json::Value>,
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
