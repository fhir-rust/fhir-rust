//! DeviceRequest
//!
//! URL: http://hl7.org/fhir/StructureDefinition/DeviceRequest
//!
//!
//!
//! Medical device request
//!
//! FHIR R3: <https://hl7.org/fhir/STU3/>

// The `types` import is unused by a handful of types that have only primitive fields.
#![allow(unused_imports)]

use crate::r3::types;
use ::serde::{Deserialize, Serialize};
use fhir_derive_macros::{Builder, Validate};

/// Base StructureDefinition for DeviceRequest Resource
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device_request::DeviceRequest;
/// use fhir::r3::types;
///
/// let value = DeviceRequest {
///     authored_on: Some(types::DateTime("2019-11-01T09:29:23Z".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `authoredOn` is the name this serializes to on the wire.
/// assert_eq!(json["authoredOn"], ::serde_json::json!("2019-11-01T09:29:23Z"));
///
/// let back: DeviceRequest = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate, Builder)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DeviceRequest {
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

    /// External Request identifier
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<types::Identifier>,

    /// Protocol or definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<types::Reference>,

    /// What request fulfills
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub based_on: Vec<types::Reference>,

    /// What request replaces
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prior_request: Vec<types::Reference>,

    /// Identifier of composite request
    pub group_identifier: Option<types::Identifier>,

    /// draft | active | suspended | completed | entered-in-error | cancelled
    pub status: Option<crate::coded::Coded<crate::r3::codes::RequestStatus>>,
    /// Primitive extension sibling for [`status`](Self::status) (FHIR `_status`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_status")]
    pub status_ext: Option<types::Element>,

    /// proposal | plan | original-order | encoded | reflex-order
    pub intent: types::CodeableConcept,

    /// Indicates how quickly the {{title}} should be addressed with respect to
    /// other requests
    pub priority: Option<crate::coded::Coded<crate::r3::codes::RequestPriority>>,
    /// Primitive extension sibling for [`priority`](Self::priority) (FHIR `_priority`):
    /// carries `id` and/or `extension` for the primitive value.
    #[serde(rename = "_priority")]
    pub priority_ext: Option<types::Element>,

    /// Device requested
    /// The `DeviceRequest.code[x]` choice element (1..1); see [`DeviceRequestCode`]. It is `Option` even though the specification makes it mandatory, because a choice enum has no default.
    #[serde(flatten)]
    pub code: Option<DeviceRequestCode>,

    /// Focus of request
    pub subject: types::Reference,

    /// Encounter or Episode motivating request
    pub context: Option<types::Reference>,

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

    /// Who/what is requesting diagnostics
    pub requester: Option<DeviceRequestRequester>,

    /// Fille role
    pub performer_type: Option<types::CodeableConcept>,

    /// Requested Filler
    pub performer: Option<types::Reference>,

    /// Coded Reason for request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_code: Vec<types::CodeableConcept>,

    /// Linked Reason for request
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reason_reference: Vec<types::Reference>,

    /// Additional clinical information
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_info: Vec<types::Reference>,

    /// Notes or comments
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<types::Annotation>,

    /// Request provenance
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relevant_history: Vec<types::Reference<crate::r3::resources::Provenance>>,
}

/// The individual who initiated the request and has responsibility for its
/// activation.
///
/// # Examples
///
/// ```
/// use fhir::r3::resources::device_request::DeviceRequestRequester;
/// use fhir::r3::types;
///
/// let value = DeviceRequestRequester {
///     id: Some(types::String("abc".to_string())),
///     ..Default::default()
/// };
/// let json = ::serde_json::to_value(&value).unwrap();
/// // `id` is the name this serializes to on the wire.
/// assert_eq!(json["id"], ::serde_json::json!("abc"));
///
/// let back: DeviceRequestRequester = ::serde_json::from_value(json).unwrap();
/// assert_eq!(value, back);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, Validate)]
#[serde(rename_all = "camelCase")]
#[fhir_version("r3")]
pub struct DeviceRequestRequester {
    /// xml:id (or equivalent in JSON)
    pub id: Option<types::String>,

    /// Additional Content defined by implementations
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extension: Vec<types::Extension>,

    /// Extensions that cannot be ignored
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifier_extension: Vec<types::Extension>,

    /// Individual making the request
    pub agent: types::Reference,

    /// Organization agent is acting for
    pub on_behalf_of: Option<types::Reference<crate::r3::resources::Organization>>,
}

/// The `DeviceRequest.code[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceRequestCode {
    /// `codeReference` variant.
    #[fhir("codeReference")]
    Reference(Box<types::Reference>),
    /// `codeCodeableConcept` variant.
    #[fhir("codeCodeableConcept")]
    CodeableConcept(Box<types::CodeableConcept>),
}

/// The `DeviceRequest.occurrence[x]` choice element (see `spec/11-choice-types.md`).
#[derive(Debug, Clone, PartialEq, Eq, fhir_derive_macros::FhirChoice, Validate)]
#[fhir_version("r3")]
#[allow(clippy::large_enum_variant)]
pub enum DeviceRequestOccurrence {
    /// `occurrenceDateTime` variant.
    #[fhir("occurrenceDateTime")]
    DateTime(crate::r3::choice::Primitive<types::DateTime>),
    /// `occurrencePeriod` variant.
    #[fhir("occurrencePeriod")]
    Period(Box<types::Period>),
    /// `occurrenceTiming` variant.
    #[fhir("occurrenceTiming")]
    Timing(Box<types::Timing>),
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
