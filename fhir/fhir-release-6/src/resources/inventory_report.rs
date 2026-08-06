//! InventoryReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InventoryReport
//!
//! Version: 6.0.0-ballot3
//!
//! A report of inventory or stock items
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A report of inventory or stock items.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_report::InventoryReport;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InventoryReport {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier for the report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | requested | active | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::InventoryreportStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// snapshot | difference
    pub count_type: crate::coded::Coded<crate::r6::codes::InventoryreportCounttype>,
    /// Primitive extension sibling for [`count_type`](Self::count_type) (FHIR `_countType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_countType")]
    pub count_type_ext: Option<types::Element>,

    /// addition | subtraction
    pub operation_type: Option<types::CodeableConcept>,

    /// The reason for this count - regular count, ad-hoc count, new arrivals,
    /// etc
    pub operation_type_reason: Option<types::CodeableConcept>,

    /// When the report has been submitted
    pub reported_date_time: types::DateTime,
    /// Primitive extension sibling for [`reported_date_time`](Self::reported_date_time) (FHIR `_reportedDateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reportedDateTime")]
    pub reported_date_time_ext: Option<types::Element>,

    /// Who submits the report
    pub reporter: Option<types::Reference>,

    /// The period the report refers to
    pub reporting_period: Option<types::Period>,

    /// An inventory listing section (grouped by any of the attributes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inventory_listing: Vec<InventoryReportInventoryListing>,

    /// A note associated with the InventoryReport
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// An inventory listing section (grouped by any of the attributes).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_report::InventoryReportInventoryListing;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub location: Option<types::Reference>,

    /// The status of the items that are being reported
    pub item_status: Option<types::CodeableConcept>,

    /// The date and time when the items were counted
    pub counting_date_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`counting_date_time`](Self::counting_date_time) (FHIR `_countingDateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_countingDateTime")]
    pub counting_date_time_ext: Option<types::Element>,

    /// The item or items in this listing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<InventoryReportInventoryListingItem>,
}

/// The item or items in this listing.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_report::InventoryReportInventoryListingItem;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
