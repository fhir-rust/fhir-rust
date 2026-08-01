//! TestPlan
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestPlan
//!
//! Version: 5.0.0
//!
//! TestPlan Resource: A plan for executing testing on an artifact or specifications
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// TestPlan
///
/// A plan for executing testing on an artifact or specifications. It describes
/// the intended scope, the required preconditions, the test cases with their
/// data and assertions, and the exit criteria for considering the plan
/// successfully executed. TestPlan is typically used to document and coordinate
/// conformance, acceptance, unit, or performance testing of FHIR resources and
/// other specifications. It captures the overall test strategy for an artifact
/// such as an implementation guide, capability statement, or profile, including
/// the tools required, the dependencies that must be satisfied before testing
/// begins, and the individual test cases (each with its own scope, test data,
/// test run instructions, and assertions) that together demonstrate conformance.
/// A TestPlan is often referenced by, or paired with, one or more TestScript
/// resources that provide the executable, machine-readable steps for a given
/// test case.
///
/// See also: [`CodeableConcept`](crate::r5::types::CodeableConcept) for
/// categorizing the plan, and [`Reference`](crate::r5::types::Reference) for
/// linking to the scoped artifacts, predecessor test plans, or `TestScript`
/// resources under test.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlan;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlan {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Canonical identifier for this test plan, represented as a URI (globally unique)
    pub url: Option<types::Uri>,
    /// Primitive extension sibling for [`url`](Self::url) (FHIR `_url`).
    #[serde(rename = "_url")]
    pub url_ext: Option<types::Element>,

    /// Business identifier identifier for the test plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Business version of the test plan
    pub version: Option<types::String>,
    /// Primitive extension sibling for [`version`](Self::version) (FHIR `_version`).
    #[serde(rename = "_version")]
    pub version_ext: Option<types::Element>,

    /// The `TestPlan.versionAlgorithm[x]` choice element (0..1); see [`TestPlanVersionAlgorithm`].
    #[serde(flatten)]
    pub version_algorithm: Option<TestPlanVersionAlgorithm>,

    /// Name for this test plan (computer friendly)
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Name for this test plan (human friendly)
    pub title: Option<types::String>,
    /// Primitive extension sibling for [`title`](Self::title) (FHIR `_title`).
    #[serde(rename = "_title")]
    pub title_ext: Option<types::Element>,

    /// The publication status of this test plan: draft | active | retired | unknown
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

    /// Natural language description of the test plan
    pub description: Option<types::Markdown>,
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
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

    /// The category of the Test Plan, e.g. acceptance, unit, or performance testing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// The artifact(s) under test, referenced via a conformance resource, narrative criteria, or an external reference
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<types::Reference>,

    /// A description of test tools to be used in the test plan - narrative for now
    pub test_tools: Option<types::Markdown>,
    /// Primitive extension sibling for [`test_tools`](Self::test_tools) (FHIR `_testTools`).
    #[serde(rename = "_testTools")]
    pub test_tools_ext: Option<types::Element>,

    /// The required criteria to execute the test plan - e.g. preconditions, previous tests
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependency: Vec<TestPlanDependency>,

    /// The threshold or criteria for the test plan to be considered successfully executed - narrative
    pub exit_criteria: Option<types::Markdown>,
    /// Primitive extension sibling for [`exit_criteria`](Self::exit_criteria) (FHIR `_exitCriteria`).
    #[serde(rename = "_exitCriteria")]
    pub exit_criteria_ext: Option<types::Element>,

    /// The individual test cases, each with its own scope, dependencies, test run, data, and assertions, that constitute this plan
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_case: Vec<TestPlanTestCase>,
}

/// TestPlan.dependency
///
/// The required criteria to execute the test plan - e.g. preconditions,
/// previous tests.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlanDependency;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Link to predecessor test plans
    pub predecessor: Option<types::Reference>,
}

/// TestPlan.testCase
///
/// The test cases that constitute this plan.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlanTestCase;
/// use fhir::r5::types;
///
/// let value = TestPlanTestCase {
///     sequence: Some(types::Integer(42)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `sequence` is the name this serializes to on the wire.
/// assert_eq!(json["sequence"], ::serde_json::json!(42));
///
/// let back: TestPlanTestCase = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCase {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Sequence of test case in the test plan
    pub sequence: Option<types::Integer>,
    /// Primitive extension sibling for [`sequence`](Self::sequence) (FHIR `_sequence`).
    #[serde(rename = "_sequence")]
    pub sequence_ext: Option<types::Element>,

    /// The scope or artifact covered by the case
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<types::Reference>,

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
}

/// TestPlan.testCase.dependency
///
/// Required criteria to execute the test case.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlanTestCaseDependency;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`description`](Self::description) (FHIR `_description`).
    #[serde(rename = "_description")]
    pub description_ext: Option<types::Element>,

    /// Link to predecessor test plans
    pub predecessor: Option<types::Reference>,
}

/// TestPlan.testCase.testRun
///
/// The actual test to be executed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlanTestCaseTestRun;
/// use fhir::r5::types;
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
    /// Primitive extension sibling for [`narrative`](Self::narrative) (FHIR `_narrative`).
    #[serde(rename = "_narrative")]
    pub narrative_ext: Option<types::Element>,

    /// The test cases in a structured language e.g. gherkin, Postman, or FHIR TestScript
    pub script: Option<TestPlanTestCaseTestRunScript>,
}

/// TestPlan.testCase.testRun.script
///
/// The test cases in a structured language e.g. gherkin, Postman, or FHIR
/// TestScript.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlanTestCaseTestRunScript;
/// use fhir::r5::types;
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
pub struct TestPlanTestCaseTestRunScript {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The language for the test cases e.g. 'gherkin', 'testscript'
    pub language: Option<types::CodeableConcept>,

    /// The `TestPlan.testCase.testRun.script.source[x]` choice element (0..1); see [`TestPlanTestCaseTestRunScriptSource`].
    #[serde(flatten)]
    pub source: Option<TestPlanTestCaseTestRunScriptSource>,
}

/// TestPlan.testCase.testData
///
/// The test data used in the test case.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlanTestCaseTestData;
/// use fhir::r5::types;
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

    /// The `TestPlan.testCase.testData.source[x]` choice element (0..1); see [`TestPlanTestCaseTestDataSource`].
    #[serde(flatten)]
    pub source: Option<TestPlanTestCaseTestDataSource>,
}

/// TestPlan.testCase.assertion
///
/// Test assertions or expectations.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_plan::TestPlanTestCaseAssertion;
/// use fhir::r5::types;
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
pub struct TestPlanTestCaseAssertion {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Assertion type - for example 'informative' or 'required'
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<types::CodeableConcept>,

    /// The focus or object of the assertion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub object: Vec<types::CodeableReference>,

    /// The actual result assertion
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<types::CodeableReference>,
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
/// The `TestPlan.testCase.testData.source[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanTestCaseTestDataSource {
    /// `sourceString` variant.
    #[fhir("sourceString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}

/// The `TestPlan.testCase.testRun.script.source[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanTestCaseTestRunScriptSource {
    /// `sourceString` variant.
    #[fhir("sourceString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `sourceReference` variant.
    #[fhir("sourceReference")]
    Reference(Box<types::Reference>),
}

/// The `TestPlan.versionAlgorithm[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TestPlanVersionAlgorithm {
    /// `versionAlgorithmString` variant.
    #[fhir("versionAlgorithmString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `versionAlgorithmCoding` variant.
    #[fhir("versionAlgorithmCoding")]
    Coding(Box<types::Coding>),
}
