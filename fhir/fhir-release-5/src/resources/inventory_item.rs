//! InventoryItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InventoryItem
//!
//! Version: 5.0.0
//!
//! InventoryItem Resource: functional description of an inventory item used in inventory and supply-related workflows.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A functional description of an inventory item used in inventory and
/// supply-related workflows.
///
/// The InventoryItem resource represents a class of item that is stocked,
/// counted, ordered, or otherwise tracked within an inventory or supply chain.
/// It is a lightweight, inventory-oriented description rather than a full
/// clinical definition: it captures the item's business identifiers, status,
/// category, and type codes, one or more names (brand, common, functional, or
/// generic), the organizations responsible for it, human-readable descriptive
/// characteristics, associations to related products, the base unit of measure
/// and net content, and physical instance details such as serial number, lot or
/// batch number, and expiry. In FHIR R5 it is intended for materials
/// management, warehousing, stock counting, ordering, and related administrative
/// and supply-chain workflows, and may point to a more detailed clinical product
/// resource when one exists.
///
/// # Related resources
///
/// An inventory item frequently references organizations via
/// [`Organization`](crate::r5::resources::organization::Organization), and its
/// physical instances may be associated with a
/// [`Location`](crate::r5::resources::location::Location). When the tracked item
/// is a clinical product, the product reference commonly points to a
/// [`Medication`](crate::r5::resources::medication::Medication) or
/// [`Device`](crate::r5::resources::device::Device). Coded fields use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and quantities use
/// [`Quantity`](crate::r5::types::Quantity).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItem;
/// use fhir::r5::types;
///
/// let value = InventoryItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InventoryItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
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

    /// Business identifier for the inventory item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Lifecycle status of this inventory record: active, inactive, entered-in-error, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::InventoryitemStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Category or class of the item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Coded designation of the specific type of item being inventoried, drawn from a code system.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// The item name(s) - the brand name, or common name, functional name, generic name or others
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<InventoryItemName>,

    /// Organization(s) responsible for the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub responsible_organization: Vec<InventoryItemResponsibleOrganization>,

    /// Descriptive characteristics of the item
    pub description: Option<InventoryItemDescription>,

    /// The usage status like recalled, in use, discarded
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inventory_status: Vec<types::CodeableConcept>,

    /// The base unit of measure - the unit in which the product is used or counted
    pub base_unit: Option<types::CodeableConcept>,

    /// Net content or amount present in the item
    pub net_content: Option<types::Quantity>,

    /// Association with other items or products
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub association: Vec<InventoryItemAssociation>,

    /// Characteristic of the item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<InventoryItemCharacteristic>,

    /// Instances or occurrences of the product
    pub instance: Option<InventoryItemInstance>,

    /// Reference to a more detailed clinical product resource, such as a Medication or Device, that this inventory item represents.
    pub product_reference: Option<types::Reference>,
}

/// The item name(s) - the brand name, or common name, functional name, generic
/// name or others.
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItemName;
/// use fhir::r5::types;
///
/// let value = InventoryItemName {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InventoryItemName = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemName {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of name e.g. 'brand-name', 'functional-name', 'common-name'
    pub name_type: types::Coding,

    /// The language used to express the item name
    pub language: types::Code,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// The name or designation of the item
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
}

/// Organization(s) responsible for the product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItemResponsibleOrganization;
/// use fhir::r5::types;
///
/// let value = InventoryItemResponsibleOrganization {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InventoryItemResponsibleOrganization = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemResponsibleOrganization {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The role of the organization e.g. manufacturer, distributor, or other
    pub role: types::CodeableConcept,

    /// An organization that is associated with the item
    pub organization: types::Reference,
}

/// Descriptive characteristics of the item.
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItemDescription;
/// use fhir::r5::types;
///
/// let value = InventoryItemDescription {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: InventoryItemDescription = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemDescription {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language that is used in the item description
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Textual description of the item
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Association with other items or products.
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItemAssociation;
/// use fhir::r5::types;
///
/// let value = InventoryItemAssociation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InventoryItemAssociation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemAssociation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of association between the device and the other item
    pub association_type: types::CodeableConcept,

    /// The related item or product
    pub related_item: types::Reference,

    /// The quantity of the product in this product
    pub quantity: types::Ratio,
}

/// Characteristic of the item.
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItemCharacteristic;
/// use fhir::r5::types;
///
/// let value = InventoryItemCharacteristic {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: InventoryItemCharacteristic = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The characteristic that is being defined
    pub characteristic_type: types::CodeableConcept,

    /// The `InventoryItem.characteristic.value[x]` choice element (0..1); see [`InventoryItemCharacteristicValue`].
    #[serde(flatten)]
    pub value: Option<InventoryItemCharacteristicValue>,
}

/// Instances or occurrences of the product.
/// # Examples
///
/// ```
/// use fhir::r5::resources::inventory_item::InventoryItemInstance;
/// use fhir::r5::types;
///
/// let value = InventoryItemInstance {
///     lot_number: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `lotNumber` is the name this serializes to on the wire.
/// assert_eq!(json["lotNumber"], ::serde_json::json!("abc"));
///
/// let back: InventoryItemInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The identifier for the physical instance, typically a serial number
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The lot or batch number of the item
    pub lot_number: Option<types::String>,
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`).
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// The expiry date or date and time for the product
    pub expiry: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiry`](Self::expiry) (FHIR `_expiry`).
    #[serde(rename = "_expiry")]
    pub expiry_ext: Option<types::Element>,

    /// The subject that the item is associated with
    pub subject: Option<types::Reference>,

    /// The location that the item is associated with
    pub location: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = InventoryItem;

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
/// The `InventoryItem.characteristic.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum InventoryItemCharacteristicValue {
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r5::choice::Primitive<types::Decimal>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueUrl` variant.
    #[fhir("valueUrl")]
    Url(crate::r5::choice::Primitive<types::Url>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueAnnotation` variant.
    #[fhir("valueAnnotation")]
    Annotation(Box<types::Annotation>),
    /// `valueAddress` variant.
    #[fhir("valueAddress")]
    Address(Box<types::Address>),
    /// `valueDuration` variant.
    #[fhir("valueDuration")]
    Duration(Box<types::Duration>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}
