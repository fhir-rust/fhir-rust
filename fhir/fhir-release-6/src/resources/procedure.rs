//! Procedure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Procedure
//!
//! Version: 6.0.0-ballot3
//!
//! An action that is being or was performed on an individual or entity
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An action that is or was performed on or for a patient, practitioner,
/// device, organization, or location. For example, this can be a physical
/// intervention on a patient like an operation, or less invasive like long
/// term services, counseling, or hypnotherapy. This can be a quality or safety
/// inspection for a location, organization, or device. This can be an
/// accreditation procedure on a practitioner for licensing.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::procedure::Procedure;
/// use fhir::r6::types;
///
/// let value = Procedure {
///     recorded: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `recorded` is the name this serializes to on the wire.
/// assert_eq!(json["recorded"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: Procedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct Procedure {
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

    /// External Identifiers for this procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A request for this procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed |
    /// entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r6::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Classification of the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Identification of the procedure
    pub code: Option<types::CodeableConcept>,

    /// Individual or entity the procedure was performed on
    pub subject: types::Reference,

    /// Who is the target of the procedure when it is not the subject of record
    /// only
    pub focus: Option<types::Reference>,

    /// The Encounter during which this Procedure was created
    pub encounter: Option<types::Reference>,

    /// When the procedure occurred or is occurring
    /// The `Procedure.occurrence[x]` choice element (0..1); see [`ProcedureOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ProcedureOccurrence>,

    /// When the procedure was first captured in the subject's record
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Who recorded the procedure
    pub recorder: Option<types::Reference>,

    /// Reported rather than primary record
    /// The `Procedure.reported[x]` choice element (0..1); see [`ProcedureReported`].
    #[serde(flatten)]
    pub reported: Option<ProcedureReported>,

    /// Who performed the procedure and what they did
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ProcedurePerformer>,

    /// Where the procedure happened
    pub location: Option<types::Reference>,

    /// The justification that the procedure was performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Target body sites
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// Target body structure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_structure: Vec<types::Reference>,

    /// The result of procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outcome: Vec<types::CodeableReference>,

    /// Any report resulting from the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report: Vec<types::Reference>,

    /// Complication following the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub complication: Vec<types::CodeableReference>,

    /// Instructions for follow up
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub follow_up: Vec<types::CodeableReference>,

    /// Additional information about the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Manipulated, implanted, or removed device
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focal_device: Vec<ProcedureFocalDevice>,

    /// Items used during procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub used: Vec<types::CodeableReference>,

    /// Extra information relevant to the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,
}

/// A device that is implanted, removed or otherwise manipulated (calibration,
/// battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as
/// a focal portion of the Procedure.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::procedure::ProcedureFocalDevice;
/// use fhir::r6::types;
///
/// let value = ProcedureFocalDevice {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ProcedureFocalDevice = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ProcedureFocalDevice {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Kind of change to device
    pub action: Option<types::CodeableConcept>,

    /// Device that was changed
    pub manipulated: types::Reference,
}

/// Indicates who or what performed the procedure and how they were involved.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::procedure::ProcedurePerformer;
/// use fhir::r6::types;
///
/// let value = ProcedurePerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ProcedurePerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r6")]
pub struct ProcedurePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Type of performance
    pub function: Option<types::CodeableConcept>,

    /// Who performed the procedure
    pub actor: types::Reference,

    /// Organization the device or practitioner was acting for
    pub on_behalf_of: Option<types::Reference>,

    /// When the performer performed the procedure
    pub period: Option<types::Period>,
}

/// The `Procedure.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceString` variant.
    #[fhir("occurrenceString")]
    String(crate::r6::choice::Primitive<types::String>),
    /// `occurrenceAge` variant.
    #[fhir("occurrenceAge")]
    Age(Box<types::Age>),
    /// `occurrenceRange` variant.
    #[fhir("occurrenceRange")]
    Range(Box<types::Range>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `Procedure.reported[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureReported {
    /// `reportedBoolean` variant.
    #[fhir("reportedBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
    /// `reportedReference` variant.
    #[fhir("reportedReference")]
    Reference(Box<types::Reference>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = Procedure;

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
