//! SpecimenDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
//!
//! Version: 4.0.1
//!
//! Kind of specimen
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A kind of specimen with associated set of requirements.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::specimen_definition::SpecimenDefinition;
/// use fhir::r4::types;
///
/// let value = SpecimenDefinition {
///     time_aspect: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `timeAspect` is the name this serializes to on the wire.
/// assert_eq!(json["timeAspect"], ::serde_json::json!("abc"));
///
/// let back: SpecimenDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SpecimenDefinition {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Business identifier of a kind of specimen
    pub identifier: Option<types::Identifier>,

    /// Kind of material to collect
    pub type_collected: Option<types::CodeableConcept>,

    /// Patient preparation for collection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patient_preparation: Vec<types::CodeableConcept>,

    /// Time aspect for collection
    pub time_aspect: Option<types::String>,
    /// Primitive extension sibling for [`time_aspect`](Self::time_aspect) (FHIR `_timeAspect`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_timeAspect")]
    pub time_aspect_ext: Option<types::Element>,

    /// Specimen collection procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub collection: Vec<types::CodeableConcept>,

    /// Specimen in container intended for testing by lab
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub type_tested: Vec<SpecimenDefinitionTypeTested>,
}

/// Specimen conditioned in a container as expected by the testing laboratory.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::specimen_definition::SpecimenDefinitionTypeTested;
/// use fhir::r4::types;
///
/// let value = SpecimenDefinitionTypeTested {
///     is_derived: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isDerived` is the name this serializes to on the wire.
/// assert_eq!(json["isDerived"], ::serde_json::json!(true));
///
/// let back: SpecimenDefinitionTypeTested = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SpecimenDefinitionTypeTested {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Primary or secondary specimen
    pub is_derived: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_derived`](Self::is_derived) (FHIR `_isDerived`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_isDerived")]
    pub is_derived_ext: Option<types::Element>,

    /// Type of intended specimen
    pub r#type: Option<types::CodeableConcept>,

    /// preferred | alternate
    pub preference: crate::coded::Coded<crate::r4::codes::SpecimenContainedPreference>,
    /// Primitive extension sibling for [`preference`](Self::preference) (FHIR `_preference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preference")]
    pub preference_ext: Option<types::Element>,

    /// The specimen's container
    pub container: Option<SpecimenDefinitionTypeTestedContainer>,

    /// Specimen requirements
    pub requirement: Option<types::String>,
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// Specimen retention time
    pub retention_time: Option<types::Duration>,

    /// Rejection criterion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejection_criterion: Vec<types::CodeableConcept>,

    /// Specimen handling before testing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handling: Vec<SpecimenDefinitionTypeTestedHandling>,
}

/// The specimen's container.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::specimen_definition::SpecimenDefinitionTypeTestedContainer;
/// use fhir::r4::types;
///
/// let value = SpecimenDefinitionTypeTestedContainer {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: SpecimenDefinitionTypeTestedContainer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenDefinitionTypeTestedContainerDe")]
#[fhir_version("r4")]
pub struct SpecimenDefinitionTypeTestedContainer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Container material
    pub material: Option<types::CodeableConcept>,

    /// Kind of container associated with the kind of specimen
    pub r#type: Option<types::CodeableConcept>,

    /// Color of container cap
    pub cap: Option<types::CodeableConcept>,

    /// Container description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Container capacity
    pub capacity: Option<types::Quantity>,

    /// Minimum volume
    /// The `SpecimenDefinition.typeTested.container.minimumVolume[x]` choice element (0..1); see [`SpecimenDefinitionTypeTestedContainerMinimumVolume`].
    #[serde(flatten)]
    pub minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,

    /// Additive associated with container
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,

    /// Specimen container preparation
    pub preparation: Option<types::String>,
    /// Primitive extension sibling for [`preparation`](Self::preparation) (FHIR `_preparation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preparation")]
    pub preparation_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenDefinitionTypeTestedContainerDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    material: Option<types::CodeableConcept>,
    r#type: Option<types::CodeableConcept>,
    cap: Option<types::CodeableConcept>,
    description: Option<types::String>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    capacity: Option<types::Quantity>,
    #[serde(flatten)]
    minimum_volume: crate::r4::choice::Slot<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    #[serde(default)]
    additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    preparation: Option<types::String>,
    #[serde(rename = "_preparation")]
    preparation_ext: Option<types::Element>,
}

impl ::core::convert::From<SpecimenDefinitionTypeTestedContainerDe>
    for SpecimenDefinitionTypeTestedContainer
{
    fn from(v: SpecimenDefinitionTypeTestedContainerDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            material: v.material,
            r#type: v.r#type,
            cap: v.cap,
            description: v.description,
            description_ext: v.description_ext,
            capacity: v.capacity,
            minimum_volume: v.minimum_volume.0,
            additive: v.additive,
            preparation: v.preparation,
            preparation_ext: v.preparation_ext,
        }
    }
}

/// Substance introduced in the kind of container to preserve, maintain or
/// enhance the specimen. Examples: Formalin, Citrate, EDTA.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::specimen_definition::SpecimenDefinitionTypeTestedContainerAdditive;
/// use fhir::r4::types;
///
/// let value = SpecimenDefinitionTypeTestedContainerAdditive {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SpecimenDefinitionTypeTestedContainerAdditive = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SpecimenDefinitionTypeTestedContainerAdditiveDe")]
#[fhir_version("r4")]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Additive associated with container
    /// The `SpecimenDefinition.typeTested.container.additive.additive[x]` choice element (1..1); see [`SpecimenDefinitionTypeTestedContainerAdditiveAdditive`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub additive: Option<SpecimenDefinitionTypeTestedContainerAdditiveAdditive>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SpecimenDefinitionTypeTestedContainerAdditiveDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    additive: crate::r4::choice::Slot<SpecimenDefinitionTypeTestedContainerAdditiveAdditive>,
}

impl ::core::convert::From<SpecimenDefinitionTypeTestedContainerAdditiveDe>
    for SpecimenDefinitionTypeTestedContainerAdditive
{
    fn from(v: SpecimenDefinitionTypeTestedContainerAdditiveDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            additive: v.additive.0,
        }
    }
}

/// Set of instructions for preservation/transport of the specimen at a defined
/// temperature interval, prior the testing process.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::specimen_definition::SpecimenDefinitionTypeTestedHandling;
/// use fhir::r4::types;
///
/// let value = SpecimenDefinitionTypeTestedHandling {
///     instruction: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instruction` is the name this serializes to on the wire.
/// assert_eq!(json["instruction"], ::serde_json::json!("abc"));
///
/// let back: SpecimenDefinitionTypeTestedHandling = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SpecimenDefinitionTypeTestedHandling {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Temperature qualifier
    pub temperature_qualifier: Option<types::CodeableConcept>,

    /// Temperature range
    pub temperature_range: Option<types::Range>,

    /// Maximum preservation time
    pub max_duration: Option<types::Duration>,

    /// Preservation instruction
    pub instruction: Option<types::String>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,
}

/// The `SpecimenDefinition.typeTested.container.minimumVolume[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    /// `minimumVolumeQuantity` variant.
    #[fhir("minimumVolumeQuantity")]
    Quantity(Box<types::Quantity>),
    /// `minimumVolumeString` variant.
    #[fhir("minimumVolumeString")]
    String(crate::r4::choice::Primitive<types::String>),
}

/// The `SpecimenDefinition.typeTested.container.additive.additive[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
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
    type T = SpecimenDefinition;

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
