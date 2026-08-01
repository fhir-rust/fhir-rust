//! Timing
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Timing
//!
//!
//!
//! A timing schedule that specifies an event that may occur multiple times
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Timing Type
///
/// # Examples
///
/// ```
/// use fhir::r2::types::timing::Timing;
/// use fhir::r2::types;
///
/// let value = Timing {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: Timing = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Timing {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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

    /// QD | QOD | Q4H | Q6H | BID | TID | QID | AM | PM +
    pub code: Option<types::CodeableConcept>,
}

/// A set of rules that describe when the event should occur.
///
/// # Examples
///
/// ```
/// use fhir::r2::types::timing::TimingRepeat;
/// use fhir::r2::types;
///
/// let value = TimingRepeat {
///     duration_units: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `durationUnits` is the name this serializes to on the wire.
/// assert_eq!(json["durationUnits"], ::serde_json::json!("final"));
///
/// let back: TimingRepeat = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TimingRepeat {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub duration_units: Option<types::Code>,
    /// Primitive extension sibling for [`duration_units`](Self::duration_units) (FHIR `_durationUnits`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_durationUnits")]
    pub duration_units_ext: Option<types::Element>,

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
    pub period_units: Option<types::Code>,
    /// Primitive extension sibling for [`period_units`](Self::period_units) (FHIR `_periodUnits`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_periodUnits")]
    pub period_units_ext: Option<types::Element>,

    /// Regular life events the event is tied to
    pub when: Option<types::Code>,
    /// Primitive extension sibling for [`when`](Self::when) (FHIR `_when`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_when")]
    pub when_ext: Option<types::Element>,
}

/// The `Timing.repeat.bounds[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum TimingRepeatBounds {
    /// `boundsQuantity` variant.
    #[fhir("boundsQuantity")]
    Quantity(Box<types::Quantity>),
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
