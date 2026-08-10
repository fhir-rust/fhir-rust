//! Location
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Location
//!
//!
//!
//! Details and position information for a physical place
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Location Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::location::Location;
/// use fhir::r3::types;
///
/// let value = Location {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: Location = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Location {
    /// Logical id of this artifact
    pub id: Option<types::Id>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique code or number identifying the location to its users
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | suspended | inactive
    pub status: Option<crate::coded::Coded<crate::r3::codes::LocationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The Operational status of the location (typically only for a bed/room)
    pub operational_status: Option<types::Coding>,

    /// Name of the location as used by humans
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// A list of alternate names that the location is known as, or was known
    /// as in the past
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub alias: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// Additional details about the location that could be displayed as
    /// further information to identify the location beyond its name
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// instance | kind
    pub mode: Option<crate::coded::Coded<crate::r3::codes::LocationMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Type of function performed
    pub r#type: Option<types::CodeableConcept>,

    /// Contact details of the location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,

    /// Physical location
    pub address: Option<types::Address>,

    /// Physical form of the location
    pub physical_type: Option<types::CodeableConcept>,

    /// The absolute geographic location
    pub position: Option<LocationPosition>,

    /// Organization responsible for provisioning and upkeep
    pub managing_organization: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Another Location this one is physically part of
    pub part_of: Option<types::Reference<crate::r3::resources::Location>>,

    /// Technical endpoints providing access to services operated for the
    /// location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r3::resources::Endpoint>>,
}

/// The absolute geographic location of the Location, expressed using the WGS84
/// datum (This is the same co-ordinate system used in KML).
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::location::LocationPosition;
/// use fhir::r3::types;
///
/// let value = LocationPosition {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: LocationPosition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct LocationPosition {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Longitude with WGS84 datum
    pub longitude: types::Decimal,
    /// Primitive extension sibling for [`longitude`](Self::longitude) (FHIR `_longitude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_longitude")]
    pub longitude_ext: Option<types::Element>,

    /// Latitude with WGS84 datum
    pub latitude: types::Decimal,
    /// Primitive extension sibling for [`latitude`](Self::latitude) (FHIR `_latitude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_latitude")]
    pub latitude_ext: Option<types::Element>,

    /// Altitude with WGS84 datum
    pub altitude: Option<types::Decimal>,
    /// Primitive extension sibling for [`altitude`](Self::altitude) (FHIR `_altitude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_altitude")]
    pub altitude_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Location;

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
