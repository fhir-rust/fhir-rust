//! MedicationRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/MedicationRequest
//!
//!
//!
//! Ordering of medication for patient or group
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for MedicationRequest Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication_request::MedicationRequest;
/// use fhir::r3::types;
///
/// let value = MedicationRequest {
///     authored_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authoredOn` is the name this serializes to on the wire.
/// assert_eq!(json["authoredOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: MedicationRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MedicationRequest {
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

    /// External ids for this request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// What request fulfills
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// Composite request this is part of
    pub group_identifier: Option<types::Identifier>,

    /// active | on-hold | cancelled | completed | entered-in-error | stopped |
    /// draft | unknown
    pub status: Option<crate::coded::Coded<crate::r3::codes::MedicationRequestStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | order | instance-order
    pub intent: crate::coded::Coded<crate::r3::codes::MedicationRequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Type of medication usage
    pub category: Option<types::CodeableConcept>,

    /// routine | urgent | stat | asap
    pub priority: Option<crate::coded::Coded<crate::r3::codes::MedicationRequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Medication to be taken
    /// The `MedicationRequest.medication[x]` choice element (1..1); see [`MedicationRequestMedication`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub medication: Option<MedicationRequestMedication>,

    /// Who or group medication request is for
    pub subject: types::Reference,

    /// Created during encounter/admission/stay
    pub context: Option<types::Reference>,

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
    pub requester: Option<MedicationRequestRequester>,

    /// Person who entered the request
    pub recorder: Option<types::Reference<crate::r3::resources::Practitioner>>,

    /// Reason or indication for writing the prescription
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Condition or Observation that supports why the prescription is being
    /// written
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

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
    pub prior_prescription: Option<types::Reference<crate::r3::resources::MedicationRequest>>,

    /// Clinical Issue with action
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detected_issue: Vec<types::Reference<crate::r3::resources::DetectedIssue>>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_history: Vec<types::Reference<crate::r3::resources::Provenance>>,
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
/// use fhir::r3::resources::medication_request::MedicationRequestDispenseRequest;
/// use fhir::r3::types;
///
/// let value = MedicationRequestDispenseRequest {
///     number_of_repeats_allowed: Some(types::PositiveInt(1)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `numberOfRepeatsAllowed` is the name this serializes to on the wire.
/// assert_eq!(json["numberOfRepeatsAllowed"], ::serde_json::json!(1));
///
/// let back: MedicationRequestDispenseRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MedicationRequestDispenseRequest {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Time period supply is authorized for
    pub validity_period: Option<types::Period>,

    /// Number of refills authorized
    pub number_of_repeats_allowed: Option<types::PositiveInt>,
    /// Primitive extension sibling for [`number_of_repeats_allowed`](Self::number_of_repeats_allowed) (FHIR `_numberOfRepeatsAllowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_numberOfRepeatsAllowed")]
    pub number_of_repeats_allowed_ext: Option<types::Element>,

    /// Amount of medication to supply per dispense
    pub quantity: Option<types::Quantity>,

    /// Number of days supply per dispense
    pub expected_supply_duration: Option<types::Duration>,

    /// Intended dispenser
    pub performer: Option<types::Reference<crate::r3::resources::Organization>>,
}

/// The individual, organization or device that initiated the request and has
/// responsibility for its activation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication_request::MedicationRequestRequester;
/// use fhir::r3::types;
///
/// let value = MedicationRequestRequester {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: MedicationRequestRequester = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct MedicationRequestRequester {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who ordered the initial medication(s)
    pub agent: types::Reference,

    /// Organization agent is acting for
    pub on_behalf_of: Option<types::Reference<crate::r3::resources::Organization>>,
}

/// Indicates whether or not substitution can or should be part of the
/// dispense. In some cases substitution must happen, in other cases
/// substitution must not happen. This block explains the prescriber's intent.
/// If nothing is specified substitution may be done.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::medication_request::MedicationRequestSubstitution;
/// use fhir::r3::types;
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
#[fhir_version("r3")]
pub struct MedicationRequestSubstitution {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Whether substitution is allowed or not
    pub allowed: types::Boolean,
    /// Primitive extension sibling for [`allowed`](Self::allowed) (FHIR `_allowed`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_allowed")]
    pub allowed_ext: Option<types::Element>,

    /// Why should (not) substitution be made
    pub reason: Option<types::CodeableConcept>,
}

/// The `MedicationRequest.medication[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum MedicationRequestMedication {
    /// `medicationCodeableConcept` variant.
    #[fhir("medicationCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `medicationReference` variant.
    #[fhir("medicationReference")]
    Reference(Box<types::Reference>),
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
