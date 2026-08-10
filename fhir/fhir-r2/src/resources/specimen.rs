//! Specimen
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Specimen
//!
//!
//!
//! Sample for analysis
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Specimen Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::specimen::Specimen;
/// use fhir::r2::types;
///
/// let value = Specimen {
///     received_time: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `receivedTime` is the name this serializes to on the wire.
/// assert_eq!(json["receivedTime"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Specimen = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Specimen {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// available | unavailable | unsatisfactory | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r2::codes::SpecimenStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of material that forms the specimen
    pub r#type: Option<types::CodeableConcept>,

    /// Specimen from which this specimen originated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<types::Reference<crate::r2::resources::Specimen>>,

    /// Where the specimen came from. This may be from the patient(s) or from
    /// the environment or a device
    pub subject: types::Reference,

    /// Identifier assigned by the lab
    pub accession_identifier: Option<types::Identifier>,

    /// The time when specimen was received for processing
    pub received_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`received_time`](Self::received_time) (FHIR `_receivedTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_receivedTime")]
    pub received_time_ext: Option<types::Element>,

    /// Collection details
    pub collection: Option<SpecimenCollection>,

    /// Treatment and processing step details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub treatment: Vec<SpecimenTreatment>,

    /// Direct container of specimen (tube/slide, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container: Vec<SpecimenContainer>,
}

/// Details concerning the specimen collection.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::specimen::SpecimenCollection;
/// use fhir::r2::types;
///
/// let value = SpecimenCollection {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: SpecimenCollection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenCollectionDe")]
#[fhir_version("r2")]
pub struct SpecimenCollection {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who collected the specimen
    pub collector: Option<types::Reference<crate::r2::resources::Practitioner>>,

    /// Collector comments
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub comment: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comment_ext: Vec<Option<types::Element>>,

    /// Collection time
    /// The `Specimen.collection.collected[x]` choice element (0..1); see [`SpecimenCollectionCollected`].
    #[serde(flatten)]
    pub collected: Option<SpecimenCollectionCollected>,

    /// The quantity of specimen collected
    pub quantity: Option<types::Quantity>,

    /// Technique used to perform collection
    pub method: Option<types::CodeableConcept>,

    /// Anatomical collection site
    pub body_site: Option<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenCollectionDe {
    id: Option<types::Id>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    collector: Option<types::Reference<crate::r2::resources::Practitioner>>,
    #[serde(default)]
    comment: ::fhir_core::PrimVec<types::String>,
    #[serde(rename = "_comment")]
    #[serde(default)]
    comment_ext: Vec<Option<types::Element>>,
    #[serde(flatten)]
    collected: crate::r2::choice::Slot<SpecimenCollectionCollected>,
    quantity: Option<types::Quantity>,
    method: Option<types::CodeableConcept>,
    body_site: Option<types::CodeableConcept>,
}

impl ::core::convert::From<SpecimenCollectionDe> for SpecimenCollection {
    fn from(v: SpecimenCollectionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            collector: v.collector,
            comment: v.comment,
            comment_ext: v.comment_ext,
            collected: v.collected.0,
            quantity: v.quantity,
            method: v.method,
            body_site: v.body_site,
        }
    }
}

/// The container holding the specimen. The recursive nature of containers;
/// i.e. blood in tube in tray in rack is not addressed here.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::specimen::SpecimenContainer;
/// use fhir::r2::types;
///
/// let value = SpecimenContainer {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: SpecimenContainer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenContainerDe")]
#[fhir_version("r2")]
pub struct SpecimenContainer {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id for the container
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Textual description of the container
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Kind of container directly associated with specimen
    pub r#type: Option<types::CodeableConcept>,

    /// Container volume or size
    pub capacity: Option<types::Quantity>,

    /// Quantity of specimen within container
    pub specimen_quantity: Option<types::Quantity>,

    /// Additive associated with container
    /// The `Specimen.container.additive[x]` choice element (0..1); see [`SpecimenContainerAdditive`].
    #[serde(flatten)]
    pub additive: Option<SpecimenContainerAdditive>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenContainerDe {
    id: Option<types::Id>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    r#type: Option<types::CodeableConcept>,
    capacity: Option<types::Quantity>,
    specimen_quantity: Option<types::Quantity>,
    #[serde(flatten)]
    additive: crate::r2::choice::Slot<SpecimenContainerAdditive>,
}

impl ::core::convert::From<SpecimenContainerDe> for SpecimenContainer {
    fn from(v: SpecimenContainerDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            description: v.description,
            description_ext: v.description_ext,
            r#type: v.r#type,
            capacity: v.capacity,
            specimen_quantity: v.specimen_quantity,
            additive: v.additive.0,
        }
    }
}

/// Details concerning treatment and processing steps for the specimen.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::specimen::SpecimenTreatment;
/// use fhir::r2::types;
///
/// let value = SpecimenTreatment {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: SpecimenTreatment = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct SpecimenTreatment {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of procedure
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Indicates the treatment or processing step applied to the specimen
    pub procedure: Option<types::CodeableConcept>,

    /// Material used in the processing step
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<types::Reference<crate::r2::resources::Substance>>,
}

/// The `Specimen.collection.collected[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenCollectionCollected {
    /// `collectedDateTime` variant.
    #[fhir("collectedDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `collectedPeriod` variant.
    #[fhir("collectedPeriod")]
    Period(Box<types::Period>),
}

/// The `Specimen.container.additive[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenContainerAdditive {
    /// `additiveCodeableConcept` variant.
    #[fhir("additiveCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `additiveReference` variant.
    #[fhir("additiveReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Specimen;

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
