//! OperationOutcome
//!
//! URL: http://hl7.org/fhir/StructureDefinition/OperationOutcome
//!
//! Version: 6.0.0-ballot3
//!
//! Information about the success/failure of an action
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A collection of error, warning, or information messages that result from a
/// system action.
///
/// # Examples
///
/// ```ignore
/// use fhir::r6::resources::operation_outcome::OperationOutcome;
///
/// let value = OperationOutcome::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: OperationOutcome = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct OperationOutcome {
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

    /// A single issue associated with the action
    pub issue: ::vec1::Vec1<OperationOutcomeIssue>,
}

/// An error, warning, or information message that results from a system
/// action.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::operation_outcome::OperationOutcomeIssue;
/// use fhir::r6::types;
///
/// let value = OperationOutcomeIssue {
///     diagnostics: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `diagnostics` is the name this serializes to on the wire.
/// assert_eq!(json["diagnostics"], ::serde_json::json!("abc"));
///
/// let back: OperationOutcomeIssue = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct OperationOutcomeIssue {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// fatal | error | warning | information | success
    pub severity: crate::coded::Coded<crate::r6::codes::IssueSeverity>,
    /// Primitive extension sibling for [`severity`](Self::severity) (FHIR `_severity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_severity")]
    pub severity_ext: Option<types::Element>,

    /// Error or warning code
    pub code: crate::coded::Coded<crate::r6::codes::IssueType>,
    /// Primitive extension sibling for [`code`](Self::code) (FHIR `_code`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_code")]
    pub code_ext: Option<types::Element>,

    /// Additional details about the error
    pub details: Option<types::CodeableConcept>,

    /// Additional diagnostic information about the issue
    pub diagnostics: Option<types::String>,
    /// Primitive extension sibling for [`diagnostics`](Self::diagnostics) (FHIR `_diagnostics`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_diagnostics")]
    pub diagnostics_ext: Option<types::Element>,

    /// Deprecated: Path of element(s) related to issue
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub location: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`location`](Self::location) (FHIR `_location`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_location")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location_ext: Vec<Option<types::Element>>,

    /// FHIRPath of element(s) related to issue
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub expression: ::fhir_core::PrimVec<types::String>,
    /// Primitive extension sibling for [`expression`](Self::expression) (FHIR `_expression`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_expression")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub expression_ext: Vec<Option<types::Element>>,
}
