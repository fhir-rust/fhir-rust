//! PackagedProductDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/PackagedProductDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! A medically related item or items, in a container or package
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A medically related item or items, in a container or package.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::packaged_product_definition::PackagedProductDefinition;
/// use fhir::r6::types;
///
/// let value = PackagedProductDefinition {
///     status_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusDate` is the name this serializes to on the wire.
/// assert_eq!(json["statusDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: PackagedProductDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PackagedProductDefinition {
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

    /// A unique identifier for this package as whole - not for the content of
    /// the package
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A name for this package. Typically as listed in a drug formulary,
    /// catalogue, inventory etc
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// A high level category e.g. medicinal product, raw material, shipping
    /// container etc
    pub r#type: Option<types::CodeableConcept>,

    /// The product that this is a pack for
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub package_for: Vec<types::Reference<crate::r6::resources::MedicinalProductDefinition>>,

    /// The status within the lifecycle of this item. High level - not intended
    /// to duplicate details elsewhere e.g. legal status, or
    /// authorization/marketing status
    pub status: Option<types::CodeableConcept>,

    /// The date at which the given status became applicable
    pub status_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_date`](Self::status_date) (FHIR `_statusDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_statusDate")]
    pub status_date_ext: Option<types::Element>,

    /// A total of the complete count of contained items of a particular
    /// type/form, independent of sub-packaging or organization. This can be
    /// considered as the pack size. See also packaging.containedItem.amount
    /// (especially the long definition)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained_item_quantity: Vec<types::Quantity>,

    /// Textual description. Note that this is not the name of the package or
    /// product
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The legal status of supply of the packaged item as classified by the
    /// regulator
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub legal_status_of_supply: Vec<PackagedProductDefinitionLegalStatusOfSupply>,

    /// Allows specifying that an item is on the market for sale, or that it is
    /// not available, and the dates and locations associated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marketing_status: Vec<types::MarketingStatus>,

    /// Identifies if the drug product is supplied with another item such as a
    /// diluent or adjuvant
    pub copackaged_indicator: Option<types::Boolean>,
    /// Primitive extension sibling for [`copackaged_indicator`](Self::copackaged_indicator) (FHIR `_copackagedIndicator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copackagedIndicator")]
    pub copackaged_indicator_ext: Option<types::Element>,

    /// Manufacturer of this package type (multiple means these are all
    /// possible manufacturers)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference<crate::r6::resources::Organization>>,

    /// Additional information or supporting documentation about the packaged
    /// product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attached_document: Vec<types::Reference<crate::r6::resources::DocumentReference>>,

    /// A packaging item, as a container for medically related items, possibly
    /// with other packaging items within, or a packaging component, such as
    /// bottle cap
    pub packaging: Option<PackagedProductDefinitionPackaging>,

    /// Allows the key features to be recorded, such as "hospital pack", "nurse
    /// prescribable"
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<PackagedProductDefinitionPackagingProperty>,
}

/// The legal status of supply of the packaged item as classified by the
/// regulator.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::packaged_product_definition::PackagedProductDefinitionLegalStatusOfSupply;
/// use fhir::r6::types;
///
/// let value = PackagedProductDefinitionLegalStatusOfSupply {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PackagedProductDefinitionLegalStatusOfSupply = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PackagedProductDefinitionLegalStatusOfSupply {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The actual status of supply. In what situation this package type may be
    /// supplied for use
    pub code: Option<types::CodeableConcept>,

    /// The place where the legal status of supply applies
    pub jurisdiction: Option<types::CodeableConcept>,
}

/// A packaging item, as a container for medically related items, possibly with
/// other packaging items within, or a packaging component, such as bottle cap
/// (which is not a device or a medication manufactured item).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::packaged_product_definition::PackagedProductDefinitionPackaging;
/// use fhir::r6::types;
///
/// let value = PackagedProductDefinitionPackaging {
///     component_part: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `componentPart` is the name this serializes to on the wire.
/// assert_eq!(json["componentPart"], ::serde_json::json!(true));
///
/// let back: PackagedProductDefinitionPackaging = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PackagedProductDefinitionPackaging {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// An identifier that is specific to this particular part of the
    /// packaging. Including possibly a Data Carrier Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The physical type of the container of the items
    pub r#type: Option<types::CodeableConcept>,

    /// Is this a part of the packaging (e.g. a cap or bottle stopper), rather
    /// than the packaging itself (e.g. a bottle or vial)
    pub component_part: Option<types::Boolean>,
    /// Primitive extension sibling for [`component_part`](Self::component_part) (FHIR `_componentPart`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_componentPart")]
    pub component_part_ext: Option<types::Element>,

    /// The quantity of this level of packaging in the package that contains it
    /// (with the outermost level being 1)
    pub quantity: Option<types::Integer>,
    /// Primitive extension sibling for [`quantity`](Self::quantity) (FHIR `_quantity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_quantity")]
    pub quantity_ext: Option<types::Element>,

    /// Material type of the package item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub material: Vec<types::CodeableConcept>,

    /// A possible alternate material for this part of the packaging, that is
    /// allowed to be used instead of the usual material
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_material: Vec<types::CodeableConcept>,

    /// Shelf Life and storage information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shelf_life_storage: Vec<types::ProductShelfLife>,

    /// Manufacturer of this packaging item (multiple means these are all
    /// potential manufacturers)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference<crate::r6::resources::Organization>>,

    /// General characteristics of this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<PackagedProductDefinitionPackagingProperty>,

    /// The item(s) within the packaging
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained_item: Vec<PackagedProductDefinitionPackagingContainedItem>,

    /// Allows containers (and parts of containers) within containers, still as
    /// a part of single packaged product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packaging: Vec<PackagedProductDefinitionPackaging>,
}

/// The item(s) within the packaging.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::packaged_product_definition::PackagedProductDefinitionPackagingContainedItem;
/// use fhir::r6::types;
///
/// let value = PackagedProductDefinitionPackagingContainedItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PackagedProductDefinitionPackagingContainedItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct PackagedProductDefinitionPackagingContainedItem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The actual item(s) of medication, as manufactured, or a device, or
    /// other medically related item (food, biologicals, raw materials, medical
    /// fluids, gases etc.), as contained in the package
    pub item: types::CodeableReference,

    /// The number of this type of item within this packaging or for continuous
    /// items such as liquids it is the quantity (for example 25ml). See also
    /// PackagedProductDefinition.containedItemQuantity (especially the long
    /// definition)
    pub amount: Option<types::Quantity>,
}

/// General characteristics of this item.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::packaged_product_definition::PackagedProductDefinitionPackagingProperty;
/// use fhir::r6::types;
///
/// let value = PackagedProductDefinitionPackagingProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: PackagedProductDefinitionPackagingProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "PackagedProductDefinitionPackagingPropertyDe")]
#[fhir_version("r6")]
pub struct PackagedProductDefinitionPackagingProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A code expressing the type of characteristic
    pub r#type: types::CodeableConcept,

    /// A value for the characteristic
    /// The `PackagedProductDefinition.packaging.property.value[x]` choice element (0..1); see [`PackagedProductDefinitionPackagingPropertyValue`].
    #[serde(flatten)]
    pub value: Option<PackagedProductDefinitionPackagingPropertyValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackagedProductDefinitionPackagingPropertyDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<PackagedProductDefinitionPackagingPropertyValue>,
}

impl ::core::convert::From<PackagedProductDefinitionPackagingPropertyDe>
    for PackagedProductDefinitionPackagingProperty
{
    fn from(v: PackagedProductDefinitionPackagingPropertyDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            value: v.value.0,
        }
    }
}

/// The `PackagedProductDefinition.packaging.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum PackagedProductDefinitionPackagingPropertyValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = PackagedProductDefinition;

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
