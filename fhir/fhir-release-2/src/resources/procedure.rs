//! Procedure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Procedure
//!
//!
//!
//! An action that is being or was performed on a patient
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Procedure Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::procedure::Procedure;
/// use fhir::r2::types;
///
/// let value = Procedure {
///     not_performed: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `notPerformed` is the name this serializes to on the wire.
/// assert_eq!(json["notPerformed"], ::serde_json::json!(true));
///
/// let back: Procedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct Procedure {
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

    /// External Identifiers for this procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Who the procedure was performed on
    pub subject: types::Reference,

    /// in-progress | aborted | completed | entered-in-error
    pub status: crate::coded::Coded<crate::r2::codes::ProcedureStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Classification of the procedure
    pub category: Option<types::CodeableConcept>,

    /// Identification of the procedure
    pub code: types::CodeableConcept,

    /// True if procedure was not performed as scheduled
    pub not_performed: Option<types::Boolean>,
    /// Primitive extension sibling for [`not_performed`](Self::not_performed) (FHIR `_notPerformed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_notPerformed")]
    pub not_performed_ext: Option<types::Element>,

    /// Reason procedure was not performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_not_performed: Vec<types::CodeableConcept>,

    /// Target body sites
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// Reason procedure performed
    /// The `Procedure.reason[x]` choice element (0..1); see [`ProcedureReason`].
    #[serde(flatten)]
    pub reason: Option<ProcedureReason>,

    /// The people who performed the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ProcedurePerformer>,

    /// Date/Period the procedure was performed
    /// The `Procedure.performed[x]` choice element (0..1); see [`ProcedurePerformed`].
    #[serde(flatten)]
    pub performed: Option<ProcedurePerformed>,

    /// The encounter associated with the procedure
    pub encounter: Option<types::Reference>,

    /// Where the procedure happened
    pub location: Option<types::Reference>,

    /// The result of procedure
    pub outcome: Option<types::CodeableConcept>,

    /// Any report resulting from the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report: Vec<types::Reference>,

    /// Complication following the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub complication: Vec<types::CodeableConcept>,

    /// Instructions for follow up
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub follow_up: Vec<types::CodeableConcept>,

    /// A request for this procedure
    pub request: Option<types::Reference>,

    /// Additional information about the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<types::Annotation>,

    /// Device changed in procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focal_device: Vec<ProcedureFocalDevice>,

    /// Items used during procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub used: Vec<types::Reference>,
}

/// A device that is implanted, removed or otherwise manipulated (calibration,
/// battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as
/// a focal portion of the Procedure.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::procedure::ProcedureFocalDevice;
/// use fhir::r2::types;
///
/// let value = ProcedureFocalDevice {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ProcedureFocalDevice = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ProcedureFocalDevice {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Kind of change to device
    pub action: Option<types::CodeableConcept>,

    /// Device that was changed
    pub manipulated: types::Reference,
}

/// Limited to 'real' people rather than equipment.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::procedure::ProcedurePerformer;
/// use fhir::r2::types;
///
/// let value = ProcedurePerformer {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: ProcedurePerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct ProcedurePerformer {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The reference to the practitioner
    pub actor: Option<types::Reference>,

    /// The role the actor was in
    pub role: Option<types::CodeableConcept>,
}

/// The `Procedure.reason[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedureReason {
    /// `reasonCodeableConcept` variant.
    #[fhir("reasonCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `reasonReference` variant.
    #[fhir("reasonReference")]
    Reference(Box<types::Reference>),
}

/// The `Procedure.performed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r2")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedurePerformed {
    /// `performedDateTime` variant.
    #[fhir("performedDateTime")]
    DateTime(crate::r2::choice::Primitive<types::DateTime>),
    /// `performedPeriod` variant.
    #[fhir("performedPeriod")]
    Period(Box<types::Period>),
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
