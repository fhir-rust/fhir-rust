//! TriggerDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
//!
//!
//!
//! Defines an expected trigger for a module
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for TriggerDefinition Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::trigger_definition::TriggerDefinition;
/// use fhir::r3::types;
///
/// let value = TriggerDefinition {
///     event_name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `eventName` is the name this serializes to on the wire.
/// assert_eq!(json["eventName"], ::serde_json::json!("abc"));
///
/// let back: TriggerDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "TriggerDefinitionDe")]
#[fhir_version("r3")]
pub struct TriggerDefinition {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// named-event | periodic | data-added | data-modified | data-removed |
    /// data-accessed | data-access-ended
    pub r#type: crate::coded::Coded<crate::r3::codes::TriggerType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Triggering event name
    pub event_name: Option<types::String>,
    /// Primitive extension sibling for [`event_name`](Self::event_name) (FHIR `_eventName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_eventName")]
    pub event_name_ext: Option<types::Element>,

    /// Timing of the event
    /// The `TriggerDefinition.eventTiming[x]` choice element (0..1); see [`TriggerDefinitionEventTiming`].
    #[serde(flatten)]
    pub event_timing: Option<TriggerDefinitionEventTiming>,

    /// Triggering data of the event
    pub event_data: Option<types::DataRequirement>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TriggerDefinitionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    r#type: crate::coded::Coded<crate::r3::codes::TriggerType>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    event_name: Option<types::String>,
    #[serde(rename = "_eventName")]
    event_name_ext: Option<types::Element>,
    #[serde(flatten)]
    event_timing: crate::r3::choice::Slot<TriggerDefinitionEventTiming>,
    event_data: Option<types::DataRequirement>,
}

impl ::core::convert::From<TriggerDefinitionDe> for TriggerDefinition {
    fn from(v: TriggerDefinitionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            event_name: v.event_name,
            event_name_ext: v.event_name_ext,
            event_timing: v.event_timing.0,
            event_data: v.event_data,
        }
    }
}

/// The `TriggerDefinition.eventTiming[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum TriggerDefinitionEventTiming {
    /// `eventTimingTiming` variant.
    #[fhir("eventTimingTiming")]
    Timing(Box<types::Timing>),
    /// `eventTimingReference` variant.
    #[fhir("eventTimingReference")]
    Reference(Box<types::Reference>),
    /// `eventTimingDate` variant.
    #[fhir("eventTimingDate")]
    Date(crate::r3::choice::Primitive<types::Date>),
    /// `eventTimingDateTime` variant.
    #[fhir("eventTimingDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TriggerDefinition;

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
