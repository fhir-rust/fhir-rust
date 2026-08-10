//! Procedure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Procedure
//!
//! Version: 5.0.0
//!
//! Procedure Resource: An action that is or was performed on or for a patient, practitioner, device, organization, or location.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// An action that is or was performed on or for a patient, practitioner, device,
/// organization, or location. For example, this can be a physical intervention on
/// a patient like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy. It can also be a quality or safety inspection for
/// a location, organization, or device, or an accreditation procedure on a
/// practitioner.
///
/// In FHIR R5 the Procedure resource captures the clinical or administrative record
/// of an activity that has been carried out, is in progress, or was planned but not
/// done. It records what was done, to or for whom, by whom, when, where, and why,
/// along with any outcomes, reports, complications, and follow-up. Because it spans
/// everything from surgical operations and diagnostic interventions to counseling,
/// physiotherapy, and non-clinical inspections and accreditations, its scope is
/// deliberately broad; the specific activity is conveyed by the coded `code` field
/// rather than by many distinct resource types. A Procedure is typically fulfilled
/// from an order such as a ServiceRequest referenced by `based_on`, is situated
/// within an encounter, and may reference the diagnostic reports, observations, or
/// conditions that justify or result from it.
///
/// Related resources: the `subject` and `focus` are commonly a
/// [`Patient`](crate::r5::resources::patient::Patient); the activity is described
/// with a [`CodeableConcept`](crate::r5::types::CodeableConcept); it is often
/// requested by a `ServiceRequest` and set in the context of an `Encounter`, and
/// its performers and locations are given as
/// [`Reference`](crate::r5::types::Reference) values.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::procedure::Procedure;
/// use fhir::r5::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Procedure {
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

    /// External Identifiers for this procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`).
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`).
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// A request for this procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Required status of the activity in its lifecycle: preparation, in-progress, not-done, on-hold, stopped, completed, entered-in-error, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// Classification of the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// Coded identification of the specific activity performed, typically drawn from a clinical terminology such as SNOMED CT.
    pub code: Option<types::CodeableConcept>,

    /// Required reference to the individual or entity the procedure was performed on or for, most often a patient.
    pub subject: types::Reference,

    /// Who is the target of the procedure when it is not the subject of record only
    pub focus: Option<types::Reference>,

    /// The Encounter during which this Procedure was created
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `Procedure.occurrence[x]` choice element (0..1); see [`ProcedureOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ProcedureOccurrence>,

    /// When the procedure was first captured in the subject's record
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`).
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Who recorded the procedure
    pub recorder: Option<types::Reference>,

    /// The `Procedure.reported[x]` choice element (0..1); see [`ProcedureReported`].
    #[serde(flatten)]
    pub reported: Option<ProcedureReported>,

    /// The people or devices that carried out the procedure and the role each played, as described by ProcedurePerformer.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ProcedurePerformer>,

    /// Where the procedure happened
    pub location: Option<types::Reference<crate::r5::resources::Location>>,

    /// The justification that the procedure was performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Target body sites
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// The result of procedure
    pub outcome: Option<types::CodeableConcept>,

    /// Any report resulting from the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report: Vec<types::Reference>,

    /// Complication following the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub complication: Vec<types::CodeableReference>,

    /// Instructions for follow up
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub follow_up: Vec<types::CodeableConcept>,

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

/// Who performed the procedure and what they did.
/// # Examples
///
/// ```
/// use fhir::r5::resources::procedure::ProcedurePerformer;
/// use fhir::r5::types;
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
    pub on_behalf_of: Option<types::Reference<crate::r5::resources::Organization>>,

    /// When the performer performed the procedure
    pub period: Option<types::Period>,
}

/// Manipulated, implanted, or removed device.
/// # Examples
///
/// ```
/// use fhir::r5::resources::procedure::ProcedureFocalDevice;
/// use fhir::r5::types;
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
    pub manipulated: types::Reference<crate::r5::resources::Device>,
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
/// The `Procedure.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceString` variant.
    #[fhir("occurrenceString")]
    String(crate::r5::choice::Primitive<types::String>),
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

/// The `Procedure.reported[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureReported {
    /// `reportedBoolean` variant.
    #[fhir("reportedBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `reportedReference` variant.
    #[fhir("reportedReference")]
    Reference(Box<types::Reference>),
}
