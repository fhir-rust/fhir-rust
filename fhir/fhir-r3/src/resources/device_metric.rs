//! DeviceMetric
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
//!
//!
//!
//! Measurement, calculation or setting capability of a medical device
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DeviceMetric Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device_metric::DeviceMetric;
/// use fhir::r3::types;
///
/// let value = DeviceMetric {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: DeviceMetric = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DeviceMetric {
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

    /// Unique identifier of this DeviceMetric
    pub identifier: types::Identifier,

    /// Identity of metric, for example Heart Rate or PEEP Setting
    pub r#type: types::CodeableConcept,

    /// Unit of Measure for the Metric
    pub unit: Option<types::CodeableConcept>,

    /// Describes the link to the source Device
    pub source: Option<types::Reference<crate::r3::resources::Device>>,

    /// Describes the link to the parent DeviceComponent
    pub parent: Option<types::Reference<crate::r3::resources::DeviceComponent>>,

    /// on | off | standby | entered-in-error
    pub operational_status: Option<crate::coded::Coded<crate::r3::codes::MetricOperationalStatus>>,
    /// Primitive extension sibling for [`operational_status`](Self::operational_status) (FHIR `_operationalStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_operationalStatus")]
    pub operational_status_ext: Option<types::Element>,

    /// black | red | green | yellow | blue | magenta | cyan | white
    pub color: Option<crate::coded::Coded<crate::r3::codes::MetricColor>>,
    /// Primitive extension sibling for [`color`](Self::color) (FHIR `_color`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_color")]
    pub color_ext: Option<types::Element>,

    /// measurement | setting | calculation | unspecified
    pub category: crate::coded::Coded<crate::r3::codes::MetricCategory>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// Describes the measurement repetition time
    pub measurement_period: Option<types::Timing>,

    /// Describes the calibrations that have been performed or that are
    /// required to be performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calibration: Vec<DeviceMetricCalibration>,
}

/// Describes the calibrations that have been performed or that are required to
/// be performed.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device_metric::DeviceMetricCalibration;
/// use fhir::r3::types;
///
/// let value = DeviceMetricCalibration {
///     time: Some(types::Instant("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `time` is the name this serializes to on the wire.
/// assert_eq!(json["time"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceMetricCalibration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DeviceMetricCalibration {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// unspecified | offset | gain | two-point
    pub r#type: Option<crate::coded::Coded<crate::r3::codes::MetricCalibrationType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// not-calibrated | calibration-required | calibrated | unspecified
    pub state: Option<crate::coded::Coded<crate::r3::codes::MetricCalibrationState>>,
    /// Primitive extension sibling for [`state`](Self::state) (FHIR `_state`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_state")]
    pub state_ext: Option<types::Element>,

    /// Describes the time last calibration has been performed
    pub time: Option<types::Instant>,
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_time")]
    pub time_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceMetric;

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
