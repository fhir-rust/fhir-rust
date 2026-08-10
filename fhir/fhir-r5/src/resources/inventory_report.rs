//! InventoryReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InventoryReport
//!
//! Version: 5.0.0
//!
//! InventoryReport Resource: A report of inventory or stock items.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A report of inventory or stock items.
///
/// An InventoryReport communicates the current or changed state of stock items
/// held at one or more locations, either as a full snapshot or as a difference
/// from a previous count. It groups items into inventory listing sections and,
/// within each section, records the item type, category, and counted quantity.
/// In FHIR R5 it supports supply-chain and stock-management workflows such as
/// periodic counts, receipt of new arrivals, and reconciliation. Each report
/// carries a status and a count type (snapshot or difference), is tied to a
/// reporting period and a reporter, and organizes the counted stock into one
/// or more inventory listing sections, each containing individual items with
/// their category, quantity, and item type or reference.
///
/// # See also
///
/// - [`types::Reference`] — used for the `reporter` and listing `location`.
/// - [`types::CodeableConcept`] and [`types::CodeableReference`] — used to
///   describe item categories, statuses, and item types.
/// - `SupplyRequest` and `SupplyDelivery` — related supply-chain resources
///   that this report may reconcile against.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_report::InventoryReport;
/// use fhir::r5::types;
///
/// let value = InventoryReport {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InventoryReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryReport {
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

    /// Business identifier for the report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The current status of this report in its workflow: draft | requested | active | entered-in-error
    pub status: crate::r5::coded::Coded<crate::r5::codes::InventoryreportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Whether the report is a full snapshot of stock or only the difference from a previous count: snapshot | difference
    pub count_type: crate::r5::coded::Coded<crate::r5::codes::InventoryreportCounttype>,
    /// Primitive extension sibling for [`count_type`](Self::count_type) (FHIR `_countType`).
    #[serde(rename = "_countType")]
    pub count_type_ext: Option<types::Element>,

    /// addition | subtraction
    pub operation_type: Option<types::CodeableConcept>,

    /// The reason for this count - regular count, ad-hoc count, new arrivals, etc
    pub operation_type_reason: Option<types::CodeableConcept>,

    /// When the report has been submitted, typically the date and time of finalization rather than data capture
    pub reported_date_time: types::DateTime,
    /// Primitive extension sibling for [`reported_date_time`](Self::reported_date_time) (FHIR `_reportedDateTime`).
    #[serde(rename = "_reportedDateTime")]
    pub reported_date_time_ext: Option<types::Element>,

    /// The person, device, or [`Organization`](crate::r5::resources::organization::Organization) that submits the report
    pub reporter: Option<types::Reference>,

    /// The period the report refers to
    pub reporting_period: Option<types::Period>,

    /// One or more inventory listing sections, each grouping counted items by location, status, and/or count date
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inventory_listing: Vec<InventoryReportInventoryListing>,

    /// A note associated with the InventoryReport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// An inventory listing section (grouped by any of the attributes).
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_report::InventoryReportInventoryListing;
/// use fhir::r5::types;
///
/// let value = InventoryReportInventoryListing {
///     counting_date_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `countingDateTime` is the name this serializes to on the wire.
/// assert_eq!(json["countingDateTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: InventoryReportInventoryListing = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryReportInventoryListing {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Location of the inventory items
    pub location: Option<types::Reference<crate::r5::resources::Location>>,

    /// The status of the items that are being reported
    pub item_status: Option<types::CodeableConcept>,

    /// The date and time when the items were counted
    pub counting_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`counting_date_time`](Self::counting_date_time) (FHIR `_countingDateTime`).
    #[serde(rename = "_countingDateTime")]
    pub counting_date_time_ext: Option<types::Element>,

    /// The item or items in this listing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<InventoryReportInventoryListingItem>,
}

/// The item or items in this listing.
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_report::InventoryReportInventoryListingItem;
/// use fhir::r5::types;
///
/// let value = InventoryReportInventoryListingItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InventoryReportInventoryListingItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryReportInventoryListingItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The inventory category or classification of the items being reported
    pub category: Option<types::CodeableConcept>,

    /// The quantity of the item or items being reported
    pub quantity: types::Quantity,

    /// The code or reference to the item type
    pub item: types::CodeableReference,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = InventoryReport;

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
