//! TriggerDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
//!
//! Version: 4.3.0
//!
//! Defines an expected trigger for a module
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for TriggerDefinition Type: A description of a
/// triggering event. Triggering events can be named events, data events, or
/// periodic, as determined by the type element.
///
/// # Examples
///
/// ```
/// use fhir::r4b::types::trigger_definition::TriggerDefinition;
/// use fhir::r4b::types;
///
/// let value = TriggerDefinition {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: TriggerDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4b")]
pub struct TriggerDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// named-event | periodic | data-changed | data-added | data-modified |
    /// data-removed | data-accessed | data-access-ended
    pub r#type: crate::coded::Coded<crate::r4b::codes::TriggerType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Name or URI that identifies the event
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Timing of the event
    /// The `TriggerDefinition.timing[x]` choice element (0..1); see [`TriggerDefinitionTiming`].
    #[serde(flatten)]
    pub timing: Option<TriggerDefinitionTiming>,

    /// Triggering data of the event (multiple = 'and')
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<types::DataRequirement>,

    /// Whether the event triggers (boolean expression)
    pub condition: Option<types::Expression>,
}

/// The `TriggerDefinition.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum TriggerDefinitionTiming {
    /// `timingTiming` variant.
    #[fhir("timingTiming")]
    Timing(Box<types::Timing>),
    /// `timingReference` variant.
    #[fhir("timingReference")]
    Reference(Box<types::Reference>),
    /// `timingDate` variant.
    #[fhir("timingDate")]
    Date(crate::r4b::choice::Primitive<types::Date>),
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r4b::choice::Primitive<types::DateTime>),
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
