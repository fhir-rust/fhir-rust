//! Procedure
//!
//! URL: http://hl7.org/fhir/StructureDefinition/Procedure
//!
//!
//!
//! An action that is being or was performed on a patient
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for Procedure Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::procedure::Procedure;
/// use fhir::r3::types;
///
/// let value = Procedure {
///     not_done: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `notDone` is the name this serializes to on the wire.
/// assert_eq!(json["notDone"], ::serde_json::json!(true));
///
/// let back: Procedure = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
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
    pub contained: Vec<crate::r3::resources::Resource>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Identifiers for this procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// A request for this procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// preparation | in-progress | suspended | aborted | completed |
    /// entered-in-error | unknown
    pub status: crate::coded::Coded<crate::r3::codes::EventStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// True if procedure was not performed as scheduled
    pub not_done: Option<types::Boolean>,
    /// Primitive extension sibling for [`not_done`](Self::not_done) (FHIR `_notDone`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_notDone")]
    pub not_done_ext: Option<types::Element>,

    /// Reason procedure was not performed
    pub not_done_reason: Option<types::CodeableConcept>,

    /// Classification of the procedure
    pub category: Option<types::CodeableConcept>,

    /// Identification of the procedure
    pub code: Option<types::CodeableConcept>,

    /// Who the procedure was performed on
    pub subject: types::Reference,

    /// Encounter or episode associated with the procedure
    pub context: Option<types::Reference>,

    /// Date/Period the procedure was performed
    /// The `Procedure.performed[x]` choice element (0..1); see [`ProcedurePerformed`].
    #[serde(flatten)]
    pub performed: Option<ProcedurePerformed>,

    /// The people who performed the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<ProcedurePerformer>,

    /// Where the procedure happened
    pub location: Option<types::Reference>,

    /// Coded reason procedure performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition that is the reason the procedure performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

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
    pub complication: Vec<types::CodeableConcept>,

    /// A condition that is a result of the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub complication_detail: Vec<types::Reference>,

    /// Instructions for follow up
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub follow_up: Vec<types::CodeableConcept>,

    /// Additional information about the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Device changed in procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focal_device: Vec<ProcedureFocalDevice>,

    /// Items used during procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub used_reference: Vec<types::Reference>,

    /// Coded items used during the procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub used_code: Vec<types::CodeableConcept>,
}

/// A device that is implanted, removed or otherwise manipulated (calibration,
/// battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as
/// a focal portion of the Procedure.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::procedure::ProcedureFocalDevice;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ProcedureFocalDevice {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

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
/// use fhir::r3::resources::procedure::ProcedurePerformer;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct ProcedurePerformer {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The role the actor was in
    pub role: Option<types::CodeableConcept>,

    /// The reference to the practitioner
    pub actor: types::Reference,

    /// Organization the device or practitioner was acting for
    pub on_behalf_of: Option<types::Reference>,
}

/// The `Procedure.performed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum ProcedurePerformed {
    /// `performedDateTime` variant.
    #[fhir("performedDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
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
