//! TriggerDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TriggerDefinition
//!
//! Version: 5.0.0
//!
//! TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// TriggerDefinition describes a triggering event that can initiate an action,
/// such as executing a knowledge artifact or evaluating a rule. Triggering
/// events can be named events, data events, or periodic events, as determined
/// by the `type` element. It is used throughout FHIR R5 in resources such as
/// PlanDefinition and EventDefinition to specify when and how an activity
/// should be triggered.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::trigger_definition::TriggerDefinition;
/// use fhir::r5::types;
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
pub struct TriggerDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::TriggerType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Name or URI that identifies the event
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Coded definition of the event
    pub code: Option<types::CodeableConcept>,

    /// Reference to a SubscriptionTopic resource that defines this event
    pub subscription_topic: Option<types::Canonical>,
    /// Primitive extension sibling for [`subscription_topic`](Self::subscription_topic) (FHIR `_subscriptionTopic`).
    #[serde(rename = "_subscriptionTopic")]
    pub subscription_topic_ext: Option<types::Element>,

    /// The `TriggerDefinition.timing[x]` choice element (0..1); see [`TriggerDefinitionTiming`].
    #[serde(flatten)]
    pub timing: Option<TriggerDefinitionTiming>,

    /// Triggering data of the event (multiple = 'and')
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<types::DataRequirement>,

    /// Whether the event triggers (boolean expression)
    pub condition: Option<types::Expression>,
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
/// The `TriggerDefinition.timing[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
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
    Date(crate::r5::choice::Primitive<types::Date>),
    /// `timingDateTime` variant.
    #[fhir("timingDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
}
