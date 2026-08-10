//! ImagingManifest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingManifest
//!
//!
//!
//! Key Object Selection
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ImagingManifest Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::imaging_manifest::ImagingManifest;
///
/// let value = ImagingManifest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingManifest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImagingManifest {
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// SOP Instance UID
    pub identifier: Option<types::Identifier>,

    /// Patient of the selected objects
    pub patient: types::Reference<crate::r3::resources::Patient>,

    /// Time when the selection of instances was made
    pub authoring_time: Option<types::DateTime>,
    /// Primitive extension sibling for [`authoring_time`](Self::authoring_time) (FHIR `_authoringTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoringTime")]
    pub authoring_time_ext: Option<types::Element>,

    /// Author (human or machine)
    pub author: Option<types::Reference>,

    /// Description text
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Study identity of the selected instances
    pub study: ::vec1::Vec1<ImagingManifestStudy>,
}

/// Study identity and locating information of the DICOM SOP instances in the
/// selection.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::imaging_manifest::ImagingManifestStudy;
///
/// let value = ImagingManifestStudy::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingManifestStudy = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImagingManifestStudy {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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

    /// Reference to ImagingStudy
    pub imaging_study: Option<types::Reference<crate::r3::resources::ImagingStudy>>,

    /// Study access service endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r3::resources::Endpoint>>,

    /// Series identity of the selected instances
    pub series: ::vec1::Vec1<ImagingManifestStudySeries>,
}

/// Series identity and locating information of the DICOM SOP instances in the
/// selection.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::imaging_manifest::ImagingManifestStudySeries;
///
/// let value = ImagingManifestStudySeries::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ImagingManifestStudySeries = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImagingManifestStudySeries {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Series instance UID
    pub uid: types::Oid,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Series access endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r3::resources::Endpoint>>,

    /// The selected instance
    pub instance: ::vec1::Vec1<ImagingManifestStudySeriesInstance>,
}

/// Identity and locating information of the selected DICOM SOP instances.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::imaging_manifest::ImagingManifestStudySeriesInstance;
/// use fhir::r3::types;
///
/// let value = ImagingManifestStudySeriesInstance {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImagingManifestStudySeriesInstance = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ImagingManifestStudySeriesInstance {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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
}
