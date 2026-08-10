//! Timing
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Timing
//!
//!
//!
//! A timing schedule that specifies an event that may occur multiple times
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Timing Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::timing::Timing;
/// use fhir::r3::types;
///
/// let value = Timing {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: Timing = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Timing {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// When the event occurs
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<types::DateTime>,
    /// Primitive extension sibling for [`event`](Self::event) (FHIR `_event`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_event")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_ext: Vec<Option<types::Element>>,

    /// When the event is to occur
    pub repeat: Option<TimingRepeat>,

    /// BID | TID | QID | AM | PM | QD | QOD | Q4H | Q6H +
    pub code: Option<types::CodeableConcept>,
}

/// A set of rules that describe when the event is scheduled.
///
/// # Examples
///
/// ```
/// use fhir::r3::types::timing::TimingRepeat;
/// use fhir::r3::types;
///
/// let value = TimingRepeat {
///     count_max: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `countMax` is the name this serializes to on the wire.
/// assert_eq!(json["countMax"], ::serde_json::json!(42));
///
/// let back: TimingRepeat = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "TimingRepeatDe")]
#[fhir_version("r3")]
pub struct TimingRepeat {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Length/Range of lengths, or (Start and/or end) limits
    /// The `Timing.repeat.bounds[x]` choice element (0..1); see [`TimingRepeatBounds`].
    #[serde(flatten)]
    pub bounds: Option<TimingRepeatBounds>,

    /// Number of times to repeat
    pub count: Option<types::Integer>,
    /// Primitive extension sibling for [`count`](Self::count) (FHIR `_count`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_count")]
    pub count_ext: Option<types::Element>,

    /// Maximum number of times to repeat
    pub count_max: Option<types::Integer>,
    /// Primitive extension sibling for [`count_max`](Self::count_max) (FHIR `_countMax`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_countMax")]
    pub count_max_ext: Option<types::Element>,

    /// How long when it happens
    pub duration: Option<types::Decimal>,
    /// Primitive extension sibling for [`duration`](Self::duration) (FHIR `_duration`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_duration")]
    pub duration_ext: Option<types::Element>,

    /// How long when it happens (Max)
    pub duration_max: Option<types::Decimal>,
    /// Primitive extension sibling for [`duration_max`](Self::duration_max) (FHIR `_durationMax`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_durationMax")]
    pub duration_max_ext: Option<types::Element>,

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    pub duration_unit: Option<types::Code>,
    /// Primitive extension sibling for [`duration_unit`](Self::duration_unit) (FHIR `_durationUnit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_durationUnit")]
    pub duration_unit_ext: Option<types::Element>,

    /// Event occurs frequency times per period
    pub frequency: Option<types::Integer>,
    /// Primitive extension sibling for [`frequency`](Self::frequency) (FHIR `_frequency`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_frequency")]
    pub frequency_ext: Option<types::Element>,

    /// Event occurs up to frequencyMax times per period
    pub frequency_max: Option<types::Integer>,
    /// Primitive extension sibling for [`frequency_max`](Self::frequency_max) (FHIR `_frequencyMax`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_frequencyMax")]
    pub frequency_max_ext: Option<types::Element>,

    /// Event occurs frequency times per period
    pub period: Option<types::Decimal>,
    /// Primitive extension sibling for [`period`](Self::period) (FHIR `_period`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_period")]
    pub period_ext: Option<types::Element>,

    /// Upper limit of period (3-4 hours)
    pub period_max: Option<types::Decimal>,
    /// Primitive extension sibling for [`period_max`](Self::period_max) (FHIR `_periodMax`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_periodMax")]
    pub period_max_ext: Option<types::Element>,

    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    pub period_unit: Option<types::Code>,
    /// Primitive extension sibling for [`period_unit`](Self::period_unit) (FHIR `_periodUnit`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_periodUnit")]
    pub period_unit_ext: Option<types::Element>,

    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_of_week: Vec<crate::coded::Coded<crate::r3::codes::DaysOfWeek>>,
    /// Primitive extension sibling for [`day_of_week`](Self::day_of_week) (FHIR `_dayOfWeek`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dayOfWeek")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_of_week_ext: Vec<Option<types::Element>>,

    /// Time of day for action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time_of_day: Vec<types::Time>,
    /// Primitive extension sibling for [`time_of_day`](Self::time_of_day) (FHIR `_timeOfDay`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_timeOfDay")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time_of_day_ext: Vec<Option<types::Element>>,

    /// Regular life events the event is tied to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub when: Vec<crate::coded::Coded<crate::r3::codes::EventTiming>>,
    /// Primitive extension sibling for [`when`](Self::when) (FHIR `_when`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_when")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub when_ext: Vec<Option<types::Element>>,

    /// Minutes from event (before or after)
    pub offset: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`offset`](Self::offset) (FHIR `_offset`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_offset")]
    pub offset_ext: Option<types::Element>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimingRepeatDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(flatten)]
    bounds: crate::r3::choice::Slot<TimingRepeatBounds>,
    count: Option<types::Integer>,
    #[serde(rename = "_count")]
    count_ext: Option<types::Element>,
    count_max: Option<types::Integer>,
    #[serde(rename = "_countMax")]
    count_max_ext: Option<types::Element>,
    duration: Option<types::Decimal>,
    #[serde(rename = "_duration")]
    duration_ext: Option<types::Element>,
    duration_max: Option<types::Decimal>,
    #[serde(rename = "_durationMax")]
    duration_max_ext: Option<types::Element>,
    duration_unit: Option<types::Code>,
    #[serde(rename = "_durationUnit")]
    duration_unit_ext: Option<types::Element>,
    frequency: Option<types::Integer>,
    #[serde(rename = "_frequency")]
    frequency_ext: Option<types::Element>,
    frequency_max: Option<types::Integer>,
    #[serde(rename = "_frequencyMax")]
    frequency_max_ext: Option<types::Element>,
    period: Option<types::Decimal>,
    #[serde(rename = "_period")]
    period_ext: Option<types::Element>,
    period_max: Option<types::Decimal>,
    #[serde(rename = "_periodMax")]
    period_max_ext: Option<types::Element>,
    period_unit: Option<types::Code>,
    #[serde(rename = "_periodUnit")]
    period_unit_ext: Option<types::Element>,
    #[serde(default)]
    day_of_week: Vec<crate::coded::Coded<crate::r3::codes::DaysOfWeek>>,
    #[serde(rename = "_dayOfWeek")]
    #[serde(default)]
    day_of_week_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    time_of_day: Vec<types::Time>,
    #[serde(rename = "_timeOfDay")]
    #[serde(default)]
    time_of_day_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    when: Vec<crate::coded::Coded<crate::r3::codes::EventTiming>>,
    #[serde(rename = "_when")]
    #[serde(default)]
    when_ext: Vec<Option<types::Element>>,
    offset: Option<types::UnsignedInt>,
    #[serde(rename = "_offset")]
    offset_ext: Option<types::Element>,
}

impl ::core::convert::From<TimingRepeatDe> for TimingRepeat {
    fn from(v: TimingRepeatDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            bounds: v.bounds.0,
            count: v.count,
            count_ext: v.count_ext,
            count_max: v.count_max,
            count_max_ext: v.count_max_ext,
            duration: v.duration,
            duration_ext: v.duration_ext,
            duration_max: v.duration_max,
            duration_max_ext: v.duration_max_ext,
            duration_unit: v.duration_unit,
            duration_unit_ext: v.duration_unit_ext,
            frequency: v.frequency,
            frequency_ext: v.frequency_ext,
            frequency_max: v.frequency_max,
            frequency_max_ext: v.frequency_max_ext,
            period: v.period,
            period_ext: v.period_ext,
            period_max: v.period_max,
            period_max_ext: v.period_max_ext,
            period_unit: v.period_unit,
            period_unit_ext: v.period_unit_ext,
            day_of_week: v.day_of_week,
            day_of_week_ext: v.day_of_week_ext,
            time_of_day: v.time_of_day,
            time_of_day_ext: v.time_of_day_ext,
            when: v.when,
            when_ext: v.when_ext,
            offset: v.offset,
            offset_ext: v.offset_ext,
        }
    }
}

/// The `Timing.repeat.bounds[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum TimingRepeatBounds {
    /// `boundsDuration` variant.
    #[fhir("boundsDuration")]
    Duration(Box<types::Duration>),
    /// `boundsRange` variant.
    #[fhir("boundsRange")]
    Range(Box<types::Range>),
    /// `boundsPeriod` variant.
    #[fhir("boundsPeriod")]
    Period(Box<types::Period>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Timing;

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
