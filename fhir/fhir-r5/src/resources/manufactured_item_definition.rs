//! ManufacturedItemDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition
//!
//! Version: 5.0.0
//!
//! ManufacturedItemDefinition Resource: The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// The definition and characteristics of a medicinal manufactured item, such as
/// a tablet, capsule, patch, or ampoule, as contained in a packaged medicinal
/// product. It captures the item exactly as it is manufactured, before any
/// transformation that may be needed for administration, and records its
/// manufactured dose form, unit of presentation, manufacturers, marketing
/// status, ingredients, physical components, and general characteristics.
///
/// In FHIR R5 this resource belongs to the medication definition family and is
/// used chiefly in medicinal product regulatory submissions, such as those
/// following the ISO IDMP standards, as well as in packaging and supply-chain
/// descriptions. A ManufacturedItemDefinition is typically referenced from a
/// packaged product to say what physical items a pack contains, and its
/// ingredients and constituents may point to more detailed substance and
/// ingredient definitions. Coded fields such as the manufactured dose form use
/// [`CodeableConcept`](crate::r5::types::CodeableConcept), and availability is
/// expressed through [`MarketingStatus`](crate::r5::types::MarketingStatus).
///
/// # Related resources
///
/// See also the `PackagedProductDefinition`, `MedicinalProductDefinition`, and
/// `Ingredient` resources, which together describe a medicinal product and how
/// its manufactured items are packaged, marketed, and composed.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::manufactured_item_definition::ManufacturedItemDefinition;
/// use fhir::r5::types;
///
/// let value = ManufacturedItemDefinition {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: ManufacturedItemDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinition {
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

    /// Business identifier for this manufactured item, distinct from the resource's logical id
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Publication status of this definition, one of draft, active, retired, or unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// A descriptive name applied to this item, suitable for labeling or catalog display
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Dose form of the item as manufactured, such as tablet or capsule, before any transformation needed for administration
    pub manufactured_dose_form: types::CodeableConcept,

    /// The "real-world" units in which the quantity of the item is described
    pub unit_of_presentation: Option<types::CodeableConcept>,

    /// Manufacturer of the item, one of several possible, referencing an Organization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference<crate::r5::resources::Organization>>,

    /// Allows specifying that an item is on the market for sale, or that it is
    /// not available, and the dates and locations associated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub marketing_status: Vec<types::MarketingStatus>,

    /// The ingredients of this manufactured item. Only needed if these are not
    /// specified by incoming references from the Ingredient resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ingredient: Vec<types::CodeableConcept>,

    /// General characteristics of this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ManufacturedItemDefinitionProperty>,

    /// Physical parts of the manufactured item, that it is intrisically made from
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ManufacturedItemDefinitionComponent>,
}

/// General characteristics of this item.
/// # Examples
///
/// ```
/// use fhir::r5::resources::manufactured_item_definition::ManufacturedItemDefinitionProperty;
/// use fhir::r5::types;
///
/// let value = ManufacturedItemDefinitionProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ManufacturedItemDefinitionProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ManufacturedItemDefinitionPropertyDe")]
pub struct ManufacturedItemDefinitionProperty {
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

    /// The `ManufacturedItemDefinition.property.value[x]` choice element (0..1); see [`ManufacturedItemDefinitionPropertyValue`].
    #[serde(flatten)]
    pub value: Option<ManufacturedItemDefinitionPropertyValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ManufacturedItemDefinitionPropertyDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<ManufacturedItemDefinitionPropertyValue>,
}

impl ::core::convert::From<ManufacturedItemDefinitionPropertyDe>
    for ManufacturedItemDefinitionProperty
{
    fn from(v: ManufacturedItemDefinitionPropertyDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            value: v.value.0,
        }
    }
}

/// Physical parts of the manufactured item, that it is intrisically made from.
/// This is distinct from the ingredients that are part of its chemical makeup.
/// # Examples
///
/// ```
/// use fhir::r5::resources::manufactured_item_definition::ManufacturedItemDefinitionComponent;
/// use fhir::r5::types;
///
/// let value = ManufacturedItemDefinitionComponent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ManufacturedItemDefinitionComponent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionComponent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Defining type of the component e.g. shell, layer, ink
    pub r#type: types::CodeableConcept,

    /// The function of this component within the item e.g. delivers active
    /// ingredient, masks taste
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<types::CodeableConcept>,

    /// The measurable amount of total quantity of all substances in the
    /// component, expressable in different ways (e.g. by mass or volume)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub amount: Vec<types::Quantity>,

    /// A reference to a constituent of the manufactured item as a whole, linked
    /// here so that its component location within the item can be indicated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constituent: Vec<ManufacturedItemDefinitionComponentConstituent>,

    /// General characteristics of this component
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ManufacturedItemDefinitionProperty>,

    /// A component that this component contains or is made from
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ManufacturedItemDefinitionComponent>,
}

/// A reference to a constituent of the manufactured item as a whole, linked here
/// so that its component location within the item can be indicated.
/// # Examples
///
/// ```
/// use fhir::r5::resources::manufactured_item_definition::ManufacturedItemDefinitionComponentConstituent;
/// use fhir::r5::types;
///
/// let value = ManufacturedItemDefinitionComponentConstituent {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ManufacturedItemDefinitionComponentConstituent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionComponentConstituent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The measurable amount of the substance, expressable in different ways
    /// (e.g. by mass or volume)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub amount: Vec<types::Quantity>,

    /// The physical location of the constituent/ingredient within the component
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::CodeableConcept>,

    /// The function of this constituent within the component e.g. binder
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<types::CodeableConcept>,

    /// The ingredient that is the constituent of the given component
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_ingredient: Vec<types::CodeableReference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ManufacturedItemDefinition;

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
/// The `ManufacturedItemDefinition.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ManufacturedItemDefinitionPropertyValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueDate` variant.
    #[fhir("valueDate")]
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
}
