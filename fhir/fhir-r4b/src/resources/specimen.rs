//! Specimen
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Specimen
//!
//! Version: 4.3.0
//!
//! Sample for analysis
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A sample to be used for analysis.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::specimen::Specimen;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct Specimen {
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

    /// External Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Identifier assigned by the lab
    pub accession_identifier: Option<types::Identifier>,

    /// available | unavailable | unsatisfactory | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r4b::codes::SpecimenStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Kind of material that forms the specimen
    pub r#type: Option<types::CodeableConcept>,

    /// Where the specimen came from. This may be from patient(s), from a
    /// location (e.g., the source of an environmental sample), or a sampling
    /// of a substance or a device
    pub subject: Option<types::Reference>,

    /// The time when specimen was received for processing
    pub received_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`received_time`](Self::received_time) (FHIR `_receivedTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_receivedTime")]
    pub received_time_ext: Option<types::Element>,

    /// Specimen from which this specimen originated
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent: Vec<types::Reference<crate::r4b::resources::Specimen>>,

    /// Why the specimen was collected
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request: Vec<types::Reference<crate::r4b::resources::ServiceRequest>>,

    /// Collection details
    pub collection: Option<SpecimenCollection>,

    /// Processing and processing step details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processing: Vec<SpecimenProcessing>,

    /// Direct container of specimen (tube/slide, etc.)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub container: Vec<SpecimenContainer>,

    /// State of the specimen
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub condition: Vec<types::CodeableConcept>,

    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// Details concerning the specimen collection.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::specimen::SpecimenCollection;
/// use fhir::r4b::types;
///
/// let value = SpecimenCollection {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SpecimenCollection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenCollectionDe")]
#[fhir_version("r4b")]
pub struct SpecimenCollection {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who collected the specimen
    pub collector: Option<types::Reference>,

    /// Collection time
    /// The `Specimen.collection.collected[x]` choice element (0..1); see [`SpecimenCollectionCollected`].
    #[serde(flatten)]
    pub collected: Option<SpecimenCollectionCollected>,

    /// How long it took to collect specimen
    pub duration: Option<types::Duration>,

    /// The quantity of specimen collected
    pub quantity: Option<types::Quantity>,

    /// Technique used to perform collection
    pub method: Option<types::CodeableConcept>,

    /// Anatomical collection site
    pub body_site: Option<types::CodeableConcept>,

    /// Whether or how long patient abstained from food and/or drink
    /// The `Specimen.collection.fastingStatus[x]` choice element (0..1); see [`SpecimenCollectionFastingStatus`].
    #[serde(flatten)]
    pub fasting_status: Option<SpecimenCollectionFastingStatus>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenCollectionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    collector: Option<types::Reference>,
    #[serde(flatten)]
    collected: crate::r4b::choice::Slot<SpecimenCollectionCollected>,
    duration: Option<types::Duration>,
    quantity: Option<types::Quantity>,
    method: Option<types::CodeableConcept>,
    body_site: Option<types::CodeableConcept>,
    #[serde(flatten)]
    fasting_status: crate::r4b::choice::Slot<SpecimenCollectionFastingStatus>,
}

impl ::core::convert::From<SpecimenCollectionDe> for SpecimenCollection {
    fn from(v: SpecimenCollectionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            collector: v.collector,
            collected: v.collected.0,
            duration: v.duration,
            quantity: v.quantity,
            method: v.method,
            body_site: v.body_site,
            fasting_status: v.fasting_status.0,
        }
    }
}

/// The container holding the specimen. The recursive nature of containers;
/// i.e. blood in tube in tray in rack is not addressed here.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::specimen::SpecimenContainer;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
pub struct SpecimenContainer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    id: Option<types::String>,
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
    additive: crate::r4b::choice::Slot<SpecimenContainerAdditive>,
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

/// Details concerning processing and processing steps for the specimen.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::specimen::SpecimenProcessing;
/// use fhir::r4b::types;
///
/// let value = SpecimenProcessing {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: SpecimenProcessing = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenProcessingDe")]
#[fhir_version("r4b")]
pub struct SpecimenProcessing {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Textual description of procedure
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Indicates the treatment step applied to the specimen
    pub procedure: Option<types::CodeableConcept>,

    /// Material used in the processing step
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<types::Reference<crate::r4b::resources::Substance>>,

    /// Date and time of specimen processing
    /// The `Specimen.processing.time[x]` choice element (0..1); see [`SpecimenProcessingTime`].
    #[serde(flatten)]
    pub time: Option<SpecimenProcessingTime>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenProcessingDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    procedure: Option<types::CodeableConcept>,
    #[serde(default)]
    additive: Vec<types::Reference<crate::r4b::resources::Substance>>,
    #[serde(flatten)]
    time: crate::r4b::choice::Slot<SpecimenProcessingTime>,
}

impl ::core::convert::From<SpecimenProcessingDe> for SpecimenProcessing {
    fn from(v: SpecimenProcessingDe) -> Self {
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

/// The `Specimen.collection.collected[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenCollectionCollected {
    /// `collectedDateTime` variant.
    #[fhir("collectedDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `collectedPeriod` variant.
    #[fhir("collectedPeriod")]
    Period(Box<types::Period>),
}

/// The `Specimen.collection.fastingStatus[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenCollectionFastingStatus {
    /// `fastingStatusCodeableConcept` variant.
    #[fhir("fastingStatusCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `fastingStatusDuration` variant.
    #[fhir("fastingStatusDuration")]
    Duration(Box<types::Duration>),
}

/// The `Specimen.container.additive[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenContainerAdditive {
    /// `additiveCodeableConcept` variant.
    #[fhir("additiveCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `additiveReference` variant.
    #[fhir("additiveReference")]
    Reference(Box<types::Reference>),
}

/// The `Specimen.processing.time[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenProcessingTime {
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
