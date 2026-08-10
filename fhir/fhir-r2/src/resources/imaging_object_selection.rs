//! ImagingObjectSelection
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingObjectSelection
//!
//!
//!
//! Key Object Selection
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ImagingObjectSelection Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::imaging_object_selection::ImagingObjectSelection;
///
/// let value = ImagingObjectSelection::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingObjectSelection = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImagingObjectSelection {
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

    /// Instance UID
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Patient of the selected objects
    pub patient: types::Reference<crate::r2::resources::Patient>,

    /// Reason for selection
    pub title: types::CodeableConcept,

    /// Description text
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Author (human or machine)
    pub author: Option<types::Reference>,

    /// Authoring time of the selection
    pub authoring_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`authoring_time`](Self::authoring_time) (FHIR `_authoringTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoringTime")]
    pub authoring_time_ext: Option<types::Element>,

    /// Study identity of the selected instances
    pub study: ::vec1::Vec1<ImagingObjectSelectionStudy>,
}

/// Study identity and locating information of the DICOM SOP instances in the
/// selection.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::imaging_object_selection::ImagingObjectSelectionStudy;
///
/// let value = ImagingObjectSelectionStudy::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingObjectSelectionStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImagingObjectSelectionStudy {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Study instance UID
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Retrieve study URL
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Reference to ImagingStudy
    pub imaging_study: Option<types::Reference<crate::r2::resources::ImagingStudy>>,

    /// Series identity of the selected instances
    pub series: ::vec1::Vec1<ImagingObjectSelectionStudySeries>,
}

/// Series identity and locating information of the DICOM SOP instances in the
/// selection.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::imaging_object_selection::ImagingObjectSelectionStudySeries;
///
/// let value = ImagingObjectSelectionStudySeries::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingObjectSelectionStudySeries = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImagingObjectSelectionStudySeries {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Series instance UID
    pub uid: Option<types::Oid>,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Retrieve series URL
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// The selected instance
    pub instance: ::vec1::Vec1<ImagingObjectSelectionStudySeriesInstance>,
}

/// Identity and locating information of the selected DICOM SOP instances.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::imaging_object_selection::ImagingObjectSelectionStudySeriesInstance;
/// use fhir::r2::types;
///
/// let value = ImagingObjectSelectionStudySeriesInstance {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ImagingObjectSelectionStudySeriesInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImagingObjectSelectionStudySeriesInstance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// SOP class UID of instance
    pub sop_class: types::Oid,
    /// Primitive extension sibling for [`sop_class`](Self::sop_class) (FHIR `_sopClass`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sopClass")]
    pub sop_class_ext: Option<types::Element>,

    /// Selected instance UID
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Retrieve instance URL
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// The frame set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frames: Vec<ImagingObjectSelectionStudySeriesInstanceFrames>,
}

/// Identity and location information of the frames in the selected instance.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::imaging_object_selection::ImagingObjectSelectionStudySeriesInstanceFrames;
///
/// let value = ImagingObjectSelectionStudySeriesInstanceFrames::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingObjectSelectionStudySeriesInstanceFrames = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ImagingObjectSelectionStudySeriesInstanceFrames {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Frame numbers
    pub frame_numbers: ::vec1::Vec1<types::UnsignedInt>,
    /// Primitive extension sibling for [`frame_numbers`](Self::frame_numbers) (FHIR `_frameNumbers`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_frameNumbers")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub frame_numbers_ext: Vec<Option<types::Element>>,

    /// Retrieve frame URL
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}
