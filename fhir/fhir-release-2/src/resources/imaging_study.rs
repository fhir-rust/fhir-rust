//! ImagingStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
//!
//!
//!
//! A set of images produced in single study (one or more series of references
//! images)
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ImagingStudy Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::imaging_study::ImagingStudy;
/// use fhir::r2::types;
///
/// let value = ImagingStudy {
///     started: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `started` is the name this serializes to on the wire.
/// assert_eq!(json["started"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ImagingStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// When the study was started
    pub started: Option<types::DateTime>,
    /// Primitive extension sibling for [`started`](Self::started) (FHIR `_started`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_started")]
    pub started_ext: Option<types::Element>,

    /// Who the images are of
    pub patient: types::Reference,

    /// Formal identifier for the study
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

    /// Order(s) that caused this study to be performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<types::Reference>,

    /// All series modality if actual acquisition modalities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modality_list: Vec<types::Coding>,

    /// Referring physician (0008,0090)
    pub referrer: Option<types::Reference>,

    /// ONLINE | OFFLINE | NEARLINE | UNAVAILABLE (0008,0056)
    pub availability: Option<types::Code>,
    /// Primitive extension sibling for [`availability`](Self::availability) (FHIR `_availability`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availability")]
    pub availability_ext: Option<types::Element>,

    /// Retrieve URI
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Number of Study Related Series
    pub number_of_series: types::UnsignedInt,
    /// Primitive extension sibling for [`number_of_series`](Self::number_of_series) (FHIR `_numberOfSeries`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfSeries")]
    pub number_of_series_ext: Option<types::Element>,

    /// Number of Study Related Instances
    pub number_of_instances: types::UnsignedInt,
    /// Primitive extension sibling for [`number_of_instances`](Self::number_of_instances) (FHIR `_numberOfInstances`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfInstances")]
    pub number_of_instances_ext: Option<types::Element>,

    /// Type of procedure performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<types::Reference>,

    /// Who interpreted images
    pub interpreter: Option<types::Reference>,

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
/// use fhir::r2::resources::imaging_study::ImagingStudySeries;
/// use fhir::r2::types;
///
/// let value = ImagingStudySeries {
///     number: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `number` is the name this serializes to on the wire.
/// assert_eq!(json["number"], ::serde_json::json!(0));
///
/// let back: ImagingStudySeries = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImagingStudySeries {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Numeric identifier of this series
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// The modality of the instances in the series
    pub modality: types::Coding,

    /// Formal identifier for this series
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// A description of the series
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Number of Series Related Instances
    pub number_of_instances: types::UnsignedInt,
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

    /// Location of the referenced instance(s)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

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

    /// A single SOP instance from the series
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ImagingStudySeriesInstance>,
}

/// A single SOP Instance within the series, e.g. an image, or presentation
/// state.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::imaging_study::ImagingStudySeriesInstance;
/// use fhir::r2::types;
///
/// let value = ImagingStudySeriesInstance {
///     r#type: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("abc"));
///
/// let back: ImagingStudySeriesInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImagingStudySeriesInstance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The number of this instance in the series
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// Formal identifier for this instance
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// DICOM class type
    pub sop_class: types::Oid,
    /// Primitive extension sibling for [`sop_class`](Self::sop_class) (FHIR `_sopClass`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sopClass")]
    pub sop_class_ext: Option<types::Element>,

    /// Type of instance (image etc.)
    pub r#type: Option<types::String>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Description of instance
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Content of the instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content: Vec<types::Attachment>,
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
