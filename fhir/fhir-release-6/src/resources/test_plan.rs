//! TestPlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestPlan
//!
//! Version: 6.0.0-ballot3
//!
//! Description of intented testing
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A plan for executing testing on an artifact or specifications
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlan;
/// use fhir::r6::types;
///
/// let value = TestPlan {
///     copyright_label: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `copyrightLabel` is the name this serializes to on the wire.
/// assert_eq!(json["copyrightLabel"], ::serde_json::json!("abc"));
///
/// let back: TestPlan = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlan {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this test plan, represented as a URI (globally
    /// unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier for the test plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the test plan
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// How to compare versions
    /// The `TestPlan.versionAlgorithm[x]` choice element (0..1); see [`TestPlanVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<TestPlanVersionAlgorithm>,

    /// Name for this test plan (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this test plan (human friendly)
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

    /// Natural language description of the test plan
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// The context that the content is intended to support
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<types::UsageContext>,

    /// Intended jurisdiction where the test plan applies (if applicable)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<types::CodeableConcept>,

    /// Why this test plan is defined
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

    /// The category of the Test Plan - can be acceptance, unit, performance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// What is being tested with this Test Plan - a conformance resource, or
    /// narrative criteria, or an external reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<TestPlanScope>,

    /// A description of test tools to be used in the test plan - narrative for
    /// now
    pub test_tools: Option<types::Markdown>,
    /// Primitive extension sibling for [`test_tools`](Self::test_tools) (FHIR `_testTools`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_testTools")]
    pub test_tools_ext: Option<types::Element>,

    /// The required criteria to execute the test plan - e.g. preconditions,
    /// previous tests
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency: Vec<TestPlanDependency>,

    /// The threshold or criteria for the test plan to be considered
    /// successfully executed - narrative
    pub exit_criteria: Option<types::Markdown>,
    /// Primitive extension sibling for [`exit_criteria`](Self::exit_criteria) (FHIR `_exitCriteria`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_exitCriteria")]
    pub exit_criteria_ext: Option<types::Element>,

    /// The test cases that constitute this plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_case: Vec<TestPlanTestCase>,
}

/// The required criteria to execute the test plan - e.g. preconditions,
/// previous tests...
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanDependency;
/// use fhir::r6::types;
///
/// let value = TestPlanDependency {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: TestPlanDependency = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanDependency {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of the dependency criterium
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Link to predecessor test plans
    pub predecessor: Option<types::Reference<crate::r6::resources::TestPlan>>,
}

/// What is being tested with this Test Plan - a conformance resource, or
/// narrative criteria, or an external reference...
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanScope;
/// use fhir::r6::types;
///
/// let value = TestPlanScope {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestPlanScope = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanScope {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific conformance artifact, or narrative criteria, or an
    /// external reference being tested
    /// The `TestPlan.scope.artifact[x]` choice element (1..1); see [`TestPlanScopeArtifact`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub artifact: Option<TestPlanScopeArtifact>,
}

/// The individual test cases that are part of this plan, when they they are
/// made explicit.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCase;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCase {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: TestPlanTestCase = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCase {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Key that identifies this test case
    pub key: types::Id,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,

    /// Narrative description explaining the purpose of this test case
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Sequence of test case in the test plan
    pub sequence: Option<types::Integer>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// The scope or artifact covered by the case
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<TestPlanTestCaseScope>,

    /// Links or references to the testing requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirement: Vec<TestPlanTestCaseRequirement>,

    /// Required criteria to execute the test case
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency: Vec<TestPlanTestCaseDependency>,

    /// The actual test to be executed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_run: Vec<TestPlanTestCaseTestRun>,

    /// The test data used in the test case
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_data: Vec<TestPlanTestCaseTestData>,

    /// Test assertions or expectations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assertion: Vec<TestPlanTestCaseAssertion>,

    /// Nested test cases
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_case: Vec<TestPlanTestCase>,
}

/// The test assertions - the expectations of test results from the execution
/// of the test case.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCaseAssertion;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCaseAssertion {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestPlanTestCaseAssertion = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCaseAssertion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The expected outcome for this assertion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The focus or object of the assertion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub object: Vec<types::CodeableReference>,

    /// The actual result assertion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::CodeableReference>,
}

/// The required criteria to execute the test case - e.g. preconditions,
/// previous tests.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCaseDependency;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCaseDependency {
///     description: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `description` is the name this serializes to on the wire.
/// assert_eq!(json["description"], ::serde_json::json!("# Heading"));
///
/// let back: TestPlanTestCaseDependency = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCaseDependency {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Description of the criteria
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Canonical reference to the TestPlan dependency instance
    pub reference: Option<types::Canonical>,
    /// Primitive extension sibling for [`reference`](Self::reference) (FHIR `_reference`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_reference")]
    pub reference_ext: Option<types::Element>,

    /// TestPlan dependency instance testCase key identifier
    pub key: Option<types::Id>,
    /// Primitive extension sibling for [`key`](Self::key) (FHIR `_key`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_key")]
    pub key_ext: Option<types::Element>,
}

/// Links or references providing traceability to the testing requirements for
/// this assert.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCaseRequirement;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCaseRequirement {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestPlanTestCaseRequirement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCaseRequirement {
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

/// The scope or artifact covered by the case, when the individual test case is
/// associated with a testable artifact.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCaseScope;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCaseScope {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestPlanTestCaseScope = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCaseScope {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The specific conformance artifact, or narrative criteria, or an
    /// external reference covered by the case
    /// The `TestPlan.testCase.scope.artifact[x]` choice element (1..1); see [`TestPlanTestCaseScopeArtifact`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub artifact: Option<TestPlanTestCaseScopeArtifact>,
}

/// The test data used in the test case.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCaseTestData;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCaseTestData {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestPlanTestCaseTestData = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCaseTestData {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The type of test data description, e.g. 'synthea'
    pub r#type: types::Coding,

    /// The actual test resources when they exist
    pub content: Option<types::Reference>,

    /// Pointer to a definition of test resources - narrative or structured
    /// e.g. synthetic data generation, etc
    /// The `TestPlan.testCase.testData.source[x]` choice element (0..1); see [`TestPlanTestCaseTestDataSource`].
    #[serde(flatten)]
    pub source: Option<TestPlanTestCaseTestDataSource>,
}

/// The actual test to be executed.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCaseTestRun;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCaseTestRun {
///     narrative: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `narrative` is the name this serializes to on the wire.
/// assert_eq!(json["narrative"], ::serde_json::json!("# Heading"));
///
/// let back: TestPlanTestCaseTestRun = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCaseTestRun {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The narrative description of the tests
    pub narrative: Option<types::Markdown>,
    /// Primitive extension sibling for [`narrative`](Self::narrative) (FHIR `_narrative`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_narrative")]
    pub narrative_ext: Option<types::Element>,

    /// The test cases in a structured language
    pub script: Option<TestPlanTestCaseTestRunScript>,
}

/// The test cases in a structured language e.g. Gherkin, Postman, or FHIR
/// TestScript.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::test_plan::TestPlanTestCaseTestRunScript;
/// use fhir::r6::types;
///
/// let value = TestPlanTestCaseTestRunScript {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestPlanTestCaseTestRunScript = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct TestPlanTestCaseTestRunScript {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The structured language for the test case script
    pub language: Option<types::CodeableConcept>,

    /// The actual content of the script, reference to test resource
    /// (TestScript) or externally defined content
    /// The `TestPlan.testCase.testRun.script.source[x]` choice element (0..1); see [`TestPlanTestCaseTestRunScriptSource`].
    #[serde(flatten)]
    pub source: Option<TestPlanTestCaseTestRunScriptSource>,
}

/// The `TestPlan.versionAlgorithm[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}

/// The `TestPlan.scope.artifact[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanScopeArtifact {
    /// `artifactCanonical` variant.
    #[fhir("artifactCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `artifactMarkdown` variant.
    #[fhir("artifactMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `artifactUri` variant.
    #[fhir("artifactUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

/// The `TestPlan.testCase.scope.artifact[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanTestCaseScopeArtifact {
    /// `artifactCanonical` variant.
    #[fhir("artifactCanonical")]
    Canonical(crate::r6::choice::Primitive<types::Canonical>),
    /// `artifactMarkdown` variant.
    #[fhir("artifactMarkdown")]
    Markdown(crate::r6::choice::Primitive<types::Markdown>),
    /// `artifactUri` variant.
    #[fhir("artifactUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
}

/// The `TestPlan.testCase.testData.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanTestCaseTestDataSource {
    /// `sourceString` variant.
    #[fhir("sourceString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `sourceUri` variant.
    #[fhir("sourceUri")]
    Uri(crate::r6::choice::Primitive<types::Uri>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}

/// The `TestPlan.testCase.testRun.script.source[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanTestCaseTestRunScriptSource {
    /// `sourceString` variant.
    #[fhir("sourceString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `sourceAttachment` variant.
    #[fhir("sourceAttachment")]
    Attachment(Box<types::Attachment>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TestPlan;

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
