//! SupplyDelivery
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SupplyDelivery
//!
//! Version: 6.0.0-ballot3
//!
//! Record of movement of supplies from one location to another
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Record of delivery of what is supplied.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::supply_delivery::SupplyDelivery;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

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

    /// in-progress | completed | abandoned | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::SupplydeliveryStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Patient for whom the item is supplied
    pub patient: Option<types::Reference>,

    /// Category of supply event
    pub r#type: Option<types::CodeableConcept>,

    /// Stage or event of delivery
    pub stage: types::CodeableConcept,

    /// The item that is delivered or supplied
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplied_item: Vec<SupplyDeliverySuppliedItem>,

    /// When event occurred
    /// The `SupplyDelivery.occurrence[x]` choice element (0..1); see [`SupplyDeliveryOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<SupplyDeliveryOccurrence>,

    /// The item supplier
    pub supplier: Option<types::Reference>,

    /// Where the delivery was sent
    pub destination: Option<types::Reference>,

    /// Who received the delivery
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receiver: Vec<types::Reference>,
}

/// The item that is being delivered or has been supplied.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::supply_delivery::SupplyDeliverySuppliedItem;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// A description of the supplied item's condition (e.g., box is damaged)
    pub condition: Option<types::CodeableConcept>,

    /// Medication, Substance, Device or Biologically Derived Product supplied
    /// The `SupplyDelivery.suppliedItem.item[x]` choice element (0..1); see [`SupplyDeliverySuppliedItemItem`].
    #[serde(flatten)]
    pub item: Option<SupplyDeliverySuppliedItemItem>,
}

/// The `SupplyDelivery.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SupplyDeliveryOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `SupplyDelivery.suppliedItem.item[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
