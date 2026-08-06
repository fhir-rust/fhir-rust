//! OperationDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OperationDefinition
//!
//! Version: 4.0.1
//!
//! Definition of an operation or a named query
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A formal computable definition of an operation (on the RESTful interface)
/// or a named query (using the search interaction).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::operation_definition::OperationDefinition;
/// use fhir::r4::types;
///
/// let value = OperationDefinition {
///     affects_state: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `affectsState` is the name this serializes to on the wire.
/// assert_eq!(json["affectsState"], ::serde_json::json!(true));
///
/// let back: OperationDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct OperationDefinition {
    /// Logical id of this artifact
    pub id: Option<types::String>,

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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this operation definition, represented as a
    /// URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business version of the operation definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this operation definition (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this operation definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r4::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// operation | query
    pub kind: crate::coded::Coded<crate::r4::codes::OperationKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the operation definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for operation definition (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this operation definition is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Whether content is changed by the operation
    pub affects_state: Option<types::Boolean>,
    /// Primitive extension sibling for [`affects_state`](Self::affects_state) (FHIR `_affectsState`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_affectsState")]
    pub affects_state_ext: Option<types::Element>,

    /// Name used to invoke the operation
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Additional information about use
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Marks this as a profile of the base
    pub base: Option<types::Canonical>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,

    /// Types this operation applies to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<crate::coded::Coded<crate::r4::codes::ResourceTypes>>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_ext: Vec<Option<types::Element>>,

    /// Invoke at the system level?
    pub system: types::Boolean,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Invoke at the type level?
    pub r#type: types::Boolean,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Invoke on an instance?
    pub instance: types::Boolean,
    /// Primitive extension sibling for [`instance`](Self::instance) (FHIR `_instance`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instance")]
    pub instance_ext: Option<types::Element>,

    /// Validation information for in parameters
    pub input_profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`input_profile`](Self::input_profile) (FHIR `_inputProfile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_inputProfile")]
    pub input_profile_ext: Option<types::Element>,

    /// Validation information for out parameters
    pub output_profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`output_profile`](Self::output_profile) (FHIR `_outputProfile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_outputProfile")]
    pub output_profile_ext: Option<types::Element>,

    /// Parameters for the operation/query
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<OperationDefinitionParameter>,

    /// Define overloaded variants for when generating code
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overload: Vec<OperationDefinitionOverload>,
}

/// Defines an appropriate combination of parameters to use when invoking this
/// operation, to help code generators when generating overloaded parameter
/// sets for this operation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::operation_definition::OperationDefinitionOverload;
/// use fhir::r4::types;
///
/// let value = OperationDefinitionOverload {
///     comment: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `comment` is the name this serializes to on the wire.
/// assert_eq!(json["comment"], ::serde_json::json!("abc"));
///
/// let back: OperationDefinitionOverload = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct OperationDefinitionOverload {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of parameter to include in overload
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_name: Vec<types::String>,
    /// Primitive extension sibling for [`parameter_name`](Self::parameter_name) (FHIR `_parameterName`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_parameterName")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_name_ext: Vec<Option<types::Element>>,

    /// Comments to go on overload
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
}

/// The parameters for the operation/query.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::operation_definition::OperationDefinitionParameter;
/// use fhir::r4::types;
///
/// let value = OperationDefinitionParameter {
///     r#type: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `type` is the name this serializes to on the wire.
/// assert_eq!(json["type"], ::serde_json::json!("final"));
///
/// let back: OperationDefinitionParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct OperationDefinitionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name in Parameters.parameter.name or in URL
    pub name: types::Code,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// in | out
    pub r#use: crate::coded::Coded<crate::r4::codes::OperationParameterUse>,
    /// Primitive extension sibling for [`r#use`](Self::r#use) (FHIR `_use`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// Minimum Cardinality
    pub min: types::Integer,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Maximum Cardinality (a number or *)
    pub max: types::String,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,

    /// Description of meaning/use
    pub documentation: Option<types::String>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// What type this parameter has
    pub r#type: Option<types::Code>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// If type is Reference | canonical, allowed targets
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_profile: Vec<types::Canonical>,
    /// Primitive extension sibling for [`target_profile`](Self::target_profile) (FHIR `_targetProfile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetProfile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_profile_ext: Vec<Option<types::Element>>,

    /// number | date | string | token | reference | composite | quantity | uri
    /// | special
    pub search_type: Option<crate::coded::Coded<crate::r4::codes::SearchParamType>>,
    /// Primitive extension sibling for [`search_type`](Self::search_type) (FHIR `_searchType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_searchType")]
    pub search_type_ext: Option<types::Element>,

    /// ValueSet details if this is coded
    pub binding: Option<OperationDefinitionParameterBinding>,

    /// References to this parameter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referenced_from: Vec<OperationDefinitionParameterReferencedFrom>,

    /// Parts of a nested Parameter
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<OperationDefinitionParameter>,
}

/// Binds to a value set if this parameter is coded (code, Coding,
/// CodeableConcept).
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::operation_definition::OperationDefinitionParameterBinding;
/// use fhir::r4::types;
///
/// let value = OperationDefinitionParameterBinding {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: OperationDefinitionParameterBinding = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct OperationDefinitionParameterBinding {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// required | extensible | preferred | example
    pub strength: crate::coded::Coded<crate::r4::codes::BindingStrength>,
    /// Primitive extension sibling for [`strength`](Self::strength) (FHIR `_strength`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_strength")]
    pub strength_ext: Option<types::Element>,

    /// Source of value set
    pub value_set: types::Canonical,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,
}

/// Identifies other resource parameters within the operation invocation that
/// are expected to resolve to this resource.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::operation_definition::OperationDefinitionParameterReferencedFrom;
/// use fhir::r4::types;
///
/// let value = OperationDefinitionParameterReferencedFrom {
///     source_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sourceId` is the name this serializes to on the wire.
/// assert_eq!(json["sourceId"], ::serde_json::json!("abc"));
///
/// let back: OperationDefinitionParameterReferencedFrom = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct OperationDefinitionParameterReferencedFrom {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Referencing parameter
    pub source: types::String,
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Element id of reference
    pub source_id: Option<types::String>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = OperationDefinition;

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
