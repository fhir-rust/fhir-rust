//! ImagingSelection
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingSelection
//!
//! Version: 6.0.0-ballot3
//!
//! A selection of DICOM SOP instances and/or frames
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A selection of DICOM SOP instances and/or frames within a single Study and
/// Series. This might include additional specifics such as an image region, an
/// Observation from a DICOM SR Content Item, or a Segment Number from a DICOM
/// Segmentation SOP Instance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::imaging_selection::ImagingSelection;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImagingSelection {
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

    /// Business Identifiers for Imaging Selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// available | entered-in-error | inactive | unknown
    pub status: crate::coded::Coded<crate::r6::codes::ImagingselectionStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classifies the imaging selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Imaging Selection purpose text or code
    pub code: types::CodeableConcept,

    /// Subject of the selected instances
    pub subject: Option<types::Reference>,

    /// Date / Time when this imaging selection was created
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// Selectors of the instances (human or machine)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ImagingSelectionPerformer>,

    /// Associated requests
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The imaging study from which the imaging selection is derived
    pub derived_from: Option<types::Reference>,

    /// DICOM Study Instance UID
    pub study_uid: Option<types::Id>,
    /// Primitive extension sibling for [`study_uid`](Self::study_uid) (FHIR `_studyUid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_studyUid")]
    pub study_uid_ext: Option<types::Element>,

    /// DICOM Series Instance UID
    pub series_uid: Option<types::Id>,
    /// Primitive extension sibling for [`series_uid`](Self::series_uid) (FHIR `_seriesUid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_seriesUid")]
    pub series_uid_ext: Option<types::Element>,

    /// DICOM Series Number
    pub series_number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`series_number`](Self::series_number) (FHIR `_seriesNumber`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_seriesNumber")]
    pub series_number_ext: Option<types::Element>,

    /// The Frame of Reference UID for the selected images
    pub frame_of_reference_uid: Option<types::Id>,
    /// Primitive extension sibling for [`frame_of_reference_uid`](Self::frame_of_reference_uid) (FHIR `_frameOfReferenceUid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_frameOfReferenceUid")]
    pub frame_of_reference_uid_ext: Option<types::Element>,

    /// Body part examined
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableReference>,

    /// Related resources that are the focus for the imaging selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference<crate::r6::resources::ImagingSelection>>,

    /// The network services providing retrieval for the images referenced in
    /// the imaging selection
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r6::resources::Endpoint>>,

    /// The selected instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ImagingSelectionInstance>,

    /// A specific 3D region in a DICOM frame of reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_region_3_d: Vec<ImagingSelectionImageRegion3D>,
}

/// Each imaging selection might includes a 3D image region, specified by a
/// region type and a set of 3D coordinates.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::imaging_selection::ImagingSelectionImageRegion3D;
///
/// let value = ImagingSelectionImageRegion3D::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingSelectionImageRegion3D = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImagingSelectionImageRegion3D {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// point | multipoint | polyline | polygon | ellipse | ellipsoid
    pub region_type: crate::coded::Coded<crate::r6::codes::Imagingselection3Dgraphictype>,
    /// Primitive extension sibling for [`region_type`](Self::region_type) (FHIR `_regionType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_regionType")]
    pub region_type_ext: Option<types::Element>,

    /// Specifies the coordinates that define the image region
    pub coordinate: ::vec1::Vec1<types::Decimal>,
    /// Primitive extension sibling for [`coordinate`](Self::coordinate) (FHIR `_coordinate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_coordinate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coordinate_ext: Vec<Option<types::Element>>,
}

/// Each imaging selection includes one or more selected DICOM SOP instances.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::imaging_selection::ImagingSelectionInstance;
/// use fhir::r6::types;
///
/// let value = ImagingSelectionInstance {
///     sop_class: Some(types::Oid("urn:oid:1.2.3".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sopClass` is the name this serializes to on the wire.
/// assert_eq!(json["sopClass"], ::serde_json::json!("urn:oid:1.2.3"));
///
/// let back: ImagingSelectionInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
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
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// DICOM Instance Number
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// DICOM SOP Class UID
    pub sop_class: Option<types::Oid>,
    /// Primitive extension sibling for [`sop_class`](Self::sop_class) (FHIR `_sopClass`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sopClass")]
    pub sop_class_ext: Option<types::Element>,

    /// The selected subset of the SOP Instance
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub subset: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`subset`](Self::subset) (FHIR `_subset`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subset")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subset_ext: Vec<Option<types::Element>>,

    /// A specific 2D region in a DICOM image / frame
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_region_2_d: Vec<ImagingSelectionInstanceImageRegion2D>,
}

/// Each imaging selection instance or frame list might includes an image
/// region, specified by a region type and a set of 2D coordinates.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::imaging_selection::ImagingSelectionInstanceImageRegion2D;
///
/// let value = ImagingSelectionInstanceImageRegion2D::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingSelectionInstanceImageRegion2D = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ImagingSelectionInstanceImageRegion2D {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// point | polyline | multipoint | circle | ellipse
    pub region_type: crate::coded::Coded<crate::r6::codes::Imagingselection2Dgraphictype>,
    /// Primitive extension sibling for [`region_type`](Self::region_type) (FHIR `_regionType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_regionType")]
    pub region_type_ext: Option<types::Element>,

    /// Specifies the coordinates that define the image region
    pub coordinate: ::vec1::Vec1<types::Decimal>,
    /// Primitive extension sibling for [`coordinate`](Self::coordinate) (FHIR `_coordinate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_coordinate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coordinate_ext: Vec<Option<types::Element>>,
}

/// Selectors of the instances – human or machine.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::imaging_selection::ImagingSelectionPerformer;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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
