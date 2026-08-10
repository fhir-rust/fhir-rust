//! MedicationAdministration
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationAdministration
//!
//! Version: 5.0.0
//!
//! MedicationAdministration Resource: Describes the event of a patient consuming or otherwise being administered a medication.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// MedicationAdministration records the event of a patient actually consuming or
/// otherwise being administered a medication. In FHIR R5 it represents the point
/// in the medication process where a dose is given, as opposed to being ordered
/// or dispensed, and it captures who received the medication, what was given,
/// when and where it occurred, the dose and route, and who performed the act.
/// An administration can be as simple as swallowing a single tablet or as
/// involved as a long running intravenous infusion recorded over a period of
/// time. Clinically and administratively it supports medication reconciliation,
/// adherence monitoring, billing, safety surveillance, and audit of the
/// medication administration record (MAR). The event ties back to the
/// authorizing request or prescription and to the specific encounter between the
/// patient and the health care practitioner. It can also record that a planned
/// dose was intentionally withheld or wasted by using a status of not-done
/// together with an appropriate statusReason.
///
/// Related resources: the recipient is referenced through [`subject`], typically
/// a [`Patient`](crate::r5::resources::patient::Patient); the context is the
/// [`Encounter`](crate::r5::resources::encounter::Encounter); the substance is
/// described via a [`CodeableReference`](crate::r5::types::CodeableReference) to
/// a [`Medication`](crate::r5::resources::medication::Medication); and the
/// authorization is the associated
/// [`MedicationRequest`](crate::r5::resources::medication_request::MedicationRequest).
/// See also `MedicationDispense` and `MedicationStatement` for other stages of
/// the medication process.
///
/// [`subject`]: MedicationAdministration::subject
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_administration::MedicationAdministration;
/// use fhir::r5::types;
///
/// let value = MedicationAdministration {
///     is_sub_potent: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `isSubPotent` is the name this serializes to on the wire.
/// assert_eq!(json["isSubPotent"], ::serde_json::json!(true));
///
/// let back: MedicationAdministration = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MedicationAdministrationDe")]
pub struct MedicationAdministration {
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

    /// External identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Plan this is fulfilled by this administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference<crate::r5::resources::CarePlan>>,

    /// Part of referenced event
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub part_of: Vec<types::Reference>,

    /// Current state of the administration event, drawn from the required status value set: in-progress, not-done, on-hold, completed, entered-in-error, stopped, or unknown.
    pub status: crate::r5::coded::Coded<crate::r5::codes::MedicationAdminStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason administration not performed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub status_reason: Vec<types::CodeableConcept>,

    /// Type of medication administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// What was administered, given either as a coded medication or as a reference to a Medication resource via a CodeableReference.
    pub medication: types::CodeableReference,

    /// Who received the medication, most commonly a reference to the Patient who was administered the dose.
    pub subject: types::Reference,

    /// Encounter administered as part of
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// Additional information to support administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// The `MedicationAdministration.occurence[x]` choice element (0..1); see [`MedicationAdministrationOccurence`].
    #[serde(flatten)]
    pub occurence: Option<MedicationAdministrationOccurence>,

    /// When the MedicationAdministration was first captured in the subject's record
    pub recorded: Option<types::DateTime>,
    /// Primitive extension sibling for [`recorded`](Self::recorded) (FHIR `_recorded`).
    #[serde(rename = "_recorded")]
    pub recorded_ext: Option<types::Element>,

    /// Full dose was not administered
    pub is_sub_potent: Option<types::Boolean>,
    /// Primitive extension sibling for [`is_sub_potent`](Self::is_sub_potent) (FHIR `_isSubPotent`).
    #[serde(rename = "_isSubPotent")]
    pub is_sub_potent_ext: Option<types::Element>,

    /// Reason full dose was not administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_potent_reason: Vec<types::CodeableConcept>,

    /// Who or what performed the medication administration and what type of performance they did
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<MedicationAdministrationPerformer>,

    /// Concept, condition or observation that supports why the medication was administered
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Request administration performed against
    pub request: Option<types::Reference<crate::r5::resources::MedicationRequest>>,

    /// Device used to administer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::CodeableReference>,

    /// Information about the administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Details of how the medication was taken, including dose amount, site, route, method, and rate of administration.
    pub dosage: Option<MedicationAdministrationDosage>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference<crate::r5::resources::Provenance>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationAdministrationDe {
    id: Option<types::String>,
    meta: Option<types::Meta>,
    implicit_rules: Option<types::Uri>,
    #[serde(rename = "_implicitRules")]
    implicit_rules_ext: Option<types::Element>,
    language: Option<types::Code>,
    #[serde(rename = "_language")]
    language_ext: Option<types::Element>,
    text: Option<types::Narrative>,
    #[serde(default)]
    contained: Vec<crate::r5::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    based_on: Vec<types::Reference<crate::r5::resources::CarePlan>>,
    #[serde(default)]
    part_of: Vec<types::Reference>,
    status: crate::r5::coded::Coded<crate::r5::codes::MedicationAdminStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    status_reason: Vec<types::CodeableConcept>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    medication: types::CodeableReference,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(default)]
    supporting_information: Vec<types::Reference>,
    #[serde(flatten)]
    occurence: crate::r5::choice::Slot<MedicationAdministrationOccurence>,
    recorded: Option<types::DateTime>,
    #[serde(rename = "_recorded")]
    recorded_ext: Option<types::Element>,
    is_sub_potent: Option<types::Boolean>,
    #[serde(rename = "_isSubPotent")]
    is_sub_potent_ext: Option<types::Element>,
    #[serde(default)]
    sub_potent_reason: Vec<types::CodeableConcept>,
    #[serde(default)]
    performer: Vec<MedicationAdministrationPerformer>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    request: Option<types::Reference<crate::r5::resources::MedicationRequest>>,
    #[serde(default)]
    device: Vec<types::CodeableReference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    dosage: Option<MedicationAdministrationDosage>,
    #[serde(default)]
    event_history: Vec<types::Reference<crate::r5::resources::Provenance>>,
}

impl ::core::convert::From<MedicationAdministrationDe> for MedicationAdministration {
    fn from(v: MedicationAdministrationDe) -> Self {
        Self {
            id: v.id,
            meta: v.meta,
            implicit_rules: v.implicit_rules,
            implicit_rules_ext: v.implicit_rules_ext,
            language: v.language,
            language_ext: v.language_ext,
            text: v.text,
            contained: v.contained,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            identifier: v.identifier,
            based_on: v.based_on,
            part_of: v.part_of,
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            category: v.category,
            medication: v.medication,
            subject: v.subject,
            encounter: v.encounter,
            supporting_information: v.supporting_information,
            occurence: v.occurence.0,
            recorded: v.recorded,
            recorded_ext: v.recorded_ext,
            is_sub_potent: v.is_sub_potent,
            is_sub_potent_ext: v.is_sub_potent_ext,
            sub_potent_reason: v.sub_potent_reason,
            performer: v.performer,
            reason: v.reason,
            request: v.request,
            device: v.device,
            note: v.note,
            dosage: v.dosage,
            event_history: v.event_history,
        }
    }
}

/// Who or what performed the medication administration and what type of
/// performance they did.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_administration::MedicationAdministrationPerformer;
/// use fhir::r5::types;
///
/// let value = MedicationAdministrationPerformer {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministrationPerformer = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministrationPerformer {
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

    /// Who or what performed the medication administration
    pub actor: types::CodeableReference,
}

/// Details of how medication was taken.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_administration::MedicationAdministrationDosage;
/// use fhir::r5::types;
///
/// let value = MedicationAdministrationDosage {
///     text: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `text` is the name this serializes to on the wire.
/// assert_eq!(json["text"], ::serde_json::json!("abc"));
///
/// let back: MedicationAdministrationDosage = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MedicationAdministrationDosageDe")]
pub struct MedicationAdministrationDosage {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<types::String>,
    /// Primitive extension sibling for [`text`](Self::text) (FHIR `_text`).
    #[serde(rename = "_text")]
    pub text_ext: Option<types::Element>,

    /// Body site administered to
    pub site: Option<types::CodeableConcept>,

    /// Path of substance into body
    pub route: Option<types::CodeableConcept>,

    /// How drug was administered
    pub method: Option<types::CodeableConcept>,

    /// Amount of medication per dose
    pub dose: Option<types::Quantity>,

    /// The `MedicationAdministration.dosage.rate[x]` choice element (0..1); see [`MedicationAdministrationDosageRate`].
    #[serde(flatten)]
    pub rate: Option<MedicationAdministrationDosageRate>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationAdministrationDosageDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    text: Option<types::String>,
    #[serde(rename = "_text")]
    text_ext: Option<types::Element>,
    site: Option<types::CodeableConcept>,
    route: Option<types::CodeableConcept>,
    method: Option<types::CodeableConcept>,
    dose: Option<types::Quantity>,
    #[serde(flatten)]
    rate: crate::r5::choice::Slot<MedicationAdministrationDosageRate>,
}

impl ::core::convert::From<MedicationAdministrationDosageDe> for MedicationAdministrationDosage {
    fn from(v: MedicationAdministrationDosageDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            text: v.text,
            text_ext: v.text_ext,
            site: v.site,
            route: v.route,
            method: v.method,
            dose: v.dose,
            rate: v.rate.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationAdministration;

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
/// The `MedicationAdministration.dosage.rate[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationDosageRate {
    /// `rateRatio` variant.
    #[fhir("rateRatio")]
    Ratio(Box<types::Ratio>),
    /// `rateQuantity` variant.
    #[fhir("rateQuantity")]
    Quantity(Box<types::Quantity>),
}

/// The `MedicationAdministration.occurence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationAdministrationOccurence {
    /// `occurenceDateTime` variant.
    #[fhir("occurenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurencePeriod` variant.
    #[fhir("occurencePeriod")]
    Period(Box<types::Period>),
    /// `occurenceTiming` variant.
    #[fhir("occurenceTiming")]
    Timing(Box<types::Timing>),
}
