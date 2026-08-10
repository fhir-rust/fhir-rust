//! ContactPoint
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ContactPoint
//!
//! Version: 5.0.0
//!
//! ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! A ContactPoint captures a single technology-mediated way of reaching a person or organization, such as a phone number, email address, fax, or pager.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Details for a technology mediated contact point for a person or organization, such as a phone number, email, fax, or pager, including its use, rank among alternatives, and the period during which it is valid.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::contact_point::ContactPoint;
/// use fhir::r5::types;
///
/// let value = ContactPoint {
///     value: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `value` is the name this serializes to on the wire.
/// assert_eq!(json["value"], ::serde_json::json!("abc"));
///
/// let back: ContactPoint = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ContactPoint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The kind of communication medium this contact point represents, e.g. phone, fax, email, pager, url, sms, other.
    pub system: Option<crate::r5::coded::Coded<crate::r5::codes::ContactPointSystem>>, // « ContactPointSystem! » « C »
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,
    /// The actual contact point value, such as a phone number or email address, in the format appropriate for the system.
    pub value: Option<types::String>, // « C »
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
    #[serde(rename = "use")]
    /// The purpose or context for this contact point, e.g. home, work, temp, old, mobile.
    pub use1: Option<types::Code>, // « ContactPointUse! »
    /// Primitive extension sibling for [`use1`](Self::use1) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,
    /// The relative preference order for use of this contact point among an ordered set of alternatives, with lower values indicating higher preference.
    pub rank: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`rank`](Self::rank) (FHIR `_rank`).
    #[serde(rename = "_rank")]
    pub rank_ext: Option<types::Element>,
    /// The time period during which this contact point is or was in use.
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ContactPoint;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            system: None,
            value: None,
            use1: None,
            rank: None,
            period: None,
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
