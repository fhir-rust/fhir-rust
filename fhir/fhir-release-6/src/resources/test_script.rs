//! TestScript
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestScript
//!
//! Version: 6.0.0-ballot3
//!
//! Describes a set of tests
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScript;
/// use fhir::r6::types;
///
/// let value = TestScript {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: TestScript = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScript {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this test script, represented as a URI
    /// (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the test script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the test script
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `TestScript.versionAlgorithm[x]` choice element (0..1); see [`TestScriptVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<TestScriptVersionAlgorithm>,

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
    pub status: crate::coded::Coded<crate::r6::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// For testing only - never for real usage
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

    /// Name of the publisher/steward (organization or individual)
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

    /// The context that the content is intended to support
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

    /// Copyright holder and year(s)
    pub copyright_label: Option<types::String>,
    /// Primitive extension sibling for [`copyright_label`](Self::copyright_label) (FHIR `_copyrightLabel`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_copyrightLabel")]
    pub copyright_label_ext: Option<types::Element>,

    /// An abstract server representing a client or sender in a message
    /// exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_system: Vec<TestScriptTestSystem>,

    /// Required capability that is assumed to function correctly on the FHIR
    /// server being tested
    pub metadata: Option<TestScriptMetadata>,

    /// Indication of the artifact(s) that are tested by this test case
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<TestScriptScope>,

    /// Fixture in the test script - by reference (uri)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixture: Vec<TestScriptFixture>,

    /// Reference of the validation profile
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile: Vec<types::Canonical>,
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

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

    /// A common collection of actions in this script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub common: Vec<TestScriptCommon>,
}

/// A common collection of actions that can be re-used in a TestScript.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::test_script::TestScriptCommon;
///
/// let value = TestScriptCommon::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptCommon = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptCommon {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Key that identifies this common collection of actions
    pub key: types::Id,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Tracking/logging name of this common collection of actions
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Tracking/reporting short description of this common collection of
    /// actions
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Parameter(s) to convey input values to this common collection of
    /// actions in this script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<TestScriptCommonParameter>,

    /// A common operation or assert that can be re-used in this script
    pub action: ::vec1::Vec1<TestScriptCommonAction>,
}

/// An action will contain either an operation or an assertion but not both.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptCommonAction;
/// use fhir::r6::types;
///
/// let value = TestScriptCommonAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptCommonAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptCommonAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The common operation that can be re-used in this script
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The common assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// Optional named parameter(s) to provide input values to this common
/// collection of actions from this or an external TestScript.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptCommonParameter;
/// use fhir::r6::types;
///
/// let value = TestScriptCommonParameter {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: TestScriptCommonParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptCommonParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Tracking/logging name of this parameter to be used in this common
    /// collection of actions
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Tracking/reporting short description of this parameter to be used this
    /// common collection of actions
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Fixture in the test script - by reference (uri). All fixtures are required
/// for the test script to execute.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptFixture;
/// use fhir::r6::types;
///
/// let value = TestScriptFixture {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptFixture = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptFixture {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether or not to implicitly create the fixture during setup
    pub autocreate: types::Boolean,
    /// Primitive extension sibling for [`autocreate`](Self::autocreate) (FHIR `_autocreate`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_autocreate")]
    pub autocreate_ext: Option<types::Element>,

    /// Whether or not to implicitly delete the fixture during teardown
    pub autodelete: types::Boolean,
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
/// use fhir::r6::resources::test_script::TestScriptMetadata;
///
/// let value = TestScriptMetadata::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptMetadata = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptMetadata {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
/// use fhir::r6::resources::test_script::TestScriptMetadataCapability;
/// use fhir::r6::types;
///
/// let value = TestScriptMetadataCapability {
///     description: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("abc"));
///
/// let back: TestScriptMetadataCapability = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptMetadataCapability {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Are the capabilities required?
    pub required: types::Boolean,
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// Are the capabilities validated?
    pub validated: types::Boolean,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<types::Integer>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<types::Uri>,
    /// Primitive extension sibling for [`link`](Self::link) (FHIR `_link`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_link")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_ext: Vec<Option<types::Element>>,

    /// Required Capability Statement
    pub capabilities: types::Canonical,
    /// Primitive extension sibling for [`capabilities`](Self::capabilities) (FHIR `_capabilities`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_capabilities")]
    pub capabilities_ext: Option<types::Element>,
}

/// A link to the FHIR specification that this test is covering.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptMetadataLink;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct TestScriptMetadataLink {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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

/// The scope indicates a conformance artifact that is tested by the test(s)
/// within this test case and the expectation of the test outcome(s) as well as
/// the intended test phase inclusion.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptScope;
/// use fhir::r6::types;
///
/// let value = TestScriptScope {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptScope = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptScope {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific conformance artifact being tested
    pub artifact: types::Canonical,
    /// Primitive extension sibling for [`artifact`](Self::artifact) (FHIR `_artifact`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_artifact")]
    pub artifact_ext: Option<types::Element>,

    /// required | optional | strict
    pub conformance: Option<types::CodeableConcept>,

    /// unit | integration | production
    pub phase: Option<types::CodeableConcept>,
}

/// A series of required setup operations before tests are executed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptSetup;
/// use fhir::r6::types;
///
/// let value = TestScriptSetup {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptSetup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A setup common or operation or assert to perform
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<TestScriptSetupAction>,
}

/// Action would contain either a common or operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptSetupAction;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct TestScriptSetupAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Links or references to common collection(s) of actions
    pub common: Option<TestScriptSetupActionCommon>,

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
/// use fhir::r6::resources::test_script::TestScriptSetupActionAssert;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct TestScriptSetupActionAssert {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    pub direction: Option<crate::coded::Coded<crate::r6::codes::AssertDirectionCodes>>,
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

    /// The FHIRPath expression to evaluate against the source fixture
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

    /// Mime type to compare against the 'Content-Type' header
    pub content_type: Option<types::Code>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// fail | pass | skip | stop
    pub default_manual_completion:
        Option<crate::coded::Coded<crate::r6::codes::AssertManualCompletionCodes>>,
    /// Primitive extension sibling for [`default_manual_completion`](Self::default_manual_completion) (FHIR `_defaultManualCompletion`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_defaultManualCompletion")]
    pub default_manual_completion_ext: Option<types::Element>,

    /// The FHIRPath expression to be evaluated
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
    /// notEmpty | contains | notContains | eval | manualEval
    pub operator: Option<crate::coded::Coded<crate::r6::codes::AssertOperatorCodes>>,
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

    /// delete | get | options | patch | post | put | head
    pub request_method: Option<crate::coded::Coded<crate::r6::codes::HttpOperations>>,
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
    pub resource: Option<types::Uri>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// continue | switchingProtocols | okay | created | accepted |
    /// nonAuthoritativeInformation | noContent | resetContent | partialContent
    /// | multipleChoices | movedPermanently | found | seeOther | notModified |
    /// useProxy | temporaryRedirect | permanentRedirect | badRequest |
    /// unauthorized | paymentRequired | forbidden | notFound |
    /// methodNotAllowed | notAcceptable | proxyAuthenticationRequired |
    /// requestTimeout | conflict | gone | lengthRequired | preconditionFailed
    /// | contentTooLarge | uriTooLong | unsupportedMediaType |
    /// rangeNotSatisfiable | expectationFailed | misdirectedRequest |
    /// unprocessableContent | upgradeRequired | internalServerError |
    /// notImplemented | badGateway | serviceUnavailable | gatewayTimeout |
    /// httpVersionNotSupported
    pub response: Option<crate::coded::Coded<crate::r6::codes::AssertResponseCodeTypes>>,
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

    /// If this assert fails, will the current test execution stop?
    pub stop_test_on_fail: types::Boolean,
    /// Primitive extension sibling for [`stop_test_on_fail`](Self::stop_test_on_fail) (FHIR `_stopTestOnFail`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_stopTestOnFail")]
    pub stop_test_on_fail_ext: Option<types::Element>,

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
    pub warning_only: types::Boolean,
    /// Primitive extension sibling for [`warning_only`](Self::warning_only) (FHIR `_warningOnly`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_warningOnly")]
    pub warning_only_ext: Option<types::Element>,

    /// Links or references to the testing requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirement: Vec<TestScriptSetupActionAssertRequirement>,
}

/// Links or references providing traceability to the testing requirements for
/// this assert.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptSetupActionAssertRequirement;
/// use fhir::r6::types;
///
/// let value = TestScriptSetupActionAssertRequirement {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionAssertRequirement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptSetupActionAssertRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical reference to the Requirements instance
    pub reference: types::Canonical,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// Requirements statement key identifier
    pub key: types::Id,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,
}

/// Links or references to common collection(s) of actions in this or an
/// external TestScript instance.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptSetupActionCommon;
/// use fhir::r6::types;
///
/// let value = TestScriptSetupActionCommon {
///     test_script: Some(types::Canonical("http://example.org/vs".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `testScript` is the name this serializes to on the wire.
/// assert_eq!(json["testScript"], ::serde_json::json!("http://example.org/vs"));
///
/// let back: TestScriptSetupActionCommon = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptSetupActionCommon {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical reference to the common TestScript instance
    pub test_script: Option<types::Canonical>,
    /// Primitive extension sibling for [`test_script`](Self::test_script) (FHIR `_testScript`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_testScript")]
    pub test_script_ext: Option<types::Element>,

    /// Common key reference that identifies the common collection of actions
    /// to perform
    pub key_ref: types::Id,
    /// Primitive extension sibling for [`key_ref`](Self::key_ref) (FHIR `_keyRef`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_keyRef")]
    pub key_ref_ext: Option<types::Element>,

    /// Parameter(s) to convey input values to the identified common collection
    /// of actions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<TestScriptSetupActionCommonParameter>,
}

/// Optional named parameter(s) to provide input values to the identified
/// common collection of actions from this or an external TestScript.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptSetupActionCommonParameter;
/// use fhir::r6::types;
///
/// let value = TestScriptSetupActionCommonParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestScriptSetupActionCommonParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptSetupActionCommonParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Name of the parameter from the identified common collection of actions
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Value to assign to the parameter from the identified common collection
    /// of actions
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
/// use fhir::r6::resources::test_script::TestScriptSetupActionOperation;
/// use fhir::r6::types;
///
/// let value = TestScriptSetupActionOperation {
///     content_type: Some(types::Code("final".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `contentType` is the name this serializes to on the wire.
/// assert_eq!(json["contentType"], ::serde_json::json!("final"));
///
/// let back: TestScriptSetupActionOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptSetupActionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The operation code type that will be executed
    pub r#type: Option<types::Coding>,

    /// Resource type
    pub resource: Option<types::Uri>,
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

    /// Mime type to accept in the payload of the response, with charset etc
    pub accept: Option<types::Code>,
    /// Primitive extension sibling for [`accept`](Self::accept) (FHIR `_accept`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_accept")]
    pub accept_ext: Option<types::Element>,

    /// Mime type of the request payload contents, with charset etc
    pub content_type: Option<types::Code>,
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
    pub encode_request_url: types::Boolean,
    /// Primitive extension sibling for [`encode_request_url`](Self::encode_request_url) (FHIR `_encodeRequestUrl`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_encodeRequestUrl")]
    pub encode_request_url_ext: Option<types::Element>,

    /// delete | get | options | patch | post | put | head
    pub method: Option<crate::coded::Coded<crate::r6::codes::HttpOperations>>,
    /// Primitive extension sibling for [`method`](Self::method) (FHIR `_method`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_method")]
    pub method_ext: Option<types::Element>,

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
/// use fhir::r6::resources::test_script::TestScriptSetupActionOperationRequestHeader;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct TestScriptSetupActionOperationRequestHeader {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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

/// A series of operations required to clean up after all the tests are
/// executed (successfully or otherwise).
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::test_script::TestScriptTeardown;
///
/// let value = TestScriptTeardown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptTeardown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptTeardown {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
/// use fhir::r6::resources::test_script::TestScriptTeardownAction;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct TestScriptTeardownAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Links or references to common collection(s) of actions
    pub common: Option<TestScriptSetupActionCommon>,

    /// The teardown operation to perform
    pub operation: TestScriptSetupActionOperation,
}

/// A test in this script.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptTest;
/// use fhir::r6::types;
///
/// let value = TestScriptTest {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: TestScriptTest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptTest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action: Vec<TestScriptTestAction>,
}

/// Action would contain either an operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptTestAction;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct TestScriptTestAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Links or references to common collection(s) of actions
    pub common: Option<TestScriptSetupActionCommon>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The setup assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// An abstract server used in operations within this test script in the origin
/// element.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptTestSystem;
/// use fhir::r6::types;
///
/// let value = TestScriptTestSystem {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: TestScriptTestSystem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestScriptTestSystem {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The index of the abstract test system
    pub index: types::PositiveInt,
    /// Primitive extension sibling for [`index`](Self::index) (FHIR `_index`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_index")]
    pub index_ext: Option<types::Element>,

    /// Label for the test system
    pub title: types::String,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// Features met by the test system
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<types::Canonical>,
    /// Primitive extension sibling for [`actor`](Self::actor) (FHIR `_actor`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_actor")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_ext: Vec<Option<types::Element>>,

    /// Formatted information of this test system
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The url path of this test system
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// Variable is set based either on element value in response body or on header
/// field value in the response headers.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_script::TestScriptVariable;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
pub struct TestScriptVariable {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
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

    /// The FHIRPath expression against the fixture body
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

/// The `TestScript.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TestScriptVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
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
