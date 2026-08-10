//! BiologicallyDerivedProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
//!
//! Version: 5.0.0
//!
//! BiologicallyDerivedProduct Resource: A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// BiologicallyDerivedProduct
///
/// A biological material originating from a biological entity intended to be
/// transplanted or infused into another (possibly the same) biological entity.
/// This resource captures products such as organs, tissues, fluids, cells, and
/// biological agents, along with their provenance, collection, storage, and
/// instance-specific properties. It supports traceability of the material from
/// its biological source event through processing and distribution.
///
/// In clinical and administrative workflows, `BiologicallyDerivedProduct` is
/// typically used by blood banks, tissue banks, organ procurement organizations,
/// and cellular therapy laboratories to record and track individual product
/// instances as they move from collection through processing, storage, and
/// eventual transplantation or infusion. Each instance may reference a parent
/// product (for example, an aliquot derived from a larger donation), carry a
/// biological source event identifier that links related products back to a
/// single donation or pooling event, and record the processing facilities
/// responsible for labeling and distribution. The resource is commonly
/// referenced by other resources that order or perform the transplant or
/// infusion procedure.
///
/// # Related resources
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — often the source or
///   recipient referenced via the `collection.source` or `request` elements.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for
///   `product_code` and property `type` codes.
/// - `Procedure` and `ServiceRequest` — commonly reference this resource to
///   describe the collection or the transplant/infusion request.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::biologically_derived_product::BiologicallyDerivedProduct;
/// use fhir::r5::types;
///
/// let value = BiologicallyDerivedProduct {
///     expiration_date: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `expirationDate` is the name this serializes to on the wire.
/// assert_eq!(json["expirationDate"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: BiologicallyDerivedProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProduct {
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

    /// Broad classification of the product: organ | tissue | fluid | cells | biologicalAgent
    pub product_category: Option<types::Coding>,

    /// A code that identifies the specific kind of this biologically derived product, such as a whole organ or a specific tissue or cell type
    pub product_code: Option<types::CodeableConcept>,

    /// The parent biologically-derived product, when this instance is an aliquot or derivative of another product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<types::Reference<crate::r5::resources::BiologicallyDerivedProduct>>,

    /// Request to obtain and/or infuse this product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<types::Reference<crate::r5::resources::ServiceRequest>>,

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// An identifier that supports traceability to the event during which material
    /// in this product from one or more biological entities was obtained or pooled
    pub biological_source_event: Option<types::Identifier>,

    /// Processing facilities responsible for the labeling and distribution of this
    /// biologically derived product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processing_facility: Vec<types::Reference<crate::r5::resources::Organization>>,

    /// A unique identifier for an aliquot of a product
    pub division: Option<types::String>,
    /// Primitive extension sibling for [`division`](Self::division) (FHIR `_division`).
    #[serde(rename = "_division")]
    pub division_ext: Option<types::Element>,

    /// available | unavailable
    pub product_status: Option<types::Coding>,

    /// Date, and where relevant time, of expiration
    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`).
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// How and by whom this product was collected, and from what source
    pub collection: Option<BiologicallyDerivedProductCollection>,

    /// Product storage temperature requirements
    pub storage_temp_requirements: Option<types::Range>,

    /// A property that is specific to this BiologicallyDerviedProduct instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<BiologicallyDerivedProductProperty>,
}

/// BiologicallyDerivedProductCollection
///
/// How this product was collected, including who performed the collection, the
/// source patient or organization, and the time it took place.
/// # Examples
///
/// ```
/// use fhir::r5::resources::biologically_derived_product::BiologicallyDerivedProductCollection;
/// use fhir::r5::types;
///
/// let value = BiologicallyDerivedProductCollection {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BiologicallyDerivedProductCollection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "BiologicallyDerivedProductCollectionDe")]
pub struct BiologicallyDerivedProductCollection {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Individual performing collection
    pub collector: Option<types::Reference>,

    /// The patient who underwent the medical procedure to collect the product or
    /// the organization that facilitated the collection
    pub source: Option<types::Reference>,

    /// The `BiologicallyDerivedProduct.collection.collected[x]` choice element (0..1); see [`BiologicallyDerivedProductCollectionCollected`].
    #[serde(flatten)]
    pub collected: Option<BiologicallyDerivedProductCollectionCollected>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BiologicallyDerivedProductCollectionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    collector: Option<types::Reference>,
    source: Option<types::Reference>,
    #[serde(flatten)]
    collected: crate::r5::choice::Slot<BiologicallyDerivedProductCollectionCollected>,
}

impl ::core::convert::From<BiologicallyDerivedProductCollectionDe>
    for BiologicallyDerivedProductCollection
{
    fn from(v: BiologicallyDerivedProductCollectionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            collector: v.collector,
            source: v.source,
            collected: v.collected.0,
        }
    }
}

/// BiologicallyDerivedProductProperty
///
/// A property that is specific to this BiologicallyDerivedProduct instance,
/// expressed as a typed code paired with one of several possible value types.
/// # Examples
///
/// ```
/// use fhir::r5::resources::biologically_derived_product::BiologicallyDerivedProductProperty;
/// use fhir::r5::types;
///
/// let value = BiologicallyDerivedProductProperty {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BiologicallyDerivedProductProperty = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "BiologicallyDerivedProductPropertyDe")]
pub struct BiologicallyDerivedProductProperty {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that specifies the property
    pub r#type: types::CodeableConcept,

    /// The `BiologicallyDerivedProduct.property.value[x]` choice element (0..1); see [`BiologicallyDerivedProductPropertyValue`].
    #[serde(flatten)]
    pub value: Option<BiologicallyDerivedProductPropertyValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BiologicallyDerivedProductPropertyDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    r#type: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<BiologicallyDerivedProductPropertyValue>,
}

impl ::core::convert::From<BiologicallyDerivedProductPropertyDe>
    for BiologicallyDerivedProductProperty
{
    fn from(v: BiologicallyDerivedProductPropertyDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            r#type: v.r#type,
            value: v.value.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = BiologicallyDerivedProduct;

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
/// The `BiologicallyDerivedProduct.collection.collected[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum BiologicallyDerivedProductCollectionCollected {
    /// `collectedDateTime` variant.
    #[fhir("collectedDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `collectedPeriod` variant.
    #[fhir("collectedPeriod")]
    Period(Box<types::Period>),
}

/// The `BiologicallyDerivedProduct.property.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum BiologicallyDerivedProductPropertyValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r5::choice::Primitive<types::Integer>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
}
