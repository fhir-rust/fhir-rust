//! OperationDefinition
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OperationDefinition
//!
//! Version: 5.0.0
//!
//! OperationDefinition Resource: A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction).
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A formal computable definition of an operation (on the RESTful interface) or
/// a named query (using the search interaction).
///
/// OperationDefinition is a canonical, conformance resource that formally and
/// computably describes the inputs, outputs, and behavior of an operation or a
/// named query that a FHIR server can support beyond the standard RESTful
/// interactions. In FHIR R5 it is used to declare custom operations, which are
/// invoked at a `$operation` endpoint, or named queries, which are invoked via
/// the search interaction. The definition specifies each parameter's name, use
/// (input or output), cardinality, data type, and any value set binding, and it
/// records the levels at which the operation may be invoked: system-wide, on a
/// resource type, or on a specific instance. Servers and implementation guides
/// publish OperationDefinition resources so that clients and tooling can
/// discover, validate, and generate code for the operations an endpoint offers,
/// and the resource also indicates whether invoking the operation changes server
/// state.
///
/// See also: [`CapabilityStatement`](crate::r5::resources::capability_statement::CapabilityStatement),
/// which references the operations a server supports, and the general-purpose
/// `Parameters` resource used to carry an operation's actual input and output
/// values at invocation time. Coded parameters bind to value sets via
/// [`CodeableConcept`](crate::r5::types::CodeableConcept)-style terminology.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::operation_definition::OperationDefinition;
/// use fhir::r5::types;
///
/// let value = OperationDefinition {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: OperationDefinition = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "OperationDefinitionDe")]
pub struct OperationDefinition {
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

    /// Canonical identifier for this operation definition, represented as an absolute URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the implementation guide (business identifier)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the operation definition
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `OperationDefinition.versionAlgorithm[x]` choice element (0..1); see [`OperationDefinitionVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<OperationDefinitionVersionAlgorithm>,

    /// Name for this operation definition (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this operation definition (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Publication lifecycle status of this definition: draft, active, retired, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Whether this defines an operation invoked at an endpoint or a named query invoked via search.
    pub kind: crate::r5::coded::Coded<crate::r5::codes::OperationKind>,
    /// Primitive extension sibling for [`kind`](Self::kind) (FHIR `_kind`).
    #[serde(rename = "_kind")]
    pub kind_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`).
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date last changed
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`).
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<types::String>,
    /// Primitive extension sibling for [`publisher`](Self::publisher) (FHIR `_publisher`).
    #[serde(rename = "_publisher")]
    pub publisher_ext: Option<types::Element>,

    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<types::ContactDetail>,

    /// Natural language description of the operation definition
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`).
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`).
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`).
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// Whether content is changed by the operation
    pub affects_state: Option<types::Boolean>,
    /// Primitive extension sibling for [`affects_state`](Self::affects_state) (FHIR `_affectsState`).
    #[serde(rename = "_affectsState")]
    pub affects_state_ext: Option<types::Element>,

    /// Token used to name the operation in the invocation URL, following the dollar-sign prefix.
    pub code: types::Code,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`).
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Additional information about use
    pub comment: Option<types::Markdown>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,

    /// Marks this as a profile of the base
    pub base: Option<types::Canonical>,
    /// Primitive extension sibling for [`base`](Self::base) (FHIR `_base`).
    #[serde(rename = "_base")]
    pub base_ext: Option<types::Element>,

    /// Types this operation applies to
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub resource: ::fhir_core::PrimVec<types::Code>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_ext: Vec<Option<types::Element>>,

    /// Invoke at the system level?
    pub system: types::Boolean,
    /// Primitive extension sibling for [`system`](Self::system) (FHIR `_system`).
    #[serde(rename = "_system")]
    pub system_ext: Option<types::Element>,

    /// Invoke at the type level?
    pub r#type: types::Boolean,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Invoke on an instance?
    pub instance: types::Boolean,
    /// Primitive extension sibling for [`instance`](Self::instance) (FHIR `_instance`).
    #[serde(rename = "_instance")]
    pub instance_ext: Option<types::Element>,

    /// Validation information for in parameters
    pub input_profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`input_profile`](Self::input_profile) (FHIR `_inputProfile`).
    #[serde(rename = "_inputProfile")]
    pub input_profile_ext: Option<types::Element>,

    /// Validation information for out parameters
    pub output_profile: Option<types::Canonical>,
    /// Primitive extension sibling for [`output_profile`](Self::output_profile) (FHIR `_outputProfile`).
    #[serde(rename = "_outputProfile")]
    pub output_profile_ext: Option<types::Element>,

    /// The input and output parameters that define the operation or query interface.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<OperationDefinitionParameter>,

    /// Define overloaded variants for when  generating code
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overload: Vec<OperationDefinitionOverload>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OperationDefinitionDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    url: Option<types::Uri>,
    #[serde(rename = "_url")]
    url_ext: Option<types::Element>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    version: Option<types::String>,
    #[serde(rename = "_version")]
    version_ext: Option<types::Element>,
    #[serde(flatten)]
    version_algorithm: crate::r5::choice::Slot<OperationDefinitionVersionAlgorithm>,
    name: types::String,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    kind: crate::r5::coded::Coded<crate::r5::codes::OperationKind>,
    #[serde(rename = "_kind")]
    kind_ext: Option<types::Element>,
    experimental: Option<types::Boolean>,
    #[serde(rename = "_experimental")]
    experimental_ext: Option<types::Element>,
    date: Option<types::DateTime>,
    #[serde(rename = "_date")]
    date_ext: Option<types::Element>,
    publisher: Option<types::String>,
    #[serde(rename = "_publisher")]
    publisher_ext: Option<types::Element>,
    #[serde(default)]
    contact: Vec<types::ContactDetail>,
    description: Option<types::Markdown>,
    #[serde(rename = "_description")]
    description_ext: Option<types::Element>,
    #[serde(default)]
    use_context: Vec<types::UsageContext>,
    #[serde(default)]
    jurisdiction: Vec<types::CodeableConcept>,
    purpose: Option<types::Markdown>,
    #[serde(rename = "_purpose")]
    purpose_ext: Option<types::Element>,
    copyright: Option<types::Markdown>,
    #[serde(rename = "_copyright")]
    copyright_ext: Option<types::Element>,
    copyright_label: Option<types::String>,
    #[serde(rename = "_copyrightLabel")]
    copyright_label_ext: Option<types::Element>,
    affects_state: Option<types::Boolean>,
    #[serde(rename = "_affectsState")]
    affects_state_ext: Option<types::Element>,
    code: types::Code,
    #[serde(rename = "_code")]
    code_ext: Option<types::Element>,
    comment: Option<types::Markdown>,
    #[serde(rename = "_comment")]
    comment_ext: Option<types::Element>,
    base: Option<types::Canonical>,
    #[serde(rename = "_base")]
    base_ext: Option<types::Element>,
    #[serde(default)]
    resource: ::fhir_core::PrimVec<types::Code>,
    #[serde(rename = "_resource")]
    #[serde(default)]
    resource_ext: Vec<Option<types::Element>>,
    system: types::Boolean,
    #[serde(rename = "_system")]
    system_ext: Option<types::Element>,
    r#type: types::Boolean,
    #[serde(rename = "_type")]
    type_ext: Option<types::Element>,
    instance: types::Boolean,
    #[serde(rename = "_instance")]
    instance_ext: Option<types::Element>,
    input_profile: Option<types::Canonical>,
    #[serde(rename = "_inputProfile")]
    input_profile_ext: Option<types::Element>,
    output_profile: Option<types::Canonical>,
    #[serde(rename = "_outputProfile")]
    output_profile_ext: Option<types::Element>,
    #[serde(default)]
    parameter: Vec<OperationDefinitionParameter>,
    #[serde(default)]
    overload: Vec<OperationDefinitionOverload>,
}

impl ::core::convert::From<OperationDefinitionDe> for OperationDefinition {
    fn from(v: OperationDefinitionDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            url: v.url,
            url_ext: v.url_ext,
            identifier: v.identifier,
            version: v.version,
            version_ext: v.version_ext,
            version_algorithm: v.version_algorithm.0,
            name: v.name,
            name_ext: v.name_ext,
            title: v.title,
            title_ext: v.title_ext,
            status: v.status,
            status_ext: v.status_ext,
            kind: v.kind,
            kind_ext: v.kind_ext,
            experimental: v.experimental,
            experimental_ext: v.experimental_ext,
            date: v.date,
            date_ext: v.date_ext,
            publisher: v.publisher,
            publisher_ext: v.publisher_ext,
            contact: v.contact,
            description: v.description,
            description_ext: v.description_ext,
            use_context: v.use_context,
            jurisdiction: v.jurisdiction,
            purpose: v.purpose,
            purpose_ext: v.purpose_ext,
            copyright: v.copyright,
            copyright_ext: v.copyright_ext,
            copyright_label: v.copyright_label,
            copyright_label_ext: v.copyright_label_ext,
            affects_state: v.affects_state,
            affects_state_ext: v.affects_state_ext,
            code: v.code,
            code_ext: v.code_ext,
            comment: v.comment,
            comment_ext: v.comment_ext,
            base: v.base,
            base_ext: v.base_ext,
            resource: v.resource,
            resource_ext: v.resource_ext,
            system: v.system,
            system_ext: v.system_ext,
            r#type: v.r#type,
            type_ext: v.type_ext,
            instance: v.instance,
            instance_ext: v.instance_ext,
            input_profile: v.input_profile,
            input_profile_ext: v.input_profile_ext,
            output_profile: v.output_profile,
            output_profile_ext: v.output_profile_ext,
            parameter: v.parameter,
            overload: v.overload,
        }
    }
}

/// Parameters for the operation/query.
///
/// Each parameter describes an input or output of the operation, including its
/// name, use (in or out), cardinality, type, and any value set binding.
/// # Examples
///
/// ```
/// use fhir::r5::resources::operation_definition::OperationDefinitionParameter;
/// use fhir::r5::types;
///
/// let value = OperationDefinitionParameter {
///     documentation: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `documentation` is the name this serializes to on the wire.
/// assert_eq!(json["documentation"], ::serde_json::json!("# Heading"));
///
/// let back: OperationDefinitionParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// in | out
    pub r#use: crate::r5::coded::Coded<crate::r5::codes::OperationParameterUse>,
    /// Primitive extension sibling for [`use`](Self::r#use) (FHIR `_use`).
    #[serde(rename = "_use")]
    pub use_ext: Option<types::Element>,

    /// instance | type | system
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub scope:
        ::fhir_core::PrimVec<crate::r5::coded::Coded<crate::r5::codes::OperationParameterScope>>,
    /// Primitive extension sibling for [`scope`](Self::scope) (FHIR `_scope`).
    #[serde(rename = "_scope")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope_ext: Vec<Option<types::Element>>,

    /// Minimum Cardinality
    pub min: types::Integer,
    /// Primitive extension sibling for [`min`](Self::min) (FHIR `_min`).
    #[serde(rename = "_min")]
    pub min_ext: Option<types::Element>,

    /// Maximum Cardinality (a number or *)
    pub max: types::String,
    /// Primitive extension sibling for [`max`](Self::max) (FHIR `_max`).
    #[serde(rename = "_max")]
    pub max_ext: Option<types::Element>,

    /// Description of meaning/use
    pub documentation: Option<types::Markdown>,
    /// Primitive extension sibling for [`documentation`](Self::documentation) (FHIR `_documentation`).
    #[serde(rename = "_documentation")]
    pub documentation_ext: Option<types::Element>,

    /// What type this parameter has
    pub r#type: Option<types::Code>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// Allowed sub-type this parameter can have (if type is abstract)
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub allowed_type: ::fhir_core::PrimVec<crate::r5::coded::Coded<crate::r5::codes::FhirTypes>>,
    /// Primitive extension sibling for [`allowed_type`](Self::allowed_type) (FHIR `_allowedType`).
    #[serde(rename = "_allowedType")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_type_ext: Vec<Option<types::Element>>,

    /// If type is Reference | canonical, allowed targets. If type is 'Resource', then this constrains the allowed resource types
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub target_profile: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`target_profile`](Self::target_profile) (FHIR `_targetProfile`).
    #[serde(rename = "_targetProfile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_profile_ext: Vec<Option<types::Element>>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    pub search_type: Option<crate::r5::coded::Coded<crate::r5::codes::SearchParamType>>,
    /// Primitive extension sibling for [`search_type`](Self::search_type) (FHIR `_searchType`).
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

/// ValueSet details if this is coded.
///
/// Binds the parameter to a value set with a given strength, applicable when
/// the parameter is a coded data type.
/// # Examples
///
/// ```
/// use fhir::r5::resources::operation_definition::OperationDefinitionParameterBinding;
/// use fhir::r5::types;
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
    pub strength: crate::r5::coded::Coded<crate::r5::codes::BindingStrength>,
    /// Primitive extension sibling for [`strength`](Self::strength) (FHIR `_strength`).
    #[serde(rename = "_strength")]
    pub strength_ext: Option<types::Element>,

    /// Source of value set
    pub value_set: types::Canonical,
    /// Primitive extension sibling for [`value_set`](Self::value_set) (FHIR `_valueSet`).
    #[serde(rename = "_valueSet")]
    pub value_set_ext: Option<types::Element>,
}

/// References to this parameter.
///
/// Identifies other parameters that reference this parameter, used to describe
/// relationships between parameters (for example, a search parameter that
/// references a target parameter).
/// # Examples
///
/// ```
/// use fhir::r5::resources::operation_definition::OperationDefinitionParameterReferencedFrom;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`source`](Self::source) (FHIR `_source`).
    #[serde(rename = "_source")]
    pub source_ext: Option<types::Element>,

    /// Element id of reference
    pub source_id: Option<types::String>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`).
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,
}

/// Define overloaded variants for when generating code.
///
/// Describes a named combination of parameters that together form a valid
/// invocation, used primarily to guide code generation for client libraries.
/// # Examples
///
/// ```
/// use fhir::r5::resources::operation_definition::OperationDefinitionOverload;
/// use fhir::r5::types;
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
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub parameter_name: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`parameter_name`](Self::parameter_name) (FHIR `_parameterName`).
    #[serde(rename = "_parameterName")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_name_ext: Vec<Option<types::Element>>,

    /// Comments to go on overload
    pub comment: Option<types::String>,
    /// Primitive extension sibling for [`comment`](Self::comment) (FHIR `_comment`).
    #[serde(rename = "_comment")]
    pub comment_ext: Option<types::Element>,
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
/// The `OperationDefinition.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum OperationDefinitionVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
