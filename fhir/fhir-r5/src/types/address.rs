//! Address
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Address
//!
//! Version: 5.0.0
//!
//! Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world. The ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>
//!
//! This module defines the `Address` complex data type used to represent postal addresses.

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An address expressed using postal conventions (as opposed to GPS or other
/// location definition formats). This data type may be used to convey addresses
/// for use in delivering mail as well as for visiting locations which might not
/// be valid for mail delivery. There are a variety of postal address formats
/// defined around the world, so the individual line elements are flexible.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::address::Address;
/// use fhir::r5::types;
///
/// let value = Address {
///     postal_code: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `postalCode` is the name this serializes to on the wire.
/// assert_eq!(json["postalCode"], ::serde_json::json!("abc"));
///
/// let back: Address = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,
    /// The purpose of this address (home, work, temp, old, billing).
    #[serde(rename = "use")]
    pub use1: Option<types::Code>, // « AddressUse! »
    /// Primitive extension sibling for [`use1`](Self::use1) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Distinguishes between a postal address (physical/mailing) and a physical
    /// address (visiting location).
    #[serde(rename = "type")]
    pub r#type: Option<crate::r5::coded::Coded<crate::r5::codes::AddressType>>, // « AddressType! »
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// A full text representation of the address, useful when it is not
    /// possible or desired to break the address into its component parts.
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// The street address lines, in the order in which they appear on labels
    /// (e.g. street name, number, direction, or post office box).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<types::String>,
    /// Primitive extension sibling for [`line`](Self::line) (FHIR `_line`).
    #[serde(rename = "_line")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_ext: Vec<Option<types::Element>>,

    /// The name of the city, town, suburb, village, or other community or
    /// delivery center.
    pub city: Option<types::String>,
    /// Primitive extension sibling for [`city`](Self::city) (FHIR `_city`).
    #[serde(rename = "_city")]
    pub city_ext: Option<types::Element>,

    /// The name of the administrative area, such as a county, that is a
    /// subdivision of the state and is not usually included in postal addresses.
    pub district: Option<types::String>,
    /// Primitive extension sibling for [`district`](Self::district) (FHIR `_district`).
    #[serde(rename = "_district")]
    pub district_ext: Option<types::Element>,

    /// The sub-unit of the country, such as a state, province, or county.
    pub state: Option<types::String>,
    /// Primitive extension sibling for [`state`](Self::state) (FHIR `_state`).
    #[serde(rename = "_state")]
    pub state_ext: Option<types::Element>,

    /// A postal code designating the region, used for mail sorting.
    pub postal_code: Option<types::String>,
    /// Primitive extension sibling for [`postal_code`](Self::postal_code) (FHIR `_postalCode`).
    #[serde(rename = "_postalCode")]
    pub postal_code_ext: Option<types::Element>,

    /// The name of the country, using ISO 3166 codes or the full name where
    /// there is no known code.
    pub country: Option<types::String>,
    /// Primitive extension sibling for [`country`](Self::country) (FHIR `_country`).
    #[serde(rename = "_country")]
    pub country_ext: Option<types::Element>,

    /// The time period during which this address was, or is, in use.
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Address;

    #[test]
    fn test_default() {
        let actual = T::default();
        let expect = T {
            use1: None,
            r#type: None,
            text: None,
            line: vec![],
            city: None,
            district: None,
            state: None,
            postal_code: None,
            country: None,
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
            let json = json!(
                {
                    "line": []
                }
            );
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

#[cfg(test)]
mod drift_tests {
    use super::*;

    /// `Address.use` had no `_use` sibling, so a primitive extension on it
    /// was dropped in silence (spec 09). No published example carries one,
    /// which is why the corpus gate could not catch this and a targeted test
    /// has to.
    #[test]
    fn a_primitive_extension_on_use_survives() {
        let json = r#"{"use":"home","_use":{"extension":[{"url":"http://example.org/x","valueString":"why"}]}}"#;
        let parsed: Address = ::serde_json::from_str(json).expect("deserialize");
        assert!(parsed.use_ext.is_some(), "_use was dropped");
        assert_eq!(::serde_json::to_string(&parsed).expect("serialize"), json);
    }
}
