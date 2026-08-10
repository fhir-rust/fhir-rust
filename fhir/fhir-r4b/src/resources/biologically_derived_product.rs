//! BiologicallyDerivedProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
//!
//! Version: 4.3.0
//!
//! A material substance originating from a biological entity
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A material substance originating from a biological entity intended to be
/// transplanted or infused into another (possibly the same) biological entity.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::biologically_derived_product::BiologicallyDerivedProduct;
/// use fhir::r4b::types;
///
/// let value = BiologicallyDerivedProduct {
///     quantity: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `quantity` is the name this serializes to on the wire.
/// assert_eq!(json["quantity"], ::serde_json::json!(42));
///
/// let back: BiologicallyDerivedProduct = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct BiologicallyDerivedProduct {
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

    /// External ids for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// organ | tissue | fluid | cells | biologicalAgent
    pub product_category: Option<crate::coded::Coded<crate::r4b::codes::ProductCategory>>,
    /// Primitive extension sibling for [`product_category`](Self::product_category) (FHIR `_productCategory`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_productCategory")]
    pub product_category_ext: Option<types::Element>,

    /// What this biologically derived product is
    pub product_code: Option<types::CodeableConcept>,

    /// available | unavailable
    pub status: Option<crate::coded::Coded<crate::r4b::codes::ProductStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Procedure request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<types::Reference<crate::r4b::resources::ServiceRequest>>,

    /// The amount of this biologically derived product
    pub quantity: Option<types::Integer>,
    /// Primitive extension sibling for [`quantity`](Self::quantity) (FHIR `_quantity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_quantity")]
    pub quantity_ext: Option<types::Element>,

    /// BiologicallyDerivedProduct parent
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<types::Reference<crate::r4b::resources::BiologicallyDerivedProduct>>,

    /// How this product was collected
    pub collection: Option<BiologicallyDerivedProductCollection>,

    /// Any processing of the product during collection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processing: Vec<BiologicallyDerivedProductProcessing>,

    /// Any manipulation of product post-collection
    pub manipulation: Option<BiologicallyDerivedProductManipulation>,

    /// Product storage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub storage: Vec<BiologicallyDerivedProductStorage>,
}

/// How this product was collected.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::biologically_derived_product::BiologicallyDerivedProductCollection;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

    /// Who is product from
    pub source: Option<types::Reference>,

    /// Time of product collection
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
    collected: crate::r4b::choice::Slot<BiologicallyDerivedProductCollectionCollected>,
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

/// Any manipulation of product post-collection that is intended to alter the
/// product. For example a buffy-coat enrichment or CD8 reduction of Peripheral
/// Blood Stem Cells to make it more suitable for infusion.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::biologically_derived_product::BiologicallyDerivedProductManipulation;
/// use fhir::r4b::types;
///
/// let value = BiologicallyDerivedProductManipulation {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: BiologicallyDerivedProductManipulation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "BiologicallyDerivedProductManipulationDe")]
#[fhir_version("r4b")]
pub struct BiologicallyDerivedProductManipulation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of manipulation
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Time of manipulation
    /// The `BiologicallyDerivedProduct.manipulation.time[x]` choice element (0..1); see [`BiologicallyDerivedProductManipulationTime`].
    #[serde(flatten)]
    pub time: Option<BiologicallyDerivedProductManipulationTime>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BiologicallyDerivedProductManipulationDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(flatten)]
    time: crate::r4b::choice::Slot<BiologicallyDerivedProductManipulationTime>,
}

impl ::core::convert::From<BiologicallyDerivedProductManipulationDe>
    for BiologicallyDerivedProductManipulation
{
    fn from(v: BiologicallyDerivedProductManipulationDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            description: v.description,
            description_ext: v.description_ext,
            time: v.time.0,
        }
    }
}

/// Any processing of the product during collection that does not change the
/// fundamental nature of the product. For example adding anti-coagulants
/// during the collection of Peripheral Blood Stem Cells.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::biologically_derived_product::BiologicallyDerivedProductProcessing;
/// use fhir::r4b::types;
///
/// let value = BiologicallyDerivedProductProcessing {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: BiologicallyDerivedProductProcessing = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "BiologicallyDerivedProductProcessingDe")]
#[fhir_version("r4b")]
pub struct BiologicallyDerivedProductProcessing {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of of processing
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Procesing code
    pub procedure: Option<types::CodeableConcept>,

    /// Substance added during processing
    pub additive: Option<types::Reference<crate::r4b::resources::Substance>>,

    /// Time of processing
    /// The `BiologicallyDerivedProduct.processing.time[x]` choice element (0..1); see [`BiologicallyDerivedProductProcessingTime`].
    #[serde(flatten)]
    pub time: Option<BiologicallyDerivedProductProcessingTime>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BiologicallyDerivedProductProcessingDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    procedure: Option<types::CodeableConcept>,
    additive: Option<types::Reference<crate::r4b::resources::Substance>>,
    #[serde(flatten)]
    time: crate::r4b::choice::Slot<BiologicallyDerivedProductProcessingTime>,
}

impl ::core::convert::From<BiologicallyDerivedProductProcessingDe>
    for BiologicallyDerivedProductProcessing
{
    fn from(v: BiologicallyDerivedProductProcessingDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            description: v.description,
            description_ext: v.description_ext,
            procedure: v.procedure,
            additive: v.additive,
            time: v.time.0,
        }
    }
}

/// Product storage.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::biologically_derived_product::BiologicallyDerivedProductStorage;
/// use fhir::r4b::types;
///
/// let value = BiologicallyDerivedProductStorage {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: BiologicallyDerivedProductStorage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct BiologicallyDerivedProductStorage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of storage
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Storage temperature
    pub temperature: Option<types::Decimal>,
    /// Primitive extension sibling for [`temperature`](Self::temperature) (FHIR `_temperature`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_temperature")]
    pub temperature_ext: Option<types::Element>,

    /// farenheit | celsius | kelvin
    pub scale: Option<crate::coded::Coded<crate::r4b::codes::ProductStorageScale>>,
    /// Primitive extension sibling for [`scale`](Self::scale) (FHIR `_scale`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_scale")]
    pub scale_ext: Option<types::Element>,

    /// Storage timeperiod
    pub duration: Option<types::Period>,
}

/// The `BiologicallyDerivedProduct.collection.collected[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum BiologicallyDerivedProductCollectionCollected {
    /// `collectedDateTime` variant.
    #[fhir("collectedDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `collectedPeriod` variant.
    #[fhir("collectedPeriod")]
    Period(Box<types::Period>),
}

/// The `BiologicallyDerivedProduct.manipulation.time[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum BiologicallyDerivedProductManipulationTime {
    /// `timeDateTime` variant.
    #[fhir("timeDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `timePeriod` variant.
    #[fhir("timePeriod")]
    Period(Box<types::Period>),
}

/// The `BiologicallyDerivedProduct.processing.time[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum BiologicallyDerivedProductProcessingTime {
    /// `timeDateTime` variant.
    #[fhir("timeDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `timePeriod` variant.
    #[fhir("timePeriod")]
    Period(Box<types::Period>),
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
