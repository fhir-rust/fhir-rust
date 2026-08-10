//! SupplyDelivery
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
//!
//! Version: 4.3.0
//!
//! Delivery of bulk Supplies
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Record of delivery of what is supplied.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::supply_delivery::SupplyDelivery;
/// use fhir::r4b::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SupplyDeliveryDe")]
#[fhir_version("r4b")]
pub struct SupplyDelivery {
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
    pub contained: Vec<crate::r4b::resources::Resource>,

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
    pub based_on: Vec<types::Reference<crate::r4b::resources::SupplyRequest>>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// in-progress | completed | abandoned | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r4b::codes::SupplydeliveryStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Patient for whom the item is supplied
    pub patient: Option<types::Reference<crate::r4b::resources::Patient>>,

    /// Category of dispense event
    pub r#type: Option<types::CodeableConcept>,

    /// The item that is delivered or supplied
    pub supplied_item: Option<SupplyDeliverySuppliedItem>,

    /// When event occurred
    /// The `SupplyDelivery.occurrence[x]` choice element (0..1); see [`SupplyDeliveryOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<SupplyDeliveryOccurrence>,

    /// Dispenser
    pub supplier: Option<types::Reference>,

    /// Where the Supply was sent
    pub destination: Option<types::Reference<crate::r4b::resources::Location>>,

    /// Who collected the Supply
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receiver: Vec<types::Reference>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SupplyDeliveryDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r4b::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    based_on: Vec<types::Reference<crate::r4b::resources::SupplyRequest>>,
    #[serde(default)]
    part_of: Vec<types::Reference>,
    status: Option<crate::coded::Coded<crate::r4b::codes::SupplydeliveryStatus>>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    patient: Option<types::Reference<crate::r4b::resources::Patient>>,
    r#type: Option<types::CodeableConcept>,
    supplied_item: Option<SupplyDeliverySuppliedItem>,
    #[serde(flatten)]
    occurrence: crate::r4b::choice::Slot<SupplyDeliveryOccurrence>,
    supplier: Option<types::Reference>,
    destination: Option<types::Reference<crate::r4b::resources::Location>>,
    #[serde(default)]
    receiver: Vec<types::Reference>,
}

impl ::core::convert::From<SupplyDeliveryDe> for SupplyDelivery {
    fn from(v: SupplyDeliveryDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            based_on: v.based_on,
            part_of: v.part_of,
            status: v.status,
            status_ext: v.status_ext,
            patient: v.patient,
            r#type: v.r#type,
            supplied_item: v.supplied_item,
            occurrence: v.occurrence.0,
            supplier: v.supplier,
            destination: v.destination,
            receiver: v.receiver,
        }
    }
}

/// The item that is being delivered or has been supplied.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::supply_delivery::SupplyDeliverySuppliedItem;
/// use fhir::r4b::types;
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
#[serde(from = "SupplyDeliverySuppliedItemDe")]
#[fhir_version("r4b")]
pub struct SupplyDeliverySuppliedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Amount dispensed
    pub quantity: Option<types::Quantity>,

    /// Medication, Substance, or Device supplied
    /// The `SupplyDelivery.suppliedItem.item[x]` choice element (0..1); see [`SupplyDeliverySuppliedItemItem`].
    #[serde(flatten)]
    pub item: Option<SupplyDeliverySuppliedItemItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SupplyDeliverySuppliedItemDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    quantity: Option<types::Quantity>,
    #[serde(flatten)]
    item: crate::r4b::choice::Slot<SupplyDeliverySuppliedItemItem>,
}

impl ::core::convert::From<SupplyDeliverySuppliedItemDe> for SupplyDeliverySuppliedItem {
    fn from(v: SupplyDeliverySuppliedItemDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            quantity: v.quantity,
            item: v.item.0,
        }
    }
}

/// The `SupplyDelivery.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum SupplyDeliveryOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `SupplyDelivery.suppliedItem.item[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum SupplyDeliverySuppliedItemItem {
    /// `itemCodeableConcept` variant.
    #[fhir("itemCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `itemReference` variant.
    #[fhir("itemReference")]
    Reference(Box<types::Reference>),
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
