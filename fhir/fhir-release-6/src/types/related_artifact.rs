//! RelatedArtifact
//!
//! URL: http://hl7.org/fhir/StructureDefinition/RelatedArtifact
//!
//! Version: 6.0.0-ballot3
//!
//! Related artifacts for a knowledge resource
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// RelatedArtifact Type: Related artifacts such as additional documentation,
/// justification, or bibliographic references.
///
/// # Examples
///
/// ```
/// use fhir::r6::types::related_artifact::RelatedArtifact;
/// use fhir::r6::types;
///
/// let value = RelatedArtifact {
///     publication_date: Some(types::Date("2019-11-01".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `publicationDate` is the name this serializes to on the wire.
/// assert_eq!(json["publicationDate"], ::serde_json::json!("2019-11-01"));
///
/// let back: RelatedArtifact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct RelatedArtifact {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// documentation | justification | citation | predecessor | successor |
    /// derived-from | depends-on | composed-of | part-of | amends |
    /// amended-with | appends | appended-with | cites | cited-by | comments-on
    /// | comment-in | contains | contained-in | corrects | correction-in |
    /// replaces | replaced-with | retracts | retracted-by | signs | similar-to
    /// | supports | supported-with | transforms | transformed-into |
    /// transformed-with | documents | specification-of | created-with |
    /// cite-as
    pub r#type: crate::coded::Coded<crate::r6::codes::RelatedArtifactType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Additional classifiers
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifier: Vec<types::CodeableConcept>,

    /// Short label
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Brief description of the related artifact
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// Bibliographic citation for the artifact
    pub citation: Option<types::Markdown>,
    /// Primitive extension sibling for [`citation`](Self::citation) (FHIR `_citation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_citation")]
    pub citation_ext: Option<types::Element>,

    /// What document is being referenced
    pub document: Option<types::Attachment>,

    /// What artifact is being referenced
    pub resource: Option<types::Canonical>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// What artifact, if not a conformance resource
    pub resource_reference: Option<types::Reference>,

    /// draft | active | retired | unknown
    pub publication_status: Option<crate::coded::Coded<crate::r6::codes::PublicationStatus>>,
    /// Primitive extension sibling for [`publication_status`](Self::publication_status) (FHIR `_publicationStatus`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicationStatus")]
    pub publication_status_ext: Option<types::Element>,

    /// Date of publication of the artifact being referred to
    pub publication_date: Option<types::Date>,
    /// Primitive extension sibling for [`publication_date`](Self::publication_date) (FHIR `_publicationDate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publicationDate")]
    pub publication_date_ext: Option<types::Element>,
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
