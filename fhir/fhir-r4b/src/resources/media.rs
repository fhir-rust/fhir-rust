//! Media
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Media
//!
//! Version: 4.3.0
//!
//! A photo, video, or audio recording acquired or used in healthcare. The
//! actual content may be inline or provided by direct reference
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A photo, video, or audio recording acquired or used in healthcare. The
/// actual content may be inline or provided by direct reference.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::media::Media;
/// use fhir::r4b::types;
///
/// let value = Media {
///     device_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `deviceName` is the name this serializes to on the wire.
/// assert_eq!(json["deviceName"], ::serde_json::json!("abc"));
///
/// let back: Media = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MediaDe")]
#[fhir_version("r4b")]
pub struct Media {
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifier(s) for the image
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Procedure that caused this media to be created
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed |
    /// entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of media as image, video, or audio
    pub r#type: Option<types::CodeableConcept>,

    /// The type of acquisition equipment/process
    pub modality: Option<types::CodeableConcept>,

    /// Imaging view, e.g. Lateral or Antero-posterior
    pub view: Option<types::CodeableConcept>,

    /// Who/What this Media is a record of
    pub subject: Option<types::Reference>,

    /// Encounter associated with media
    pub encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,

    /// When Media was collected
    /// The `Media.created[x]` choice element (0..1); see [`MediaCreated`].
    #[serde(flatten)]
    pub created: Option<MediaCreated>,

    /// Date/Time this version was made available
    pub issued: Option<types::Instant>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// The person who generated the image
    pub operator: Option<types::Reference>,

    /// Why was event performed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Observed body part
    pub body_site: Option<types::CodeableConcept>,

    /// Name of the device/manufacturer
    pub device_name: Option<types::String>,
    /// Primitive extension sibling for [`device_name`](Self::device_name) (FHIR `_deviceName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_deviceName")]
    pub device_name_ext: Option<types::Element>,

    /// Observing Device
    pub device: Option<types::Reference>,

    /// Height of the image in pixels (photo/video)
    pub height: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`height`](Self::height) (FHIR `_height`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_height")]
    pub height_ext: Option<types::Element>,

    /// Width of the image in pixels (photo/video)
    pub width: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`width`](Self::width) (FHIR `_width`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_width")]
    pub width_ext: Option<types::Element>,

    /// Number of frames if > 1 (photo)
    pub frames: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`frames`](Self::frames) (FHIR `_frames`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_frames")]
    pub frames_ext: Option<types::Element>,

    /// Length in seconds (audio / video)
    pub duration: Option<types::Decimal>,
    /// Primitive extension sibling for [`duration`](Self::duration) (FHIR `_duration`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_duration")]
    pub duration_ext: Option<types::Element>,

    /// Actual Media - reference or data
    pub content: types::Attachment,

    /// Comments made about the media
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MediaDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r4b::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    #[serde(default)]
    part_of: Vec<types::Reference>,
    status: crate::coded::Coded<crate::r4b::codes::EventStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    r#type: Option<types::CodeableConcept>,
    modality: Option<types::CodeableConcept>,
    view: Option<types::CodeableConcept>,
    subject: Option<types::Reference>,
    encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,
    #[serde(flatten)]
    created: crate::r4b::choice::Slot<MediaCreated>,
    issued: Option<types::Instant>,
    #[serde(rename = "_issued")]
    issued_ext: Option<types::Element>,
    operator: Option<types::Reference>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    body_site: Option<types::CodeableConcept>,
    device_name: Option<types::String>,
    #[serde(rename = "_deviceName")]
    device_name_ext: Option<types::Element>,
    device: Option<types::Reference>,
    height: Option<types::PositiveInt>,
    #[serde(rename = "_height")]
    height_ext: Option<types::Element>,
    width: Option<types::PositiveInt>,
    #[serde(rename = "_width")]
    width_ext: Option<types::Element>,
    frames: Option<types::PositiveInt>,
    #[serde(rename = "_frames")]
    frames_ext: Option<types::Element>,
    duration: Option<types::Decimal>,
    #[serde(rename = "_duration")]
    duration_ext: Option<types::Element>,
    content: types::Attachment,
    #[serde(default)]
    note: Vec<types::Annotation>,
}

impl ::core::convert::From<MediaDe> for Media {
    fn from(v: MediaDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            based_on: v.based_on,
            part_of: v.part_of,
            status: v.status,
            status_ext: v.status_ext,
            r#type: v.r#type,
            modality: v.modality,
            view: v.view,
            subject: v.subject,
            encounter: v.encounter,
            created: v.created.0,
            issued: v.issued,
            issued_ext: v.issued_ext,
            operator: v.operator,
            reason_code: v.reason_code,
            body_site: v.body_site,
            device_name: v.device_name,
            device_name_ext: v.device_name_ext,
            device: v.device,
            height: v.height,
            height_ext: v.height_ext,
            width: v.width,
            width_ext: v.width_ext,
            frames: v.frames,
            frames_ext: v.frames_ext,
            duration: v.duration,
            duration_ext: v.duration_ext,
            content: v.content,
            note: v.note,
        }
    }
}

/// The `Media.created[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum MediaCreated {
    /// `createdDateTime` variant.
    #[fhir("createdDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
    /// `createdPeriod` variant.
    #[fhir("createdPeriod")]
    Period(Box<types::Period>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Media;

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
