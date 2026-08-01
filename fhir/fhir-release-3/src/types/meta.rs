//! Meta
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Meta
//!
//!
//!
//! Metadata about a resource
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Meta Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::meta::Meta;
/// use fhir::r3::types;
///
/// let value = Meta {
///     version_id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `versionId` is the name this serializes to on the wire.
/// assert_eq!(json["versionId"], ::serde_json::json!("pat-1"));
///
/// let back: Meta = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct Meta {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Version specific identifier
    pub version_id: Option<types::Id>,
    /// Primitive extension sibling for [`version_id`](Self::version_id) (FHIR `_versionId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_versionId")]
    pub version_id_ext: Option<types::Element>,

    /// When the resource version last changed
    pub last_updated: Option<types::Instant>,
    /// Primitive extension sibling for [`last_updated`](Self::last_updated) (FHIR `_lastUpdated`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_lastUpdated")]
    pub last_updated_ext: Option<types::Element>,

    /// Profiles this resource claims to conform to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Uri>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// Security Labels applied to this resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<types::Coding>,

    /// Tags applied to this resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<types::Coding>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Meta;

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
