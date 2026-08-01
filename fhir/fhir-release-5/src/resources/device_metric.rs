//! DeviceMetric
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceMetric
//!
//! Version: 5.0.0
//!
//! DeviceMetric Resource: Describes a measurement, calculation or setting capability of a device.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Describes a measurement, calculation or setting capability of a device.
///
/// The DeviceMetric resource describes a single measurement, calculation, or
/// setting capability of a medical device, such as a heart rate reading or a
/// PEEP setting on a ventilator. It is derived from the ISO/IEEE 11073-10201
/// Domain Information Model standard but is more widely applicable. Each metric
/// links to its parent Device and can describe its unit of measure, operational
/// status, category, measurement frequency, and any calibrations performed.
/// DeviceMetric instances are commonly referenced by observation and monitoring
/// workflows, where a device produces one or more metrics that are subsequently
/// reported as clinical measurements for a subject such as a [`Patient`](crate::r5::resources::patient::Patient).
///
/// See also: the parent `Device` resource that this metric describes, and
/// [`CodeableConcept`](crate::r5::types::CodeableConcept) for the coded `type`,
/// `unit`, and `category` values used throughout this resource.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_metric::DeviceMetric;
/// use fhir::r5::types;
///
/// let value = DeviceMetric {
///     color: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `color` is the name this serializes to on the wire.
/// assert_eq!(json["color"], ::serde_json::json!("final"));
///
/// let back: DeviceMetric = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetric {
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

    /// Instance identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Identity of metric, for example Heart Rate or PEEP Setting; a coded concept identifying what is measured.
    pub r#type: types::CodeableConcept,

    /// Unit of Measure for the Metric
    pub unit: Option<types::CodeableConcept>,

    /// Reference to the parent Device that this metric describes and produces measurements for.
    pub device: types::Reference,

    /// Indicates the current operational status of the device metric: on | off | standby | entered-in-error.
    pub operational_status:
        Option<crate::r5::coded::Coded<crate::r5::codes::MetricOperationalStatus>>,
    /// Primitive extension sibling for [`operational_status`](Self::operational_status) (FHIR `_operationalStatus`).
    #[serde(rename = "_operationalStatus")]
    pub operational_status_ext: Option<types::Element>,

    /// Color name (from CSS4) or #RRGGBB code
    pub color: Option<types::Code>,
    /// Primitive extension sibling for [`color`](Self::color) (FHIR `_color`).
    #[serde(rename = "_color")]
    pub color_ext: Option<types::Element>,

    /// Classifies the metric as a measurement, setting, calculation, or unspecified kind.
    pub category: crate::r5::coded::Coded<crate::r5::codes::MetricCategory>,
    /// Primitive extension sibling for [`category`](Self::category) (FHIR `_category`).
    #[serde(rename = "_category")]
    pub category_ext: Option<types::Element>,

    /// Indicates how often the metric is taken or recorded
    pub measurement_frequency: Option<types::Quantity>,

    /// Describes any calibrations that have been performed, or that are required, for this metric.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calibration: Vec<DeviceMetricCalibration>,
}

/// Describes the calibrations that have been performed or that are required to be performed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::device_metric::DeviceMetricCalibration;
/// use fhir::r5::types;
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
pub struct DeviceMetricCalibration {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// unspecified | offset | gain | two-point
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::MetricCalibrationType>>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// not-calibrated | calibration-required | calibrated | unspecified
    pub state: Option<crate::r5::coded::Coded<crate::r5::codes::MetricCalibrationState>>,
    /// Primitive extension sibling for [`state`](Self::state) (FHIR `_state`).
    #[serde(rename = "_state")]
    pub state_ext: Option<types::Element>,

    /// Describes the time last calibration has been performed
    pub time: Option<types::Instant>,
    /// Primitive extension sibling for [`time`](Self::time) (FHIR `_time`).
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
