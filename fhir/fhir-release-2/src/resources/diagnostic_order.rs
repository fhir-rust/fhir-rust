//! DiagnosticOrder
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DiagnosticOrder
//!
//!
//!
//! A request for a diagnostic service
//!
//! FHIR R2: <https://hl7.org/fhir/DSTU2/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r2::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DiagnosticOrder Resource
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::diagnostic_order::DiagnosticOrder;
/// use fhir::r2::types;
///
/// let value = DiagnosticOrder {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: DiagnosticOrder = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DiagnosticOrder {
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
    pub contained: Vec<::serde_json::Value>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Who and/or what test is about
    pub subject: types::Reference,

    /// Who ordered the test
    pub orderer: Option<types::Reference>,

    /// Identifiers assigned to this order
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// The encounter that this diagnostic order is associated with
    pub encounter: Option<types::Reference>,

    /// Explanation/Justification for test
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason: Vec<types::CodeableConcept>,

    /// Additional clinical information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_information: Vec<types::Reference>,

    /// If the whole order relates to specific specimens
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference>,

    /// proposed | draft | planned | requested | received | accepted |
    /// in-progress | review | completed | cancelled | suspended | rejected |
    /// failed
    pub status: Option<crate::coded::Coded<crate::r2::codes::DiagnosticOrderStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// routine | urgent | stat | asap
    pub priority: Option<crate::coded::Coded<crate::r2::codes::DiagnosticOrderPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// A list of events of interest in the lifecycle
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<DiagnosticOrderEvent>,

    /// The items the orderer requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<DiagnosticOrderItem>,

    /// Other notes and comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,
}

/// A summary of the events of interest that have occurred as the request is
/// processed; e.g. when the order was made, various processing steps
/// (specimens received), when it was completed.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::diagnostic_order::DiagnosticOrderEvent;
/// use fhir::r2::types;
///
/// let value = DiagnosticOrderEvent {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: DiagnosticOrderEvent = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DiagnosticOrderEvent {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// proposed | draft | planned | requested | received | accepted |
    /// in-progress | review | completed | cancelled | suspended | rejected |
    /// failed
    pub status: crate::coded::Coded<crate::r2::codes::DiagnosticOrderStatus>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// More information about the event and its context
    pub description: Option<types::CodeableConcept>,

    /// The date at which the event happened
    pub date_time: types::DateTime,
    /// Primitive extension sibling for [`date_time`](Self::date_time) (FHIR `_dateTime`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_dateTime")]
    pub date_time_ext: Option<types::Element>,

    /// Who recorded or did this
    pub actor: Option<types::Reference>,
}

/// The specific diagnostic investigations that are requested as part of this
/// request. Sometimes, there can only be one item per request, but in most
/// contexts, more than one investigation can be requested.
///
/// # Examples
///
/// ```
/// use fhir::r2::resources::diagnostic_order::DiagnosticOrderItem;
/// use fhir::r2::types;
///
/// let value = DiagnosticOrderItem {
///     id: Some(types::Id("pat-1".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("pat-1"));
///
/// let back: DiagnosticOrderItem = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r2")]
pub struct DiagnosticOrderItem {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::Id>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Code to indicate the item (test or panel) being ordered
    pub code: types::CodeableConcept,

    /// If this item relates to specific specimens
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specimen: Vec<types::Reference>,

    /// Location of requested test (if applicable)
    pub body_site: Option<types::CodeableConcept>,

    /// proposed | draft | planned | requested | received | accepted |
    /// in-progress | review | completed | cancelled | suspended | rejected |
    /// failed
    pub status: Option<crate::coded::Coded<crate::r2::codes::DiagnosticOrderStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// Events specific to this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<DiagnosticOrderEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    type T = DiagnosticOrder;

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
