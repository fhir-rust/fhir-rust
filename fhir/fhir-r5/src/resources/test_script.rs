//! TestScript
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestScript
//!
//! Version: 5.0.0
//!
//! TestScript Resource: A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.
///
/// TestScript is a conformance resource that describes an executable suite of
/// tests, along with the fixtures, variables, and required server capabilities
/// needed to run them. It organizes work into optional setup, one or more
/// tests, and an optional teardown, where each action is either an operation
/// invoked against a server or an assertion checked against a response. In FHIR
/// R5 it is typically paired with TestReport, which records the outcome of
/// executing a TestScript.
///
/// TestScript is used by FHIR implementers, conformance testing tools, and
/// certification programs to define reusable, machine-executable test suites
/// that exercise a system's REST API operations (such as create, read,
/// search, and update) and validate the responses against expected
/// structural and business-rule assertions. Each abstract `origin` and
/// `destination` server represents a client or server participant in the
/// exchange, `fixture` and `variable` elements supply the data used during
/// execution, and `setup`, `test`, and `teardown` describe the ordered
/// sequence of operations and assertions to run. Because a TestScript is a
/// canonical, versionable conformance resource, it can be published,
/// referenced by an implementation guide, and shared across organizations to
/// support interoperability testing.
///
/// # See also
///
/// - `TestReport` — records the outcome of executing a TestScript.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for coded elements such as scope and phase.
/// - [`Identifier`](crate::r5::types::Identifier) — used for the test script's business identifier.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScript;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "TestScriptDe")]
pub struct TestScript {
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

    /// Canonical identifier for this test script, represented as a URI (globally unique), used to reference it from other artifacts
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Additional identifier for the test script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the test script
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `TestScript.versionAlgorithm[x]` choice element (0..1); see [`TestScriptVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<TestScriptVersionAlgorithm>,

    /// Name for this test script (computer friendly)
    pub name: types::String,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this test script (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication status of this test script: draft | active | retired | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

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

    /// Natural language description of the test script
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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

    /// An abstract server representing a client or sender in a message exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<TestScriptOrigin>,

    /// An abstract server representing a destination or receiver in a message exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destination: Vec<TestScriptDestination>,

    /// Required capability that is assumed to function correctly on the FHIR server being tested
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
    /// Primitive extension sibling for [`profile`](Self::profile) (FHIR `_profile`).
    #[serde(rename = "_profile")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profile_ext: Vec<Option<types::Element>>,

    /// Placeholder for evaluated elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variable: Vec<TestScriptVariable>,

    /// A series of required setup operations, run once before any tests are executed, that establish the preconditions needed for the test suite
    pub setup: Option<TestScriptSetup>,

    /// A test in this script, each containing an ordered sequence of operations and assertions that exercise the system under test
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test: Vec<TestScriptTest>,

    /// A series of required clean up steps, run once after all tests complete, that remove fixtures and restore the server to its prior state
    pub teardown: Option<TestScriptTeardown>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestScriptDe {
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
    version_algorithm: crate::r5::choice::Slot<TestScriptVersionAlgorithm>,
    name: types::String,
    #[serde(rename = "_name")]
    name_ext: Option<types::Element>,
    title: Option<types::String>,
    #[serde(rename = "_title")]
    title_ext: Option<types::Element>,
    status: crate::r5::coded::Coded<crate::r5::codes::PublicationStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
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
    #[serde(default)]
    origin: Vec<TestScriptOrigin>,
    #[serde(default)]
    destination: Vec<TestScriptDestination>,
    metadata: Option<TestScriptMetadata>,
    #[serde(default)]
    scope: Vec<TestScriptScope>,
    #[serde(default)]
    fixture: Vec<TestScriptFixture>,
    #[serde(default)]
    profile: Vec<types::Canonical>,
    #[serde(rename = "_profile")]
    #[serde(default)]
    profile_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    variable: Vec<TestScriptVariable>,
    setup: Option<TestScriptSetup>,
    #[serde(default)]
    test: Vec<TestScriptTest>,
    teardown: Option<TestScriptTeardown>,
}

impl ::core::convert::From<TestScriptDe> for TestScript {
    fn from(v: TestScriptDe) -> Self {
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
            origin: v.origin,
            destination: v.destination,
            metadata: v.metadata,
            scope: v.scope,
            fixture: v.fixture,
            profile: v.profile,
            profile_ext: v.profile_ext,
            variable: v.variable,
            setup: v.setup,
            test: v.test,
            teardown: v.teardown,
        }
    }
}

/// An abstract server representing a client or sender in a message exchange.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptOrigin;
/// use fhir::r5::types;
///
/// let value = TestScriptOrigin {
///     url: Some(types::Url("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: TestScriptOrigin = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptOrigin {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The index of the abstract origin server starting at 1
    pub index: types::Integer,
    /// Primitive extension sibling for [`index`](Self::index) (FHIR `_index`).
    #[serde(rename = "_index")]
    pub index_ext: Option<types::Element>,

    /// FHIR-Client | FHIR-SDC-FormFiller
    pub profile: types::Coding,

    /// The url path of the origin server
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// An abstract server representing a destination or receiver in a message exchange.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptDestination;
/// use fhir::r5::types;
///
/// let value = TestScriptDestination {
///     url: Some(types::Url("http://example.org".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `url` is the name this serializes to on the wire.
/// assert_eq!(json["url"], ::serde_json::json!("http://example.org"));
///
/// let back: TestScriptDestination = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptDestination {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The index of the abstract destination server starting at 1
    pub index: types::Integer,
    /// Primitive extension sibling for [`index`](Self::index) (FHIR `_index`).
    #[serde(rename = "_index")]
    pub index_ext: Option<types::Element>,

    /// FHIR-Server | FHIR-SDC-FormManager | FHIR-SDC-FormReceiver | FHIR-SDC-FormProcessor
    pub profile: types::Coding,

    /// The url path of the destination server
    pub url: Option<types::Url>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// Required capability that is assumed to function correctly on the FHIR server being tested.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::test_script::TestScriptMetadata;
///
/// let value = TestScriptMetadata::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptMetadata = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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

    /// Capabilities  that are assumed to function correctly on the FHIR server being tested
    pub capability: vec1::Vec1<TestScriptMetadataCapability>,
}

/// Links to the FHIR specification that describe the capabilities being tested.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptMetadataLink;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Short description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,
}

/// Capabilities that are assumed to function correctly on the FHIR server being tested.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptMetadataCapability;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`required`](Self::required) (FHIR `_required`).
    #[serde(rename = "_required")]
    pub required_ext: Option<types::Element>,

    /// Are the capabilities validated?
    pub validated: types::Boolean,
    /// Primitive extension sibling for [`validated`](Self::validated) (FHIR `_validated`).
    #[serde(rename = "_validated")]
    pub validated_ext: Option<types::Element>,

    /// The expected capabilities of the server
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Which origin server these requirements apply to
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin: Vec<types::Integer>,
    /// Primitive extension sibling for [`origin`](Self::origin) (FHIR `_origin`).
    #[serde(rename = "_origin")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origin_ext: Vec<Option<types::Element>>,

    /// Which server these requirements apply to
    pub destination: Option<types::Integer>,
    /// Primitive extension sibling for [`destination`](Self::destination) (FHIR `_destination`).
    #[serde(rename = "_destination")]
    pub destination_ext: Option<types::Element>,

    /// Links to the FHIR specification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link: Vec<types::Uri>,
    /// Primitive extension sibling for [`link`](Self::link) (FHIR `_link`).
    #[serde(rename = "_link")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_ext: Vec<Option<types::Element>>,

    /// Required Capability Statement
    pub capabilities: types::Canonical,
    /// Primitive extension sibling for [`capabilities`](Self::capabilities) (FHIR `_capabilities`).
    #[serde(rename = "_capabilities")]
    pub capabilities_ext: Option<types::Element>,
}

/// Indication of the artifact(s) that are tested by this test case.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptScope;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`artifact`](Self::artifact) (FHIR `_artifact`).
    #[serde(rename = "_artifact")]
    pub artifact_ext: Option<types::Element>,

    /// required | optional | strict
    pub conformance: Option<types::CodeableConcept>,

    /// unit | integration | production
    pub phase: Option<types::CodeableConcept>,
}

/// Fixture in the test script - by reference (uri).
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptFixture;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`autocreate`](Self::autocreate) (FHIR `_autocreate`).
    #[serde(rename = "_autocreate")]
    pub autocreate_ext: Option<types::Element>,

    /// Whether or not to implicitly delete the fixture during teardown
    pub autodelete: types::Boolean,
    /// Primitive extension sibling for [`autodelete`](Self::autodelete) (FHIR `_autodelete`).
    #[serde(rename = "_autodelete")]
    pub autodelete_ext: Option<types::Element>,

    /// Reference of the resource
    pub resource: Option<types::Reference>,
}

/// Placeholder for evaluated elements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptVariable;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Default, hard-coded, or user-defined value for this variable
    pub default_value: Option<types::String>,
    /// Primitive extension sibling for [`default_value`](Self::default_value) (FHIR `_defaultValue`).
    #[serde(rename = "_defaultValue")]
    pub default_value_ext: Option<types::Element>,

    /// Natural language description of the variable
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The FHIRPath expression against the fixture body
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`).
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// HTTP header field name for source
    pub header_field: Option<types::String>,
    /// Primitive extension sibling for [`header_field`](Self::header_field) (FHIR `_headerField`).
    #[serde(rename = "_headerField")]
    pub header_field_ext: Option<types::Element>,

    /// Hint help text for default value to enter
    pub hint: Option<types::String>,
    /// Primitive extension sibling for [`hint`](Self::hint) (FHIR `_hint`).
    #[serde(rename = "_hint")]
    pub hint_ext: Option<types::Element>,

    /// XPath or JSONPath against the fixture body
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// Fixture Id of source expression or headerField within this variable
    pub source_id: Option<types::Id>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`).
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,
}

/// A series of required setup operations before tests are executed.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::test_script::TestScriptSetup;
///
/// let value = TestScriptSetup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptSetup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestScriptSetup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A setup operation or assert to perform
    pub action: vec1::Vec1<TestScriptSetupAction>,
}

/// A setup operation or assert to perform.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptSetupAction;
/// use fhir::r5::types;
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
pub struct TestScriptSetupAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// The setup operation to perform.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptSetupActionOperation;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// Tracking/logging operation label
    pub label: Option<types::String>,
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`).
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Tracking/reporting operation description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Mime type to accept in the payload of the response, with charset etc
    pub accept: Option<types::Code>,
    /// Primitive extension sibling for [`accept`](Self::accept) (FHIR `_accept`).
    #[serde(rename = "_accept")]
    pub accept_ext: Option<types::Element>,

    /// Mime type of the request payload contents, with charset etc
    pub content_type: Option<types::Code>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`).
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// Server responding to the request
    pub destination: Option<types::Integer>,
    /// Primitive extension sibling for [`destination`](Self::destination) (FHIR `_destination`).
    #[serde(rename = "_destination")]
    pub destination_ext: Option<types::Element>,

    /// Whether or not to send the request url in encoded format
    pub encode_request_url: types::Boolean,
    /// Primitive extension sibling for [`encode_request_url`](Self::encode_request_url) (FHIR `_encodeRequestUrl`).
    #[serde(rename = "_encodeRequestUrl")]
    pub encode_request_url_ext: Option<types::Element>,

    /// delete | get | options | patch | post | put | head
    pub method: Option<crate::r5::coded::Coded<crate::r5::codes::HttpOperations>>,
    /// Primitive extension sibling for [`method`](Self::method) (FHIR `_method`).
    #[serde(rename = "_method")]
    pub method_ext: Option<types::Element>,

    /// Server initiating the request
    pub origin: Option<types::Integer>,
    /// Primitive extension sibling for [`origin`](Self::origin) (FHIR `_origin`).
    #[serde(rename = "_origin")]
    pub origin_ext: Option<types::Element>,

    /// Explicitly defined path parameters
    pub params: Option<types::String>,
    /// Primitive extension sibling for [`params`](Self::params) (FHIR `_params`).
    #[serde(rename = "_params")]
    pub params_ext: Option<types::Element>,

    /// Each operation can have one or more header elements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request_header: Vec<TestScriptSetupActionOperationRequestHeader>,

    /// Fixture Id of mapped request
    pub request_id: Option<types::Id>,
    /// Primitive extension sibling for [`request_id`](Self::request_id) (FHIR `_requestId`).
    #[serde(rename = "_requestId")]
    pub request_id_ext: Option<types::Element>,

    /// Fixture Id of mapped response
    pub response_id: Option<types::Id>,
    /// Primitive extension sibling for [`response_id`](Self::response_id) (FHIR `_responseId`).
    #[serde(rename = "_responseId")]
    pub response_id_ext: Option<types::Element>,

    /// Fixture Id of body for PUT and POST requests
    pub source_id: Option<types::Id>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`).
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,

    /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests
    pub target_id: Option<types::Id>,
    /// Primitive extension sibling for [`target_id`](Self::target_id) (FHIR `_targetId`).
    #[serde(rename = "_targetId")]
    pub target_id_ext: Option<types::Element>,

    /// Request URL
    pub url: Option<types::String>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,
}

/// Each operation can have one or more header elements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptSetupActionOperationRequestHeader;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`field`](Self::field) (FHIR `_field`).
    #[serde(rename = "_field")]
    pub field_ext: Option<types::Element>,

    /// HTTP headerfield value
    pub value: types::String,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,
}

/// The assertion to perform.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptSetupActionAssert;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`label`](Self::label) (FHIR `_label`).
    #[serde(rename = "_label")]
    pub label_ext: Option<types::Element>,

    /// Tracking/reporting assertion description
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// response | request
    pub direction: Option<crate::r5::coded::Coded<crate::r5::codes::AssertDirectionCodes>>,
    /// Primitive extension sibling for [`direction`](Self::direction) (FHIR `_direction`).
    #[serde(rename = "_direction")]
    pub direction_ext: Option<types::Element>,

    /// Id of the source fixture to be evaluated
    pub compare_to_source_id: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_id`](Self::compare_to_source_id) (FHIR `_compareToSourceId`).
    #[serde(rename = "_compareToSourceId")]
    pub compare_to_source_id_ext: Option<types::Element>,

    /// The FHIRPath expression to evaluate against the source fixture
    pub compare_to_source_expression: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_expression`](Self::compare_to_source_expression) (FHIR `_compareToSourceExpression`).
    #[serde(rename = "_compareToSourceExpression")]
    pub compare_to_source_expression_ext: Option<types::Element>,

    /// XPath or JSONPath expression to evaluate against the source fixture
    pub compare_to_source_path: Option<types::String>,
    /// Primitive extension sibling for [`compare_to_source_path`](Self::compare_to_source_path) (FHIR `_compareToSourcePath`).
    #[serde(rename = "_compareToSourcePath")]
    pub compare_to_source_path_ext: Option<types::Element>,

    /// Mime type to compare against the 'Content-Type' header
    pub content_type: Option<types::Code>,
    /// Primitive extension sibling for [`content_type`](Self::content_type) (FHIR `_contentType`).
    #[serde(rename = "_contentType")]
    pub content_type_ext: Option<types::Element>,

    /// fail | pass | skip | stop
    pub default_manual_completion:
        Option<crate::r5::coded::Coded<crate::r5::codes::AssertManualCompletionCodes>>,
    /// Primitive extension sibling for [`default_manual_completion`](Self::default_manual_completion) (FHIR `_defaultManualCompletion`).
    #[serde(rename = "_defaultManualCompletion")]
    pub default_manual_completion_ext: Option<types::Element>,

    /// The FHIRPath expression to be evaluated
    pub expression: Option<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`).
    #[serde(rename = "_expression")]
    pub expression_ext: Option<types::Element>,

    /// HTTP header field name
    pub header_field: Option<types::String>,
    /// Primitive extension sibling for [`header_field`](Self::header_field) (FHIR `_headerField`).
    #[serde(rename = "_headerField")]
    pub header_field_ext: Option<types::Element>,

    /// Fixture Id of minimum content resource
    pub minimum_id: Option<types::String>,
    /// Primitive extension sibling for [`minimum_id`](Self::minimum_id) (FHIR `_minimumId`).
    #[serde(rename = "_minimumId")]
    pub minimum_id_ext: Option<types::Element>,

    /// Perform validation on navigation links?
    pub navigation_links: Option<types::Boolean>,
    /// Primitive extension sibling for [`navigation_links`](Self::navigation_links) (FHIR `_navigationLinks`).
    #[serde(rename = "_navigationLinks")]
    pub navigation_links_ext: Option<types::Element>,

    /// equals | notEquals | in | notIn | greaterThan | lessThan | empty | notEmpty | contains | notContains | eval | manualEval
    pub operator: Option<crate::r5::coded::Coded<crate::r5::codes::AssertOperatorCodes>>,
    /// Primitive extension sibling for [`operator`](Self::operator) (FHIR `_operator`).
    #[serde(rename = "_operator")]
    pub operator_ext: Option<types::Element>,

    /// XPath or JSONPath expression
    pub path: Option<types::String>,
    /// Primitive extension sibling for [`path`](Self::path) (FHIR `_path`).
    #[serde(rename = "_path")]
    pub path_ext: Option<types::Element>,

    /// delete | get | options | patch | post | put | head
    pub request_method: Option<crate::r5::coded::Coded<crate::r5::codes::HttpOperations>>,
    /// Primitive extension sibling for [`request_method`](Self::request_method) (FHIR `_requestMethod`).
    #[serde(rename = "_requestMethod")]
    pub request_method_ext: Option<types::Element>,

    /// Request URL comparison value
    #[serde(rename = "requestURL")]
    pub request_url: Option<types::String>,
    /// Primitive extension sibling for [`request_url`](Self::request_url) (FHIR `_requestURL`).
    #[serde(rename = "_requestURL")]
    pub request_url_ext: Option<types::Element>,

    /// Resource type
    pub resource: Option<types::Uri>,
    /// Primitive extension sibling for [`resource`](Self::resource) (FHIR `_resource`).
    #[serde(rename = "_resource")]
    pub resource_ext: Option<types::Element>,

    /// HTTP response status code family
    pub response: Option<crate::r5::coded::Coded<crate::r5::codes::AssertResponseCodeTypes>>,
    /// Primitive extension sibling for [`response`](Self::response) (FHIR `_response`).
    #[serde(rename = "_response")]
    pub response_ext: Option<types::Element>,

    /// HTTP response code to test
    pub response_code: Option<types::String>,
    /// Primitive extension sibling for [`response_code`](Self::response_code) (FHIR `_responseCode`).
    #[serde(rename = "_responseCode")]
    pub response_code_ext: Option<types::Element>,

    /// Fixture Id of source expression or headerField
    pub source_id: Option<types::Id>,
    /// Primitive extension sibling for [`source_id`](Self::source_id) (FHIR `_sourceId`).
    #[serde(rename = "_sourceId")]
    pub source_id_ext: Option<types::Element>,

    /// If this assert fails, will the current test execution stop?
    pub stop_test_on_fail: types::Boolean,
    /// Primitive extension sibling for [`stop_test_on_fail`](Self::stop_test_on_fail) (FHIR `_stopTestOnFail`).
    #[serde(rename = "_stopTestOnFail")]
    pub stop_test_on_fail_ext: Option<types::Element>,

    /// Profile Id of validation profile reference
    pub validate_profile_id: Option<types::Id>,
    /// Primitive extension sibling for [`validate_profile_id`](Self::validate_profile_id) (FHIR `_validateProfileId`).
    #[serde(rename = "_validateProfileId")]
    pub validate_profile_id_ext: Option<types::Element>,

    /// The value to compare to
    pub value: Option<types::String>,
    /// Primitive extension sibling for [`value`](Self::value) (FHIR `_value`).
    #[serde(rename = "_value")]
    pub value_ext: Option<types::Element>,

    /// Will this assert produce a warning only on error?
    pub warning_only: types::Boolean,
    /// Primitive extension sibling for [`warning_only`](Self::warning_only) (FHIR `_warningOnly`).
    #[serde(rename = "_warningOnly")]
    pub warning_only_ext: Option<types::Element>,

    /// Links or references to the testing requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirement: Vec<TestScriptSetupActionAssertRequirement>,
}

/// Links or references to the testing requirements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptSetupActionAssertRequirement;
/// use fhir::r5::types;
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
#[serde(from = "TestScriptSetupActionAssertRequirementDe")]
pub struct TestScriptSetupActionAssertRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `TestScript.setup.action.assert.requirement.link[x]` choice element (0..1); see [`TestScriptSetupActionAssertRequirementLink`].
    #[serde(flatten)]
    pub link: Option<TestScriptSetupActionAssertRequirementLink>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestScriptSetupActionAssertRequirementDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    link: crate::r5::choice::Slot<TestScriptSetupActionAssertRequirementLink>,
}

impl ::core::convert::From<TestScriptSetupActionAssertRequirementDe>
    for TestScriptSetupActionAssertRequirement
{
    fn from(v: TestScriptSetupActionAssertRequirementDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            link: v.link.0,
        }
    }
}

/// A test in this script.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::test_script::TestScriptTest;
///
/// let value = TestScriptTest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptTest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Tracking/reporting short description of the test
    pub description: Option<types::String>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// A test operation or assert to perform
    pub action: vec1::Vec1<TestScriptTestAction>,
}

/// A test operation or assert to perform.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptTestAction;
/// use fhir::r5::types;
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
pub struct TestScriptTestAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The setup operation to perform
    pub operation: Option<TestScriptSetupActionOperation>,

    /// The setup assertion to perform
    pub assert: Option<TestScriptSetupActionAssert>,
}

/// A series of required clean up steps.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::test_script::TestScriptTeardown;
///
/// let value = TestScriptTeardown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestScriptTeardown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
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
    pub action: vec1::Vec1<TestScriptTeardownAction>,
}

/// One or more teardown operations to perform.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_script::TestScriptTeardownAction;
/// use fhir::r5::types;
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
pub struct TestScriptTeardownAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The teardown operation to perform
    pub operation: TestScriptSetupActionOperation,
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
/// The `TestScript.setup.action.assert.requirement.link[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TestScriptSetupActionAssertRequirementLink {
    /// `linkUri` variant.
    #[fhir("linkUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `linkCanonical` variant.
    #[fhir("linkCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}

/// The `TestScript.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TestScriptVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
