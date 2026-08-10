//! Address
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Address
//!
//!
//!
//! A postal address
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Address Type
///
/// # Examples
///
/// ```
/// use fhir::r2::types::address::Address;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct Address {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// home | work | temp | old - purpose of this address
    pub r#use: Option<crate::coded::Coded<crate::r2::codes::AddressUse>>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// postal | physical | both
    pub r#type: Option<crate::coded::Coded<crate::r2::codes::AddressType>>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Text representation of the address
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Street name, number, direction & P.O. Box etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<types::String>,
    /// Primitive extension sibling for [`line`](Self::line) (FHIR `_line`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_line")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_ext: Vec<Option<types::Element>>,

    /// Name of city, town etc.
    pub city: Option<types::String>,
    /// Primitive extension sibling for [`city`](Self::city) (FHIR `_city`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_city")]
    pub city_ext: Option<types::Element>,

    /// District name (aka county)
    pub district: Option<types::String>,
    /// Primitive extension sibling for [`district`](Self::district) (FHIR `_district`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_district")]
    pub district_ext: Option<types::Element>,

    /// Sub-unit of country (abbreviations ok)
    pub state: Option<types::String>,
    /// Primitive extension sibling for [`state`](Self::state) (FHIR `_state`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_state")]
    pub state_ext: Option<types::Element>,

    /// Postal code for area
    pub postal_code: Option<types::String>,
    /// Primitive extension sibling for [`postal_code`](Self::postal_code) (FHIR `_postalCode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_postalCode")]
    pub postal_code_ext: Option<types::Element>,

    /// Country (can be ISO 3166 3 letter code)
    pub country: Option<types::String>,
    /// Primitive extension sibling for [`country`](Self::country) (FHIR `_country`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_country")]
    pub country_ext: Option<types::Element>,

    /// Time period when address was/is in use
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Address;

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
