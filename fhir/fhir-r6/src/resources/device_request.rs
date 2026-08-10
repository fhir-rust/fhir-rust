//! DeviceRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
//!
//! Version: 6.0.0-ballot3
//!
//! Medical device request
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Represents a request a device to be provided to a specific patient. The
/// device may be an implantable device to be subsequently implanted, or an
/// external assistive device, such as a walker, to be delivered and
/// subsequently be used.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_request::DeviceRequest;
/// use fhir::r6::types;
///
/// let value = DeviceRequest {
///     do_not_perform: Some(types::Boolean(true)),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `doNotPerform` is the name this serializes to on the wire.
/// assert_eq!(json["doNotPerform"], ::serde_json::json!(true));
///
/// let back: DeviceRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DeviceRequestDe")]
#[fhir_version("r6")]
pub struct DeviceRequest {
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
    pub contained: Vec<crate::r6::resources::Resource>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// External Request identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Instantiates FHIR protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates_canonical: ::fhir_core::PrimVec<types::Canonical>,
    /// Primitive extension sibling for [`instantiates_canonical`](Self::instantiates_canonical) (FHIR `_instantiatesCanonical`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instantiates_canonical_ext: Vec<Option<types::Element>>,

    /// Instantiates external protocol or definition
    #[serde(default, skip_serializing_if = "::fhir_core::PrimVec::is_empty")]
    pub instantiates_uri: ::fhir_core::PrimVec<types::Uri>,
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
    pub replaces: Vec<types::Reference<crate::r6::resources::DeviceRequest>>,

    /// Identifier of composite request
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | on-hold | entered-in-error | ended | completed |
    /// revoked | unknown
    pub status: Option<crate::coded::Coded<crate::r6::codes::RequestStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | directive | order | original-order | reflex-order |
    /// filler-order | instance-order | option
    pub intent: crate::coded::Coded<crate::r6::codes::RequestIntent>,
    /// Primitive extension sibling for [`intent`](Self::intent) (FHIR `_intent`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_intent")]
    pub intent_ext: Option<types::Element>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// True if the request is to stop or not to start using the device
    pub do_not_perform: Option<types::Boolean>,
    /// Primitive extension sibling for [`do_not_perform`](Self::do_not_perform) (FHIR `_doNotPerform`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_doNotPerform")]
    pub do_not_perform_ext: Option<types::Element>,

    /// Device requested
    pub code: types::CodeableReference,

    /// Quantity of devices to supply
    pub quantity: Option<types::Integer>,
    /// Primitive extension sibling for [`quantity`](Self::quantity) (FHIR `_quantity`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_quantity")]
    pub quantity_ext: Option<types::Element>,

    /// Device details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<DeviceRequestParameter>,

    /// Focus of request
    pub subject: types::Reference,

    /// Encounter motivating request
    pub encounter: Option<types::Reference<crate::r6::resources::Encounter>>,

    /// Desired time or schedule for use
    /// The `DeviceRequest.occurrence[x]` choice element (0..1); see [`DeviceRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<DeviceRequestOccurrence>,

    /// When recorded
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Who/what submitted the device request
    pub requester: Option<types::Reference>,

    /// Requested Filler
    pub performer: Option<types::CodeableReference>,

    /// Coded/Linked Reason for request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// PRN status of request
    pub as_needed: Option<types::Boolean>,
    /// Primitive extension sibling for [`as_needed`](Self::as_needed) (FHIR `_asNeeded`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_asNeeded")]
    pub as_needed_ext: Option<types::Element>,

    /// Device usage reason
    pub as_needed_for: Option<types::CodeableConcept>,

    /// Associated insurance coverage
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub insurance: Vec<types::Reference>,

    /// Additional clinical information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Notes or comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Request provenance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference<crate::r6::resources::Provenance>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceRequestDe {
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
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    #[serde(default)]
    instantiates_canonical: ::fhir_core::PrimVec<types::Canonical>,
    #[serde(rename = "_instantiatesCanonical")]
    #[serde(default)]
    instantiates_canonical_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    instantiates_uri: ::fhir_core::PrimVec<types::Uri>,
    #[serde(rename = "_instantiatesUri")]
    #[serde(default)]
    instantiates_uri_ext: Vec<Option<types::Element>>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    #[serde(default)]
    replaces: Vec<types::Reference<crate::r6::resources::DeviceRequest>>,
    group_identifier: Option<types::Identifier>,
    status: Option<crate::coded::Coded<crate::r6::codes::RequestStatus>>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    intent: crate::coded::Coded<crate::r6::codes::RequestIntent>,
    #[serde(rename = "_intent")]
    intent_ext: Option<types::Element>,
    priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    do_not_perform: Option<types::Boolean>,
    #[serde(rename = "_doNotPerform")]
    do_not_perform_ext: Option<types::Element>,
    code: types::CodeableReference,
    quantity: Option<types::Integer>,
    #[serde(rename = "_quantity")]
    quantity_ext: Option<types::Element>,
    #[serde(default)]
    parameter: Vec<DeviceRequestParameter>,
    subject: types::Reference,
    encounter: Option<types::Reference<crate::r6::resources::Encounter>>,
    #[serde(flatten)]
    occurrence: crate::r6::choice::Slot<DeviceRequestOccurrence>,
    authored_on: Option<types::DateTime>,
    #[serde(rename = "_authoredOn")]
    authored_on_ext: Option<types::Element>,
    requester: Option<types::Reference>,
    performer: Option<types::CodeableReference>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    as_needed: Option<types::Boolean>,
    #[serde(rename = "_asNeeded")]
    as_needed_ext: Option<types::Element>,
    as_needed_for: Option<types::CodeableConcept>,
    #[serde(default)]
    insurance: Vec<types::Reference>,
    #[serde(default)]
    supporting_info: Vec<types::Reference>,
    #[serde(default)]
    note: Vec<types::Annotation>,
    #[serde(default)]
    relevant_history: Vec<types::Reference<crate::r6::resources::Provenance>>,
}

impl ::core::convert::From<DeviceRequestDe> for DeviceRequest {
    fn from(v: DeviceRequestDe) -> Self {
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
            group_identifier: v.group_identifier,
            status: v.status,
            status_ext: v.status_ext,
            intent: v.intent,
            intent_ext: v.intent_ext,
            priority: v.priority,
            priority_ext: v.priority_ext,
            do_not_perform: v.do_not_perform,
            do_not_perform_ext: v.do_not_perform_ext,
            code: v.code,
            quantity: v.quantity,
            quantity_ext: v.quantity_ext,
            parameter: v.parameter,
            subject: v.subject,
            encounter: v.encounter,
            occurrence: v.occurrence.0,
            authored_on: v.authored_on,
            authored_on_ext: v.authored_on_ext,
            requester: v.requester,
            performer: v.performer,
            reason: v.reason,
            as_needed: v.as_needed,
            as_needed_ext: v.as_needed_ext,
            as_needed_for: v.as_needed_for,
            insurance: v.insurance,
            supporting_info: v.supporting_info,
            note: v.note,
            relevant_history: v.relevant_history,
        }
    }
}

/// Specific parameters for the ordered item. For example, the prism value for
/// lenses.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::device_request::DeviceRequestParameter;
/// use fhir::r6::types;
///
/// let value = DeviceRequestParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceRequestParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "DeviceRequestParameterDe")]
#[fhir_version("r6")]
pub struct DeviceRequestParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Device detail
    pub code: Option<types::CodeableConcept>,

    /// Value of detail
    /// The `DeviceRequest.parameter.value[x]` choice element (0..1); see [`DeviceRequestParameterValue`].
    #[serde(flatten)]
    pub value: Option<DeviceRequestParameterValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeviceRequestParameterDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: Option<types::CodeableConcept>,
    #[serde(flatten)]
    value: crate::r6::choice::Slot<DeviceRequestParameterValue>,
}

impl ::core::convert::From<DeviceRequestParameterDe> for DeviceRequestParameter {
    fn from(v: DeviceRequestParameterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            value: v.value.0,
        }
    }
}

/// The `DeviceRequest.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r6::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
}

/// The `DeviceRequest.parameter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceRequestParameterValue {
    /// `valueCodeableConcept` variant.
    #[fhir("valueCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
    /// `valueQuantity` variant.
    #[fhir("valueQuantity")]
    Quantity(Box<types::Quantity>),
    /// `valueRange` variant.
    #[fhir("valueRange")]
    Range(Box<types::Range>),
    /// `valueBoolean` variant.
    #[fhir("valueBoolean")]
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DeviceRequest;

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
