//! ContactPoint
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ContactPoint
//!
//! Version: 6.0.0-ballot3
//!
//! Details of a Technology mediated contact point (phone, fax, email, etc.)
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// ContactPoint Type: Details for all kinds of technology mediated contact
/// points for a person or organization, including telephone, email, etc.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::contact_point::ContactPoint;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct ContactPoint {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// phone | fax | email | pager | url | sms | other
    pub system: Option<crate::coded::Coded<crate::r6::codes::ContactPointSystem>>,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// The actual contact point details
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// home | work | temp | old | mobile - purpose of this contact point
    pub r#use: Option<crate::coded::Coded<crate::r6::codes::ContactPointUse>>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Specify preferred order of use (1 = highest)
    pub rank: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`rank`](Self::rank) (FHIR `_rank`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rank")]
    pub rank_ext: Option<types::Element>,

    /// Time period when the contact point was/is in use
    pub period: Option<types::Period>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ContactPoint;

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
