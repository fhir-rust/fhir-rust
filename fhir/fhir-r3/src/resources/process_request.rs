//! ProcessRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ProcessRequest
//!
//!
//!
//! Request to perform some action on or in regards to an existing resource
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ProcessRequest Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::process_request::ProcessRequest;
/// use fhir::r3::types;
///
/// let value = ProcessRequest {
///     created: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `created` is the name this serializes to on the wire.
/// assert_eq!(json["created"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ProcessRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ProcessRequest {
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

    /// Business Identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<crate::coded::Coded<crate::r3::codes::FmStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// cancel | poll | reprocess | status
    pub action: Option<crate::coded::Coded<crate::r3::codes::Actionlist>>,
    /// Primitive extension sibling for [`action`](Self::action) (FHIR `_action`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_action")]
    pub action_ext: Option<types::Element>,

    /// Party which is the target of the request
    pub target: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Creation date
    pub created: Option<types::DateTime>,
    /// Primitive extension sibling for [`created`](Self::created) (FHIR `_created`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_created")]
    pub created_ext: Option<types::Element>,

    /// Responsible practitioner
    pub provider: Option<types::Reference<crate::r3::resources::Practitioner>>,

    /// Responsible organization
    pub organization: Option<types::Reference<crate::r3::resources::Organization>>,

    /// Reference to the Request resource
    pub request: Option<types::Reference>,

    /// Reference to the Response resource
    pub response: Option<types::Reference>,

    /// Remove history
    pub nullify: Option<types::Boolean>,
    /// Primitive extension sibling for [`nullify`](Self::nullify) (FHIR `_nullify`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_nullify")]
    pub nullify_ext: Option<types::Element>,

    /// Reference number/string
    pub reference: Option<types::String>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Items to re-adjudicate
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<ProcessRequestItem>,

    /// Resource type(s) to include
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<types::String>,
    /// Primitive extension sibling for [`include`](Self::include) (FHIR `_include`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_include")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include_ext: Vec<Option<types::Element>>,

    /// Resource type(s) to exclude
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<types::String>,
    /// Primitive extension sibling for [`exclude`](Self::exclude) (FHIR `_exclude`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exclude")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_ext: Vec<Option<types::Element>>,

    /// Selection period
    pub period: Option<types::Period>,
}

/// List of top level items to be re-adjudicated, if none specified then the
/// entire submission is re-adjudicated.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::process_request::ProcessRequestItem;
/// use fhir::r3::types;
///
/// let value = ProcessRequestItem {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ProcessRequestItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct ProcessRequestItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Service instance
    pub sequence_link_id: types::Integer,
    /// Primitive extension sibling for [`sequence_link_id`](Self::sequence_link_id) (FHIR `_sequenceLinkId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequenceLinkId")]
    pub sequence_link_id_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ProcessRequest;

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
