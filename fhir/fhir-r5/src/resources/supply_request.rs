//! SupplyRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SupplyRequest
//!
//! Version: 5.0.0
//!
//! SupplyRequest Resource: A record of a non-patient specific request for a medication, substance, device, certain types of biologically derived product, and nutrition product used in the healthcare setting.
//!
//! FHIR: <https://build.fhir.org/>
//!
//! UML: <https://build.fhir.org/uml.html>

// Allow unused crate::r5::types as types;
#![allow(unused_imports)]

use crate::r5::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::Validate;

/// A record of a request for a medication, substance, device, or nutrition
/// product used in the healthcare setting.
///
/// The SupplyRequest resource captures a non-patient-specific request to
/// obtain supplies such as medications, substances, devices, biologically
/// derived products, and nutrition products. It records what item is being
/// requested, in what quantity, by whom, and for what reason, along with
/// delivery source and destination information. In FHIR R5 it supports supply
/// chain and inventory workflows that are distinct from patient-specific
/// medication or device requests.
///
/// This resource is typically used by pharmacy, materials management, and
/// central/ward stock replenishment systems to order or reorder items that
/// keep a healthcare setting supplied, rather than to fulfill an individual
/// clinical order for a specific patient encounter. A `SupplyRequest` may
/// still reference a patient (via `deliver_for`) when the supply is intended
/// for that individual's care, and it can track the lifecycle of the request
/// through its `status` from creation through to completion or cancellation.
/// The requested item is described using a `CodeableReference`, allowing
/// either a coded value or a reference to a more detailed resource (such as
/// a medication or device definition), and the `parameter` field allows
/// further characteristics of the item to be specified.
///
/// # See also
///
/// - [`CodeableConcept`](crate::r5::types::CodeableConcept) — used for the
///   request `category`, and via `SupplyRequestParameter.code`.
/// - `SupplyDelivery` — the resource that typically records fulfillment of a
///   `SupplyRequest`.
/// - [`Patient`](crate::r5::resources::patient::Patient) — may be referenced
///   by `deliver_for` when the supply is intended for a specific individual.
///
/// # Examples
///
/// ```
/// use fhir::r5::resources::supply_request::SupplyRequest;
/// use fhir::r5::types;
///
/// let value = SupplyRequest {
///     authored_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authoredOn` is the name this serializes to on the wire.
/// assert_eq!(json["authoredOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: SupplyRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SupplyRequestDe")]
pub struct SupplyRequest {
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

    /// Business Identifier for SupplyRequest
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Status of the request: draft | active | suspended | cancelled | completed | entered-in-error | unknown
    pub status: Option<crate::r5::coded::Coded<crate::r5::codes::SupplyrequestStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`).
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// What other request is fulfilled by this supply request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The kind of supply (central, non-stock, etc.)
    pub category: Option<types::CodeableConcept>,

    /// Indicates how quickly this SupplyRequest should be addressed: routine | urgent | asap | stat
    pub priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`).
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// The patient, or group of patients, for whom the supply request is intended
    pub deliver_for: Option<types::Reference<crate::r5::resources::Patient>>,

    /// The item being requested, as a coded value or a reference to a resource such as a medication or device
    pub item: types::CodeableReference,

    /// The requested amount of the item indicated
    pub quantity: types::Quantity,

    /// Ordered item details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<SupplyRequestParameter>,

    /// The `SupplyRequest.occurrence[x]` choice element (0..1); see [`SupplyRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<SupplyRequestOccurrence>,

    /// When the request was made
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`).
    #[serde(rename = "_authoredOn")]
    pub authored_on_ext: Option<types::Element>,

    /// Individual making the request
    pub requester: Option<types::Reference>,

    /// Who is intended to fulfill the request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supplier: Vec<types::Reference>,

    /// The reason why the supply item was requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableReference>,

    /// The origin of the supply
    pub deliver_from: Option<types::Reference>,

    /// The destination of the supply
    pub deliver_to: Option<types::Reference>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SupplyRequestDe {
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
    status: Option<crate::r5::coded::Coded<crate::r5::codes::SupplyrequestStatus>>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    category: Option<types::CodeableConcept>,
    priority: Option<crate::r5::coded::Coded<crate::r5::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    deliver_for: Option<types::Reference<crate::r5::resources::Patient>>,
    item: types::CodeableReference,
    quantity: types::Quantity,
    #[serde(default)]
    parameter: Vec<SupplyRequestParameter>,
    #[serde(flatten)]
    occurrence: crate::r5::choice::Slot<SupplyRequestOccurrence>,
    authored_on: Option<types::DateTime>,
    #[serde(rename = "_authoredOn")]
    authored_on_ext: Option<types::Element>,
    requester: Option<types::Reference>,
    #[serde(default)]
    supplier: Vec<types::Reference>,
    #[serde(default)]
    reason: Vec<types::CodeableReference>,
    deliver_from: Option<types::Reference>,
    deliver_to: Option<types::Reference>,
}

impl ::core::convert::From<SupplyRequestDe> for SupplyRequest {
    fn from(v: SupplyRequestDe) -> Self {
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
            based_on: v.based_on,
            category: v.category,
            priority: v.priority,
            priority_ext: v.priority_ext,
            deliver_for: v.deliver_for,
            item: v.item,
            quantity: v.quantity,
            parameter: v.parameter,
            occurrence: v.occurrence.0,
            authored_on: v.authored_on,
            authored_on_ext: v.authored_on_ext,
            requester: v.requester,
            supplier: v.supplier,
            reason: v.reason,
            deliver_from: v.deliver_from,
            deliver_to: v.deliver_to,
        }
    }
}

/// Ordered item details.
///
/// The SupplyRequestParameter backbone element specifies additional
/// characteristics of the requested item, such as a coded detail and its
/// associated value expressed as a codeable concept, quantity, range, or
/// boolean.
/// # Examples
///
/// ```
/// use fhir::r5::resources::supply_request::SupplyRequestParameter;
/// use fhir::r5::types;
///
/// let value = SupplyRequestParameter {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: SupplyRequestParameter = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SupplyRequestParameterDe")]
pub struct SupplyRequestParameter {
    /// Unique id for inter-element referencing
    pub id: Option<types::String>,

    /// Additional content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Item detail
    pub code: Option<types::CodeableConcept>,

    /// The `SupplyRequest.parameter.value[x]` choice element (0..1); see [`SupplyRequestParameterValue`].
    #[serde(flatten)]
    pub value: Option<SupplyRequestParameterValue>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SupplyRequestParameterDe {
    id: Option<types::String>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    code: Option<types::CodeableConcept>,
    #[serde(flatten)]
    value: crate::r5::choice::Slot<SupplyRequestParameterValue>,
}

impl ::core::convert::From<SupplyRequestParameterDe> for SupplyRequestParameter {
    fn from(v: SupplyRequestParameterDe) -> Self {
        Self {
            id: v.id,
            extension: v.extension,
            modifier_extension: v.modifier_extension,
            code: v.code,
            value: v.value.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = SupplyRequest;

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
/// The `SupplyRequest.occurrence[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SupplyRequestOccurrence {
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

/// The `SupplyRequest.parameter.value[x]` choice element (see spec/11-choice-types.md).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[allow(clippy::large_enum_variant)]
pub enum SupplyRequestParameterValue {
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
    Boolean(crate::r5::choice::Primitive<types::Boolean>),
}
