//! ImagingStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
//!
//!
//!
//! A set of images produced in single study (one or more series of references
//! images)
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ImagingStudy Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::imaging_study::ImagingStudy;
/// use fhir::r3::types;
///
/// let value = ImagingStudy {
///     number_of_series: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfSeries` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfSeries"], ::serde_json::json!(0));
///
/// let back: ImagingStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImagingStudy {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Formal DICOM identifier for the study
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Related workflow identifier ("Accession Number")
    pub accession: Option<types::Identifier>,

    /// Other identifiers for the study
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// ONLINE | OFFLINE | NEARLINE | UNAVAILABLE
    pub availability: Option<types::Code>,
    /// Primitive extension sibling for [`availability`](Self::availability) (FHIR `_availability`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availability")]
    pub availability_ext: Option<types::Element>,

    /// All series modality if actual acquisition modalities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modality_list: Vec<types::Coding>,

    /// Who the images are of
    pub patient: types::Reference,

    /// Originating context
    pub context: Option<types::Reference>,

    /// When the study was started
    pub started: Option<types::DateTime>,
    /// Primitive extension sibling for [`started`](Self::started) (FHIR `_started`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_started")]
    pub started_ext: Option<types::Element>,

    /// Request fulfilled
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Referring physician
    pub referrer: Option<types::Reference>,

    /// Who interpreted images
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interpreter: Vec<types::Reference>,

    /// Study access endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,

    /// Number of Study Related Series
    pub number_of_series: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_series`](Self::number_of_series) (FHIR `_numberOfSeries`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfSeries")]
    pub number_of_series_ext: Option<types::Element>,

    /// Number of Study Related Instances
    pub number_of_instances: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_instances`](Self::number_of_instances) (FHIR `_numberOfInstances`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfInstances")]
    pub number_of_instances_ext: Option<types::Element>,

    /// The performed Procedure reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_reference: Vec<types::Reference>,

    /// The performed procedure code
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure_code: Vec<types::CodeableConcept>,

    /// Why the study was requested
    pub reason: Option<types::CodeableConcept>,

    /// Institution-generated description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Each study has one or more series of instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub series: Vec<ImagingStudySeries>,
}

/// Each study has one or more series of images or other content.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::imaging_study::ImagingStudySeries;
/// use fhir::r3::types;
///
/// let value = ImagingStudySeries {
///     number_of_instances: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfInstances` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfInstances"], ::serde_json::json!(0));
///
/// let back: ImagingStudySeries = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImagingStudySeries {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Formal DICOM identifier for this series
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Numeric identifier of this series
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// The modality of the instances in the series
    pub modality: types::Coding,

    /// A short human readable summary of the series
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Number of Series Related Instances
    pub number_of_instances: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_instances`](Self::number_of_instances) (FHIR `_numberOfInstances`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfInstances")]
    pub number_of_instances_ext: Option<types::Element>,

    /// ONLINE | OFFLINE | NEARLINE | UNAVAILABLE
    pub availability: Option<types::Code>,
    /// Primitive extension sibling for [`availability`](Self::availability) (FHIR `_availability`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availability")]
    pub availability_ext: Option<types::Element>,

    /// Series access endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,

    /// Body part examined
    pub body_site: Option<types::Coding>,

    /// Body part laterality
    pub laterality: Option<types::Coding>,

    /// When the series started
    pub started: Option<types::DateTime>,
    /// Primitive extension sibling for [`started`](Self::started) (FHIR `_started`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_started")]
    pub started_ext: Option<types::Element>,

    /// Who performed the series
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// A single SOP instance from the series
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ImagingStudySeriesInstance>,
}

/// A single SOP instance within the series, e.g. an image, or presentation
/// state.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::imaging_study::ImagingStudySeriesInstance;
/// use fhir::r3::types;
///
/// let value = ImagingStudySeriesInstance {
///     number: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `number` is the name this serializes to on the wire.
/// assert_eq!(json["number"], ::serde_json::json!(0));
///
/// let back: ImagingStudySeriesInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImagingStudySeriesInstance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Formal DICOM identifier for this instance
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// The number of this instance in the series
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// DICOM class type
    pub sop_class: types::Oid,
    /// Primitive extension sibling for [`sop_class`](Self::sop_class) (FHIR `_sopClass`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sopClass")]
    pub sop_class_ext: Option<types::Element>,

    /// Description of instance
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImagingStudy;

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
