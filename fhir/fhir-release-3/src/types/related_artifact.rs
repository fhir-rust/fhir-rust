//! RelatedArtifact
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RelatedArtifact
//!
//!
//!
//! Related artifacts for a knowledge resource
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for RelatedArtifact Type
///
/// # Examples
///
/// ```
/// use fhir::r3::types::related_artifact::RelatedArtifact;
/// use fhir::r3::types;
///
/// let value = RelatedArtifact {
///     display: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `display` is the name this serializes to on the wire.
/// assert_eq!(json["display"], ::serde_json::json!("abc"));
///
/// let back: RelatedArtifact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct RelatedArtifact {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// documentation | justification | citation | predecessor | successor |
    /// derived-from | depends-on | composed-of
    pub r#type: crate::coded::Coded<crate::r3::codes::RelatedArtifactType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Brief description of the related artifact
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Bibliographic citation for the artifact
    pub citation: Option<types::String>,
    /// Primitive extension sibling for [`citation`](Self::citation) (FHIR `_citation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_citation")]
    pub citation_ext: Option<types::Element>,

    /// Where the artifact can be accessed
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// What document is being referenced
    pub document: Option<types::Attachment>,

    /// What resource is being referenced
    pub resource: Option<types::Reference>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = RelatedArtifact;

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
