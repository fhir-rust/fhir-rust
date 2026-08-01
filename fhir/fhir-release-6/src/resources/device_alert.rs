//! DeviceAlert
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceAlert
//!
//! Version: 6.0.0-ballot3
//!
//! Describes a noteworthy condition or occurrence determined to exist by a
//! medical device
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Describes a physiological or technical alert condition report originated by
/// a device. The DeviceAlert resource is derived from the ISO/IEEE 11073-10201
/// Domain Information Model standard, but is more widely applicable.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_alert::DeviceAlert;
/// use fhir::r6::types;
///
/// let value = DeviceAlert {
///     label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `label` is the name this serializes to on the wire.
/// assert_eq!(json["label"], ::serde_json::json!("abc"));
///
/// let back: DeviceAlert = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceAlert {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// in-progress | completed | entered-in-error
    pub status: crate::coded::Coded<crate::r6::codes::DevicealertStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// physiological | technical
    pub r#type: Option<crate::coded::Coded<crate::r6::codes::DevicealertType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// high | medium | low | info
    pub priority: Option<crate::coded::Coded<crate::r6::codes::DevicealertPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// The who or what the alert is about
    pub subject: types::Reference,

    /// The device (or DeviceMetric) that detected the alert condition
    pub source: Option<types::Reference>,

    /// The condition, event, or state being reported
    pub condition: DeviceAlertCondition,

    /// The value causing the alert condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub derived_from: Vec<types::Reference>,

    /// Text to be displayed for the alert condition
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Annunciation or notification of the alert condition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signal: Vec<DeviceAlertSignal>,
}

/// The condition, event, or state being reported.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_alert::DeviceAlertCondition;
/// use fhir::r6::types;
///
/// let value = DeviceAlertCondition {
///     acknowledged: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `acknowledged` is the name this serializes to on the wire.
/// assert_eq!(json["acknowledged"], ::serde_json::json!(true));
///
/// let back: DeviceAlertCondition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceAlertCondition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The meaning of the alert
    pub code: types::CodeableConcept,

    /// Whether the alert condition has been acknowledged
    pub acknowledged: Option<types::Boolean>,
    /// Primitive extension sibling for [`acknowledged`](Self::acknowledged) (FHIR `_acknowledged`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_acknowledged")]
    pub acknowledged_ext: Option<types::Element>,

    /// The alert condition is currently occurring
    pub presence: types::Boolean,
    /// Primitive extension sibling for [`presence`](Self::presence) (FHIR `_presence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_presence")]
    pub presence_ext: Option<types::Element>,

    /// The period during which the condition was active
    pub timing: Option<types::Period>,

    /// The boundaries outside of which a value was detected to cause the alert
    /// condition
    pub limit: Option<types::Range>,
}

/// Annunciation or notification of the alert condition.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_alert::DeviceAlertSignal;
/// use fhir::r6::types;
///
/// let value = DeviceAlertSignal {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceAlertSignal = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct DeviceAlertSignal {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// on | off | paused
    pub activation_state: crate::coded::Coded<crate::r6::codes::DevicealertActivationState>,
    /// Primitive extension sibling for [`activation_state`](Self::activation_state) (FHIR `_activationState`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_activationState")]
    pub activation_state_ext: Option<types::Element>,

    /// on | latched | off | ack
    pub presence: Option<crate::coded::Coded<crate::r6::codes::DevicealertPresence>>,
    /// Primitive extension sibling for [`presence`](Self::presence) (FHIR `_presence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_presence")]
    pub presence_ext: Option<types::Element>,

    /// Where the signal is being annunciated
    pub annunciator: Option<types::CodeableReference>,

    /// How the signal is being annunciated
    pub manifestation: Option<types::CodeableConcept>,

    /// Characteristics of the signal manifestation
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// When the signal was being annunciated
    pub indication: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceAlert;

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
