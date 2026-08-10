//! BodyStructure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/BodyStructure
//!
//! Version: 6.0.0-ballot3
//!
//! Specific and identified anatomical structure
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Record details about an anatomical structure. This resource may be used
/// when a coded concept does not provide the necessary detail needed for the
/// use case.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::body_structure::BodyStructure;
///
/// let value = BodyStructure::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: BodyStructure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct BodyStructure {
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

    /// Bodystructure identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Whether this record is in active use
    pub active: Option<types::Boolean>,
    /// Primitive extension sibling for [`active`](Self::active) (FHIR `_active`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_active")]
    pub active_ext: Option<types::Element>,

    /// Kind of Structure
    pub morphology: Option<types::CodeableConcept>,

    /// Included anatomic location(s)
    pub included_structure: ::vec1::Vec1<BodyStructureIncludedStructure>,

    /// Excluded anatomic locations(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_structure: Vec<BodyStructureIncludedStructure>,

    /// Text description
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Attached images
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image: Vec<types::Attachment>,

    /// Who this is about
    pub patient: types::Reference<crate::r6::resources::Patient>,
}

/// The anatomical location(s) or region(s) of the specimen, lesion, or body
/// structure.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::body_structure::BodyStructureIncludedStructure;
/// use fhir::r6::types;
///
/// let value = BodyStructureIncludedStructure {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BodyStructureIncludedStructure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct BodyStructureIncludedStructure {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code that represents the included structure
    pub structure: types::CodeableConcept,

    /// Code that represents the included structure laterality
    pub laterality: Option<types::CodeableConcept>,

    /// Landmark relative location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_landmark_orientation: Vec<BodyStructureIncludedStructureBodyLandmarkOrientation>,

    /// Cartesian reference for structure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spatial_reference: Vec<types::Reference<crate::r6::resources::ImagingSelection>>,

    /// Code that represents the included structure qualifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualifier: Vec<types::CodeableConcept>,
}

/// Body location in relation to a specific body landmark (e.g., a body
/// structure such a navel, scar, or implanted device).
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::body_structure::BodyStructureIncludedStructureBodyLandmarkOrientation;
/// use fhir::r6::types;
///
/// let value = BodyStructureIncludedStructureBodyLandmarkOrientation {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BodyStructureIncludedStructureBodyLandmarkOrientation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Explanation of landmark
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub landmark_description: Vec<types::CodeableConcept>,

    /// Clockface orientation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clock_face_position: Vec<types::CodeableConcept>,

    /// Landmark relative location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub distance_from_landmark:
        Vec<BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark>,

    /// Relative landmark surface orientation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub surface_orientation: Vec<types::CodeableConcept>,
}

/// The distance in centimeters a certain observation is made from a body
/// landmark.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::body_structure::BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark;
/// use fhir::r6::types;
///
/// let value = BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Measurement device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::CodeableReference>,

    /// Measured distance from body landmark
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<types::Quantity>,
}
