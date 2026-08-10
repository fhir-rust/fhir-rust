//! ServiceRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
//!
//! Version: 5.0.0
//!
//! ServiceRequest Resource: A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a request for service such as diagnostic investigations,
/// treatments, or operations to be performed.
///
/// The ServiceRequest resource is used to place an order or request for a
/// service to be performed on or for a patient, group, device, or location.
/// Typical services include diagnostic tests, imaging studies, laboratory work,
/// referrals, counseling, and procedures. In FHIR R5 it carries the requested
/// code, subject, timing, requester, performer, and supporting clinical context,
/// and can be linked to fulfilling results and provenance.
///
/// Clinically, a ServiceRequest represents the intent to have a service
/// performed and progresses through a lifecycle described by its `status`
/// (draft, active, on-hold, revoked, completed, entered-in-error, or unknown)
/// and `intent` (proposal, plan, directive, order, and related values). It is
/// the FHIR workflow analogue of a paper or electronic order and is commonly
/// used to drive downstream workflows such as scheduling, specimen collection,
/// imaging acquisition, and result reporting, with the fulfilling actor
/// typically producing a DiagnosticReport, Procedure, or Observation that
/// references the originating request via `basedOn`.
///
/// # See also
///
/// - [`Patient`](crate::r5::resources::patient::Patient) — often the `subject` of the request.
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for `category`, `body_site`, and other coded fields.
/// - `DiagnosticReport`, `Procedure`, and `Observation` — typical resources that fulfill a ServiceRequest.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::service_request::ServiceRequest;
/// use fhir::r5::types;
///
/// let value = ServiceRequest {
///     do_not_perform: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doNotPerform` is the name this serializes to on the wire.
/// assert_eq!(json["doNotPerform"], ::serde_json::json!(true));
///
/// let back: ServiceRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ServiceRequestDe")]
pub struct ServiceRequest {
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

    /// Business identifiers assigned to this order by the requester, performer, or other systems
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

    /// What request fulfills
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// What request replaces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference<crate::r5::resources::ServiceRequest>>,

    /// Composite Request ID
    pub requisition: Option<types::Identifier>,

    /// The current lifecycle status of the order: draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: crate::r5::coded::Coded<crate::r5::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Whether the request is a proposal, plan, directive, order, or similar (proposal | plan | directive | order +)
    pub intent: crate::r5::coded::Coded<crate::r5::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`).
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Broad categorization of the type of service requested, e.g. imaging, laboratory, or counseling
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if service/procedure should not be performed
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`).
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// The specific service, procedure, or product being requested/ordered, coded or referenced
    pub code: Option<types::CodeableReference>,

    /// Additional order information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_detail: Vec<ServiceRequestOrderDetail>,

    /// The `ServiceRequest.quantity[x]` choice element (0..1); see [`ServiceRequestQuantity`].
    #[serde(flatten)]
    pub quantity: Option<ServiceRequestQuantity>,

    /// The individual, group, device, or location the service is ordered for, most often a [`Patient`](crate::r5::resources::patient::Patient)
    pub subject: types::Reference,

    /// What the service request is about, when it is not about the subject of record
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub focus: Vec<types::Reference>,

    /// Encounter in which the request was created
    pub encounter: Option<types::Reference<crate::r5::resources::Encounter>>,

    /// The `ServiceRequest.occurrence[x]` choice element (0..1); see [`ServiceRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ServiceRequestOccurrence>,

    /// The `ServiceRequest.asNeeded[x]` choice element (0..1); see [`ServiceRequestAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<ServiceRequestAsNeeded>,

    /// Date request signed
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Who/what is requesting service
    pub requester: Option<types::Reference>,

    /// Performer role
    pub performer_type: Option<types::CodeableConcept>,

    /// Requested performer
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performer: Vec<types::Reference>,

    /// Requested location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location: Vec<types::CodeableReference>,

    /// Explanation/Justification for procedure or service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Additional clinical information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::CodeableReference>,

    /// Procedure Samples
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference<crate::r5::resources::Specimen>>,

    /// Coded location on Body
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// BodyStructure-based location on the body
    pub body_structure: Option<types::Reference<crate::r5::resources::BodyStructure>>,

    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Patient or consumer-oriented instructions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patient_instruction: Vec<ServiceRequestPatientInstruction>,

    /// Request provenance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference<crate::r5::resources::Provenance>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceRequestDe {
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
    #[serde(default)]
    replaces: Vec<types::Reference<crate::r5::resources::ServiceRequest>>,
    requisition: Option<types::Identifier>,
    status: crate::r5::coded::Coded<crate::r5::codes::RequestStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    intent: crate::r5::coded::Coded<crate::r5::codes::RequestIntent>,
    #[serde(rename = "_intent")]
    intent_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    do_not_perform: Option<types::Boolean>,
    #[serde(rename = "_doNotPerform")]
    do_not_perform_ext: Option<types::Element>,
    code: Option<types::CodeableReference>,
    #[serde(default)]
    order_detail: Vec<ServiceRequestOrderDetail>,
    #[serde(flatten)]
    quantity: crate::r5::choice::Slot<ServiceRequestQuantity>,
    subject: types::Reference,
    #[serde(default)]
    focus: Vec<types::Reference>,
    encounter: Option<types::Reference<crate::r5::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r5::choice::Slot<ServiceRequestOccurrence>,
    #[serde(flatten)]
    as_needed: crate::r5::choice::Slot<ServiceRequestAsNeeded>,
    authored_on: Option<types::DateTime>,
    #[serde(rename = "_authoredOn")]
    authored_on_ext: Option<types::Element>,
    requester: Option<types::Reference>,
    performer_type: Option<types::CodeableConcept>,
    #[serde(default)]
    performer: Vec<types::Reference>,
    #[serde(default)]
    location: Vec<types::CodeableReference>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    #[serde(default)]
    insurance: Vec<types::Reference>,
    #[serde(default)]
    supporting_info: Vec<types::CodeableReference>,
    #[serde(default)]
    specimen: Vec<types::Reference<crate::r5::resources::Specimen>>,
    #[serde(default)]
    body_site: Vec<types::CodeableConcept>,
    body_structure: Option<types::Reference<crate::r5::resources::BodyStructure>>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    patient_instruction: Vec<ServiceRequestPatientInstruction>,
    #[serde(default)]
    relevant_history: Vec<types::Reference<crate::r5::resources::Provenance>>,
}

impl ::core::convert::From<ServiceRequestDe> for ServiceRequest {
    fn from(v: ServiceRequestDe) -> Self {
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
            instantiates_canonical: v.instantiates_canonical,
            instantiates_canonical_ext: v.instantiates_canonical_ext,
            instantiates_uri: v.instantiates_uri,
            instantiates_uri_ext: v.instantiates_uri_ext,
            based_on: v.based_on,
            replaces: v.replaces,
            requisition: v.requisition,
            status: v.status,
            status_ext: v.status_ext,
            intent: v.intent,
            intent_ext: v.intent_ext,
            category: v.category,
            priority: v.priority,
            priority_ext: v.priority_ext,
            do_not_perform: v.do_not_perform,
            do_not_perform_ext: v.do_not_perform_ext,
            code: v.code,
            order_detail: v.order_detail,
            quantity: v.quantity.0,
            subject: v.subject,
            focus: v.focus,
            encounter: v.encounter,
            occurrence: v.occurrence.0,
            as_needed: v.as_needed.0,
            authored_on: v.authored_on,
            authored_on_ext: v.authored_on_ext,
            requester: v.requester,
            performer_type: v.performer_type,
            performer: v.performer,
            location: v.location,
            reason: v.reason,
            insurance: v.insurance,
            supporting_info: v.supporting_info,
            specimen: v.specimen,
            body_site: v.body_site,
            body_structure: v.body_structure,
            note: v.note,
            patient_instruction: v.patient_instruction,
            relevant_history: v.relevant_history,
        }
    }
}

/// Additional order information for a ServiceRequest.
///
/// Captures supplementary details about the order beyond the primary requested
/// code, optionally referencing an external context and carrying a set of coded
/// parameters that further specify the service being requested.
/// # Examples
///
/// ```ignore
/// use fhir::r5::resources::service_request::ServiceRequestOrderDetail;
///
/// let value = ServiceRequestOrderDetail::default();
/// let json = ::serde_json::to_value(&value).unwrap();
/// let back: ServiceRequestOrderDetail = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequestOrderDetail {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The context of the order details by reference
    pub parameter_focus: Option<types::CodeableReference>,

    /// The parameter details for the service being requested
    pub parameter: vec1::Vec1<ServiceRequestOrderDetailParameter>,
}

/// The parameter details for the service being requested.
///
/// Each parameter pairs a coded detail with a typed value, allowing the order to
/// carry structured, machine-processable specifications such as quantities,
/// ranges, or coded qualifiers for the requested service.
/// # Examples
///
/// ```
/// use fhir::r5::resources::service_request::ServiceRequestOrderDetailParameter;
/// use fhir::r5::types;
///
/// let value = ServiceRequestOrderDetailParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ServiceRequestOrderDetailParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ServiceRequestOrderDetailParameterDe")]
pub struct ServiceRequestOrderDetailParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The detail of the order being requested
    pub code: types::CodeableConcept,

    /// The `ServiceRequest.orderDetail.parameter.value[x]` choice element (0..1); see [`ServiceRequestOrderDetailParameterValue`].
    #[serde(flatten)]
    pub value: Option<ServiceRequestOrderDetailParameterValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceRequestOrderDetailParameterDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: types::CodeableConcept,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<ServiceRequestOrderDetailParameterValue>,
}

impl ::core::convert::From<ServiceRequestOrderDetailParameterDe>
    for ServiceRequestOrderDetailParameter
{
    fn from(v: ServiceRequestOrderDetailParameterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            value: v.value.0,
        }
    }
}

/// Patient or consumer-oriented instructions for a ServiceRequest.
///
/// Provides guidance intended for the patient or consumer, expressed either as
/// inline markdown text or as a reference to a document resource carrying the
/// instructional content.
/// # Examples
///
/// ```
/// use fhir::r5::resources::service_request::ServiceRequestPatientInstruction;
/// use fhir::r5::types;
///
/// let value = ServiceRequestPatientInstruction {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: ServiceRequestPatientInstruction = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ServiceRequestPatientInstructionDe")]
pub struct ServiceRequestPatientInstruction {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// The `ServiceRequest.patientInstruction.instruction[x]` choice element (0..1); see [`ServiceRequestPatientInstructionInstruction`].
    #[serde(flatten)]
    pub instruction: Option<ServiceRequestPatientInstructionInstruction>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceRequestPatientInstructionDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(flatten)]
    instruction: crate::r5::choice::Slot<ServiceRequestPatientInstructionInstruction>,
}

impl ::core::convert::From<ServiceRequestPatientInstructionDe>
    for ServiceRequestPatientInstruction
{
    fn from(v: ServiceRequestPatientInstructionDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            instruction: v.instruction.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = ServiceRequest;

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
/// The `ServiceRequest.asNeeded[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `ServiceRequest.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r5::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `ServiceRequest.orderDetail.parameter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestOrderDetailParameterValue {
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRatio` variant.
    #[fhir("valueRatio")]
    Ratio(Box<types::Ratio>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueString` variant.
    #[fhir("valueString")]
    String(crate::r5::choice::Primitive<types::String>),
    /// `valuePeriod` variant.
    #[fhir("valuePeriod")]
    Period(Box<types::Period>),
}

/// The `ServiceRequest.patientInstruction.instruction[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestPatientInstructionInstruction {
    /// `instructionMarkdown` variant.
    #[fhir("instructionMarkdown")]
    Markdown(crate::r5::choice::Primitive<types::Markdown>),
    /// `instructionReference` variant.
    #[fhir("instructionReference")]
    Reference(Box<types::Reference>),
}

/// The `ServiceRequest.quantity[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestQuantity {
    /// `quantityQuantity` variant.
    #[fhir("quantityQuantity")]
    Quantity(Box<types::Quantity>),
    /// `quantityRatio` variant.
    #[fhir("quantityRatio")]
    Ratio(Box<types::Ratio>),
    /// `quantityRange` variant.
    #[fhir("quantityRange")]
    Range(Box<types::Range>),
}
