//! ImagingStudy
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ImagingStudy
//!
//! Version: 5.0.0
//!
//! ImagingStudy Resource: Representation of the content produced in a DICOM imaging study.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Representation of the content produced in a DICOM imaging study.
///
/// An ImagingStudy comprises a set of series, each of which includes a set of
/// Service-Object Pair Instances (SOP Instances - images or other data) acquired
/// or produced in a common context. A series is of only one modality (e.g.
/// X-ray, CT, MR, ultrasound), but a study may have multiple series of different
/// modalities. In FHIR R5 the resource is used to catalog imaging content and to
/// provide access endpoints for retrieval from PACS and other DICOM systems.
///
/// Clinically, an `ImagingStudy` acts as an index or manifest over DICOM
/// content: it does not embed pixel data itself, but instead describes the
/// study, series, and instance hierarchy along with identifiers (such as the
/// DICOM Study, Series, and SOP Instance UIDs) and network endpoints that
/// clients can use to retrieve the actual images from a PACS, VNA, or other
/// DICOM-capable system (for example via WADO-RS). This makes the resource
/// useful both for administrative tracking of imaging orders and results, and
/// for enabling downstream systems to locate and fetch imaging content without
/// duplicating large binary payloads inside the FHIR server.
///
/// Related resources: an `ImagingStudy` is typically ordered via a
/// `ServiceRequest`, references the imaged individual through `subject`
/// (commonly a [`Patient`](crate::r5::resources::patient::Patient)), may be
/// linked to an `Encounter`, and often produces or is referenced by an
/// `ImagingSelection` or `DiagnosticReport`. Modality and other coded values
/// on the study and its series use [`CodeableConcept`](crate::r5::types::CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_study::ImagingStudy;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudy {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifiers for the whole study, such as the DICOM Study Instance UID
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The overall lifecycle status of the study: registered | available | cancelled | entered-in-error | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::ImagingstudyStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// All of the distinct values for series' modalities (e.g. CT, MR, US)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modality: Vec<types::CodeableConcept>,

    /// Who or what is the subject of the study, typically a [`Patient`](crate::r5::resources::patient::Patient)
    pub subject: types::Reference,

    /// Encounter with which this imaging study is associated
    pub encounter: Option<types::Reference>,

    /// The date and time when the study was started (acquisition began)
    pub started: Option<types::DateTime>,
    /// Primitive extension sibling for [`started`](Self::started) (FHIR `_started`).
    #[serde(rename = "_started")]
    pub started_ext: Option<types::Element>,

    /// Request fulfilled
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Referring physician
    pub referrer: Option<types::Reference>,

    /// Study access endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,

    /// Number of Study Related Series
    pub number_of_series: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_series`](Self::number_of_series) (FHIR `_numberOfSeries`).
    #[serde(rename = "_numberOfSeries")]
    pub number_of_series_ext: Option<types::Element>,

    /// Number of Study Related Instances
    pub number_of_instances: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_instances`](Self::number_of_instances) (FHIR `_numberOfInstances`).
    #[serde(rename = "_numberOfInstances")]
    pub number_of_instances_ext: Option<types::Element>,

    /// The performed procedure or code
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub procedure: Vec<types::CodeableReference>,

    /// Where ImagingStudy occurred
    pub location: Option<types::Reference>,

    /// Why the study was requested / performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// User-defined comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Institution-generated description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Each study has one or more series of instances, grouped by modality
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub series: Vec<ImagingStudySeries>,
}

/// Each study has one or more series of instances.
///
/// A series is of only one modality (e.g. X-ray, CT, MR, ultrasound) and groups
/// together the SOP instances acquired or produced in a common context.
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_study::ImagingStudySeries;
/// use fhir::r5::types;
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
pub struct ImagingStudySeries {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// DICOM Series Instance UID for the series
    pub uid: types::Id,
    /// Primitive extension sibling for [`uid`](Self::uid) (FHIR `_uid`).
    #[serde(rename = "_uid")]
    pub uid_ext: Option<types::Element>,

    /// Numeric identifier of this series
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`).
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// The modality used for this series
    pub modality: types::CodeableConcept,

    /// A short human readable summary of the series
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Number of Series Related Instances
    pub number_of_instances: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_instances`](Self::number_of_instances) (FHIR `_numberOfInstances`).
    #[serde(rename = "_numberOfInstances")]
    pub number_of_instances_ext: Option<types::Element>,

    /// Series access endpoint
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference>,

    /// Body part examined
    pub body_site: Option<types::CodeableReference>,

    /// Body part laterality
    pub laterality: Option<types::CodeableConcept>,

    /// Specimen imaged
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference>,

    /// When the series started
    pub started: Option<types::DateTime>,
    /// Primitive extension sibling for [`started`](Self::started) (FHIR `_started`).
    #[serde(rename = "_started")]
    pub started_ext: Option<types::Element>,

    /// Who performed the series
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ImagingStudySeriesPerformer>,

    /// A single SOP instance from the series
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ImagingStudySeriesInstance>,
}

/// Who performed the series.
///
/// Indicates who or what performed the series and, optionally, how they were
/// involved via a function code.
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_study::ImagingStudySeriesPerformer;
/// use fhir::r5::types;
///
/// let value = ImagingStudySeriesPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ImagingStudySeriesPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudySeriesPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who performed the series
    pub actor: types::Reference,
}

/// A single SOP instance from the series.
///
/// Describes an individual Service-Object Pair (SOP) Instance such as an image
/// or other DICOM data object contained within the series.
/// # Examples
///
/// ```
/// use fhir::r5::resources::imaging_study::ImagingStudySeriesInstance;
/// use fhir::r5::types;
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
pub struct ImagingStudySeriesInstance {
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

    /// DICOM class type
    pub sop_class: types::Coding,

    /// The number of this instance in the series
    pub number: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number`](Self::number) (FHIR `_number`).
    #[serde(rename = "_number")]
    pub number_ext: Option<types::Element>,

    /// Description of instance
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
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
