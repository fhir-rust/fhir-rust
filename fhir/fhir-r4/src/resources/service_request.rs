//! ServiceRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/ServiceRequest
//!
//! Version: 4.0.1
//!
//! A request for a service to be performed
//!
//! FHIR R4: <https://hl7.org/fhir/R4/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r4::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a request for service such as diagnostic investigations,
/// treatments, or operations to be performed.
///
/// # Examples
///
/// ```
/// use fhir::r4::resources::service_request::ServiceRequest;
/// use fhir::r4::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "ServiceRequestDe")]
#[fhir_version("r4")]
pub struct ServiceRequest {
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
    pub contained: Vec<crate::r4::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Identifiers assigned to this order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

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

    /// What request replaces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replaces: Vec<types::Reference<crate::r4::resources::ServiceRequest>>,

    /// Composite Request ID
    pub requisition: Option<types::Identifier>,

    /// draft | active | on-hold | revoked | completed | entered-in-error |
    /// unknown
    pub status: crate::coded::Coded<crate::r4::codes::RequestStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r4::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// Classification of service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r4::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if service/procedure should not be performed
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// What is being requested/ordered
    pub code: Option<types::CodeableConcept>,

    /// Additional order information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_detail: Vec<types::CodeableConcept>,

    /// Service amount
    /// The `ServiceRequest.quantity[x]` choice element (0..1); see [`ServiceRequestQuantity`].
    #[serde(flatten)]
    pub quantity: Option<ServiceRequestQuantity>,

    /// Individual or Entity the service is ordered for
    pub subject: types::Reference,

    /// Encounter in which the request was created
    pub encounter: Option<types::Reference<crate::r4::resources::Encounter>>,

    /// When service should occur
    /// The `ServiceRequest.occurrence[x]` choice element (0..1); see [`ServiceRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<ServiceRequestOccurrence>,

    /// Preconditions for service
    /// The `ServiceRequest.asNeeded[x]` choice element (0..1); see [`ServiceRequestAsNeeded`].
    #[serde(flatten)]
    pub as_needed: Option<ServiceRequestAsNeeded>,

    /// Date request signed
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
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
    pub location_code: Vec<types::CodeableConcept>,

    /// Requested location
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub location_reference: Vec<types::Reference<crate::r4::resources::Location>>,

    /// Explanation/Justification for procedure or service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Explanation/Justification for service or service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Additional clinical information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Procedure Samples
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference<crate::r4::resources::Specimen>>,

    /// Location on Body
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body_site: Vec<types::CodeableConcept>,

    /// Comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Patient or consumer-oriented instructions
    pub patient_instruction: Option<types::String>,
    /// Primitive extension sibling for [`patient_instruction`](Self::patient_instruction) (FHIR `_patientInstruction`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_patientInstruction")]
    pub patient_instruction_ext: Option<types::Element>,

    /// Request provenance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference<crate::r4::resources::Provenance>>,
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
    contained: Vec<crate::r4::resources::Resource>,
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
    replaces: Vec<types::Reference<crate::r4::resources::ServiceRequest>>,
    requisition: Option<types::Identifier>,
    status: crate::coded::Coded<crate::r4::codes::RequestStatus>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    intent: crate::coded::Coded<crate::r4::codes::RequestIntent>,
    #[serde(rename = "_intent")]
    intent_ext: Option<types::Element>,
    #[serde(default)]
    category: Vec<types::CodeableConcept>,
    priority: Option<crate::coded::Coded<crate::r4::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    do_not_perform: Option<types::Boolean>,
    #[serde(rename = "_doNotPerform")]
    do_not_perform_ext: Option<types::Element>,
    code: Option<types::CodeableConcept>,
    #[serde(default)]
    order_detail: Vec<types::CodeableConcept>,
    #[serde(flatten)]
    quantity: crate::r4::choice::Slot<ServiceRequestQuantity>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r4::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r4::choice::Slot<ServiceRequestOccurrence>,
    #[serde(flatten)]
    as_needed: crate::r4::choice::Slot<ServiceRequestAsNeeded>,
    authored_on: Option<types::DateTime>,
    #[serde(rename = "_authoredOn")]
    authored_on_ext: Option<types::Element>,
    requester: Option<types::Reference>,
    performer_type: Option<types::CodeableConcept>,
    #[serde(default)]
    performer: Vec<types::Reference>,
    #[serde(default)]
    location_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    location_reference: Vec<types::Reference<crate::r4::resources::Location>>,
    #[serde(default)]
    reason_code: Vec<types::CodeableConcept>,
    #[serde(default)]
    reason_reference: Vec<types::Reference>,
    #[serde(default)]
    insurance: Vec<types::Reference>,
    #[serde(default)]
    supporting_info: Vec<types::Reference>,
    #[serde(default)]
    specimen: Vec<types::Reference<crate::r4::resources::Specimen>>,
    #[serde(default)]
    body_site: Vec<types::CodeableConcept>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    patient_instruction: Option<types::String>,
    #[serde(rename = "_patientInstruction")]
    patient_instruction_ext: Option<types::Element>,
    #[serde(default)]
    relevant_history: Vec<types::Reference<crate::r4::resources::Provenance>>,
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
            encounter: v.encounter,
            occurrence: v.occurrence.0,
            as_needed: v.as_needed.0,
            authored_on: v.authored_on,
            authored_on_ext: v.authored_on_ext,
            requester: v.requester,
            performer_type: v.performer_type,
            performer: v.performer,
            location_code: v.location_code,
            location_reference: v.location_reference,
            reason_code: v.reason_code,
            reason_reference: v.reason_reference,
            insurance: v.insurance,
            supporting_info: v.supporting_info,
            specimen: v.specimen,
            body_site: v.body_site,
            note: v.note,
            patient_instruction: v.patient_instruction,
            patient_instruction_ext: v.patient_instruction_ext,
            relevant_history: v.relevant_history,
        }
    }
}

/// The `ServiceRequest.quantity[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
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

/// The `ServiceRequest.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r4::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `ServiceRequest.asNeeded[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r4")]
#[allow(clippy::large_enum_variant)]
pub enum ServiceRequestAsNeeded {
    /// `asNeededBoolean` variant.
    #[fhir("asNeededBoolean")]
    Boolean(crate::r4::choice::Primitive<types::Boolean>),
    /// `asNeededCodeableConcept` variant.
    #[fhir("asNeededCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
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
