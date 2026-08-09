//! InventoryItem
//!
//! URL: http://hl7.org/fhir/StructureDefinition/InventoryItem
//!
//! Version: 6.0.0-ballot3
//!
//! A functional description of an inventory item used in inventory and
//! supply-related workflows
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// functional description of an inventory item used in inventory and
/// supply-related workflows.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_item::InventoryItem;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct InventoryItem {
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

    /// Business identifier for the inventory item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | inactive | entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::InventoryitemStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Category or class of the item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Code designating the specific type of item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::CodeableConcept>,

    /// The item name(s) - the brand name, or common name, functional name,
    /// generic name or others
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

    /// The base unit of measure - the unit in which the product is used or
    /// counted
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

    /// Link to a product resource used in clinical workflows
    pub product_reference: Option<types::Reference>,
}

/// Association with other items or products.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_item::InventoryItemAssociation;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// The descriptive or identifying characteristics of the item.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_item::InventoryItemCharacteristic;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// The value of the attribute
    /// The `InventoryItem.characteristic.value[x]` choice element (1..1); see [`InventoryItemCharacteristicValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub value: Option<InventoryItemCharacteristicValue>,
}

/// The descriptive characteristics of the inventory item.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_item::InventoryItemDescription;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Textual description of the item
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Instances or occurrences of the product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_item::InventoryItemInstance;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`lot_number`](Self::lot_number) (FHIR `_lotNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lotNumber")]
    pub lot_number_ext: Option<types::Element>,

    /// The expiry date or date and time for the product
    pub expiry: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiry`](Self::expiry) (FHIR `_expiry`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expiry")]
    pub expiry_ext: Option<types::Element>,

    /// The subject that the item is associated with
    pub subject: Option<types::Reference>,

    /// The location that the item is associated with
    pub location: Option<types::Reference<crate::r6::resources::Location>>,
}

/// The item name(s) - the brand name, or common name, functional name, generic
/// name.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_item::InventoryItemName;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// The name or designation of the item
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,
}

/// Organization(s) responsible for the product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::inventory_item::InventoryItemResponsibleOrganization;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub organization: types::Reference<crate::r6::resources::Organization>,
}

/// The `InventoryItem.characteristic.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum InventoryItemCharacteristicValue {
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
    /// `valueDecimal` variant.
    #[fhir("valueDecimal")]
    Decimal(crate::r6::choice::Primitive<types::Decimal>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueUrl` variant.
    #[fhir("valueUrl")]
    Url(crate::r6::choice::Primitive<types::Url>),
    /// `valueDateTime` variant.
    #[fhir("valueDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
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
