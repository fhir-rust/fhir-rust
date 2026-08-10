//! SupplyRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/SupplyRequest
//!
//! Version: 6.0.0-ballot3
//!
//! Request for a medication, substance or device
//!
//! FHIR R6: <https://hl7.org/fhir/6.0.0-ballot3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r6::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// A record of a non-patient specific request for a medication, substance,
/// device, certain types of biologically derived product, and nutrition
/// product used in the healthcare setting.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::supply_request::SupplyRequest;
/// use fhir::r6::types;
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
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[serde(from = "SupplyRequestDe")]
#[fhir_version("r6")]
pub struct SupplyRequest {
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

    /// Business Identifier for SupplyRequest
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// draft | active | suspended +
    pub status: Option<crate::coded::Coded<crate::r6::codes::SupplyrequestStatus>>,
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

    /// What other request is fulfilled by this supply request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// The kind of supply (central, non-stock, etc.)
    pub category: Option<types::CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// The patient for who the supply request is for
    pub deliver_for: Option<types::Reference<crate::r6::resources::Patient>>,

    /// Medication, Substance, or Device requested to be supplied
    pub item: types::CodeableReference,

    /// The requested amount of the item indicated
    pub quantity: types::Quantity,

    /// Ordered item details
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter: Vec<SupplyRequestParameter>,

    /// When the request should be fulfilled
    /// The `SupplyRequest.occurrence[x]` choice element (0..1); see [`SupplyRequestOccurrence`].
    #[serde(flatten)]
    pub occurrence: Option<SupplyRequestOccurrence>,

    /// When the request was made
    pub authored_on: Option<types::DateTime>,
    /// Primitive extension sibling for [`authored_on`](Self::authored_on) (FHIR `_authoredOn`):
    /// carries `id` and/or `extension` for the primitive value.
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
    contained: Vec<crate::r6::resources::Resource>,
    #[serde(default)]
    extension: Vec<types::Extension>,
    #[serde(default)]
    modifier_extension: Vec<types::Extension>,
    #[serde(default)]
    identifier: Vec<types::Identifier>,
    status: Option<crate::coded::Coded<crate::r6::codes::SupplyrequestStatus>>,
    #[serde(rename = "_status")]
    status_ext: Option<types::Element>,
    intent: crate::coded::Coded<crate::r6::codes::RequestIntent>,
    #[serde(rename = "_intent")]
    intent_ext: Option<types::Element>,
    #[serde(default)]
    based_on: Vec<types::Reference>,
    category: Option<types::CodeableConcept>,
    priority: Option<crate::coded::Coded<crate::r6::codes::RequestPriority>>,
    #[serde(rename = "_priority")]
    priority_ext: Option<types::Element>,
    deliver_for: Option<types::Reference<crate::r6::resources::Patient>>,
    item: types::CodeableReference,
    quantity: types::Quantity,
    #[serde(default)]
    parameter: Vec<SupplyRequestParameter>,
    #[serde(flatten)]
    occurrence: crate::r6::choice::Slot<SupplyRequestOccurrence>,
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
            intent: v.intent,
            intent_ext: v.intent_ext,
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

/// Specific parameters for the ordered item. For example, the size of the
/// indicated item.
///
/// # Examples
///
/// ```
/// use fhir::r6::resources::supply_request::SupplyRequestParameter;
/// use fhir::r6::types;
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
#[fhir_version("r6")]
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

    /// Value of detail
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
    value: crate::r6::choice::Slot<SupplyRequestParameterValue>,
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

/// The `SupplyRequest.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
#[allow(clippy::large_enum_variant)]
pub enum SupplyRequestOccurrence {
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

/// The `SupplyRequest.parameter.value[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r6")]
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
    Boolean(crate::r6::choice::Primitive<types::Boolean>),
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
