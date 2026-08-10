//! Period
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Period
//!
//! Version: 5.0.0
//!
//! Period Type: A time period defined by a start and end date and optionally time.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A time period defined by a start and an end date and, optionally, time.
///
/// `Period` is used throughout FHIR resources and other datatypes to express a
/// span of time, such as when a coverage was in effect, when an address was
/// valid, or when a condition was active. Either `start` or `end` (or both)
/// may be present; an absent `start` means the period began at an unknown
/// time in the past, and an absent `end` means the period is ongoing or open.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::period::Period;
/// use fhir::r5::types;
///
/// let value = Period {
///     start: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `start` is the name this serializes to on the wire.
/// assert_eq!(json["start"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Period = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The start date/time of the period, if known. If absent, the period is
    /// assumed to have started at some time before or at its end.
    pub start: Option<types::DateTime>, // « C »
    /// Primitive extension sibling for [`start`](Self::start) (FHIR `_start`).
    #[serde(rename = "_start")]
    pub start_ext: Option<types::Element>,
    /// The end date/time of the period. If absent, the period is ongoing or
    /// its end is unknown.
    pub end: Option<types::DateTime>, // « C »
    /// Primitive extension sibling for [`end`](Self::end) (FHIR `_end`).
    #[serde(rename = "_end")]
    pub end_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Period;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            start: None,
            end: None,
            ..Default::default()
        };
        assert_eq!(actual, expect);
    }

    mod serde_json {
        use super::*;
        use ::serde_json::json;

        #[test]
        fn test_serde_json_from_value() {
            let json = json!({});
            let actual: T = ::serde_json::from_value(json).expect("from_value");
            let expect: T = T::default();
            assert_eq!(actual, expect);
        }

        #[test]
        fn test_serde_json_to_value() {
            let actual: ::serde_json::Value =
                ::serde_json::to_value(T::default()).expect("to_value");
            let expect: ::serde_json::Value = json!({});
            assert_eq!(actual, expect);
        }
    }
}
