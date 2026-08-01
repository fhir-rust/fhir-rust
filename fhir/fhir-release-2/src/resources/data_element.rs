//! DataElement
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DataElement
//!
//!
//!
//! Resource data element
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DataElement Resource
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::data_element::DataElement;
///
/// let value = DataElement::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: DataElement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DataElement {
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

    /// Globally unique logical id for data element
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Logical id to reference this data element
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Logical id for this version of the data element
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Descriptive label for this element definition
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

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
    pub contact: Vec<DataElementContact>,

    /// Date for this version of the data element
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::CodeableConcept>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::String>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// comparable | fully-specified | equivalent | convertable | scaleable |
    /// flexible
    pub stringency: Option<crate::coded::Coded<crate::r2::codes::DataelementStringency>>,
    /// Primitive extension sibling for [`stringency`](Self::stringency) (FHIR `_stringency`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_stringency")]
    pub stringency_ext: Option<types::Element>,

    /// External specification mapped to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mapping: Vec<DataElementMapping>,

    /// Definition of element
    pub element: ::vec1::Vec1<types::ElementDefinition>,
}

/// Contacts to assist a user in finding and communicating with the publisher.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::data_element::DataElementContact;
/// use fhir::r2::types;
///
/// let value = DataElementContact {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: DataElementContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DataElementContact {
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

/// Identifies a specification (other than a terminology) that the elements
/// which make up the DataElement have some correspondence with.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::data_element::DataElementMapping;
/// use fhir::r2::types;
///
/// let value = DataElementMapping {
///     uri: Some(types::Uri("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `uri` is the name this serializes to on the wire.
/// assert_eq!(json["uri"], ::serde_json::json!("http://example.org"));
///
/// let back: DataElementMapping = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DataElementMapping {
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
