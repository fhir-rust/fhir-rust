//! TestReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestReport
//!
//! Version: 5.0.0
//!
//! TestReport Resource: A summary of information based on the results of executing a TestScript.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A summary of information based on the results of executing a TestScript.
///
/// A TestReport captures the outcome of running a TestScript against a system
/// under test. It records the overall status and result, the participants
/// involved in the execution (test engine, client, server), and the detailed
/// per-action results for the setup, test, and teardown phases.
///
/// TestReport is central to FHIR conformance and interoperability testing: it
/// provides an auditable, machine-readable record of how a target system
/// behaved when exercised by a corresponding `TestScript`, including which
/// setup preconditions were satisfied, which individual tests passed or
/// failed, and which teardown cleanup steps were run. Implementers and
/// certification programs use TestReport instances to verify conformance to
/// implementation guides, to diagnose interoperability failures between
/// systems, and to build dashboards or CI pipelines that track test coverage
/// and pass rates over time. Because each TestReport references the specific
/// version of the TestScript that produced it (via `test_script`), reports
/// remain reproducible and traceable even as test suites evolve.
///
/// # See also
///
/// - `TestScript` — the executable test definition that a TestReport is the result of running.
/// - [`Identifier`](crate::r5::types::Identifier) — used for the report's external identifier.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) and related coded types used throughout the FHIR data model.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReport;
/// use fhir::r5::types;
///
/// let value = TestReport {
///     name: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `name` is the name this serializes to on the wire.
/// assert_eq!(json["name"], ::serde_json::json!("abc"));
///
/// let back: TestReport = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReport {
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

    /// External identifier
    pub identifier: Option<types::Identifier>,

    /// Informal name of the executed TestReport
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`).
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// Lifecycle status of this report's execution: completed | in-progress | waiting | stopped | entered-in-error.
    pub status: crate::r5::coded::Coded<crate::r5::codes::ReportStatusCodes>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Canonical URL to the version-specific TestScript that was executed to produce this TestReport.
    pub test_script: types::Canonical,
    /// Primitive extension sibling for [`test_script`](Self::test_script) (FHIR `_testScript`).
    #[serde(rename = "_testScript")]
    pub test_script_ext: Option<types::Element>,

    /// Overall outcome of the executed test run: pass | fail | pending.
    pub result: crate::r5::coded::Coded<crate::r5::codes::ReportResultCodes>,
    /// Primitive extension sibling for [`result`](Self::result) (FHIR `_result`).
    #[serde(rename = "_result")]
    pub result_ext: Option<types::Element>,

    /// The final score (percentage of tests passed) resulting from the execution of the TestScript
    pub score: Option<types::Decimal>,
    /// Primitive extension sibling for [`score`](Self::score) (FHIR `_score`).
    #[serde(rename = "_score")]
    pub score_ext: Option<types::Element>,

    /// Name of the tester producing this report (Organization or individual)
    pub tester: Option<types::String>,
    /// Primitive extension sibling for [`tester`](Self::tester) (FHIR `_tester`).
    #[serde(rename = "_tester")]
    pub tester_ext: Option<types::Element>,

    /// The date and time at which the TestScript was executed and this TestReport was generated.
    pub issued: Option<types::DateTime>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`).
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// A participant in the test execution, either the execution engine, a client, or a server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<TestReportParticipant>,

    /// The results of the series of required setup operations before the tests were executed
    pub setup: Option<TestReportSetup>,

    /// A test executed from the test script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test: Vec<TestReportTest>,

    /// The results of running the series of required clean up steps
    pub teardown: Option<TestReportTeardown>,
}

/// A participant in the test execution, either the execution engine, a client, or a server.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReportParticipant;
/// use fhir::r5::types;
///
/// let value = TestReportParticipant {
///     display: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `display` is the name this serializes to on the wire.
/// assert_eq!(json["display"], ::serde_json::json!("abc"));
///
/// let back: TestReportParticipant = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// test-engine | client | server
    pub r#type: crate::r5::coded::Coded<crate::r5::codes::ReportParticipantType>,
    /// Primitive extension sibling for [`type`](Self::r#type) (FHIR `_type`).
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The uri of the participant. An absolute URL is preferred
    pub uri: types::Uri,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`).
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// The display name of the participant
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`).
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,
}

/// The results of the series of required setup operations before the tests were executed.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::test_report::TestReportSetup;
///
/// let value = TestReportSetup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestReportSetup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetup {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// A setup operation or assert that was executed
    pub action: vec1::Vec1<TestReportSetupAction>,
}

/// A setup operation or assert that was executed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReportSetupAction;
/// use fhir::r5::types;
///
/// let value = TestReportSetupAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestReportSetupAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetupAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The operation to perform
    pub operation: Option<TestReportSetupActionOperation>,

    /// The assertion to perform
    pub assert: Option<TestReportSetupActionAssert>,
}

/// The operation to perform.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReportSetupActionOperation;
/// use fhir::r5::types;
///
/// let value = TestReportSetupActionOperation {
///     message: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `message` is the name this serializes to on the wire.
/// assert_eq!(json["message"], ::serde_json::json!("# Heading"));
///
/// let back: TestReportSetupActionOperation = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetupActionOperation {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// pass | skip | fail | warning | error
    pub result: crate::r5::coded::Coded<crate::r5::codes::ReportActionResultCodes>,
    /// Primitive extension sibling for [`result`](Self::result) (FHIR `_result`).
    #[serde(rename = "_result")]
    pub result_ext: Option<types::Element>,

    /// A message associated with the result
    pub message: Option<types::Markdown>,
    /// Primitive extension sibling for [`message`](Self::message) (FHIR `_message`).
    #[serde(rename = "_message")]
    pub message_ext: Option<types::Element>,

    /// A link to further details on the result
    pub detail: Option<types::Uri>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`).
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,
}

/// The assertion to perform.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReportSetupActionAssert;
/// use fhir::r5::types;
///
/// let value = TestReportSetupActionAssert {
///     message: Some(types::Markdown("# Heading".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `message` is the name this serializes to on the wire.
/// assert_eq!(json["message"], ::serde_json::json!("# Heading"));
///
/// let back: TestReportSetupActionAssert = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetupActionAssert {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// pass | skip | fail | warning | error
    pub result: crate::r5::coded::Coded<crate::r5::codes::ReportActionResultCodes>,
    /// Primitive extension sibling for [`result`](Self::result) (FHIR `_result`).
    #[serde(rename = "_result")]
    pub result_ext: Option<types::Element>,

    /// A message associated with the result
    pub message: Option<types::Markdown>,
    /// Primitive extension sibling for [`message`](Self::message) (FHIR `_message`).
    #[serde(rename = "_message")]
    pub message_ext: Option<types::Element>,

    /// A link to further details on the result
    pub detail: Option<types::String>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`).
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,

    /// Links or references to the testing requirements
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requirement: Vec<TestReportSetupActionAssertRequirement>,
}

/// Links or references to the testing requirements.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReportSetupActionAssertRequirement;
/// use fhir::r5::types;
///
/// let value = TestReportSetupActionAssertRequirement {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestReportSetupActionAssertRequirement = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportSetupActionAssertRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `TestReport.setup.action.assert.requirement.link[x]` choice element (0..1); see [`TestReportSetupActionAssertRequirementLink`].
    #[serde(flatten)]
    pub link: Option<TestReportSetupActionAssertRequirementLink>,
}

/// A test executed from the test script.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::test_report::TestReportTest;
///
/// let value = TestReportTest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestReportTest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTest {
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

    /// A test operation or assert that was performed
    pub action: vec1::Vec1<TestReportTestAction>,
}

/// A test operation or assert that was performed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReportTestAction;
/// use fhir::r5::types;
///
/// let value = TestReportTestAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestReportTestAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTestAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The operation performed
    pub operation: Option<TestReportSetupActionOperation>,

    /// The assertion performed
    pub assert: Option<TestReportSetupActionAssert>,
}

/// The results of running the series of required clean up steps.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::test_report::TestReportTeardown;
///
/// let value = TestReportTeardown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestReportTeardown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTeardown {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// One or more teardown operations performed
    pub action: vec1::Vec1<TestReportTeardownAction>,
}

/// One or more teardown operations performed.
/// # Examples
///
/// ```
/// use fhir::r5::resources::test_report::TestReportTeardownAction;
/// use fhir::r5::types;
///
/// let value = TestReportTeardownAction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: TestReportTeardownAction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TestReportTeardownAction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The teardown operation performed
    pub operation: TestReportSetupActionOperation,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = TestReport;

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
/// The `TestReport.setup.action.assert.requirement.link[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum TestReportSetupActionAssertRequirementLink {
    /// `linkUri` variant.
    #[fhir("linkUri")]
    Uri(crate::r5::choice::Primitive<types::Uri>),
    /// `linkCanonical` variant.
    #[fhir("linkCanonical")]
    Canonical(crate::r5::choice::Primitive<types::Canonical>),
}
