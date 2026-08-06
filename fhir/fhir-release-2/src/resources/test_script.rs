//! TestScript
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestScript
//!
//!
//!
//! Describes a set of tests
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for TestScript Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::test_script::TestScript;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
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
    pub contained: Vec<crate::r2::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Absolute URL used to reference this TestScript
    pub url: types::Uri,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Logical id for this version of the TestScript
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// Informal name for this TestScript
    pub name: types::String,
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

    /// External identifier
    pub identifier: Option<types::Identifier>,

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
    pub contact: Vec<TestScriptContact>,

    /// Date for this version of the TestScript
    pub date: Option<types::DateTime>,
    /// Primitive extension sibling for [`date`](Self::date) (FHIR `_date`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_date")]
    pub date_ext: Option<types::Element>,

    /// Natural language description of the TestScript
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Content intends to support these contexts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::CodeableConcept>,

    /// Scope and Usage this Test Script is for
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

    /// Required capability that is assumed to function correctly on the FHIR
    /// server being tested
    pub metadata: Option<TestScriptMetadata>,

    /// Whether or not the tests apply to more than one FHIR server
    pub multiserver: Option<types::Boolean>,
    /// Primitive extension sibling for [`multiserver`](Self::multiserver) (FHIR `_multiserver`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_multiserver")]
    pub multiserver_ext: Option<types::Element>,

    /// Fixture in the test script - by reference (uri)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixture: Vec<TestScriptFixture>,

    /// Reference of the validation profile
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Reference>,

    /// Placeholder for evaluated elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable: Vec<TestScriptVariable>,

    /// A series of required setup operations before tests are executed
    pub setup: Option<TestScriptSetup>,

    /// A test in this script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test: Vec<TestScriptTest>,

    /// A series of required clean up steps
    pub teardown: Option<TestScriptTeardown>,
}

/// Contacts to assist a user in finding and communicating with the publisher.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::test_script::TestScriptContact;
/// use fhir::r2::types;
///
/// let value = TestScriptContact {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: TestScriptContact = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptContact {
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

/// Fixture in the test script - by reference (uri). All fixtures are required
/// for the test script to execute.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::test_script::TestScriptFixture;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct TestScriptFixture {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
/// use fhir::r2::resources::test_script::TestScriptMetadata;
///
/// let value = TestScriptMetadata::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptMetadata = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptMetadata {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
/// use fhir::r2::resources::test_script::TestScriptMetadataCapability;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct TestScriptMetadataCapability {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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

    /// Which server these requirements apply to
    pub destination: Option<types::Integer>,
    /// Primitive extension sibling for [`destination`](Self::destination) (FHIR `_destination`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_destination")]
    pub destination_ext: Option<types::Element>,

    /// Links to the FHIR specification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<types::Uri>,
    /// Primitive extension sibling for [`link`](Self::link) (FHIR `_link`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_link")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_ext: Vec<Option<types::Element>>,

    /// Required Conformance
    pub conformance: types::Reference,
}

/// A link to the FHIR specification that this test is covering.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::test_script::TestScriptMetadataLink;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct TestScriptMetadataLink {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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

/// A series of required setup operations before tests are executed.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::test_script::TestScriptSetup;
///
/// let value = TestScriptSetup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptSetup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptSetup {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Capabilities that are assumed to function correctly on the FHIR server
    /// being tested
    pub metadata: Option<TestScriptMetadata>,

    /// A setup operation or assert to perform
    pub action: ::vec1::Vec1<TestScriptSetupAction>,
}

/// Action would contain either an operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::test_script::TestScriptSetupAction;
/// use fhir::r2::types;
///
/// let value = TestScriptSetupAction {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: TestScriptSetupAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptSetupAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
/// use fhir::r2::resources::test_script::TestScriptSetupActionAssert;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct TestScriptSetupActionAssert {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
    pub direction: Option<crate::coded::Coded<crate::r2::codes::AssertDirectionCodes>>,
    /// Primitive extension sibling for [`direction`](Self::direction) (FHIR `_direction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_direction")]
    pub direction_ext: Option<types::Element>,

    /// Id of fixture used to compare the "sourceId/path" evaluations to
    pub compare_to_source_id: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_id`](Self::compare_to_source_id) (FHIR `_compareToSourceId`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compareToSourceId")]
    pub compare_to_source_id_ext: Option<types::Element>,

    /// XPath or JSONPath expression against fixture used to compare the
    /// "sourceId/path" evaluations to
    pub compare_to_source_path: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_path`](Self::compare_to_source_path) (FHIR `_compareToSourcePath`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_compareToSourcePath")]
    pub compare_to_source_path_ext: Option<types::Element>,

    /// xml | json
    pub content_type: Option<crate::coded::Coded<crate::r2::codes::ContentType>>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

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
    /// notEmpty | contains | notContains
    pub operator: Option<crate::coded::Coded<crate::r2::codes::AssertOperatorCodes>>,
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

    /// Resource type
    pub resource: Option<types::Code>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// okay | created | noContent | notModified | bad | forbidden | notFound |
    /// methodNotAllowed | conflict | gone | preconditionFailed | unprocessable
    pub response: Option<crate::coded::Coded<crate::r2::codes::AssertResponseCodeTypes>>,
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

/// The operation to perform.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::test_script::TestScriptSetupActionOperation;
/// use fhir::r2::types;
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
#[fhir_version("r2")]
pub struct TestScriptSetupActionOperation {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The setup operation type that will be executed
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

    /// xml | json
    pub accept: Option<crate::coded::Coded<crate::r2::codes::ContentType>>,
    /// Primitive extension sibling for [`accept`](Self::accept) (FHIR `_accept`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_accept")]
    pub accept_ext: Option<types::Element>,

    /// xml | json
    pub content_type: Option<crate::coded::Coded<crate::r2::codes::ContentType>>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// Which server to perform the operation on
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

    /// Explicitly defined path parameters
    pub params: Option<types::String>,
    /// Primitive extension sibling for [`params`](Self::params) (FHIR `_params`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_params")]
    pub params_ext: Option<types::Element>,

    /// Each operation can have one ore more header elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request_header: Vec<TestScriptSetupActionOperationRequestHeader>,

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
/// use fhir::r2::resources::test_script::TestScriptSetupActionOperationRequestHeader;
/// use fhir::r2::types;
///
/// let value = TestScriptSetupActionOperationRequestHeader {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: TestScriptSetupActionOperationRequestHeader = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptSetupActionOperationRequestHeader {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
/// use fhir::r2::resources::test_script::TestScriptTeardown;
///
/// let value = TestScriptTeardown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptTeardown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptTeardown {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
/// use fhir::r2::resources::test_script::TestScriptTeardownAction;
/// use fhir::r2::types;
///
/// let value = TestScriptTeardownAction {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: TestScriptTeardownAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptTeardownAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The teardown operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,
}

/// A test in this script.
///
/// # Examples
///
/// ```ignore
/// use fhir::r2::resources::test_script::TestScriptTest;
///
/// let value = TestScriptTest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptTest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptTest {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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

    /// Capabilities that are expected to function correctly on the FHIR server
    /// being tested
    pub metadata: Option<TestScriptMetadata>,

    /// A test operation or assert to perform
    pub action: ::vec1::Vec1<TestScriptTestAction>,
}

/// Action would contain either an operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::test_script::TestScriptTestAction;
/// use fhir::r2::types;
///
/// let value = TestScriptTestAction {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: TestScriptTestAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptTestAction {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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
/// use fhir::r2::resources::test_script::TestScriptVariable;
/// use fhir::r2::types;
///
/// let value = TestScriptVariable {
///     header_field: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `headerField` is the name this serializes to on the wire.
/// assert_eq!(json["headerField"], ::serde_json::json!("abc"));
///
/// let back: TestScriptVariable = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct TestScriptVariable {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

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

    /// HTTP header field name for source
    pub header_field: Option<types::String>,
    /// Primitive extension sibling for [`header_field`](Self::header_field) (FHIR `_headerField`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_headerField")]
    pub header_field_ext: Option<types::Element>,

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
