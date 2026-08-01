//! SupplyDelivery
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
//!
//! Version: 5.0.0
//!
//! SupplyDelivery Resource: Record of delivery of what is supplied.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Record of delivery of what is supplied.
///
/// The SupplyDelivery resource documents the supply and delivery of items such
/// as medication, devices, biologically derived products, and other substances.
/// It captures what was delivered, the quantity, when it occurred, who supplied
/// it, and where it was sent. It is commonly used in supply chain and inventory
/// workflows to record fulfillment of a supply request or order.
///
/// Clinically and administratively, SupplyDelivery represents the actual event
/// of an item changing hands, as distinct from the request or plan for that
/// item (typically expressed as a `SupplyRequest`). A single supply request may
/// be fulfilled by one or more SupplyDelivery instances, for example when a
/// large order is dispatched in multiple partial shipments. The resource tracks
/// the status of the delivery event (in-progress, completed, abandoned, or
/// entered-in-error), the party responsible for supplying the item, and the
/// destination or recipient, making it useful for supply chain tracking,
/// pharmacy dispensing logistics, and equipment or product distribution
/// workflows.
///
/// Related resources: the recipient of a delivery is often a
/// [`Patient`](crate::r5::resources::patient::Patient); category and item
/// codes are typically represented with
/// [`CodeableConcept`](crate::r5::types::CodeableConcept); and the requesting
/// order that this delivery fulfills is commonly a `SupplyRequest`.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::supply_delivery::SupplyDelivery;
/// use fhir::r5::types;
///
/// let value = SupplyDelivery {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SupplyDelivery = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplyDelivery {
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

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Fulfills plan, proposal or order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Lifecycle status of the delivery event: in-progress | completed | abandoned | entered-in-error
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::SupplydeliveryStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reference to the patient for whom the supplied item is intended
    pub patient: Option<types::Reference>,

    /// Category of supply event, e.g. medication, device, or resupply
    pub r#type: Option<types::CodeableConcept>,

    /// One or more items, with quantity, that were delivered as part of this event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplied_item: Vec<SupplyDeliverySuppliedItem>,

    /// The `SupplyDelivery.occurrence[x]` choice element (0..1); see [`SupplyDeliveryOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<SupplyDeliveryOccurrence>,

    /// Reference to the organization or practitioner that supplied the item
    pub supplier: Option<types::Reference>,

    /// Reference to the location where the delivery was sent
    pub destination: Option<types::Reference>,

    /// Who received the delivery
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receiver: Vec<types::Reference>,
}

/// The item that is delivered or supplied.
///
/// Describes a single item that is delivered as part of the supply delivery,
/// including the amount supplied and a reference or coded value identifying the
/// medication, substance, device, or biologically derived product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::supply_delivery::SupplyDeliverySuppliedItem;
/// use fhir::r5::types;
///
/// let value = SupplyDeliverySuppliedItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SupplyDeliverySuppliedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplyDeliverySuppliedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Amount supplied
    pub quantity: Option<types::Quantity>,

    /// The `SupplyDelivery.suppliedItem.item[x]` choice element (0..1); see [`SupplyDeliverySuppliedItemItem`].
    #[serde(flatten)]
    pub item: Option<SupplyDeliverySuppliedItemItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SupplyDelivery;

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
/// The `SupplyDelivery.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SupplyDeliveryOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `SupplyDelivery.suppliedItem.item[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SupplyDeliverySuppliedItemItem {
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
}
