//! Availability
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Availability
//!
//! Version: 6.0.0-ballot3
//!
//! Availability data for an {item}
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Availability Type: Availability data for an {item}, declaring what
/// days/times are available, and any exceptions. The exceptions could be
/// textual only, e.g. Public holidays, or could be time period specific and
/// indicate a specific years dates.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::availability::Availability;
/// use fhir::r6::types;
///
/// let value = Availability {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Availability = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Availability {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// When the availability applies
    pub period: Option<types::Period>,

    /// Times the {item} is available
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_time: Vec<AvailabilityAvailableTime>,

    /// Not available during this time due to provided reason
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_available_time: Vec<AvailabilityNotAvailableTime>,
}

/// A collection of times that the {item} is available.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::availability::AvailabilityAvailableTime;
/// use fhir::r6::types;
///
/// let value = AvailabilityAvailableTime {
///     all_day: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `allDay` is the name this serializes to on the wire.
/// assert_eq!(json["allDay"], ::serde_json::json!(true));
///
/// let back: AvailabilityAvailableTime = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct AvailabilityAvailableTime {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_week: Vec<crate::coded::Coded<crate::r6::codes::DaysOfWeek>>,
    /// Primitive extension sibling for [`days_of_week`](Self::days_of_week) (FHIR `_daysOfWeek`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_daysOfWeek")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_week_ext: Vec<Option<types::Element>>,

    /// Always available? i.e. 24 hour service
    pub all_day: Option<types::Boolean>,
    /// Primitive extension sibling for [`all_day`](Self::all_day) (FHIR `_allDay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allDay")]
    pub all_day_ext: Option<types::Element>,

    /// Opening time of day (ignored if allDay = true)
    pub available_start_time: Option<types::Time>,
    /// Primitive extension sibling for [`available_start_time`](Self::available_start_time) (FHIR `_availableStartTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availableStartTime")]
    pub available_start_time_ext: Option<types::Element>,

    /// Closing time of day (ignored if allDay = true)
    pub available_end_time: Option<types::Time>,
    /// Primitive extension sibling for [`available_end_time`](Self::available_end_time) (FHIR `_availableEndTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_availableEndTime")]
    pub available_end_time_ext: Option<types::Element>,
}

/// The {item} is not available during this period of time due to the provided
/// reason.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::availability::AvailabilityNotAvailableTime;
/// use fhir::r6::types;
///
/// let value = AvailabilityNotAvailableTime {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: AvailabilityNotAvailableTime = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct AvailabilityNotAvailableTime {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Reason presented to the user explaining why time not available
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Service not available during this period
    pub during: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Availability;

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
