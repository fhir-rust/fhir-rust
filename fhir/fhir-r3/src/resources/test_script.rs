//! TestScript
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestScript
//!
//!
//!
//! Describes a set of tests
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for TestScript Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScript;
/// use fhir::r3::types;
///
/// let value = TestScript {
///     version: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `version` is the name this serializes to on the wire.
/// assert_eq!(json["version"], ::serde_json::json!("abc"));
///
/// let back: TestScript = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScript {
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

    /// Logical URI to reference this test script (globally unique)
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the test script
    pub identifier: Option<types::Identifier>,

    /// Business version of the test script
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Name for this test script (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this test script (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// draft | active | retired | unknown
    pub status: crate::coded::Coded<crate::r3::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing purposes, not real usage
    pub experimental: Option<types::Boolean>,
    /// Primitive extension sibling for [`experimental`](Self::experimental) (FHIR `_experimental`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_experimental")]
    pub experimental_ext: Option<types::Element>,

    /// Date this was last changed
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

    /// Natural language description of the test script
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Context the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction for test script (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this test script is defined
    pub purpose: Option<types::Markdown>,
    /// Primitive extension sibling for [`purpose`](Self::purpose) (FHIR `_purpose`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_purpose")]
    pub purpose_ext: Option<types::Element>,

    /// Use and/or publishing restrictions
    pub copyright: Option<types::Markdown>,
    /// Primitive extension sibling for [`copyright`](Self::copyright) (FHIR `_copyright`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyright")]
    pub copyright_ext: Option<types::Element>,

    /// An abstract server representing a client or sender in a message
    /// exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<TestScriptOrigin>,

    /// An abstract server representing a destination or receiver in a message
    /// exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<TestScriptDestination>,

    /// Required capability that is assumed to function correctly on the FHIR
    /// server being tested
    pub metadata: Option<TestScriptMetadata>,

    /// Fixture in the test script - by reference (uri)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixture: Vec<TestScriptFixture>,

    /// Reference of the validation profile
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Reference>,

    /// Placeholder for evaluated elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable: Vec<TestScriptVariable>,

    /// Assert rule used within the test script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<TestScriptRule>,

    /// Assert ruleset used within the test script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ruleset: Vec<TestScriptRuleset>,

    /// A series of required setup operations before tests are executed
    pub setup: Option<TestScriptSetup>,

    /// A test in this script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test: Vec<TestScriptTest>,

    /// A series of required clean up steps
    pub teardown: Option<TestScriptTeardown>,
}

/// An abstract server used in operations within this test script in the
/// destination element.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptDestination;
/// use fhir::r3::types;
///
/// let value = TestScriptDestination {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptDestination = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptDestination {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The index of the abstract destination server starting at 1
    pub index: types::Integer,
    /// Primitive extension sibling for [`index`](Self::index) (FHIR `_index`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_index")]
    pub index_ext: Option<types::Element>,

    /// FHIR-Server | FHIR-SDC-FormManager | FHIR-SDC-FormReceiver |
    /// FHIR-SDC-FormProcessor
    pub profile: types::Coding,
}

/// Fixture in the test script - by reference (uri). All fixtures are required
/// for the test script to execute.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptFixture;
/// use fhir::r3::types;
///
/// let value = TestScriptFixture {
///     autocreate: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `autocreate` is the name this serializes to on the wire.
/// assert_eq!(json["autocreate"], ::serde_json::json!(true));
///
/// let back: TestScriptFixture = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptFixture {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether or not to implicitly create the fixture during setup
    pub autocreate: Option<types::Boolean>,
    /// Primitive extension sibling for [`autocreate`](Self::autocreate) (FHIR `_autocreate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_autocreate")]
    pub autocreate_ext: Option<types::Element>,

    /// Whether or not to implicitly delete the fixture during teardown
    pub autodelete: Option<types::Boolean>,
    /// Primitive extension sibling for [`autodelete`](Self::autodelete) (FHIR `_autodelete`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_autodelete")]
    pub autodelete_ext: Option<types::Element>,

    /// Reference of the resource
    pub resource: Option<types::Reference>,
}

/// The required capability must exist and are assumed to function correctly on
/// the FHIR server being tested.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::test_script::TestScriptMetadata;
///
/// let value = TestScriptMetadata::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptMetadata = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptMetadata {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Links to the FHIR specification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<TestScriptMetadataLink>,

    /// Capabilities that are assumed to function correctly on the FHIR server
    /// being tested
    pub capability: ::vec1::Vec1<TestScriptMetadataCapability>,
}

/// Capabilities that must exist and are assumed to function correctly on the
/// FHIR server being tested.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptMetadataCapability;
/// use fhir::r3::types;
///
/// let value = TestScriptMetadataCapability {
///     required: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `required` is the name this serializes to on the wire.
/// assert_eq!(json["required"], ::serde_json::json!(true));
///
/// let back: TestScriptMetadataCapability = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptMetadataCapability {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Are the capabilities required?
    pub required: Option<types::Boolean>,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// Are the capabilities validated?
    pub validated: Option<types::Boolean>,
    /// Primitive extension sibling for [`validated`](Self::validated) (FHIR `_validated`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_validated")]
    pub validated_ext: Option<types::Element>,

    /// The expected capabilities of the server
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Which origin server these requirements apply to
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub origin: ::fhir_core::PrimVec<types::Integer>,
    /// Primitive extension sibling for [`origin`](Self::origin) (FHIR `_origin`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_origin")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin_ext: Vec<Option<types::Element>>,

    /// Which server these requirements apply to
    pub destination: Option<types::Integer>,
    /// Primitive extension sibling for [`destination`](Self::destination) (FHIR `_destination`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_destination")]
    pub destination_ext: Option<types::Element>,

    /// Links to the FHIR specification
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub link: ::fhir_core::PrimVec<types::Uri>,
    /// Primitive extension sibling for [`link`](Self::link) (FHIR `_link`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_link")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_ext: Vec<Option<types::Element>>,

    /// Required Capability Statement
    pub capabilities: types::Reference<crate::r3::resources::CapabilityStatement>,
}

/// A link to the FHIR specification that this test is covering.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptMetadataLink;
/// use fhir::r3::types;
///
/// let value = TestScriptMetadataLink {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: TestScriptMetadataLink = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptMetadataLink {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// URL to the specification
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Short description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// An abstract server used in operations within this test script in the origin
/// element.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptOrigin;
/// use fhir::r3::types;
///
/// let value = TestScriptOrigin {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptOrigin = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptOrigin {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The index of the abstract origin server starting at 1
    pub index: types::Integer,
    /// Primitive extension sibling for [`index`](Self::index) (FHIR `_index`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_index")]
    pub index_ext: Option<types::Element>,

    /// FHIR-Client | FHIR-SDC-FormFiller
    pub profile: types::Coding,
}

/// Assert rule to be used in one or more asserts within the test script.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptRule;
/// use fhir::r3::types;
///
/// let value = TestScriptRule {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptRule {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Assert rule resource reference
    pub resource: types::Reference,

    /// Rule parameter template
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub param: Vec<TestScriptRuleParam>,
}

/// Each rule template can take one or more parameters for rule evaluation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptRuleParam;
/// use fhir::r3::types;
///
/// let value = TestScriptRuleParam {
///     value: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `value` is the name this serializes to on the wire.
/// assert_eq!(json["value"], ::serde_json::json!("abc"));
///
/// let back: TestScriptRuleParam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptRuleParam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Parameter name matching external assert rule parameter
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Parameter value defined either explicitly or dynamically
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// Contains one or more rules. Offers a way to group rules so assertions could
/// reference the group of rules and have them all applied.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::test_script::TestScriptRuleset;
///
/// let value = TestScriptRuleset::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptRuleset = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptRuleset {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Assert ruleset resource reference
    pub resource: types::Reference,

    /// The referenced rule within the ruleset
    pub rule: ::vec1::Vec1<TestScriptRulesetRule>,
}

/// The referenced rule within the external ruleset template.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptRulesetRule;
/// use fhir::r3::types;
///
/// let value = TestScriptRulesetRule {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptRulesetRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptRulesetRule {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id of referenced rule within the ruleset
    pub rule_id: types::Id,
    /// Primitive extension sibling for [`rule_id`](Self::rule_id) (FHIR `_ruleId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ruleId")]
    pub rule_id_ext: Option<types::Element>,

    /// Ruleset rule parameter template
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub param: Vec<TestScriptRulesetRuleParam>,
}

/// Each rule template can take one or more parameters for rule evaluation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptRulesetRuleParam;
/// use fhir::r3::types;
///
/// let value = TestScriptRulesetRuleParam {
///     value: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `value` is the name this serializes to on the wire.
/// assert_eq!(json["value"], ::serde_json::json!("abc"));
///
/// let back: TestScriptRulesetRuleParam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptRulesetRuleParam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Parameter name matching external assert ruleset rule parameter
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Parameter value defined either explicitly or dynamically
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A series of required setup operations before tests are executed.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::test_script::TestScriptSetup;
///
/// let value = TestScriptSetup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptSetup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A setup operation or assert to perform
    pub action: ::vec1::Vec1<TestScriptSetupAction>,
}

/// Action would contain either an operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupAction;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// Evaluates the results of previous operations to determine if the server
/// under test behaves appropriately.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionAssert;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionAssert {
///     compare_to_source_id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `compareToSourceId` is the name this serializes to on the wire.
/// assert_eq!(json["compareToSourceId"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionAssert = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionAssert {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Tracking/logging assertion label
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Tracking/reporting assertion description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// response | request
    pub direction: Option<crate::coded::Coded<crate::r3::codes::AssertDirectionCodes>>,
    /// Primitive extension sibling for [`direction`](Self::direction) (FHIR `_direction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_direction")]
    pub direction_ext: Option<types::Element>,

    /// Id of the source fixture to be evaluated
    pub compare_to_source_id: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_id`](Self::compare_to_source_id) (FHIR `_compareToSourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compareToSourceId")]
    pub compare_to_source_id_ext: Option<types::Element>,

    /// The fluentpath expression to evaluate against the source fixture
    pub compare_to_source_expression: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_expression`](Self::compare_to_source_expression) (FHIR `_compareToSourceExpression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compareToSourceExpression")]
    pub compare_to_source_expression_ext: Option<types::Element>,

    /// XPath or JSONPath expression to evaluate against the source fixture
    pub compare_to_source_path: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_path`](Self::compare_to_source_path) (FHIR `_compareToSourcePath`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compareToSourcePath")]
    pub compare_to_source_path_ext: Option<types::Element>,

    /// xml | json | ttl | none
    pub content_type: Option<crate::coded::Coded<crate::r3::codes::ContentType>>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// The fluentpath expression to be evaluated
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// HTTP header field name
    pub header_field: Option<types::String>,
    /// Primitive extension sibling for [`header_field`](Self::header_field) (FHIR `_headerField`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_headerField")]
    pub header_field_ext: Option<types::Element>,

    /// Fixture Id of minimum content resource
    pub minimum_id: Option<types::String>,
    /// Primitive extension sibling for [`minimum_id`](Self::minimum_id) (FHIR `_minimumId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_minimumId")]
    pub minimum_id_ext: Option<types::Element>,

    /// Perform validation on navigation links?
    pub navigation_links: Option<types::Boolean>,
    /// Primitive extension sibling for [`navigation_links`](Self::navigation_links) (FHIR `_navigationLinks`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_navigationLinks")]
    pub navigation_links_ext: Option<types::Element>,

    /// equals | notEquals | in | notIn | greaterThan | lessThan | empty |
    /// notEmpty | contains | notContains | eval
    pub operator: Option<crate::coded::Coded<crate::r3::codes::AssertOperatorCodes>>,
    /// Primitive extension sibling for [`operator`](Self::operator) (FHIR `_operator`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_operator")]
    pub operator_ext: Option<types::Element>,

    /// XPath or JSONPath expression
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// delete | get | options | patch | post | put
    pub request_method: Option<crate::coded::Coded<crate::r3::codes::HttpOperations>>,
    /// Primitive extension sibling for [`request_method`](Self::request_method) (FHIR `_requestMethod`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requestMethod")]
    pub request_method_ext: Option<types::Element>,

    /// Request URL comparison value
    #[serde(rename = "requestURL")]
    pub request_url: Option<types::String>,
    /// Primitive extension sibling for [`request_url`](Self::request_url) (FHIR `_requestURL`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requestURL")]
    pub request_url_ext: Option<types::Element>,

    /// Resource type
    pub resource: Option<types::Code>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// okay | created | noContent | notModified | bad | forbidden | notFound |
    /// methodNotAllowed | conflict | gone | preconditionFailed | unprocessable
    pub response: Option<crate::coded::Coded<crate::r3::codes::AssertResponseCodeTypes>>,
    /// Primitive extension sibling for [`response`](Self::response) (FHIR `_response`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_response")]
    pub response_ext: Option<types::Element>,

    /// HTTP response code to test
    pub response_code: Option<types::String>,
    /// Primitive extension sibling for [`response_code`](Self::response_code) (FHIR `_responseCode`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_responseCode")]
    pub response_code_ext: Option<types::Element>,

    /// The reference to a TestScript.rule
    pub rule: Option<TestScriptSetupActionAssertRule>,

    /// The reference to a TestScript.ruleset
    pub ruleset: Option<TestScriptSetupActionAssertRuleset>,

    /// Fixture Id of source expression or headerField
    pub source_id: Option<types::Id>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,

    /// Profile Id of validation profile reference
    pub validate_profile_id: Option<types::Id>,
    /// Primitive extension sibling for [`validate_profile_id`](Self::validate_profile_id) (FHIR `_validateProfileId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_validateProfileId")]
    pub validate_profile_id_ext: Option<types::Element>,

    /// The value to compare to
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Will this assert produce a warning only on error?
    pub warning_only: Option<types::Boolean>,
    /// Primitive extension sibling for [`warning_only`](Self::warning_only) (FHIR `_warningOnly`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_warningOnly")]
    pub warning_only_ext: Option<types::Element>,
}

/// The TestScript.rule this assert will evaluate.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionAssertRule;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionAssertRule {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionAssertRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionAssertRule {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id of the TestScript.rule
    pub rule_id: types::Id,
    /// Primitive extension sibling for [`rule_id`](Self::rule_id) (FHIR `_ruleId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ruleId")]
    pub rule_id_ext: Option<types::Element>,

    /// Rule parameter template
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub param: Vec<TestScriptSetupActionAssertRuleParam>,
}

/// Each rule template can take one or more parameters for rule evaluation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionAssertRuleParam;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionAssertRuleParam {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionAssertRuleParam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionAssertRuleParam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Parameter name matching external assert rule parameter
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Parameter value defined either explicitly or dynamically
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The TestScript.ruleset this assert will evaluate.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionAssertRuleset;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionAssertRuleset {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionAssertRuleset = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionAssertRuleset {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id of the TestScript.ruleset
    pub ruleset_id: types::Id,
    /// Primitive extension sibling for [`ruleset_id`](Self::ruleset_id) (FHIR `_rulesetId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_rulesetId")]
    pub ruleset_id_ext: Option<types::Element>,

    /// The referenced rule within the ruleset
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rule: Vec<TestScriptSetupActionAssertRulesetRule>,
}

/// The referenced rule within the external ruleset template.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionAssertRulesetRule;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionAssertRulesetRule {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionAssertRulesetRule = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionAssertRulesetRule {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Id of referenced rule within the ruleset
    pub rule_id: types::Id,
    /// Primitive extension sibling for [`rule_id`](Self::rule_id) (FHIR `_ruleId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_ruleId")]
    pub rule_id_ext: Option<types::Element>,

    /// Rule parameter template
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub param: Vec<TestScriptSetupActionAssertRulesetRuleParam>,
}

/// Each rule template can take one or more parameters for rule evaluation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionAssertRulesetRuleParam;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionAssertRulesetRuleParam {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionAssertRulesetRuleParam = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionAssertRulesetRuleParam {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Parameter name matching external assert ruleset rule parameter
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Parameter value defined either explicitly or dynamically
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The operation to perform.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionOperation;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionOperation {
///     encode_request_url: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `encodeRequestUrl` is the name this serializes to on the wire.
/// assert_eq!(json["encodeRequestUrl"], ::serde_json::json!(true));
///
/// let back: TestScriptSetupActionOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionOperation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The operation code type that will be executed
    pub r#type: Option<types::Coding>,

    /// Resource type
    pub resource: Option<types::Code>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// Tracking/logging operation label
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Tracking/reporting operation description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// xml | json | ttl | none
    pub accept: Option<crate::coded::Coded<crate::r3::codes::ContentType>>,
    /// Primitive extension sibling for [`accept`](Self::accept) (FHIR `_accept`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_accept")]
    pub accept_ext: Option<types::Element>,

    /// xml | json | ttl | none
    pub content_type: Option<crate::coded::Coded<crate::r3::codes::ContentType>>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// Server responding to the request
    pub destination: Option<types::Integer>,
    /// Primitive extension sibling for [`destination`](Self::destination) (FHIR `_destination`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_destination")]
    pub destination_ext: Option<types::Element>,

    /// Whether or not to send the request url in encoded format
    pub encode_request_url: Option<types::Boolean>,
    /// Primitive extension sibling for [`encode_request_url`](Self::encode_request_url) (FHIR `_encodeRequestUrl`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_encodeRequestUrl")]
    pub encode_request_url_ext: Option<types::Element>,

    /// Server initiating the request
    pub origin: Option<types::Integer>,
    /// Primitive extension sibling for [`origin`](Self::origin) (FHIR `_origin`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_origin")]
    pub origin_ext: Option<types::Element>,

    /// Explicitly defined path parameters
    pub params: Option<types::String>,
    /// Primitive extension sibling for [`params`](Self::params) (FHIR `_params`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_params")]
    pub params_ext: Option<types::Element>,

    /// Each operation can have one or more header elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request_header: Vec<TestScriptSetupActionOperationRequestHeader>,

    /// Fixture Id of mapped request
    pub request_id: Option<types::Id>,
    /// Primitive extension sibling for [`request_id`](Self::request_id) (FHIR `_requestId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_requestId")]
    pub request_id_ext: Option<types::Element>,

    /// Fixture Id of mapped response
    pub response_id: Option<types::Id>,
    /// Primitive extension sibling for [`response_id`](Self::response_id) (FHIR `_responseId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_responseId")]
    pub response_id_ext: Option<types::Element>,

    /// Fixture Id of body for PUT and POST requests
    pub source_id: Option<types::Id>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,

    /// Id of fixture used for extracting the [id], [type], and [vid] for GET
    /// requests
    pub target_id: Option<types::Id>,
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// Request URL
    pub url: Option<types::String>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// Header elements would be used to set HTTP headers.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptSetupActionOperationRequestHeader;
/// use fhir::r3::types;
///
/// let value = TestScriptSetupActionOperationRequestHeader {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionOperationRequestHeader = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptSetupActionOperationRequestHeader {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// HTTP header field name
    pub field: types::String,
    /// Primitive extension sibling for [`field`](Self::field) (FHIR `_field`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_field")]
    pub field_ext: Option<types::Element>,

    /// HTTP headerfield value
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// A series of operations required to clean up after the all the tests are
/// executed (successfully or otherwise).
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::test_script::TestScriptTeardown;
///
/// let value = TestScriptTeardown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptTeardown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptTeardown {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// One or more teardown operations to perform
    pub action: ::vec1::Vec1<TestScriptTeardownAction>,
}

/// The teardown action will only contain an operation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptTeardownAction;
/// use fhir::r3::types;
///
/// let value = TestScriptTeardownAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptTeardownAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptTeardownAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The teardown operation to perform
    pub operation: TestScriptSetupActionOperation,
}

/// A test in this script.
///
/// # Examples
///
/// ```ignore
/// use fhir::r3::resources::test_script::TestScriptTest;
///
/// let value = TestScriptTest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptTest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptTest {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Tracking/logging name of this test
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Tracking/reporting short description of the test
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// A test operation or assert to perform
    pub action: ::vec1::Vec1<TestScriptTestAction>,
}

/// Action would contain either an operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptTestAction;
/// use fhir::r3::types;
///
/// let value = TestScriptTestAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptTestAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptTestAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The setup assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// Variable is set based either on element value in response body or on header
/// field value in the response headers.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::test_script::TestScriptVariable;
/// use fhir::r3::types;
///
/// let value = TestScriptVariable {
///     default_value: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `defaultValue` is the name this serializes to on the wire.
/// assert_eq!(json["defaultValue"], ::serde_json::json!("abc"));
///
/// let back: TestScriptVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct TestScriptVariable {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Descriptive name for this variable
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Default, hard-coded, or user-defined value for this variable
    pub default_value: Option<types::String>,
    /// Primitive extension sibling for [`default_value`](Self::default_value) (FHIR `_defaultValue`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_defaultValue")]
    pub default_value_ext: Option<types::Element>,

    /// Natural language description of the variable
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The fluentpath expression against the fixture body
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// HTTP header field name for source
    pub header_field: Option<types::String>,
    /// Primitive extension sibling for [`header_field`](Self::header_field) (FHIR `_headerField`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_headerField")]
    pub header_field_ext: Option<types::Element>,

    /// Hint help text for default value to enter
    pub hint: Option<types::String>,
    /// Primitive extension sibling for [`hint`](Self::hint) (FHIR `_hint`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_hint")]
    pub hint_ext: Option<types::Element>,

    /// XPath or JSONPath against the fixture body
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Fixture Id of source expression or headerField within this variable
    pub source_id: Option<types::Id>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TestScript;

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
