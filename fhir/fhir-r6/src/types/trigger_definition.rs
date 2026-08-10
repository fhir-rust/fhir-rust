//! TriggerDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
//!
//! Version: 6.0.0-ballot3
//!
//! Defines an expected trigger for a module
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// TriggerDefinition Type: A description of a triggering event. Triggering
/// events can be named events, data events, or periodic, as determined by the
/// type element.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::trigger_definition::TriggerDefinition;
/// use fhir::r6::types;
///
/// let value = TriggerDefinition {
///     subscription_topic: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `subscriptionTopic` is the name this serializes to on the wire.
/// assert_eq!(json["subscriptionTopic"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: TriggerDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "TriggerDefinitionDe")]
#[fhir_version("r6")]
pub struct TriggerDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// named-event | periodic | data-changed | data-added | data-modified |
    /// data-removed | data-accessed | data-access-ended
    pub r#type: crate::coded::Coded<crate::r6::codes::TriggerType>,
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

    /// Coded definition of the event
    pub code: Option<types::CodeableConcept>,

    /// What event
    pub subscription_topic: Option<types::Canonical>,
    /// Primitive extension sibling for [`subscription_topic`](Self::subscription_topic) (FHIR `_subscriptionTopic`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_subscriptionTopic")]
    pub subscription_topic_ext: Option<types::Element>,

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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TriggerDefinitionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    r#type: crate::coded::Coded<crate::r6::codes::TriggerType>,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    name: Option<types::String>,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    subscription_topic: Option<types::Canonical>,
    #[serde(rename = "_subscriptionTopic")]
    subscription_topic_ext: Option<types::Element>,
    #[serde(flatten)]
    timing: crate::r6::choice::Slot<TriggerDefinitionTiming>,
    #[serde(default)]
    data: Vec<types::DataRequirement>,
    condition: Option<types::Expression>,
}

impl ::core::convert::From<TriggerDefinitionDe> for TriggerDefinition {
    fn from(v: TriggerDefinitionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            r#type: v.r#type,
            type_ext: v.type_ext,
            name: v.name,
            name_ext: v.name_ext,
            code: v.code,
            subscription_topic: v.subscription_topic,
            subscription_topic_ext: v.subscription_topic_ext,
            timing: v.timing.0,
            data: v.data,
            condition: v.condition,
        }
    }
}

/// The `TriggerDefinition.timing[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    Date(crate::r6::choice::Primitive<types::Date>),
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
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
