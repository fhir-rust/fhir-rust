//! SampledData
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SampledData
//!
//! Version: 4.0.1
//!
//! A series of measurements taken by a device
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for SampledData Type: A series of measurements
/// taken by a device, with upper and lower limits. There may be more than one
/// dimension in the data.
///
/// # Examples
///
/// ```
/// use fhir::r4::types::sampled_data::SampledData;
/// use fhir::r4::types;
///
/// let value = SampledData {
///     data: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `data` is the name this serializes to on the wire.
/// assert_eq!(json["data"], ::serde_json::json!("abc"));
///
/// let back: SampledData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct SampledData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Zero value and units
    pub origin: types::Quantity,

    /// Number of milliseconds between samples
    pub period: types::Decimal,
    /// Primitive extension sibling for [`period`](Self::period) (FHIR `_period`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_period")]
    pub period_ext: Option<types::Element>,

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

    /// Decimal values with spaces, or "E" | "U" | "L"
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
