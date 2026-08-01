//! SampledData
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SampledData
//!
//! Version: 5.0.0
//!
//! SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A compact way to represent a series of measurements taken by a device over time or space, such as an ECG waveform, rather than encoding each reading as a discrete element.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A series of measurements taken by a device, with upper and lower limits, decoded from a
/// compact `data` string of space-separated values or codes. It is commonly used to represent
/// waveform-like data such as ECG traces or other device-generated signal readings, where the
/// interval, factor, and limits describe how to interpret the encoded data points. There may be
/// more than one dimension in the data.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::sampled_data::SampledData;
/// use fhir::r5::types;
///
/// let value = SampledData {
///     code_map: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `codeMap` is the name this serializes to on the wire.
/// assert_eq!(json["codeMap"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: SampledData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SampledData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The base quantity that a measured value of zero represents, using the appropriate units. // Quantity(SimpleQuantity) [1..1]
    pub origin: types::Quantity,
    /// Deprecated. The length of time between sampling times, measured in intervalUnit. // « C »
    pub interval: Option<types::Decimal>,
    /// Primitive extension sibling for [`interval`](Self::interval) (FHIR `_interval`).
    #[serde(rename = "_interval")]
    pub interval_ext: Option<types::Element>,
    /// The measurement unit in which the sample interval is expressed. // « UCUMCodes! »
    pub interval_unit: types::Code,
    /// Primitive extension sibling for [`interval_unit`](Self::interval_unit) (FHIR `_intervalUnit`).
    #[serde(rename = "_intervalUnit")]
    pub interval_unit_ext: Option<types::Element>,
    /// A correction factor that is applied to the sampled data points before they are added to the origin.
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`).
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,
    /// The lower limit of detection of the measured points; below this, the device did not record.
    pub lower_limit: Option<types::Decimal>,
    /// Primitive extension sibling for [`lower_limit`](Self::lower_limit) (FHIR `_lowerLimit`).
    #[serde(rename = "_lowerLimit")]
    pub lower_limit_ext: Option<types::Element>,
    /// The upper limit of detection of the measured points; above this, the device did not record.
    pub upper_limit: Option<types::Decimal>,
    /// Primitive extension sibling for [`upper_limit`](Self::upper_limit) (FHIR `_upperLimit`).
    #[serde(rename = "_upperLimit")]
    pub upper_limit_ext: Option<types::Element>,
    /// The number of sample points at each time point, allowing for more than one dimension of data.
    pub dimensions: types::PositiveInt,
    /// Primitive extension sibling for [`dimensions`](Self::dimensions) (FHIR `_dimensions`).
    #[serde(rename = "_dimensions")]
    pub dimensions_ext: Option<types::Element>,
    /// Reference to a ConceptMap that defines the codes used in the `data` element, if any. // « ConceptMap »
    pub code_map: Option<types::Canonical>,
    /// Primitive extension sibling for [`code_map`](Self::code_map) (FHIR `_codeMap`).
    #[serde(rename = "_codeMap")]
    pub code_map_ext: Option<types::Element>,
    /// A series of data points, separated by single spaces, that give the offset from the origin for the corresponding data point. // « C »
    pub offsets: Option<types::String>,
    /// Primitive extension sibling for [`offsets`](Self::offsets) (FHIR `_offsets`).
    #[serde(rename = "_offsets")]
    pub offsets_ext: Option<types::Element>,
    /// A series of data points, separated by single spaces, encoding the measured values, using special codes for out-of-range values.
    pub data: Option<types::String>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`).
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SampledData;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            origin: types::Quantity::default(),
            interval: None,
            interval_unit: types::Code::default(),
            factor: None,
            lower_limit: None,
            upper_limit: None,
            dimensions: types::PositiveInt::default(),
            code_map: None,
            offsets: None,
            data: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;

        #[test]
        fn test_serde_json_round_trip() {
            let value = T::default();
            let json = ::serde_json::to_value(&value).expect("to_value");
            let back: T = ::serde_json::from_value(json).expect("from_value");
            assert_eq!(value, back);
        }
    }
}
