//! ManufacturedItemDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ManufacturedItemDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! The definition and characteristics of a medicinal manufactured item, such
//! as a tablet or capsule, as contained in a packaged medicinal product
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The definition and characteristics of a medicinal manufactured item, such
/// as a tablet or capsule, as contained in a packaged medicinal product.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::manufactured_item_definition::ManufacturedItemDefinition;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ManufacturedItemDefinition {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// A descriptive name applied to this item
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Dose form as manufactured (before any necessary transformation)
    pub manufactured_dose_form: types::CodeableConcept,

    /// The “real-world” units in which the quantity of the item is described
    pub unit_of_presentation: Option<types::CodeableConcept>,

    /// Manufacturer of the item, one of several possible
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub manufacturer: Vec<types::Reference>,

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

    /// Physical parts of the manufactured item, that it is intrinsically made
    /// from. This is distinct from the ingredients that are part of its
    /// chemical makeup
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ManufacturedItemDefinitionComponent>,
}

/// Physical parts of the manufactured item, that it is intrinsically made
/// from. This is distinct from the ingredients that are part of its chemical
/// makeup.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::manufactured_item_definition::ManufacturedItemDefinitionComponent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    /// component, expressible in different ways (e.g. by mass or volume)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub amount: Vec<types::Quantity>,

    /// A reference to a constituent of the manufactured item as a whole,
    /// linked here so that its component location within the item can be
    /// indicated. This not where the item's ingredient are primarily stated
    /// (for which see Ingredient.for or ManufacturedItemDefinition.ingredient)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constituent: Vec<ManufacturedItemDefinitionComponentConstituent>,

    /// General characteristics of this component
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<ManufacturedItemDefinitionProperty>,

    /// A component that this component contains or is made from
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component: Vec<ManufacturedItemDefinitionComponent>,
}

/// A reference to a constituent of the manufactured item as a whole, linked
/// here so that its component location within the item can be indicated. This
/// not where the item's ingredient are primarily stated (for which see
/// Ingredient.for or ManufacturedItemDefinition.ingredient).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::manufactured_item_definition::ManufacturedItemDefinitionComponentConstituent;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ManufacturedItemDefinitionComponentConstituent {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The measurable amount of the substance, expressible in different ways
    /// (e.g. by mass or volume)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub amount: Vec<types::Quantity>,

    /// The physical location of the constituent/ingredient within the
    /// component
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::CodeableConcept>,

    /// The function of this constituent within the component e.g. binder
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub function: Vec<types::CodeableConcept>,

    /// The ingredient that is the constituent of the given component
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub has_ingredient: Vec<types::CodeableReference>,
}

/// General characteristics of this item.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::manufactured_item_definition::ManufacturedItemDefinitionProperty;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// A value for the characteristic
    /// The `ManufacturedItemDefinition.property.value[x]` choice element (0..1); see [`ManufacturedItemDefinitionPropertyValue`].
    #[serde(flatten)]
    pub value: Option<ManufacturedItemDefinitionPropertyValue>,
}

/// The `ManufacturedItemDefinition.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueMarkdown` variant.
    #[fhir("valueMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
    /// `valueReference` variant.
    #[fhir("valueReference")]
    Reference(Box<types::Reference>),
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
