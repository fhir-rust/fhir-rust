//! Location
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Location
//!
//! Version: 5.0.0
//!
//! Location Resource: Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// Details and position information for a place where services are provided and
/// resources and participants may be stored, found, contained, or accommodated.
///
/// A Location includes both incidental locations (a place which is used for
/// healthcare without prior designation or authorization) and dedicated,
/// formally appointed locations. Locations may be private, public, mobile or
/// fixed and can vary in scale from a tiny hut to a whole building, or even a
/// space within a building. Locations are used to record where services are
/// provided and where equipment, participants and other resources are located.
///
/// In FHIR R5, Location is a primarily administrative resource used to describe
/// the physical places relevant to healthcare delivery, from geographic sites
/// such as buildings, wards, rooms, beds, vehicles, and homes to purely
/// jurisdictional or virtual places. It supports two modes: an `instance`
/// describes a single, specific physical place, while a `kind` describes a class
/// of locations that could be used generically, for example when scheduling
/// against "an available operating theatre" rather than a particular room.
/// Locations are commonly organized into hierarchies through the `part_of`
/// element (for example a bed within a room within a ward within a hospital),
/// carry geographic coordinates through the embedded position, and are
/// referenced by many operational resources to indicate where an activity
/// occurs.
///
/// # Related resources
///
/// A Location is typically managed by an [`Organization`](crate::r5::resources::organization::Organization)
/// via the managing organization, and is referenced by encounters, appointments,
/// and other workflow resources to record where care takes place. Its physical
/// form and function are described with [`CodeableConcept`](crate::r5::types::CodeableConcept)
/// values, its geographic coordinates with the nested [`LocationPosition`], and
/// its postal or physical address with an [`Address`](crate::r5::types::Address).
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::location::Location;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Logical id of this artifact
    pub id: Option<types::String>,

    /// Metadata about the resource
    pub meta: Option<types::Meta>,

    /// A set of rules under which this content was created
    pub implicit_rules: Option<types::Uri>,
    /// Primitive extension sibling for [`implicit_rules`](Self::implicit_rules) (FHIR `_implicitRules`).
    #[serde(rename = "_implicitRules")]
    pub implicit_rules_ext: Option<types::Element>,

    /// Language of the resource content
    pub language: Option<types::Code>,
    /// Primitive extension sibling for [`language`](Self::language) (FHIR `_language`).
    #[serde(rename = "_language")]
    pub language_ext: Option<types::Element>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<types::Narrative>,

    /// Contained, inline Resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained: Vec<crate::r5::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique code or number identifying the location to its users
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Overall availability status of the location as a whole, drawn from the value set active | suspended | inactive.
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::LocationStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// The operational status of the location (typically only for a bed/room)
    pub operational_status: Option<types::Coding>,

    /// Name of the location as used by humans
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// A list of alternate names that the location is known as, or was known as, in the past
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub alias: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`alias`](Self::alias) (FHIR `_alias`).
    #[serde(rename = "_alias")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alias_ext: Vec<Option<types::Element>>,

    /// Additional details about the location that could be displayed as further information to identify the location beyond its name
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Whether this record represents a specific physical place (instance) or a general class of places (kind).
    pub mode: Option<crate::r5::coded::Coded<crate::r5::codes::LocationMode>>,
    /// Primitive extension sibling for [`mode`](Self::mode) (FHIR `_mode`).
    #[serde(rename = "_mode")]
    pub mode_ext: Option<types::Element>,

    /// Type of function performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// Official contact details for the location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ExtendedContactDetail>,

    /// Physical location
    pub address: Option<types::Address>,

    /// Physical form of the location
    pub form: Option<types::CodeableConcept>,

    /// The absolute geographic location
    pub position: Option<LocationPosition>,

    /// Organization responsible for provisioning and upkeep
    pub managing_organization: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Reference to another Location that physically contains this one, enabling location hierarchies such as bed within room within ward.
    pub part_of: Option<types::Reference<crate::r5::resources::Location>>,

    /// Collection of characteristics (attributes)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub characteristic: Vec<types::CodeableConcept>,

    /// What days/times during a week is this location usually open (including exceptions)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hours_of_operation: Vec<types::Availability>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_service: Vec<types::VirtualServiceDetail>,

    /// Technical endpoints providing access to services operated for the location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint: Vec<types::Reference<crate::r5::resources::Endpoint>>,
}

/// The absolute geographic location of the Location, expressed using the WGS84
/// datum (this is the same datum used in KML and the same as used for global
/// positioning systems such as GPS).
/// # Examples
///
/// ```
/// use fhir::r5::resources::location::LocationPosition;
/// use fhir::r5::types;
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
pub struct LocationPosition {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Longitude with WGS84 datum
    pub longitude: types::Decimal,
    /// Primitive extension sibling for [`longitude`](Self::longitude) (FHIR `_longitude`).
    #[serde(rename = "_longitude")]
    pub longitude_ext: Option<types::Element>,

    /// Latitude with WGS84 datum
    pub latitude: types::Decimal,
    /// Primitive extension sibling for [`latitude`](Self::latitude) (FHIR `_latitude`).
    #[serde(rename = "_latitude")]
    pub latitude_ext: Option<types::Element>,

    /// Altitude with WGS84 datum
    pub altitude: Option<types::Decimal>,
    /// Primitive extension sibling for [`altitude`](Self::altitude) (FHIR `_altitude`).
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
