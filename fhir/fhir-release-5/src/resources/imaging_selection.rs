//! ImagingSelection
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingSelection
//!
//! Version: 5.0.0
//!
//! ImagingSelection Resource: A selection of DICOM SOP instances and/or frames within a single Study and Series.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// ImagingSelection is a selection of DICOM SOP instances and/or frames within a
/// single Study and Series.
///
/// In FHIR R5 this resource captures a curated, addressable subset of medical imaging
/// data by referencing DICOM Study, Series, and SOP Instance UIDs, and optionally narrowing
/// the selection further to individual frames or to specific 2D image regions or 3D regions
/// within a frame of reference. It is used to record clinically or scientifically significant
/// findings, key images, annotations, or measurement targets that a clinician, radiologist,
/// or automated process wishes to highlight, share, or act upon, rather than the entire study.
///
/// A selection may additionally include specifics such as an image region, an Observation
/// UID, or a Segmentation Number, allowing linkage to an Observation Resource or transferring
/// this information along with the ImagingStudy Resource. Typical uses include tumor boards,
/// teaching files, quantitative imaging biomarkers, structured reporting, and clinical
/// decision support, where precise portions of imaging data must be referenced unambiguously
/// across systems while pointing back to the source study and a retrieval endpoint.
///
/// # Related resources
///
/// The imaging data is generally derived from an [`ImagingStudy`](crate::r5::resources::imaging_study::ImagingStudy),
/// the subject is commonly a [`Patient`](crate::r5::resources::patient::Patient), and a
/// selection is frequently referenced by an [`Observation`](crate::r5::resources::observation::Observation).
/// Coded elements use [`CodeableConcept`](crate::r5::types::CodeableConcept) and
/// [`Coding`](crate::r5::types::Coding), while linkages to other resources use
/// [`Reference`](crate::r5::types::Reference).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_selection::ImagingSelection;
/// use fhir::r5::types;
///
/// let value = ImagingSelection {
///     study_uid: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `studyUid` is the name this serializes to on the wire.
/// assert_eq!(json["studyUid"], ::serde_json::json!("pat-1"));
///
/// let back: ImagingSelection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelection {
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

    /// Business Identifier for Imaging Selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Status of the imaging selection: available, entered-in-error, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::ImagingselectionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Subject of the selected instances, typically the Patient whose images are referenced.
    pub subject: Option<types::Reference>,

    /// Date / Time when this imaging selection was created
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Selector of the instances (human or machine)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ImagingSelectionPerformer>,

    /// Associated request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Classifies the imaging selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Reason for or purpose of this imaging selection, expressed as text or a coded concept.
    pub code: types::CodeableConcept,

    /// DICOM Study Instance UID identifying the study that contains the selected instances.
    pub study_uid: Option<types::Id>,
    /// Primitive extension sibling for [`study_uid`](Self::study_uid) (FHIR `_studyUid`).
    #[serde(rename = "_studyUid")]
    pub study_uid_ext: Option<types::Element>,

    /// The imaging study, commonly an ImagingStudy, from which this selection is derived.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// The network service providing retrieval for the images referenced in the imaging selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r5::resources::Endpoint>>,

    /// DICOM Series Instance UID
    pub series_uid: Option<types::Id>,
    /// Primitive extension sibling for [`series_uid`](Self::series_uid) (FHIR `_seriesUid`).
    #[serde(rename = "_seriesUid")]
    pub series_uid_ext: Option<types::Element>,

    /// DICOM Series Number
    pub series_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`series_number`](Self::series_number) (FHIR `_seriesNumber`).
    #[serde(rename = "_seriesNumber")]
    pub series_number_ext: Option<types::Element>,

    /// The Frame of Reference UID for the selected images
    pub frame_of_reference_uid: Option<types::Id>,
    /// Primitive extension sibling for [`frame_of_reference_uid`](Self::frame_of_reference_uid) (FHIR `_frameOfReferenceUid`).
    #[serde(rename = "_frameOfReferenceUid")]
    pub frame_of_reference_uid_ext: Option<types::Element>,

    /// Body part examined
    pub body_site: Option<types::CodeableReference>,

    /// Related resource that is the focus for the imaging selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference<crate::r5::resources::ImagingSelection>>,

    /// The selected instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ImagingSelectionInstance>,
}

/// Selector of the instances (human or machine).
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_selection::ImagingSelectionPerformer;
/// use fhir::r5::types;
///
/// let value = ImagingSelectionPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImagingSelectionPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performer
    pub function: Option<types::CodeableConcept>,

    /// Author (human or machine)
    pub actor: Option<types::Reference>,
}

/// The selected instances.
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_selection::ImagingSelectionInstance;
/// use fhir::r5::types;
///
/// let value = ImagingSelectionInstance {
///     number: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `number` is the name this serializes to on the wire.
/// assert_eq!(json["number"], ::serde_json::json!(0));
///
/// let back: ImagingSelectionInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstance {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// DICOM SOP Instance UID
    pub uid: types::Id,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`).
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// DICOM Instance Number
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`).
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// DICOM SOP Class UID
    pub sop_class: Option<types::Coding>,

    /// The selected subset of the SOP Instance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subset: Vec<types::String>,
    /// Primitive extension sibling for [`subset`](Self::subset) (FHIR `_subset`).
    #[serde(rename = "_subset")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subset_ext: Vec<Option<types::Element>>,

    /// A specific 2D region in a DICOM image / frame
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_region2_d: Vec<ImagingSelectionInstanceImageRegion2D>,

    /// A specific 3D region in a DICOM frame of reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_region3_d: Vec<ImagingSelectionInstanceImageRegion3D>,
}

/// A specific 2D region in a DICOM image / frame.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::imaging_selection::ImagingSelectionInstanceImageRegion2D;
///
/// let value = ImagingSelectionInstanceImageRegion2D::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingSelectionInstanceImageRegion2D = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstanceImageRegion2D {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// point | polyline | interpolated | circle | ellipse
    pub region_type: crate::r5::coded::Coded<crate::r5::codes::Imagingselection2Dgraphictype>,
    /// Primitive extension sibling for [`region_type`](Self::region_type) (FHIR `_regionType`).
    #[serde(rename = "_regionType")]
    pub region_type_ext: Option<types::Element>,

    /// Specifies the coordinates that define the image region
    pub coordinate: vec1::Vec1<types::Decimal>,
    /// Primitive extension sibling for [`coordinate`](Self::coordinate) (FHIR `_coordinate`).
    #[serde(rename = "_coordinate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coordinate_ext: Vec<Option<types::Element>>,
}

/// A specific 3D region in a DICOM frame of reference.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::imaging_selection::ImagingSelectionInstanceImageRegion3D;
///
/// let value = ImagingSelectionInstanceImageRegion3D::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingSelectionInstanceImageRegion3D = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstanceImageRegion3D {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// point | multipoint | polyline | polygon | ellipse | ellipsoid
    pub region_type: crate::r5::coded::Coded<crate::r5::codes::Imagingselection3Dgraphictype>,
    /// Primitive extension sibling for [`region_type`](Self::region_type) (FHIR `_regionType`).
    #[serde(rename = "_regionType")]
    pub region_type_ext: Option<types::Element>,

    /// Specifies the coordinates that define the image region
    pub coordinate: vec1::Vec1<types::Decimal>,
    /// Primitive extension sibling for [`coordinate`](Self::coordinate) (FHIR `_coordinate`).
    #[serde(rename = "_coordinate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coordinate_ext: Vec<Option<types::Element>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ImagingSelection;

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
