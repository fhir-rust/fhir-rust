//! BiologicallyDerivedProduct
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BiologicallyDerivedProduct
//!
//! Version: 6.0.0-ballot3
//!
//! This resource reflects an instance of a biologically derived product
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A biological material originating from a biological entity intended to be
/// transplanted or infused into another (possibly the same) biological entity.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::biologically_derived_product::BiologicallyDerivedProduct;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A category or classification of the product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub product_category: Vec<types::CodeableConcept>,

    /// A code that identifies the kind of this biologically derived product
    pub product_code: Option<types::CodeableConcept>,

    /// The parent biologically-derived product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<types::Reference<crate::r6::resources::BiologicallyDerivedProduct>>,

    /// Request to obtain and/or infuse this product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<types::Reference<crate::r6::resources::ServiceRequest>>,

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// An identifier that supports traceability to the event during which
    /// material in this product from one or more biological entities was
    /// obtained or pooled
    pub biological_source_event: Option<types::Identifier>,

    /// Processing facilities responsible for the labeling and distribution of
    /// this biologically derived product
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processing_facility: Vec<types::Reference<crate::r6::resources::Organization>>,

    /// A unique identifier for an aliquot of a product
    pub division: Option<types::String>,
    /// Primitive extension sibling for [`division`](Self::division) (FHIR `_division`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_division")]
    pub division_ext: Option<types::Element>,

    /// available | unavailable | processed | applied | discarded
    pub product_status: Option<types::Coding>,

    /// Date, and where relevant time, of expiration
    pub expiration_date: Option<types::DateTime>,
    /// Primitive extension sibling for [`expiration_date`](Self::expiration_date) (FHIR `_expirationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expirationDate")]
    pub expiration_date_ext: Option<types::Element>,

    /// How this product was collected
    pub collection: Option<BiologicallyDerivedProductCollection>,

    /// Product storage temperature requirements
    pub storage_temp_requirements: Option<types::Range>,

    /// A property that is specific to this BiologicallyDerviedProduct instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property: Vec<BiologicallyDerivedProductProperty>,
}

/// How this product was collected.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::biologically_derived_product::BiologicallyDerivedProductCollection;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// The patient who underwent the medical procedure to collect the product
    /// or the organization that facilitated the collection
    pub source: Option<types::Reference>,

    /// Time of product collection
    /// The `BiologicallyDerivedProduct.collection.collected[x]` choice element (0..1); see [`BiologicallyDerivedProductCollectionCollected`].
    #[serde(flatten)]
    pub collected: Option<BiologicallyDerivedProductCollectionCollected>,

    /// The procedure involved in the collection
    pub procedure: Option<types::Reference<crate::r6::resources::Procedure>>,
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
    collected: crate::r6::choice::Slot<BiologicallyDerivedProductCollectionCollected>,
    procedure: Option<types::Reference<crate::r6::resources::Procedure>>,
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
            procedure: v.procedure,
        }
    }
}

/// A property that is specific to this BiologicallyDerviedProduct instance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::biologically_derived_product::BiologicallyDerivedProductProperty;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Property values
    /// The `BiologicallyDerivedProduct.property.value[x]` choice element (1..1); see [`BiologicallyDerivedProductPropertyValue`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
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
    value: crate::r6::choice::Slot<BiologicallyDerivedProductPropertyValue>,
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

/// The `BiologicallyDerivedProduct.collection.collected[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum BiologicallyDerivedProductCollectionCollected {
    /// `collectedDateTime` variant.
    #[fhir("collectedDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `collectedPeriod` variant.
    #[fhir("collectedPeriod")]
    Period(Box<types::Period>),
}

/// The `BiologicallyDerivedProduct.property.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum BiologicallyDerivedProductPropertyValue {
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `valueInteger` variant.
    #[fhir("valueInteger")]
    Integer(crate::r6::choice::Primitive<types::Integer>),
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
    String(crate::r6::choice::Primitive<types::String>),
    /// `valueAttachment` variant.
    #[fhir("valueAttachment")]
    Attachment(Box<types::Attachment>),
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
