//! SampledData
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SampledData
//!
//! Version: 6.0.0-ballot3
//!
//! A series of measurements taken by a device
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// SampledData Type: A series of measurements taken by a device, with upper
/// and lower limits. There may be more than one dimension in the data.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::sampled_data::SampledData;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct SampledData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Zero value and units
    pub origin: types::Quantity,

    /// Number of intervalUnits between samples
    pub interval: Option<types::Decimal>,
    /// Primitive extension sibling for [`interval`](Self::interval) (FHIR `_interval`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_interval")]
    pub interval_ext: Option<types::Element>,

    /// The measurement unit of the interval between samples
    pub interval_unit: types::Code,
    /// Primitive extension sibling for [`interval_unit`](Self::interval_unit) (FHIR `_intervalUnit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intervalUnit")]
    pub interval_unit_ext: Option<types::Element>,

    /// Multiply data by this before adding to origin
    pub factor: Option<types::Decimal>,
    /// Primitive extension sibling for [`factor`](Self::factor) (FHIR `_factor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_factor")]
    pub factor_ext: Option<types::Element>,

    /// Lower limit of detection
    pub lower_limit: Option<types::Decimal>,
    /// Primitive extension sibling for [`lower_limit`](Self::lower_limit) (FHIR `_lowerLimit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lowerLimit")]
    pub lower_limit_ext: Option<types::Element>,

    /// Upper limit of detection
    pub upper_limit: Option<types::Decimal>,
    /// Primitive extension sibling for [`upper_limit`](Self::upper_limit) (FHIR `_upperLimit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_upperLimit")]
    pub upper_limit_ext: Option<types::Element>,

    /// Number of sample points at each time point
    pub dimensions: types::PositiveInt,
    /// Primitive extension sibling for [`dimensions`](Self::dimensions) (FHIR `_dimensions`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dimensions")]
    pub dimensions_ext: Option<types::Element>,

    /// Defines the codes used in the data
    pub code_map: Option<types::Canonical>,
    /// Primitive extension sibling for [`code_map`](Self::code_map) (FHIR `_codeMap`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_codeMap")]
    pub code_map_ext: Option<types::Element>,

    /// Offsets, typically in time, at which data values were taken
    pub offsets: Option<types::String>,
    /// Primitive extension sibling for [`offsets`](Self::offsets) (FHIR `_offsets`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_offsets")]
    pub offsets_ext: Option<types::Element>,

    /// Decimal values with spaces, or "E" | "U" | "L", or another code
    pub data: Option<types::String>,
    /// Primitive extension sibling for [`data`](Self::data) (FHIR `_data`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_data")]
    pub data_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SampledData;

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
