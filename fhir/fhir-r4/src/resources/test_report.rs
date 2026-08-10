//! TestReport
//!
//! URL: http://hl7.org/fhir/StructureDefinition/TestReport
//!
//! Version: 4.0.1
//!
//! Describes the results of a TestScript execution
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A summary of information based on the results of executing a TestScript.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::test_report::TestReport;
/// use fhir::r4::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
pub struct TestReport {
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

    /// External identifier
    pub identifier: Option<types::Identifier>,

    /// Informal name of the executed TestScript
    pub name: Option<types::String>,
    /// Primitive extension sibling for [`name`](Self::name) (FHIR `_name`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_name")]
    pub name_ext: Option<types::Element>,

    /// completed | in-progress | waiting | stopped | entered-in-error
    pub status: crate::coded::Coded<crate::r4::codes::ReportStatusCodes>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reference to the version-specific TestScript that was executed to
    /// produce this TestReport
    pub test_script: types::Reference<crate::r4::resources::TestScript>,

    /// pass | fail | pending
    pub result: crate::coded::Coded<crate::r4::codes::ReportResultCodes>,
    /// Primitive extension sibling for [`result`](Self::result) (FHIR `_result`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_result")]
    pub result_ext: Option<types::Element>,

    /// The final score (percentage of tests passed) resulting from the
    /// execution of the TestScript
    pub score: Option<types::Decimal>,
    /// Primitive extension sibling for [`score`](Self::score) (FHIR `_score`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_score")]
    pub score_ext: Option<types::Element>,

    /// Name of the tester producing this report (Organization or individual)
    pub tester: Option<types::String>,
    /// Primitive extension sibling for [`tester`](Self::tester) (FHIR `_tester`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_tester")]
    pub tester_ext: Option<types::Element>,

    /// When the TestScript was executed and this TestReport was generated
    pub issued: Option<types::DateTime>,
    /// Primitive extension sibling for [`issued`](Self::issued) (FHIR `_issued`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_issued")]
    pub issued_ext: Option<types::Element>,

    /// A participant in the test execution, either the execution engine, a
    /// client, or a server
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participant: Vec<TestReportParticipant>,

    /// The results of the series of required setup operations before the tests
    /// were executed
    pub setup: Option<TestReportSetup>,

    /// A test executed from the test script
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test: Vec<TestReportTest>,

    /// The results of running the series of required clean up steps
    pub teardown: Option<TestReportTeardown>,
}

/// A participant in the test execution, either the execution engine, a client,
/// or a server.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::test_report::TestReportParticipant;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub r#type: crate::coded::Coded<crate::r4::codes::ReportParticipantType>,
    /// Primitive extension sibling for [`r#type`](Self::r#type) (FHIR `_type`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_type")]
    pub type_ext: Option<types::Element>,

    /// The uri of the participant. An absolute URL is preferred
    pub uri: types::Uri,
    /// Primitive extension sibling for [`uri`](Self::uri) (FHIR `_uri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_uri")]
    pub uri_ext: Option<types::Element>,

    /// The display name of the participant
    pub display: Option<types::String>,
    /// Primitive extension sibling for [`display`](Self::display) (FHIR `_display`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_display")]
    pub display_ext: Option<types::Element>,
}

/// The results of the series of required setup operations before the tests
/// were executed.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::test_report::TestReportSetup;
///
/// let value = TestReportSetup::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestReportSetup = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub action: ::vec1::Vec1<TestReportSetupAction>,
}

/// Action would contain either an operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::test_report::TestReportSetupAction;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// The results of the assertion performed on the previous operations.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::test_report::TestReportSetupActionAssert;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub result: crate::coded::Coded<crate::r4::codes::ReportActionResultCodes>,
    /// Primitive extension sibling for [`result`](Self::result) (FHIR `_result`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_result")]
    pub result_ext: Option<types::Element>,

    /// A message associated with the result
    pub message: Option<types::Markdown>,
    /// Primitive extension sibling for [`message`](Self::message) (FHIR `_message`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_message")]
    pub message_ext: Option<types::Element>,

    /// A link to further details on the result
    pub detail: Option<types::String>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,
}

/// The operation performed.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::test_report::TestReportSetupActionOperation;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
    pub result: crate::coded::Coded<crate::r4::codes::ReportActionResultCodes>,
    /// Primitive extension sibling for [`result`](Self::result) (FHIR `_result`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_result")]
    pub result_ext: Option<types::Element>,

    /// A message associated with the result
    pub message: Option<types::Markdown>,
    /// Primitive extension sibling for [`message`](Self::message) (FHIR `_message`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_message")]
    pub message_ext: Option<types::Element>,

    /// A link to further details on the result
    pub detail: Option<types::Uri>,
    /// Primitive extension sibling for [`detail`](Self::detail) (FHIR `_detail`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_detail")]
    pub detail_ext: Option<types::Element>,
}

/// The results of the series of operations required to clean up after all the
/// tests were executed (successfully or otherwise).
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::test_report::TestReportTeardown;
///
/// let value = TestReportTeardown::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestReportTeardown = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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
    pub action: ::vec1::Vec1<TestReportTeardownAction>,
}

/// The teardown action will only contain an operation.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::test_report::TestReportTeardownAction;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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

/// A test executed from the test script.
///
/// # Examples
///
/// ```ignore
/// use fhir::r4::resources::test_report::TestReportTest;
///
/// let value = TestReportTest::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: TestReportTest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r4")]
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

    /// A test operation or assert that was performed
    pub action: ::vec1::Vec1<TestReportTestAction>,
}

/// Action would contain either an operation or an assertion.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::test_report::TestReportTestAction;
/// use fhir::r4::types;
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
#[fhir_version("r4")]
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
