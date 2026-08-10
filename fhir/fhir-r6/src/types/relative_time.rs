//! RelativeTime
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RelativeTime
//!
//! Version: 6.0.0-ballot3
//!
//! A point in time or an interval of time relative to an event
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// RelativeTime Type: RelativeTime is used to express a point in time or an
/// interval of time relative to an event defined in data types other than
/// dateTime.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::relative_time::RelativeTime;
/// use fhir::r6::types;
///
/// let value = RelativeTime {
///     context_definition: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `contextDefinition` is the name this serializes to on the wire.
/// assert_eq!(json["contextDefinition"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: RelativeTime = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "RelativeTimeDe")]
#[fhir_version("r6")]
pub struct RelativeTime {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific event occurrence or resource context used as a base point
    /// (reference point) in time
    pub context_reference: Option<types::Reference>,

    /// The type of event used as a base point
    pub context_definition: Option<types::Canonical>,
    /// Primitive extension sibling for [`context_definition`](Self::context_definition) (FHIR `_contextDefinition`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextDefinition")]
    pub context_definition_ext: Option<types::Element>,

    /// Path to the element defining the point in time
    pub context_path: Option<types::String>,
    /// Primitive extension sibling for [`context_path`](Self::context_path) (FHIR `_contextPath`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextPath")]
    pub context_path_ext: Option<types::Element>,

    /// Coded representation of the event used as a base point (reference
    /// point) in time
    pub context_code: Option<types::CodeableConcept>,

    /// An offset or offset range before (negative values) or after (positive
    /// values) the event
    /// The `RelativeTime.offset[x]` choice element (0..1); see [`RelativeTimeOffset`].
    #[serde(flatten)]
    pub offset: Option<RelativeTimeOffset>,

    /// Free-text description
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RelativeTimeDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    context_reference: Option<types::Reference>,
    context_definition: Option<types::Canonical>,
    #[serde(rename = "_contextDefinition")]
    context_definition_ext: Option<types::Element>,
    context_path: Option<types::String>,
    #[serde(rename = "_contextPath")]
    context_path_ext: Option<types::Element>,
    context_code: Option<types::CodeableConcept>,
    #[serde(flatten)]
    offset: crate::r6::choice::Slot<RelativeTimeOffset>,
    text: Option<types::String>,
    #[serde(rename = "_text")]
    text_ext: Option<types::Element>,
}

impl ::core::convert::From<RelativeTimeDe> for RelativeTime {
    fn from(v: RelativeTimeDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            context_reference: v.context_reference,
            context_definition: v.context_definition,
            context_definition_ext: v.context_definition_ext,
            context_path: v.context_path,
            context_path_ext: v.context_path_ext,
            context_code: v.context_code,
            offset: v.offset.0,
            text: v.text,
            text_ext: v.text_ext,
        }
    }
}

/// The `RelativeTime.offset[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum RelativeTimeOffset {
    /// `offsetDuration` variant.
    #[fhir("offsetDuration")]
    Duration(Box<types::Duration>),
    /// `offsetRange` variant.
    #[fhir("offsetRange")]
    Range(Box<types::Range>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RelativeTime;

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
