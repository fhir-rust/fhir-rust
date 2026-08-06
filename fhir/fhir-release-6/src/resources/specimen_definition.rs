//! SpecimenDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SpecimenDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! Kind of specimen
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A kind of specimen with associated set of requirements.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::specimen_definition::SpecimenDefinition;
/// use fhir::r6::types;
///
/// let value = SpecimenDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: SpecimenDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Logical canonical URL to reference this SpecimenDefinition (globally
    /// unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier
    pub identifier: Option<types::Identifier>,

    /// Business version of the SpecimenDefinition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `SpecimenDefinition.versionAlgorithm[x]` choice element (0..1); see [`SpecimenDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<SpecimenDefinitionVersionAlgorithm>,

    /// Name for this {{title}} (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this SpecimenDefinition (Human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Based on FHIR definition of another SpecimenDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`derived_from_canonical`](Self::derived_from_canonical) (FHIR `_derivedFromCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivedFromCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_canonical_ext: Vec<Option<types::Element>>,

    /// Based on external definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`derived_from_uri`](Self::derived_from_uri) (FHIR `_derivedFromUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_derivedFromUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from_uri_ext: Vec<Option<types::Element>>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If this SpecimenDefinition is not for real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Type of subject for specimen collection
    /// The `SpecimenDefinition.subject[x]` choice element (0..1); see [`SpecimenDefinitionSubject`].
    #[serde(flatten)]
    pub subject: Option<SpecimenDefinitionSubject>,

    /// Date status first applied
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// The name of the individual or organization that published the
    /// SpecimenDefinition
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the SpecimenDefinition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for this SpecimenDefinition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this SpecimenDefinition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// When SpecimenDefinition was approved by publisher
    pub approval_date: Option<types::Date>,
    /// Primitive extension sibling for [`approval_date`](Self::approval_date) (FHIR `_approvalDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_approvalDate")]
    pub approval_date_ext: Option<types::Element>,

    /// The date on which the asset content was last reviewed by the publisher
    pub last_review_date: Option<types::Date>,
    /// Primitive extension sibling for [`last_review_date`](Self::last_review_date) (FHIR `_lastReviewDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastReviewDate")]
    pub last_review_date_ext: Option<types::Element>,

    /// The effective date range for the SpecimenDefinition
    pub effective_period: Option<types::Period>,

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
/// use fhir::r6::resources::specimen_definition::SpecimenDefinitionTypeTested;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
    pub preference: crate::coded::Coded<crate::r6::codes::SpecimenContainedPreference>,
    /// Primitive extension sibling for [`preference`](Self::preference) (FHIR `_preference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preference")]
    pub preference_ext: Option<types::Element>,

    /// The specimen's container
    pub container: Option<SpecimenDefinitionTypeTestedContainer>,

    /// Requirements for specimen delivery and special handling
    pub requirement: Option<types::Markdown>,
    /// Primitive extension sibling for [`requirement`](Self::requirement) (FHIR `_requirement`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirement")]
    pub requirement_ext: Option<types::Element>,

    /// The usual time for retaining this kind of specimen
    pub retention_time: Option<types::Duration>,

    /// Specimen for single use only
    pub single_use: Option<types::Boolean>,
    /// Primitive extension sibling for [`single_use`](Self::single_use) (FHIR `_singleUse`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_singleUse")]
    pub single_use_ext: Option<types::Element>,

    /// Criterion specified for specimen rejection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejection_criterion: Vec<types::CodeableConcept>,

    /// Specimen handling before testing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handling: Vec<SpecimenDefinitionTypeTestedHandling>,

    /// Where the specimen will be tested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub testing_destination: Vec<types::CodeableConcept>,
}

/// The specimen's container.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::specimen_definition::SpecimenDefinitionTypeTestedContainer;
/// use fhir::r6::types;
///
/// let value = SpecimenDefinitionTypeTestedContainer {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: SpecimenDefinitionTypeTestedContainer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SpecimenDefinitionTypeTestedContainer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The material type used for the container
    pub material: Option<types::CodeableConcept>,

    /// Kind of container associated with the kind of specimen
    pub r#type: Option<types::CodeableConcept>,

    /// Color of container cap
    pub cap: Option<types::CodeableConcept>,

    /// The description of the kind of container
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The capacity of this kind of container
    pub capacity: Option<types::Quantity>,

    /// Minimum volume
    /// The `SpecimenDefinition.typeTested.container.minimumVolume[x]` choice element (0..1); see [`SpecimenDefinitionTypeTestedContainerMinimumVolume`].
    #[serde(flatten)]
    pub minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,

    /// Additive associated with container
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,

    /// Special processing applied to the container for this specimen type
    pub preparation: Option<types::Markdown>,
    /// Primitive extension sibling for [`preparation`](Self::preparation) (FHIR `_preparation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_preparation")]
    pub preparation_ext: Option<types::Element>,
}

/// Substance introduced in the kind of container to preserve, maintain or
/// enhance the specimen. Examples: Formalin, Citrate, EDTA.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::specimen_definition::SpecimenDefinitionTypeTestedContainerAdditive;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

/// Set of instructions for preservation/transport of the specimen at a defined
/// temperature interval, prior the testing process.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::specimen_definition::SpecimenDefinitionTypeTestedHandling;
/// use fhir::r6::types;
///
/// let value = SpecimenDefinitionTypeTestedHandling {
///     instruction: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `instruction` is the name this serializes to on the wire.
/// assert_eq!(json["instruction"], ::serde_json::json!("# Heading"));
///
/// let back: SpecimenDefinitionTypeTestedHandling = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct SpecimenDefinitionTypeTestedHandling {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Qualifies the interval of temperature
    pub temperature_qualifier: Option<types::CodeableConcept>,

    /// Temperature range for these handling instructions
    pub temperature_range: Option<types::Range>,

    /// Maximum preservation time
    pub max_duration: Option<types::Duration>,

    /// Preservation instruction
    pub instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`instruction`](Self::instruction) (FHIR `_instruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instruction")]
    pub instruction_ext: Option<types::Element>,
}

/// The `SpecimenDefinition.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `SpecimenDefinition.subject[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionSubject {
    /// `subjectCodeableConcept` variant.
    #[fhir("subjectCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `subjectReference` variant.
    #[fhir("subjectReference")]
    Reference(Box<types::Reference>),
}

/// The `SpecimenDefinition.typeTested.container.minimumVolume[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    /// `minimumVolumeQuantity` variant.
    #[fhir("minimumVolumeQuantity")]
    Quantity(Box<types::Quantity>),
    /// `minimumVolumeString` variant.
    #[fhir("minimumVolumeString")]
    String(crate::r6::choice::Primitive<types::String>),
}

/// The `SpecimenDefinition.typeTested.container.additive.additive[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
