//! Media
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Media
//!
//!
//!
//! A photo, video, or audio recording acquired or used in healthcare. The
//! actual content may be inline or provided by direct reference
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Media Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::media::Media;
/// use fhir::r3::types;
///
/// let value = Media {
///     height: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `height` is the name this serializes to on the wire.
/// assert_eq!(json["height"], ::serde_json::json!(1));
///
/// let back: Media = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Media {
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

    /// Identifier(s) for the image
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Procedure that caused this media to be created
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// photo | video | audio
    pub r#type: crate::coded::Coded<crate::r3::codes::DigitalMediaType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The type of acquisition equipment/process
    pub subtype: Option<types::CodeableConcept>,

    /// Imaging view, e.g. Lateral or Antero-posterior
    pub view: Option<types::CodeableConcept>,

    /// Who/What this Media is a record of
    pub subject: Option<types::Reference>,

    /// Encounter / Episode associated with media
    pub context: Option<types::Reference>,

    /// When Media was collected
    /// The `Media.occurrence[x]` choice element (0..1); see [`MediaOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<MediaOccurrence>,

    /// The person who generated the image
    pub operator: Option<types::Reference>,

    /// Why was event performed?
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Body part in media
    pub body_site: Option<types::CodeableConcept>,

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
    pub duration: Option<types::UnsignedInt>,
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

/// The `Media.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MediaOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
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
