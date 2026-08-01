//! ProcedureRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ProcedureRequest
//!
//!
//!
//! A request for a procedure to be performed
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for ProcedureRequest Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::procedure_request::ProcedureRequest;
/// use fhir::r2::types;
///
/// let value = ProcedureRequest {
///     ordered_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `orderedOn` is the name this serializes to on the wire.
/// assert_eq!(json["orderedOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: ProcedureRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ProcedureRequest {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Unique identifier for the request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who the procedure should be done to
    pub subject: types::Reference,

    /// What procedure to perform
    pub code: types::CodeableConcept,

    /// What part of body to perform on
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// Why procedure should occur
    /// The `ProcedureRequest.reason[x]` choice element (0..1); see [`ProcedureRequestReason`].
    #[serde(flatten)]
    pub reason: Option<ProcedureRequestReason>,

    /// When procedure should occur
    /// The `ProcedureRequest.scheduled[x]` choice element (0..1); see [`ProcedureRequestScheduled`].
    #[serde(flatten)]
    pub scheduled: Option<ProcedureRequestScheduled>,

    /// Encounter request created during
    pub encounter: Option<types::Reference>,

    /// Who should perform the procedure
    pub performer: Option<types::Reference>,

    /// proposed | draft | requested | received | accepted | in-progress |
    /// completed | suspended | rejected | aborted
    pub status: Option<crate::coded::Coded<crate::r2::codes::ProcedureRequestStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Additional information about desired procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<types::Annotation>,

    /// Preconditions for procedure
    /// The `ProcedureRequest.asNeeded[x]` choice element (0..1); see [`ProcedureRequestAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<ProcedureRequestAsNeeded>,

    /// When request was created
    pub ordered_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`ordered_on`](Self::ordered_on) (FHIR `_orderedOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_orderedOn")]
    pub ordered_on_ext: Option<types::Element>,

    /// Who made request
    pub orderer: Option<types::Reference>,

    /// routine | urgent | stat | asap
    pub priority: Option<crate::coded::Coded<crate::r2::codes::ProcedureRequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,
}

/// The `ProcedureRequest.reason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureRequestReason {
    /// `reasonCodeableConcept` variant.
    #[fhir("reasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonReference` variant.
    #[fhir("reasonReference")]
    Reference(Box<types::Reference>),
}

/// The `ProcedureRequest.scheduled[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureRequestScheduled {
    /// `scheduledDateTime` variant.
    #[fhir("scheduledDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `scheduledPeriod` variant.
    #[fhir("scheduledPeriod")]
    Period(Box<types::Period>),
    /// `scheduledTiming` variant.
    #[fhir("scheduledTiming")]
    Timing(Box<types::Timing>),
}

/// The `ProcedureRequest.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureRequestAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r2::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ProcedureRequest;

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
