//! StructureDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/StructureDefinition
//!
//!
//!
//! Structural Definition
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for StructureDefinition Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::structure_definition::StructureDefinition;
/// use fhir::r2::types;
///
/// let value = StructureDefinition {
///     fhir_version: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `fhirVersion` is the name this serializes to on the wire.
/// assert_eq!(json["fhirVersion"], ::serde_json::json!("pat-1"));
///
/// let back: StructureDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct StructureDefinition {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Absolute URL used to reference this StructureDefinition
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Other identifiers for the StructureDefinition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Logical id for this version of the StructureDefinition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Informal name for this StructureDefinition
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Use this name when displaying the value
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,

    /// draft | active | retired
    pub status: crate::coded::Coded<crate::r2::codes::ConformanceResourceStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// If for testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Name of the publisher (Organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details of the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<StructureDefinitionContact>,

    /// Date for this version of the StructureDefinition
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Natural language description of the StructureDefinition
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::CodeableConcept>,

    /// Scope and Usage this structure definition is for
    pub requirements: Option<types::String>,
    /// Primitive extension sibling for [`requirements`](Self::requirements) (FHIR `_requirements`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requirements")]
    pub requirements_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::String>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Assist with indexing and finding
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<types::Coding>,

    /// FHIR Version this StructureDefinition targets
    pub fhir_version: Option<types::Id>,
    /// Primitive extension sibling for [`fhir_version`](Self::fhir_version) (FHIR `_fhirVersion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_fhirVersion")]
    pub fhir_version_ext: Option<types::Element>,

    /// External specification that the content is mapped to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mapping: Vec<StructureDefinitionMapping>,

    /// datatype | resource | logical
    pub kind: crate::coded::Coded<crate::r2::codes::StructureDefinitionKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// Any datatype or resource, including abstract ones
    pub constrained_type: Option<types::Code>,
    /// Primitive extension sibling for [`constrained_type`](Self::constrained_type) (FHIR `_constrainedType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_constrainedType")]
    pub constrained_type_ext: Option<types::Element>,

    /// Whether the structure is abstract
    pub r#abstract: types::Boolean,
    /// Primitive extension sibling for [`r#abstract`](Self::r#abstract) (FHIR `_abstract`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_abstract")]
    pub abstract_ext: Option<types::Element>,

    /// resource | datatype | mapping | extension
    pub context_type: Option<crate::coded::Coded<crate::r2::codes::ExtensionContext>>,
    /// Primitive extension sibling for [`context_type`](Self::context_type) (FHIR `_contextType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contextType")]
    pub context_type_ext: Option<types::Element>,

    /// Where the extension can be used in instances
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context: Vec<types::String>,
    /// Primitive extension sibling for [`context`](Self::context) (FHIR `_context`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_context")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_ext: Vec<Option<types::Element>>,

    /// Structure that this set of constraints applies to
    pub base: Option<types::Uri>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,

    /// Snapshot view of the structure
    pub snapshot: Option<StructureDefinitionSnapshot>,

    /// Differential view of the structure
    pub differential: Option<StructureDefinitionDifferential>,
}

/// Contacts to assist a user in finding and communicating with the publisher.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::structure_definition::StructureDefinitionContact;
/// use fhir::r2::types;
///
/// let value = StructureDefinitionContact {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: StructureDefinitionContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct StructureDefinitionContact {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of a individual to contact
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Contact details for individual or publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub telecom: Vec<types::ContactPoint>,
}

/// A differential view is expressed relative to the base StructureDefinition -
/// a statement of differences that it applies.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::structure_definition::StructureDefinitionDifferential;
///
/// let value = StructureDefinitionDifferential::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureDefinitionDifferential = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct StructureDefinitionDifferential {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: ::vec1::Vec1<types::ElementDefinition>,
}

/// An external specification that the content is mapped to.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::structure_definition::StructureDefinitionMapping;
/// use fhir::r2::types;
///
/// let value = StructureDefinitionMapping {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: StructureDefinitionMapping = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct StructureDefinitionMapping {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Internal id when this mapping is used
    pub identity: types::Id,
    /// Primitive extension sibling for [`identity`](Self::identity) (FHIR `_identity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_identity")]
    pub identity_ext: Option<types::Element>,

    /// Identifies what this mapping refers to
    pub uri: Option<types::Uri>,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// Names what this mapping refers to
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Versions, Issues, Scope limitations etc.
    pub comments: Option<types::String>,
    /// Primitive extension sibling for [`comments`](Self::comments) (FHIR `_comments`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comments")]
    pub comments_ext: Option<types::Element>,
}

/// A snapshot view is expressed in a stand alone form that can be used and
/// interpreted without considering the base StructureDefinition.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::structure_definition::StructureDefinitionSnapshot;
///
/// let value = StructureDefinitionSnapshot::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: StructureDefinitionSnapshot = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct StructureDefinitionSnapshot {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: ::vec1::Vec1<types::ElementDefinition>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = StructureDefinition;

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
