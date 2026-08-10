//! MedicationRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
//!
//! Version: 5.0.0
//!
//! MedicationRequest Resource: An order or request for both supply of the medication and the instructions for administration of the medication to a patient.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or
/// "MedicationOrder" to generalize the use across inpatient and outpatient
/// settings, including care plans, etc., and to harmonize with workflow
/// patterns.
///
/// A MedicationRequest represents an order or request for the supply of a
/// medication and instructions for its administration, and it also serves to
/// document that a patient is expected to be taking a medication (whether or
/// not it was formally ordered) or that treatment with a medication has been
/// stopped. Clinically it is used to capture prescriptions written in
/// outpatient settings as well as medication orders placed during an
/// inpatient encounter, and it drives downstream workflows such as pharmacy
/// dispensing (see `MedicationDispense`), medication administration records
/// (see `MedicationAdministration`), and the patient's overall medication
/// list. The resource tracks the requester, the intended subject, the
/// dosage and timing instructions, dispense authorization details such as
/// refills and validity period, and any substitution restrictions, allowing
/// systems to reason about the full lifecycle of a medication order from
/// authoring through fulfillment.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — typically referenced by `subject` as the individual for whom the medication is requested.
/// - [`Encounter`](crate::r5::resources::encounter::Encounter) — the clinical encounter during which the request was created, referenced by `encounter`.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used throughout for coded values such as `status_reason`, `category`, and `performer_type`.
/// - `MedicationDispense` and `MedicationAdministration` — related resources that record fulfillment of this request.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_request::MedicationRequest;
/// use fhir::r5::types;
///
/// let value = MedicationRequest {
///     status_changed: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `statusChanged` is the name this serializes to on the wire.
/// assert_eq!(json["statusChanged"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicationRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequest {
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

    /// External ids for this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// A plan or request that is fulfilled in whole or in part by this medication request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Reference to an order/prescription that is being replaced by this MedicationRequest
    pub prior_prescription: Option<types::Reference<crate::r5::resources::MedicationRequest>>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// The current lifecycle status of the request: active | on-hold | ended | stopped | completed | cancelled | entered-in-error | draft | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::MedicationrequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// When the status was changed
    pub status_changed: Option<types::DateTime>,
    /// Primitive extension sibling for [`status_changed`](Self::status_changed) (FHIR `_statusChanged`).
    #[serde(rename = "_statusChanged")]
    pub status_changed_ext: Option<types::Element>,

    /// Whether the request represents a proposal, plan, or an actionable order: proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: crate::r5::coded::Coded<crate::r5::codes::MedicationrequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Grouping or category of medication request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if patient is to stop taking or not to start taking the medication
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`).
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// The medication being ordered, referenced or coded via a `CodeableReference`
    pub medication: types::CodeableReference,

    /// The individual or group for whom the medication has been requested, typically a [`Patient`](crate::r5::resources::patient::Patient)
    pub subject: types::Reference,

    /// The person or organization who provided the information about this request, if the source is someone other than the requestor
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub information_source: Vec<types::Reference>,

    /// Encounter created as part of encounter/admission/stay
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// Information to support fulfilling of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// When request was initially authored
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Who/What requested the Request
    pub requester: Option<types::Reference>,

    /// Reported rather than primary record
    pub reported: Option<types::Boolean>,
    /// Primitive extension sibling for [`reported`](Self::reported) (FHIR `_reported`).
    #[serde(rename = "_reported")]
    pub reported_ext: Option<types::Element>,

    /// Desired kind of performer of the medication administration
    pub performer_type: Option<types::CodeableConcept>,

    /// Intended performer of administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// Intended type of device for the administration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub device: Vec<types::CodeableReference>,

    /// Person who entered the request
    pub recorder: Option<types::Reference>,

    /// Reason or indication for ordering or not ordering the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Overall pattern of medication administration
    pub course_of_therapy_type: Option<types::CodeableConcept>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Information about the prescription
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Full representation of the dosage instructions
    pub rendered_dosage_instruction: Option<types::Markdown>,
    /// Primitive extension sibling for [`rendered_dosage_instruction`](Self::rendered_dosage_instruction) (FHIR `_renderedDosageInstruction`).
    #[serde(rename = "_renderedDosageInstruction")]
    pub rendered_dosage_instruction_ext: Option<types::Element>,

    /// Period over which the medication is to be taken
    pub effective_dose_period: Option<types::Period>,

    /// Specific instructions for how the medication should be taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage_instruction: Vec<types::Dosage>,

    /// Medication supply authorization
    pub dispense_request: Option<MedicationRequestDispenseRequest>,

    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationRequestSubstitution>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference<crate::r5::resources::Provenance>>,
}

/// Medication supply authorization.
///
/// Indicates the specific details for the dispense or medication supply part
/// of a medication request (also known as a Medication Prescription or
/// Medication Order).
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_request::MedicationRequestDispenseRequest;
/// use fhir::r5::types;
///
/// let value = MedicationRequestDispenseRequest {
///     number_of_repeats_allowed: Some(types::UnsignedInt(0)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfRepeatsAllowed` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfRepeatsAllowed"], ::serde_json::json!(0));
///
/// let back: MedicationRequestDispenseRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestDispenseRequest {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// First fill details
    pub initial_fill: Option<MedicationRequestDispenseRequestInitialFill>,

    /// Minimum period of time between dispenses
    pub dispense_interval: Option<types::Duration>,

    /// Time period supply is authorized for
    pub validity_period: Option<types::Period>,

    /// Number of refills authorized
    pub number_of_repeats_allowed: Option<types::UnsignedInt>,
    /// Primitive extension sibling for [`number_of_repeats_allowed`](Self::number_of_repeats_allowed) (FHIR `_numberOfRepeatsAllowed`).
    #[serde(rename = "_numberOfRepeatsAllowed")]
    pub number_of_repeats_allowed_ext: Option<types::Element>,

    /// Amount of medication to supply per dispense
    pub quantity: Option<types::Quantity>,

    /// Number of days supply per dispense
    pub expected_supply_duration: Option<types::Duration>,

    /// Intended performer of dispense
    pub dispenser: Option<types::Reference<crate::r5::resources::Organization>>,

    /// Additional information for the dispenser
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dispenser_instruction: Vec<types::Annotation>,

    /// Type of adherence packaging to use for the dispense
    pub dose_administration_aid: Option<types::CodeableConcept>,
}

/// First fill details.
///
/// Indicates the quantity or duration for the first dispense of the
/// medication.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_request::MedicationRequestDispenseRequestInitialFill;
/// use fhir::r5::types;
///
/// let value = MedicationRequestDispenseRequestInitialFill {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationRequestDispenseRequestInitialFill = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestDispenseRequestInitialFill {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// First fill quantity
    pub quantity: Option<types::Quantity>,

    /// First fill duration
    pub duration: Option<types::Duration>,
}

/// Any restrictions on medication substitution.
///
/// Indicates whether or not substitution can or should be part of the dispense.
/// In some cases, substitution must happen, in other cases substitution must
/// not happen. This block explains the prescriber's intent.
/// # Examples
///
/// ```
/// use fhir::r5::resources::medication_request::MedicationRequestSubstitution;
/// use fhir::r5::types;
///
/// let value = MedicationRequestSubstitution {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationRequestSubstitution = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestSubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `MedicationRequest.substitution.allowed[x]` choice element (0..1); see [`MedicationRequestSubstitutionAllowed`].
    #[serde(flatten)]
    pub allowed: Option<MedicationRequestSubstitutionAllowed>,

    /// Why should (not) substitution be made
    pub reason: Option<types::CodeableConcept>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = MedicationRequest;

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
/// The `MedicationRequest.substitution.allowed[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum MedicationRequestSubstitutionAllowed {
    /// `allowedBoolean` variant.
    #[fhir("allowedBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `allowedCodeableConcept` variant.
    #[fhir("allowedCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}
