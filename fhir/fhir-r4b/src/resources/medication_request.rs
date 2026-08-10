//! MedicationRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
//!
//! Version: 4.3.0
//!
//! Ordering of medication for patient or group
//!
//! FHIR R4B: <https://hl7.org/fhir/R4B/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4b::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// An order or request for both supply of the medication and the instructions
/// for administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or
/// "MedicationOrder" to generalize the use across inpatient and outpatient
/// settings, including care plans, etc., and to harmonize with workflow
/// patterns.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::medication_request::MedicationRequest;
/// use fhir::r4b::types;
///
/// let value = MedicationRequest {
///     do_not_perform: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doNotPerform` is the name this serializes to on the wire.
/// assert_eq!(json["doNotPerform"], ::serde_json::json!(true));
///
/// let back: MedicationRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "MedicationRequestDe")]
#[fhir_version("r4b")]
pub struct MedicationRequest {
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
    pub contained: Vec<crate::r4b::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External ids for this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// active | on-hold | cancelled | completed | entered-in-error | stopped |
    /// draft | unknown
    pub status: crate::coded::Coded<crate::r4b::codes::MedicationrequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Reason for current status
    pub status_reason: Option<types::CodeableConcept>,

    /// proposal | plan | order | original-order | reflex-order | filler-order
    /// | instance-order | option
    pub intent: crate::coded::Coded<crate::r4b::codes::MedicationrequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Type of medication usage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r4b::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if request is prohibiting action
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// Reported rather than primary record
    /// The `MedicationRequest.reported[x]` choice element (0..1); see [`MedicationRequestReported`].
    #[serde(flatten)]
    pub reported: Option<MedicationRequestReported>,

    /// Medication to be taken
    /// The `MedicationRequest.medication[x]` choice element (1..1); see [`MedicationRequestMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationRequestMedication>,

    /// Who or group medication request is for
    pub subject: types::Reference,

    /// Encounter created as part of encounter/admission/stay
    pub encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,

    /// Information to support ordering of the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// When request was initially authored
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Who/What requested the Request
    pub requester: Option<types::Reference>,

    /// Intended performer of administration
    pub performer: Option<types::Reference>,

    /// Desired kind of performer of the medication administration
    pub performer_type: Option<types::CodeableConcept>,

    /// Person who entered the request
    pub recorder: Option<types::Reference>,

    /// Reason or indication for ordering or not ordering the medication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition or observation that supports why the prescription is being
    /// written
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical: Vec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri: Vec<types::Uri>,
    /// Primitive extension sibling for [`instantiates_uri`](Self::instantiates_uri) (FHIR `_instantiatesUri`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesUri")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_uri_ext: Vec<Option<types::Element>>,

    /// What request fulfills
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// Overall pattern of medication administration
    pub course_of_therapy_type: Option<types::CodeableConcept>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Information about the prescription
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// How the medication should be taken
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dosage_instruction: Vec<types::Dosage>,

    /// Medication supply authorization
    pub dispense_request: Option<MedicationRequestDispenseRequest>,

    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationRequestSubstitution>,

    /// An order/prescription that is being replaced
    pub prior_prescription: Option<types::Reference<crate::r4b::resources::MedicationRequest>>,

    /// Clinical Issue with action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detected_issue: Vec<types::Reference<crate::r4b::resources::DetectedIssue>>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference<crate::r4b::resources::Provenance>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationRequestDe {
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
    contained: Vec<crate::r4b::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: crate::coded::Coded<crate::r4b::codes::MedicationrequestStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    status_reason: Option<types::CodeableConcept>,
    intent: crate::coded::Coded<crate::r4b::codes::MedicationrequestIntent>,
    #[serde(rename = "_intent")]
    intent_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    priority: Option<crate::coded::Coded<crate::r4b::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    do_not_perform: Option<types::Boolean>,
    #[serde(rename = "_doNotPerform")]
    do_not_perform_ext: Option<types::Element>,
    #[serde(flatten)]
    reported: crate::r4b::choice::Slot<MedicationRequestReported>,
    #[serde(flatten)]
    medication: crate::r4b::choice::Slot<MedicationRequestMedication>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r4b::resources::Encounter>>,
    #[serde(default)]
    supporting_information: Vec<types::Reference>,
    authored_on: Option<types::DateTime>,
    #[serde(rename = "_authoredOn")]
    authored_on_ext: Option<types::Element>,
    requester: Option<types::Reference>,
    performer: Option<types::Reference>,
    performer_type: Option<types::CodeableConcept>,
    recorder: Option<types::Reference>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference>,
    #[serde(default)]
    instantiates_canonical: Vec<types::Canonical>,
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default)]
    instantiates_canonical_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    instantiates_uri: Vec<types::Uri>,
    #[serde(rename = "_instantiatesUri")]
    #[serde(default)]
    instantiates_uri_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    group_identifier: Option<types::Identifier>,
    course_of_therapy_type: Option<types::CodeableConcept>,
    #[serde(default)]
    insurance: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    dosage_instruction: Vec<types::Dosage>,
    dispense_request: Option<MedicationRequestDispenseRequest>,
    substitution: Option<MedicationRequestSubstitution>,
    prior_prescription: Option<types::Reference<crate::r4b::resources::MedicationRequest>>,
    #[serde(default)]
    detected_issue: Vec<types::Reference<crate::r4b::resources::DetectedIssue>>,
    #[serde(default)]
    event_history: Vec<types::Reference<crate::r4b::resources::Provenance>>,
}

impl ::core::convert::From<MedicationRequestDe> for MedicationRequest {
    fn from(v: MedicationRequestDe) -> Self {
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
            status: v.status,
            status_ext: v.status_ext,
            status_reason: v.status_reason,
            intent: v.intent,
            intent_ext: v.intent_ext,
            category: v.category,
            priority: v.priority,
            priority_ext: v.priority_ext,
            do_not_perform: v.do_not_perform,
            do_not_perform_ext: v.do_not_perform_ext,
            reported: v.reported.0,
            medication: v.medication.0,
            subject: v.subject,
            encounter: v.encounter,
            supporting_information: v.supporting_information,
            authored_on: v.authored_on,
            authored_on_ext: v.authored_on_ext,
            requester: v.requester,
            performer: v.performer,
            performer_type: v.performer_type,
            recorder: v.recorder,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            instantiates_canonical: v.instantiates_canonical,
            instantiates_canonical_ext: v.instantiates_canonical_ext,
            instantiates_uri: v.instantiates_uri,
            instantiates_uri_ext: v.instantiates_uri_ext,
            based_on: v.based_on,
            group_identifier: v.group_identifier,
            course_of_therapy_type: v.course_of_therapy_type,
            insurance: v.insurance,
            note: v.note,
            dosage_instruction: v.dosage_instruction,
            dispense_request: v.dispense_request,
            substitution: v.substitution,
            prior_prescription: v.prior_prescription,
            detected_issue: v.detected_issue,
            event_history: v.event_history,
        }
    }
}

/// Indicates the specific details for the dispense or medication supply part
/// of a medication request (also known as a Medication Prescription or
/// Medication Order). Note that this information is not always sent with the
/// order. There may be in some settings (e.g. hospitals) institutional or
/// system support for completing the dispense details in the pharmacy
/// department.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::medication_request::MedicationRequestDispenseRequest;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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
    /// Primitive extension sibling for [`number_of_repeats_allowed`](Self::number_of_repeats_allowed) (FHIR `_numberOfRepeatsAllowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfRepeatsAllowed")]
    pub number_of_repeats_allowed_ext: Option<types::Element>,

    /// Amount of medication to supply per dispense
    pub quantity: Option<types::Quantity>,

    /// Number of days supply per dispense
    pub expected_supply_duration: Option<types::Duration>,

    /// Intended dispenser
    pub performer: Option<types::Reference<crate::r4b::resources::Organization>>,
}

/// Indicates the quantity or duration for the first dispense of the
/// medication.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::medication_request::MedicationRequestDispenseRequestInitialFill;
/// use fhir::r4b::types;
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
#[fhir_version("r4b")]
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

/// Indicates whether or not substitution can or should be part of the
/// dispense. In some cases, substitution must happen, in other cases
/// substitution must not happen. This block explains the prescriber's intent.
/// If nothing is specified substitution may be done.
///
/// # Examples
///
/// ```
/// use fhir::r4b::resources::medication_request::MedicationRequestSubstitution;
/// use fhir::r4b::types;
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
#[serde(from = "MedicationRequestSubstitutionDe")]
#[fhir_version("r4b")]
pub struct MedicationRequestSubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether substitution is allowed or not
    /// The `MedicationRequest.substitution.allowed[x]` choice element (1..1); see [`MedicationRequestSubstitutionAllowed`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub allowed: Option<MedicationRequestSubstitutionAllowed>,

    /// Why should (not) substitution be made
    pub reason: Option<types::CodeableConcept>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct MedicationRequestSubstitutionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    allowed: crate::r4b::choice::Slot<MedicationRequestSubstitutionAllowed>,
    reason: Option<types::CodeableConcept>,
}

impl ::core::convert::From<MedicationRequestSubstitutionDe> for MedicationRequestSubstitution {
    fn from(v: MedicationRequestSubstitutionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            allowed: v.allowed.0,
            reason: v.reason,
        }
    }
}

/// The `MedicationRequest.reported[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationRequestReported {
    /// `reportedBoolean` variant.
    #[fhir("reportedBoolean")]
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `reportedReference` variant.
    #[fhir("reportedReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationRequest.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationRequestMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
}

/// The `MedicationRequest.substitution.allowed[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4b")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationRequestSubstitutionAllowed {
    /// `allowedBoolean` variant.
    #[fhir("allowedBoolean")]
    Boolean(crate::r4b::choice::Primitive<types::Boolean>),
    /// `allowedCodeableConcept` variant.
    #[fhir("allowedCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
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
