//! Meta
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Meta
//!
//! Version: 5.0.0
//!
//! Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// The metadata about a resource. This is content in the resource that is
/// maintained by the infrastructure. Changes to the content might not always be
/// associated with version changes to the resource. In FHIR R5, `Meta` carries
/// system-managed information such as the version identifier, the last-updated
/// timestamp, the source of the resource, conformance profiles, and security and
/// tag labels.
///
/// # Examples
///
/// ```
/// use fhir::r5::types::meta::Meta;
/// use fhir::r5::types;
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
pub struct Meta {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Version specific identifier
    pub version_id: Option<types::Id>,
    /// Primitive extension sibling for [`version_id`](Self::version_id) (FHIR `_versionId`).
    #[serde(rename = "_versionId")]
    pub version_id_ext: Option<types::Element>,

    /// When the resource version last changed
    pub last_updated: Option<types::Instant>,
    /// Primitive extension sibling for [`last_updated`](Self::last_updated) (FHIR `_lastUpdated`).
    #[serde(rename = "_lastUpdated")]
    pub last_updated_ext: Option<types::Element>,

    /// Identifies where the resource comes from
    pub source: Option<types::Uri>,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`).
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Profiles this resource claims to conform to
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub profile: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
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
